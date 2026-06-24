# Ownership Notes

Ownership is Rust's rule for deciding who is responsible for cleaning up a value.

```text
String owner
    |
    v
heap allocation
```

When the owner goes out of scope, Rust drops the value. Passing a `String` to a function by value moves ownership. Passing `&String` or `&str` borrows it.

Use ownership when a function should consume or store a value. Use borrowing when a function only needs temporary access. Use `clone()` when two independent owners are truly needed.

