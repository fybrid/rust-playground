# 04 Option and Result

## Learning Objectives

You will use `Option` for values that may be absent and `Result` for operations that may fail. This module avoids panic-based solutions so you can practice recoverable error handling.

## Concepts

`Option<T>` is either `Some(T)` or `None`. `Result<T, E>` is either `Ok(T)` or `Err(E)`. Matching these types forces you to handle success and failure paths.

The `?` operator returns early from a function when it sees an error. It keeps error-handling code readable when your function also returns `Result`.

## Exercises

Parse a network port, divide safely, look up a user by id, and read a required setting from a map.

## Hints

Avoid `unwrap()` and `expect()` in library code. Use `match`, `if let`, `ok_or`, `map`, and `?` where they make the intent clearer.

## Self-Check Questions

When should a function return `Option` instead of `Result`? What information should an error value contain? Why is `unwrap()` risky in production code?

## Common Compiler Errors

`mismatched types` often appears when one branch returns `T` and another returns `Option<T>` or `Result<T, E>`. Make every branch return the same type.

`the ? operator can only be used` means the surrounding function does not return a compatible `Result` or `Option`.

Lifetime issues may appear when returning references from a map. The returned reference must come from the map borrowed by the caller, not from a temporary value.

