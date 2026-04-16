use crate::types::{OptionType, OptionsData, OptionsResult};
use crate::utils::{norm_cdf, norm_pdf};

/// Prices a European option using the Black-Scholes-Merton model and computes all Greeks.
///
/// # Arguments
/// * `option` - Market data and contract parameters
///
/// # Returns
/// `OptionsResult` with price, delta, gamma, vega, theta (per calendar day), and rho.
///
/// # Example
/// ```
/// use quant_core::types::{OptionsData, OptionType};
/// use quant_core::models::derivatives::black_scholes;
///
/// let option = OptionsData {
///     spot: 100.0,
///     strike: 100.0,
///     time: 1.0,
///     risk_free_rate: 0.05,
///     dividend: 0.02,
///     sigma: 0.2,
///     option_type: OptionType::Call,
/// };
/// let result = black_scholes::price(&option);
/// ```
pub fn price(option: &OptionsData) -> OptionsResult {
    let spot = option.spot;
    let strike = option.strike;
    let time = option.time;
    let rate = option.risk_free_rate;
    let sigma = option.sigma;
    let dividend = option.dividend;

    let sqrt_t = time.sqrt();
    let d1 = ((spot / strike).ln() + (rate - dividend + 0.5 * sigma * sigma) * time)
        / (sigma * sqrt_t);
    let d2 = d1 - sigma * sqrt_t;

    let exp_qt = (-dividend * time).exp();
    let exp_rt = (-rate * time).exp();
    let nd1_pdf = norm_pdf(d1);

    let price = match option.option_type {
        OptionType::Call => spot * exp_qt * norm_cdf(d1) - strike * exp_rt * norm_cdf(d2),
        OptionType::Put => strike * exp_rt * norm_cdf(-d2) - spot * exp_qt * norm_cdf(-d1),
    };

    let delta = match option.option_type {
        OptionType::Call => exp_qt * norm_cdf(d1),
        OptionType::Put => -exp_qt * norm_cdf(-d1),
    };

    let gamma = exp_qt * nd1_pdf / (spot * sigma * sqrt_t);

    // Vega: sensitivity to 1% change in volatility
    let vega = spot * exp_qt * nd1_pdf * sqrt_t / 100.0;

    // Theta: daily decay (per calendar day)
    let theta = match option.option_type {
        OptionType::Call => {
            (-spot * exp_qt * nd1_pdf * sigma / (2.0 * sqrt_t)
                - rate * strike * exp_rt * norm_cdf(d2)
                + dividend * spot * exp_qt * norm_cdf(d1))
                / 365.0
        }
        OptionType::Put => {
            (-spot * exp_qt * nd1_pdf * sigma / (2.0 * sqrt_t)
                + rate * strike * exp_rt * norm_cdf(-d2)
                - dividend * spot * exp_qt * norm_cdf(-d1))
                / 365.0
        }
    };

    let rho = match option.option_type {
        OptionType::Call => strike * time * exp_rt * norm_cdf(d2) / 100.0,
        OptionType::Put => -strike * time * exp_rt * norm_cdf(-d2) / 100.0,
    };

    OptionsResult { price, delta, gamma, vega, theta, rho }
}

/// Solves for implied volatility using Newton-Raphson iteration.
///
/// # Arguments
/// * `option`       - Contract parameters (sigma field is used as the initial guess)
/// * `market_price` - Observed market price of the option
///
/// # Returns
/// `Some(sigma)` if converged within `max_iterations`, `None` otherwise.
pub fn implied_volatility(option: &OptionsData, market_price: f64) -> Option<f64> {
    const MAX_ITERATIONS: u32 = 100;
    const TOLERANCE: f64 = 1e-6;
    const MIN_VEGA: f64 = 1e-10;

    let mut sigma = option.sigma.max(0.01); // ensure positive starting point

    for _ in 0..MAX_ITERATIONS {
        let trial = OptionsData { sigma, ..option.clone() };
        let result = price(&trial);
        let diff = result.price - market_price;

        if diff.abs() < TOLERANCE {
            return Some(sigma);
        }

        // vega stored as per-1%, convert back to raw for the NR step
        let raw_vega = result.vega * 100.0;
        if raw_vega.abs() < MIN_VEGA {
            return None;
        }

        sigma -= diff / raw_vega;
        if sigma <= 0.0 {
            sigma = 1e-4;
        }
    }

    None
}
