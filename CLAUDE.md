```
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
```

## Project Overview

This repository contains Rust practice code with a primary focus on an arithmetic expression evaluator. The main project is located in the `expr-eval` subdirectory.

## Directory Structure

- `/expr-eval`: Rust project implementing an arithmetic expression evaluator
  - `src/main.rs`: Main executable containing the expression parser and evaluator
  - `Cargo.toml`: Project metadata and dependencies
  - `Cargo.lock`: Lock file for dependencies

## Common Commands

### Build and Run

```bash
# Run the expression evaluator (prints example calculation)
cd expr-eval && cargo run

# Build the project (debug mode)
cd expr-eval && cargo build

# Build in release mode
cd expr-eval && cargo build --release
```

### Testing

At present, the project does not have explicit test functions defined. To add tests:

1. Add `#[test]` annotated functions in `src/main.rs` or create a `tests` directory
2. Run tests with: `cd expr-eval && cargo test`

## Code Architecture

The expr-eval project implements an arithmetic expression evaluator using:

### Core Components:
- **Tokenization**: Converts raw string expressions to tokens using `Tokenizer` struct
- **Parsing**: Uses recursive descent parsing with precedence handling (Shunting-yard algorithm variant)
- **Evaluation**: Implements operator precedence and associativity rules

### Key Features:
- Supports basic arithmetic operations: +, -, *, /
- Exponentiation: ^ (right-associative with highest precedence)
- Parentheses for grouping: ()
- Handles operator precedence and associativity
- Skips whitespace automatically

### Main Structs:
- `Tokenizer`: Tokenizes input strings
- `Expr`: Main evaluator with recursive descent parser
- `Token`: Represents supported tokens (numbers, operators, parentheses)

## Examples

The current implementation evaluates this example expression:
```rust
let string_expr = "92 + 5 + 5 * 27 - (92 - 12) / 4 + 26";
// Result: 238
```
