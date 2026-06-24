# Traits Notes

Traits describe shared behavior. A type implements a trait to promise that it supports certain methods.

```rust
trait Summary {
    fn summary(&self) -> String;
}
```

Traits are useful for generic code. Instead of accepting one concrete type, a function can accept any type that implements the needed behavior.

Common traits include `Debug`, `Clone`, `Copy`, `Default`, `Iterator`, `From`, and `TryFrom`.

