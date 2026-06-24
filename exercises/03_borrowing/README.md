# 03 Borrowing

## Learning Objectives

You will practice shared references, mutable references, and Rust's borrowing rules. This module teaches how to let code read or modify data without taking ownership.

## Concepts

You can have many shared references or one mutable reference, but not both at the same time. This rule prevents data races and invalid references.

References must never outlive the value they point to. In many simple cases the compiler can infer lifetimes, but the rule is always there.

## Exercises

Find the first word in borrowed text. Mutate a borrowed vector only when needed. Split borrowed text into borrowed slices. Update a struct through a mutable reference.

## Hints

Prefer `&str` when a function only needs to read text. Use `&mut T` when the caller should keep ownership but allow modification. End a borrow before starting a conflicting mutable borrow.

## Self-Check Questions

Why can Rust allow many `&T` references but only one `&mut T` reference? What is the difference between modifying a value and returning a modified copy? Why does a returned `&str` need to come from an input string?

## Common Compiler Errors

`cannot borrow as mutable because it is also borrowed as immutable` means a shared borrow is still active when you try to create a mutable borrow. Use the shared value before the mutation, or narrow the shared borrow's scope.

`cannot borrow as mutable more than once at a time` means two mutable references overlap. Do one mutation at a time.

Lifetime errors often mean you returned a reference to a temporary local value. Return owned data or return a slice of an input value instead.

