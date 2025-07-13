# Notes & TODOs

- [x] Support null
- [x] Support lit bools
- [x] Support lit strings (maybe not specification-compliant)
- [x] Support lit numbers (not fully specification-compliant, but works with rust ints and floats)
- [x] Support lit arrays
- [x] Support lit objects
- [ ] Support options (map to null?)
- [ ] Support bools
- [ ] Support strings
- [ ] Support numbers
- [ ] Support arrays
- [ ] Support objects
- [ ] Composability
- [ ] Works on types more general than `&mut Vec<u8>`

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
