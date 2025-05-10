# Frost Programming Language

Statically typed, compiled language with the (objectively) best syntax.

## Features

- The best syntax
- Statically typed
- Compiled
- Struct/enum types
- Pattern matching
- First-class functions
- Immutable by default
- Tail returns

## Syntax

```frost
fn add_with_const(x: int, y: int): int {
    // val = const, var = mutable
    val z = 5
    var sum = x + y
    // shortcuts for operations
    sum += z
    if (sum > 10) {
        sum
    } elif (sum == 10) {
        100
    } else {
        0
    }
}
```

[![CI status badge showing passing or failing state for the main branch on GitHub Actions](https://github.com/actuallyatiger/frost/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/actuallyatiger/frost/actions/workflows/ci.yml)
