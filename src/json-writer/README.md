# How it works

This crate provides 3 macros:
- `json`: Writes json to the provided buffer
- `lazy`: Defers writing to the buffer with a closure
- `lazy_move`: Like `lazy` but the closure is marked `move`.

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
- [x] Writes to types more general than `&mut Vec<u8>`
- [x] Static AST optimization
      i.e. If the object has a lot of static fields in a row, just push a
      massive byte slice to the buffer instead of calling `buf.push` a
      million times
- [x] Dynamic key support
- [ ] Add derive macro for `Serialize`

## Reach
- [ ] Size hints on values?
      ```rust
      let mut buf = Vec::new();

      let value = // some dynamic value here
      json!(buf, {
          name: value in 12..15 // meaning we expect this value to take between 12 and 15 bytes
      })
      ```
