use pyo3::prelude::*;
use quant_core::models::derivatives::black_scholes as bs;
use quant_core::models::maths;
use quant_core::types::{OptionType, OptionsData};

// ── Python-facing result type ─────────────────────────────────────────────────

/// Result of pricing a European option.
///
/// Attributes
/// ----------
/// price : float
///     Option price.
/// delta : float
///     Rate of change of price with respect to the underlying (0–1 for calls, -1–0 for puts).
/// gamma : float
///     Rate of change of delta with respect to the underlying.
/// vega : float
///     Sensitivity to a 1% change in implied volatility.
/// theta : float
///     Daily time decay (per calendar day).
/// rho : float
///     Sensitivity to a 1% change in the risk-free rate.
#[pyclass(frozen)]
pub struct OptionsResult {
    #[pyo3(get)]
    pub price: f64,
    #[pyo3(get)]
    pub delta: f64,
    #[pyo3(get)]
    pub gamma: f64,
    #[pyo3(get)]
    pub vega: f64,
    #[pyo3(get)]
    pub theta: f64,
    #[pyo3(get)]
    pub rho: f64,
}

#[pymethods]
impl OptionsResult {
    fn __repr__(&self) -> String {
        format!(
            "OptionsResult(price={:.4}, delta={:.4}, gamma={:.6}, vega={:.4}, theta={:.4}, rho={:.4})",
            self.price, self.delta, self.gamma, self.vega, self.theta, self.rho
        )
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_option_type(s: &str) -> PyResult<OptionType> {
    match s.to_lowercase().as_str() {
        "call" | "c" => Ok(OptionType::Call),
        "put" | "p" => Ok(OptionType::Put),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "option_type must be 'call' or 'put', got '{s}'"
        ))),
    }
}

// ── Derivatives ───────────────────────────────────────────────────────────────

/// Price a European option and compute all Greeks using Black-Scholes-Merton.
///
/// Parameters
/// ----------
/// spot : float
///     Current price of the underlying asset.
/// strike : float
///     Option strike price.
/// time : float
///     Time to expiration in years (e.g. 0.25 for 3 months).
/// risk_free_rate : float
///     Continuously compounded risk-free rate (e.g. 0.05 for 5%).
/// sigma : float
///     Annualised implied volatility (e.g. 0.2 for 20%).
/// option_type : str
///     ``"call"`` or ``"put"`` (or ``"c"`` / ``"p"``).
/// dividend : float, optional
///     Continuous dividend yield (default 0.0).
///
/// Returns
/// -------
/// OptionsResult
///     price, delta, gamma, vega (per 1% vol), theta (per day), rho (per 1% rate).
///
/// Examples
/// --------
/// >>> import quantbridge as qb
/// >>> r = qb.black_scholes(100, 105, 0.25, 0.05, 0.2, "call")
/// >>> r.price, r.delta
#[pyfunction]
#[pyo3(signature = (spot, strike, time, risk_free_rate, sigma, option_type, dividend=0.0))]
fn black_scholes(
    spot: f64,
    strike: f64,
    time: f64,
    risk_free_rate: f64,
    sigma: f64,
    option_type: &str,
    dividend: f64,
) -> PyResult<OptionsResult> {
    let data = OptionsData {
        spot,
        strike,
        time,
        risk_free_rate,
        sigma,
        dividend,
        option_type: parse_option_type(option_type)?,
    };
    let r = bs::price(&data);
    Ok(OptionsResult {
        price: r.price,
        delta: r.delta,
        gamma: r.gamma,
        vega: r.vega,
        theta: r.theta,
        rho: r.rho,
    })
}

/// Solve for implied volatility using Newton-Raphson iteration.
///
/// Parameters
/// ----------
/// spot : float
///     Current price of the underlying asset.
/// strike : float
///     Option strike price.
/// time : float
///     Time to expiration in years.
/// risk_free_rate : float
///     Continuously compounded risk-free rate.
/// market_price : float
///     Observed market price of the option.
/// option_type : str
///     ``"call"`` or ``"put"``.
/// dividend : float, optional
///     Continuous dividend yield (default 0.0).
/// initial_sigma : float, optional
///     Starting guess for volatility (default 0.2).
///
/// Returns
/// -------
/// float or None
///     Implied volatility if converged, ``None`` if the solver did not converge.
///
/// Examples
/// --------
/// >>> import quantbridge as qb
/// >>> qb.implied_volatility(100, 105, 0.25, 0.05, 3.50, "call")
#[pyfunction]
#[pyo3(signature = (spot, strike, time, risk_free_rate, market_price, option_type, dividend=0.0, initial_sigma=0.2))]
fn implied_volatility(
    spot: f64,
    strike: f64,
    time: f64,
    risk_free_rate: f64,
    market_price: f64,
    option_type: &str,
    dividend: f64,
    initial_sigma: f64,
) -> PyResult<Option<f64>> {
    let data = OptionsData {
        spot,
        strike,
        time,
        risk_free_rate,
        sigma: initial_sigma,
        dividend,
        option_type: parse_option_type(option_type)?,
    };
    Ok(bs::implied_volatility(&data, market_price))
}

/// Result of pricing a batch of European options.
///
/// Each attribute is a list of floats, one value per option, in the same order
/// as the input ``spots`` list.
///
/// Attributes
/// ----------
/// prices, deltas, gammas, vegas, thetas, rhos : list[float]
#[pyclass(frozen)]
pub struct BatchOptionsResult {
    #[pyo3(get)]
    pub prices: Vec<f64>,
    #[pyo3(get)]
    pub deltas: Vec<f64>,
    #[pyo3(get)]
    pub gammas: Vec<f64>,
    #[pyo3(get)]
    pub vegas: Vec<f64>,
    #[pyo3(get)]
    pub thetas: Vec<f64>,
    #[pyo3(get)]
    pub rhos: Vec<f64>,
}

#[pymethods]
impl BatchOptionsResult {
    fn __repr__(&self) -> String {
        format!("BatchOptionsResult(n={})", self.prices.len())
    }

    fn __len__(&self) -> usize {
        self.prices.len()
    }
}

/// Price a batch of European options over a list of spot prices.
///
/// All parameters except ``spots`` are scalars applied to every option.
/// The entire batch crosses the Python/Rust boundary in a single call,
/// making this significantly faster than looping over ``black_scholes``.
///
/// Parameters
/// ----------
/// spots : list[float]
///     Spot prices to price over.
/// strike : float
/// time : float
///     Time to expiration in years.
/// risk_free_rate : float
/// sigma : float
///     Annualised implied volatility.
/// option_type : str
///     ``"call"`` or ``"put"``.
/// dividend : float, optional
///     Continuous dividend yield (default 0.0).
///
/// Returns
/// -------
/// BatchOptionsResult
///     Parallel arrays: prices, deltas, gammas, vegas, thetas, rhos.
///
/// Examples
/// --------
/// >>> import quantbridge as qb
/// >>> spots = [90.0, 95.0, 100.0, 105.0, 110.0]
/// >>> r = qb.black_scholes_batch(spots, 100.0, 0.25, 0.05, 0.2, "call")
/// >>> r.prices
/// >>> r.deltas
#[pyfunction]
#[pyo3(signature = (spots, strike, time, risk_free_rate, sigma, option_type, dividend=0.0))]
fn black_scholes_batch(
    spots: Vec<f64>,
    strike: f64,
    time: f64,
    risk_free_rate: f64,
    sigma: f64,
    option_type: &str,
    dividend: f64,
) -> PyResult<BatchOptionsResult> {
    let opt_type = parse_option_type(option_type)?;
    let n = spots.len();

    let mut prices = Vec::with_capacity(n);
    let mut deltas = Vec::with_capacity(n);
    let mut gammas = Vec::with_capacity(n);
    let mut vegas  = Vec::with_capacity(n);
    let mut thetas = Vec::with_capacity(n);
    let mut rhos   = Vec::with_capacity(n);

    for spot in spots {
        let data = OptionsData {
            spot,
            strike,
            time,
            risk_free_rate,
            sigma,
            dividend,
            option_type: opt_type.clone(),
        };
        let r = bs::price(&data);
        prices.push(r.price);
        deltas.push(r.delta);
        gammas.push(r.gamma);
        vegas.push(r.vega);
        thetas.push(r.theta);
        rhos.push(r.rho);
    }

    Ok(BatchOptionsResult { prices, deltas, gammas, vegas, thetas, rhos })
}

// ── Statistics ────────────────────────────────────────────────────────────────

/// Compute sample variance (Bessel's correction, n-1).
///
/// Parameters
/// ----------
/// data : list[float]
///     Input data. Must contain at least 2 values.
///
/// Returns
/// -------
/// float or None
///     Sample variance, or ``None`` if fewer than 2 values were provided.
#[pyfunction]
fn variance(data: Vec<f64>) -> Option<f64> {
    maths::variance(&data)
}

/// Compute sample standard deviation.
///
/// Parameters
/// ----------
/// data : list[float]
///     Input data. Must contain at least 2 values.
///
/// Returns
/// -------
/// float or None
///     Sample standard deviation, or ``None`` if fewer than 2 values were provided.
#[pyfunction]
fn std_dev(data: Vec<f64>) -> Option<f64> {
    maths::standard_deviation(&data)
}

// ── Module ────────────────────────────────────────────────────────────────────

#[pymodule]
fn quantbridge(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<OptionsResult>()?;
    m.add_class::<BatchOptionsResult>()?;
    m.add_function(wrap_pyfunction!(black_scholes, m)?)?;
    m.add_function(wrap_pyfunction!(black_scholes_batch, m)?)?;
    m.add_function(wrap_pyfunction!(implied_volatility, m)?)?;
    m.add_function(wrap_pyfunction!(variance, m)?)?;
    m.add_function(wrap_pyfunction!(std_dev, m)?)?;
    Ok(())
}
