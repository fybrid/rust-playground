# Borrowing Notes

Borrowing lets code use a value without owning it.

```text
many readers:  &T  &T  &T
one writer:    &mut T
```

Rust allows many shared references or one mutable reference at a time. This prevents code from reading stale data while another part is changing it.

When the compiler reports a borrowing conflict, find where the first borrow starts and where it is last used. Shortening that range usually fixes the problem.

