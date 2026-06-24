# 02 Ownership

## Learning Objectives

You will learn ownership, move semantics, cloning, and the difference between stack and heap values. The goal is to understand why Rust can manage memory without a garbage collector.

## Concepts

Every value has one owner. When the owner goes out of scope, the value is dropped. Heap-backed values like `String` and `Vec<T>` move by default, while simple copyable values like integers are copied.

Cloning creates a new owned value. It is useful, but it has a cost, so prefer moving when the old owner is no longer needed.

## Exercises

Normalize a title by taking ownership of a `String`. Count bytes in owned strings without keeping unneeded data. Add suffixes to a list and return ownership. Clone data only when two owners are truly needed.

## Hints

If a function receives `String`, it owns that string. If it receives `&str`, it borrows text. Use `.into_iter()` when you want to consume a vector and `.iter()` when you only want to read it.

## Self-Check Questions

Why can an `i32` still be used after assignment but a `String` usually cannot? What does `.clone()` do? When is moving a value better than borrowing it?

## Common Compiler Errors

`value borrowed here after move` means ownership already moved to another variable or function. Fix it by borrowing, cloning intentionally, or changing the order of operations.

`use of moved value` means you tried to use an owner after it was consumed. Look for function calls that take `String`, `Vec<T>`, or another non-`Copy` type by value.

`cannot move out of borrowed content` means you have only a reference but tried to take ownership of the referenced value. Borrow the inner value or clone it deliberately.

