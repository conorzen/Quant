# Contributing to quant-core

## Getting Started

1. Clone the repo
```bash
git clone https://github.com/conorzen/quant-core
cd Quant
```

2. Install dependencies
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# just (command runner)
cargo install just
```

3. Build and test
```bash
just build
just test
```

---

## Branch Rules

- **Never push directly to `main`**
- Always create a branch, then open a pull request
- All PRs must be reviewed and approved by [@conorzen](https://github.com/conorzen) before merging

```bash
# Always branch from main
git checkout main
git pull
git checkout -b feat/my-new-feature

# Push your branch
git push origin feat/my-new-feature

# Then open a PR on GitHub
```

---

## Branch Naming

| Type | Pattern | Example |
|---|---|---|
| New feature | `feat/` | `feat/delta-calculation` |
| Bug fix | `fix/` | `fix/norm-cdf-precision` |
| Tests | `test/` | `test/black-scholes-greeks` |
| Docs | `docs/` | `docs/contributing-guide` |
| Refactor | `refactor/` | `refactor/types-module` |

---

## Adding a New Model

1. Create your file in the right module folder:
```
core/src/models/derivatives/my_model.rs
```

2. Declare it in `mod.rs`:
```rust
// core/src/models/derivatives/mod.rs
pub mod black_scholes;
pub mod my_model;       // ← add this
```

3. Use the shared types from `types.rs`:
```rust
use crate::types::{OptionsData, OptionsResult, OptionType};
```

4. Make your function public:
```rust
pub fn price(option: &OptionsData) -> OptionsResult {
    // ...
}
```

5. Add tests in `core/tests/`:
```rust
// core/tests/my_model_tests.rs
use quant_core::models::derivatives::my_model;
use quant_core::types::{OptionsData, OptionType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price() {
        // test a known value
    }
}
```

---

## Code Standards

- All new functions must have at least one test
- Use `f64` for all numeric types unless there is a strong reason not to
- Use `Result<T, String>` for functions that can fail
- No `.unwrap()` in public APIs

---

## Running Tests

```bash
just test              # run all tests
just test-verbose      # run tests with println! output visible
cargo test my_test     # run a specific test by name
```

---

## Questions

Open an issue or email reidconor06@gmail.com