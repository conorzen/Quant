# quant-core

A high-performance quantitative finance library built in Rust, with Python and R bindings.

## Overview

`quant-core` provides fast, reliable implementations of common quantitative finance models including derivatives pricing, stochastic processes, and statistical tools.


![CI](https://github.com/conorzen/quant-core/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
 

## Structure

```
core/         → Rust library (the engine)
py-quant/     → Python bindings (coming soon)
r-quant/      → R bindings (coming soon)
```

## Getting Started

### Rust

```toml
[dependencies]
quant-core = "0.1.0"
```

### Python (coming soon)

```bash
pip install quant-core
```

### R (coming soon)

```r
install.packages("quantcore")
```

## Development

```bash
just build      # build the project
just test       # run all tests
```

## Licence

MIT OR Apache-2.0

thanks 

Conor
