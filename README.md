# Helpful commands

## Auto-reloading
```shell
cargo install cargo-watch
cargo watch -x run
```

# Helpful notes

## Convert strings into any type
```rust
let mut index = String::new();


// shadowing and turning index into a i32
let index: i32 = index.trim().parse().expect("Index entered was not a number");

```
