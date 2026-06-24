# Lifetimes Notes

Lifetimes describe how long references are valid. Most lifetimes are inferred, but the rule is always the same: a reference cannot outlive the value it points to.

```text
value lives:     |----------------|
reference lives:    |---------|
```

Returning a reference to a local variable is invalid because the local variable is dropped when the function ends. Return owned data instead, or return a reference derived from an input.

