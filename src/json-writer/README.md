# Notes & TODOs

- [x] Support null
- [x] Support lit bools
- [x] Support lit strings (maybe not specification-compliant)
- [x] Support lit numbers (not fully specification-compliant, but works with rust ints and floats)
- [x] Support lit arrays
- [x] Support lit objects
- [ ] Support non-literals
    - [ ] options (map to null?)
    - [ ] bools
    - [ ] strings
    - [ ] numbers
    - [ ] arrays
    - [ ] objects
- [ ] Composability
- [ ] Works on types more general than `&mut Vec<u8>`
- [ ] Better optimizations on mostly-literal objects

## Non-literal support

We want to be able to do this
```rust
let mut buf = Vec::new();
let value = 5;

json!(buf, {
    number: value
})
```

What should the code gen be? First of all, let's to do *this*:
```rust
let mut buf = Vec::new();
let value = 5;

json!(buf, value)
```

After all, `5` is a fine json value. How about this?
```rust
let mut buf = Vec::new();
let value = 5;

buf.write_all(value.to_string().as_bytes()).unwrap();
```

The big idea is that whatever `value` is, we need to be able to turn it into
bytes. If we can do that, we're in business.

Actually that's not quite right, what about this?
```rust
let mut buf = Vec::new();
let value = b"boo";

json!(buf, {
    monster: value
})
```
Notice that `value` is not quoted, so it's *not* a string. Looks like we need
type checking.


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
