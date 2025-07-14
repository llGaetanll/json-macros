# A non-trivial example
Say we want to write
```json
{
    "name": "Robert",
    "age": 35,
    "married": true,
    "friends": [
        {
            "name": "Hector",
            "married": false,
            "age": 32,
        },
        {
            "name": "Marie",
            "married": true,
            "age": 38,
        },
    ]
}
```

The idea would then be
```rust
json!(buf, {
    name: "Robert",
    age: 35,
    married: true,
    friends: [
        {
            name: "Hector",
            married: false,
            age: 32,
        },
        {
            name: "Marie",
            married: true,
            age: 38,
        },
    ]
})
```

# Primitives
## null
## true
## false
## number
## string
## array
## object

