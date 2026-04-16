"""
quantbridge Python benchmarks
Mirrors the Rust criterion benchmarks in core/benches/black_scholes_bench.rs.
Includes a pure-Python Black-Scholes baseline for direct comparison.

Run:
    python py-quant/benches/benchmark.py
"""

import math
import timeit
import sys
import quantbridge as qb

# ── Shared parameters ─────────────────────────────────────────────────────────

SPOT            = 100.0
STRIKE          = 100.0
TIME            = 1.0
RISK_FREE_RATE  = 0.05
DIVIDEND        = 0.02
SIGMA           = 0.2

N_SINGLE        = 100_000
N_BATCH         = 10_000
N_BATCH_REPS    = 10
N_IV            = 10_000
N_STATS         = 100_000

STATS_DATA = [float(i) * 0.001 for i in range(1, 252)]


# ── Pure-Python baseline ──────────────────────────────────────────────────────

def _norm_cdf(x: float) -> float:
    return 0.5 * (1.0 + math.erf(x / math.sqrt(2.0)))

def _norm_pdf(x: float) -> float:
    return math.exp(-0.5 * x * x) / math.sqrt(2.0 * math.pi)

def py_black_scholes(spot, strike, time, rate, sigma, option_type, dividend=0.0):
    sqrt_t = math.sqrt(time)
    d1 = (math.log(spot / strike) + (rate - dividend + 0.5 * sigma * sigma) * time) / (sigma * sqrt_t)
    d2 = d1 - sigma * sqrt_t
    exp_qt = math.exp(-dividend * time)
    exp_rt = math.exp(-rate * time)
    nd1_pdf = _norm_pdf(d1)
    if option_type == "call":
        price = spot * exp_qt * _norm_cdf(d1) - strike * exp_rt * _norm_cdf(d2)
        delta = exp_qt * _norm_cdf(d1)
        theta = (-spot * exp_qt * nd1_pdf * sigma / (2 * sqrt_t)
                 - rate * strike * exp_rt * _norm_cdf(d2)
                 + dividend * spot * exp_qt * _norm_cdf(d1)) / 365.0
        rho   = strike * time * exp_rt * _norm_cdf(d2) / 100.0
    else:
        price = strike * exp_rt * _norm_cdf(-d2) - spot * exp_qt * _norm_cdf(-d1)
        delta = -exp_qt * _norm_cdf(-d1)
        theta = (-spot * exp_qt * nd1_pdf * sigma / (2 * sqrt_t)
                 + rate * strike * exp_rt * _norm_cdf(-d2)
                 - dividend * spot * exp_qt * _norm_cdf(-d1)) / 365.0
        rho   = -strike * time * exp_rt * _norm_cdf(-d2) / 100.0
    gamma = exp_qt * nd1_pdf / (spot * sigma * sqrt_t)
    vega  = spot * exp_qt * nd1_pdf * sqrt_t / 100.0
    return price, delta, gamma, vega, theta, rho

def py_variance(data):
    n = len(data)
    if n < 2:
        return None
    mean = sum(data) / n
    return sum((x - mean) ** 2 for x in data) / (n - 1)

def py_std_dev(data):
    v = py_variance(data)
    return math.sqrt(v) if v is not None else None


# ── Formatting helpers ────────────────────────────────────────────────────────

def _fmt(seconds: float, iterations: int) -> str:
    ns = (seconds / iterations) * 1e9
    if ns < 1_000:
        return f"{ns:.1f} ns/iter"
    us = ns / 1_000
    if us < 1_000:
        return f"{us:.2f} µs/iter"
    return f"{us / 1_000:.3f} ms/iter"

def _throughput(seconds: float, iterations: int, n_work: int = 1) -> str:
    ops = (iterations * n_work) / seconds
    if ops >= 1_000_000:
        return f"{ops / 1_000_000:.1f}M ops/sec"
    if ops >= 1_000:
        return f"{ops / 1_000:.1f}K ops/sec"
    return f"{ops:.0f} ops/sec"

def _speedup(rust_s: float, py_s: float) -> str:
    return f"{py_s / rust_s:.1f}x faster"

def run(label: str, stmt: str, n: int = N_SINGLE) -> float:
    elapsed = timeit.timeit(stmt=stmt, globals=globals(), number=n)
    print(f"  {label:<46}  {_fmt(elapsed, n):<18}  {_throughput(elapsed, n)}")
    return elapsed


# ── Benchmarks ────────────────────────────────────────────────────────────────

def bench_single_price() -> None:
    print("\nblack_scholes / single call price + Greeks")
    t_rust = run(
        "quantbridge  (Rust)",
        "qb.black_scholes(SPOT, STRIKE, TIME, RISK_FREE_RATE, SIGMA, 'call', DIVIDEND)",
    )
    t_py = run(
        "pure Python",
        "py_black_scholes(SPOT, STRIKE, TIME, RISK_FREE_RATE, SIGMA, 'call', DIVIDEND)",
    )
    print(f"  {'':46}  quantbridge is {_speedup(t_rust, t_py)}")


def bench_batch_price() -> None:
    print(f"\nblack_scholes / {N_BATCH:,} options (varying spot)")
    spots = [100.0 + i * 0.1 for i in range(N_BATCH)]

    def rust_loop():
        for s in spots:
            qb.black_scholes(s, STRIKE, TIME, RISK_FREE_RATE, SIGMA, "call", DIVIDEND)

    def py_loop():
        for s in spots:
            py_black_scholes(s, STRIKE, TIME, RISK_FREE_RATE, SIGMA, "call", DIVIDEND)

    def rust_batch():
        qb.black_scholes_batch(spots, STRIKE, TIME, RISK_FREE_RATE, SIGMA, "call", DIVIDEND)

    t_rust_loop  = timeit.timeit(rust_loop,  number=N_BATCH_REPS)
    t_py_loop    = timeit.timeit(py_loop,    number=N_BATCH_REPS)
    t_rust_batch = timeit.timeit(rust_batch, number=N_BATCH_REPS)

    for label, elapsed in [
        ("quantbridge loop  (Rust, 1 call each)", t_rust_loop),
        ("pure Python loop",                      t_py_loop),
        ("quantbridge batch (Rust, 1 call total)", t_rust_batch),
    ]:
        per_batch   = elapsed / N_BATCH_REPS
        ns_each     = per_batch / N_BATCH * 1e9
        ops_per_sec = (N_BATCH_REPS * N_BATCH) / elapsed
        print(f"  {label:<46}  {per_batch * 1e3:.3f} ms/batch   "
              f"{ns_each:.1f} ns/option   {ops_per_sec / 1_000_000:.1f}M ops/sec")

    print(f"  {'batch vs pure Python loop':<46}  {_speedup(t_rust_batch, t_py_loop)}")
    print(f"  {'batch vs quantbridge loop':<46}  {_speedup(t_rust_batch, t_rust_loop)}")


def bench_statistics() -> None:
    print("\nvariance + std_dev  (251-point series)")
    t_rust_var = run("quantbridge variance  (Rust)", "qb.variance(STATS_DATA)",  n=N_STATS)
    t_py_var   = run("pure Python variance",         "py_variance(STATS_DATA)",  n=N_STATS)
    print(f"  {'':46}  quantbridge is {_speedup(t_rust_var, t_py_var)}")

    t_rust_sd  = run("quantbridge std_dev   (Rust)", "qb.std_dev(STATS_DATA)",   n=N_STATS)
    t_py_sd    = run("pure Python std_dev",          "py_std_dev(STATS_DATA)",   n=N_STATS)
    print(f"  {'':46}  quantbridge is {_speedup(t_rust_sd, t_py_sd)}")


def bench_implied_vol() -> None:
    global market_price
    market_price = qb.black_scholes(
        SPOT, STRIKE, TIME, RISK_FREE_RATE, SIGMA, "call", DIVIDEND
    ).price
    print("\nimplied_volatility  (Newton-Raphson)")
    run(
        "quantbridge  (Rust)",
        "qb.implied_volatility(SPOT, STRIKE, TIME, RISK_FREE_RATE, market_price, 'call', DIVIDEND)",
        n=N_IV,
    )


# ── Entry point ───────────────────────────────────────────────────────────────

if __name__ == "__main__":
    version = getattr(qb, "__version__", None)
    print(f"quantbridge {version}" if version else "quantbridge")
    print(f"Python {sys.version.split()[0]}")
    print("=" * 72)

    bench_single_price()
    bench_batch_price()
    bench_implied_vol()
    bench_statistics()

    print()
