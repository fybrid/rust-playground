# 01 Basics

## Learning Objectives

You will practice variables, mutability, functions, primitive types, control flow, and basic pattern matching. These are the small building blocks that every later Rust topic depends on.

## Concepts

Rust values are immutable by default. Use `mut` only when a value must change. Functions have explicit parameter and return types, and the final expression in a block can be returned without `return`.

Pattern matching with `match` is exhaustive. That means Rust asks you to handle every possible case instead of accidentally forgetting one.

## Exercises

Temperature conversion asks you to convert between Celsius and Fahrenheit using integer arithmetic. Simple calculator asks you to model operations with an enum. Number classification asks you to return a meaningful category for a number. String formatting asks you to combine text and numbers without manual string mutation.

## Hints

Use `match` for enum-based branching. Prefer expressions over many temporary mutable variables. `format!` returns a new `String`.

## Self-Check Questions

What is the difference between `let x = 1` and `let mut x = 1`? Why does a `match` over an enum need every variant handled? When should a function return `String` instead of `&str`?

## Common Compiler Errors

`cannot assign twice to immutable variable` means you tried to change a value that was not declared with `mut`. Add `mut` only when mutation is part of the design.

`mismatched types` means an expression returns a different type than the function promised. Read the expected and found types in the compiler message.

`non-exhaustive patterns` means a `match` does not cover every possible input. Add the missing enum variant or wildcard case.

