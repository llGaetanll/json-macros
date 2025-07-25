# TODO

## Basics
- [x] Support null
- [x] Support lit bools
- [x] Support lit strings (maybe not specification-compliant)
- [x] Support lit numbers (not fully specification-compliant, but works with rust ints and floats)
- [x] Support lit arrays
- [x] Support lit objects
- [x] Support non-literals
    - [x] nulls (`Option<S: Serialize>`)
    - [x] bools (`bool`)
    - [x] strings (string-like: `String`, `&str`, `Cow<str>`, `Box<str>`, ...)
    - [x] numbers (All rust number types supported)
    - [x] arrays (`Vec<S>`, `&[S]`, `Box<[S]>`, ...)
    - [x] objects (`HashMap<(string-like), S>`, `BTreeMap<(string-like), S>`)
- [x] Support dyn object value contraction
- [x] Composability
- [ ] Add derive macro for `Serialize`
- [x] Static AST optimization
      i.e. If the object has a lot of static fields in a row, just push a
      massive byte slice to the buffer instead of calling `buf.push` a
      million times
- [ ] Dynamic key support

## Reach
- [ ] Writes to types more general than `&mut Vec<u8>`
- [ ] Size hints on values?
      ```rust
      let mut buf = Vec::new();

      let value = // some dynamic value here
      json!(buf, {
          name: value in 12..15 // meaning we expect this value to take between 12 and 15 bytes
      })
      ```

# Feature Notes

## Composability
We should be able to do this?
```rust
let michael = json!(buf, {
    name: "Michael"
});

let outer = json!(buf, {
    person: michael, // Here, michael is a value that is inserted in place
    foo: "bar"
});
```
What should the codegen be?

### Option 1: Return Closures

This
```rust
let real = json!(buf, true);

let verdict = json!(buf, {
    verdict: real
});
```

Turns into
```rust
let real = |buf: &mut Vec<u8>| {
    buf.push(b"true");
};

let verdict = |buf: &mut Vec<u8>| {
    buf.push(b'{');

    buf.push(b'"');
    buf.extend_from_slice("verdict".as_bytes());
    buf.push(b'"');

    buf.push(b':');

    real(buf);

    buf.push(b'}');
}
```

The idea is that `JsonValue::Dyn`s should generate as functions, so that they
are only executed when they are called.

### Option 2: Two Macros and Return `JsonValue`

Have another macro
```rust
let value = json!(true); // no `buf` parameter

let verdict = json!({
    verdict: value
});
```

That expands to this
```rust
let value = JsonValue::Bool(true);
let verdict = JsonValue::Object(
    vec![
        (String::from("verdict"), value)
    ]);
```

So this macro only returns a `JsonValue`, then we could just impl `Serialize` on
`JsonValue` directly.

Then the other macro
```rust
write_json!(buf, true);
```

works just like `json!`, but calls `.serialize` on it
```rust
JsonValue::Bool(true).serialize(&mut buf);
```

### Allocation Caveat
This is all very elegant but it does come at the cost of an allocation, for
example this code
```rust
let value = json!(true);
let verdict = write_json!({
    verdict: value
});
```

expands to this at compile time
```rust
let value = JsonValue::Bool(true);
let verdict = JsonValue::Object(
    vec![
        (String::from("verdict"), value)
    ]).serialize(&mut buf);
```

And recall that you can't use a slice in `JsonValue::Object`, because the size of
```rust
JsonValue::Object([(&'static str, JsonValue)]);
```
is not known at compile time.

### Option 3: Two Macros and Return Closures

The previous option allocates, unfortunate. Now this code:
```rust
let value = json!(true);
let verdict = write_json!(buf, {
    verdict: value
});
```

Expands to
```rust
let real = |buf: &mut Vec<u8>| {
    buf.push(b"true");
};

let verdict = |buf: &mut Vec<u8>| {
    buf.push(b'{');

    buf.push(b'"');
    buf.extend_from_slice("verdict".as_bytes());
    buf.push(b'"');

    buf.push(b':');

    real(buf);

    buf.push(b'}');
}

verdict(&mut buf);
```

This code does not allocate, and the closures are optimized away.

#### Problem: What *is* a `JsonValue::Dyn`?
The code above is great and all, but it does break this
```rust
let value = true;
let verdict = write_json!(buf, {
    verdict: value
});
```

Since we expect `value` to be a function, our macro now expands to
```rust
let real = true;

let verdict = |buf: &mut Vec<u8>| {
    buf.push(b'{');

    buf.push(b'"');
    buf.extend_from_slice("verdict".as_bytes());
    buf.push(b'"');

    buf.push(b':');

    real(buf);

    buf.push(b'}');
}
```

Which of course makes no sense, since `real` is not a function, its a `bool`.
What we can do instead is `impl Serialize for F: Fn(&mut Vec<u8>)`. By doing
this, we can instead generate this code:
```rust
let real = // Just something here that impls Serialize

let verdict = |buf: &mut Vec<u8>| {
    buf.push(b'{');

    buf.push(b'"');
    buf.extend_from_slice("verdict".as_bytes());
    buf.push(b'"');

    buf.push(b':');

    real.serialize(&mut buf);

    buf.push(b'}');
}
```
