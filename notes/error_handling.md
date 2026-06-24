# Error Handling Notes

Use `Option<T>` when a value may be absent and absence is expected. Use `Result<T, E>` when an operation may fail and the caller needs to know why.

```rust
fn divide(left: i32, right: i32) -> Option<i32> {
    if right == 0 {
        None
    } else {
        Some(left / right)
    }
}
```

Avoid `unwrap()` in production library code. Prefer `match`, `ok_or`, `map_err`, and `?` so failures stay visible and recoverable.

