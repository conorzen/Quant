# quant-core

A high-performance quantitative finance library built in Rust, with Python bindings.

![CI](https://github.com/conorzen/quant-core/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)

## Overview

`quant-core` provides fast, reliable implementations of common quantitative finance models — derivatives pricing, Greeks, implied volatility, and statistical tools. The same Rust engine powers both the native library and the Python package.

## Performance

Benchmarked on Apple M-series via `cargo bench`:

| Benchmark | Time |
|---|---|
| Single option price | 13.2 ns |
| 10,000 options | 218.5 µs |

Approximately 75 million option prices per second.

---

## Structure

```
core/       → Rust library (the engine)
py-quant/   → Python bindings (quantbridge)
r-quant/    → R bindings (coming soon)
```

---

## Python

### Installation

```bash
pip install quantbridge
```

Or with `uv`:

```bash
uv add quantbridge
```

### Usage

```python
import quantbridge as qb

# Price a European option and get all Greeks in one call
r = qb.black_scholes(
    spot=100.0,
    strike=105.0,
    time=0.25,           # years
    risk_free_rate=0.05,
    sigma=0.2,
    option_type="call",  # "call" or "put"
    dividend=0.0,        # optional, default 0
)

print(r.price)   # option price
print(r.delta)   # dP/dS         — 0 to 1 for calls, -1 to 0 for puts
print(r.gamma)   # d²P/dS²
print(r.vega)    # per 1% change in volatility
print(r.theta)   # per calendar day
print(r.rho)     # per 1% change in risk-free rate

# Implied volatility — solves for sigma given a market price
iv = qb.implied_volatility(
    spot=100.0,
    strike=105.0,
    time=0.25,
    risk_free_rate=0.05,
    market_price=3.50,
    option_type="call",
)
print(iv)  # e.g. 0.2031

# Statistics
returns = [0.01, -0.02, 0.015, 0.003, -0.008]
print(qb.variance(returns))  # sample variance
print(qb.std_dev(returns))   # sample standard deviation
```

### Building from source

Requires [maturin](https://github.com/PyO3/maturin) and a Rust toolchain.

```bash
uv tool install maturin
cd py-quant
maturin develop          # dev build (editable install)
maturin build --release  # release wheel → target/wheels/
```

The wheel targets the stable ABI (`abi3`) and works on Python 3.8+.

---

## Rust

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
quant-core = "0.1.0"
```

### Usage

```rust
use quant_core::models::derivatives::black_scholes;
use quant_core::types::{OptionsData, OptionType};

let option = OptionsData {
    spot: 100.0,
    strike: 105.0,
    time: 0.25,
    risk_free_rate: 0.05,
    sigma: 0.2,
    dividend: 0.0,
    option_type: OptionType::Call,
};

let result = black_scholes::price(&option);

println!("price : {:.4}", result.price);
println!("delta : {:.4}", result.delta);
println!("gamma : {:.6}", result.gamma);
println!("vega  : {:.4}", result.vega);   // per 1% vol
println!("theta : {:.4}", result.theta);  // per calendar day
println!("rho   : {:.4}", result.rho);    // per 1% rate
```

**Implied volatility:**

```rust
let iv = black_scholes::implied_volatility(&option, 3.50);
// Returns Option<f64> — None if the solver did not converge
println!("{:.4}", iv.unwrap());
```

**Statistics:**

```rust
use quant_core::models::maths;

let data = vec![0.01, -0.02, 0.015, 0.003, -0.008];

if let Some(var) = maths::variance(&data) {
    println!("variance : {:.6}", var);
}

if let Some(sd) = maths::standard_deviation(&data) {
    println!("std dev  : {:.6}", sd);
}
```

### Development

```bash
just build      # build
just test       # run all tests
cargo bench     # run benchmarks
```

---

## Licence

MIT OR Apache-2.0

Conor
