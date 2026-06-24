# 05 Structs and Enums

## Learning Objectives

You will model realistic data using structs, tuple structs, enums, and match patterns. The focus is choosing data shapes that make invalid states harder to represent.

## Concepts

Structs group named fields. Tuple structs wrap a value with a stronger domain meaning. Enums represent one value from a known set of variants, and variants can carry data.

Pattern matching lets you unpack structs and enums while keeping each case explicit.

## Exercises

Create money values, calculate cart totals, model traffic light behavior, and summarize API responses.

## Hints

Use `impl` blocks for behavior that belongs to a type. Use enum variants with data when each case needs different information. Keep arithmetic in cents to avoid floating-point money bugs.

## Self-Check Questions

When is a tuple struct better than a type alias? How does an enum prevent invalid states? Why should money often be represented as integer cents?

## Common Compiler Errors

`missing field` means a struct literal did not provide every required field. Add the field or use update syntax when copying from another value.

`no method named` means the method is not implemented for that type or the value has the wrong type.

Borrowing conflicts can appear when matching a value and trying to mutate it at the same time. Match on a reference or finish the match before mutating.

