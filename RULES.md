# Macro

1. Favors composition
2. ZERO cost
3. No allocations

## Strict
1. Order matters
2. All keys must be listed

```rust
let foo = json::strict!(buf, {
    "foo": "bar"
})?;
```

Expands to

```rust
r.object_start()?;
{
    r.string(b"foo")?;
    r.string(b"bar")?;
}
r.object_end()?;
```

## Loose
1. Order doesn't matter
2. All keys must be listed

```rust
let foo = json::loose!(buf, {
    "foo": 1
    "bar": 2,
    "baz": 3
})?;
```

expands to:

```rust
r.object_start()?;
{
    while r.more() {
        match r.string_bytes()? {
            b"foo" => {
                r.number(b"1")?;
            }
            b"bar" => {
                r.number(b"2")?;
            }
            b"baz" => {
                r.number(b"3")?;
            }
            other => err!("Unrecognized key: \"{other}\"")
        }
    }
}
r.object_end()?;
```

## Loosest
1. Order doesn't matter
2. Keys can be ommitted

```rust
let foo = json::loose!(buf, {
    "foo": 1
    "baz": 3
})?;
```

expands to:

```rust
r.object_start()?;
{
    while r.more() {
        match r.string_bytes()? {
            b"foo" => {
                r.number(b"1")?;
            }
            b"baz" => {
                r.number(b"3")?;
            }
            _ => r.skip()?
        }
    }
}
r.object_end()?;
```

TODO: maybe ordered and optional keys is more fitting? figure out api

# Composition
Always factors out the maximum

```rust
let exec_report = json!(buf, {
    "e": "executionReport",
})?;

let bal_update = json!(buf, {
    "e": "balanceUpdate"
})?;

let my_object = json!(buf, {
    "data": exec_report | bal_update
})?;
```

simplifies to

```rust
let my_object = json!(buf, {
    "data": {
        "e": "executionReport" | "balanceUpdate"
    }
})?;
```

which expands to

```rust
r.object_start()?;
{
    r.string(b"data")?;
    r.object_start()?;
    {
        r.string(b"e")?;
        
        match r.string_bytes()?.as_ref() {
            b"executionReport" | "balanceUpdate" => {},
            other => err!("Unrecognized value: \"{other}\"")
        }
    }
    r.object_end()?;
}
r.object_end()?;
```

# Examples

```rust
let o1 = json::loose!({
    "name": "Max",
    "age": 5,
    "height": 10,
})?;

let o2 = json::loose!({
    "full_name": "Max Miller",
    "height": 10,
})?;

let my_list = json::loose!({
    "data": [ o1 | o2 ]
})
```

```rust
let example = json!({
    "name": |name| { /* ... */; name },
    "age": |age| { /* ... */; age },
})?;
```

expands to

```rust
fn parse_name(r: &mut JsonReader) -> Result<_, Error> {
    r.object_start()?;
    {
        r.string(b"name")?;
        let name = r.string_bytes()?;
        let name = /* ... */;

        r.string(b"age")?;
        let age = r.string_bytes()?;
        let age = /* ... */;
    }
    r.object_end()?;
}
```

```rust
let arr = json!({
    "data": [example]
})
```

expands to

```rust
r.array_start()?;
{
    while r.more() {

    }
}
r.array_end()?;
```
