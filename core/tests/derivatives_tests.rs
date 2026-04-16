use quant_core::models::derivatives::black_scholes;
use quant_core::types::{OptionsData, OptionType};

#[cfg(test)]
mod tests {
    use super::*;

    fn default_option() -> OptionsData {
        OptionsData {
            spot: 100.0,
            strike: 100.0,
            time: 1.0,
            risk_free_rate: 0.05,
            dividend: 0.02,
            sigma: 0.2,
            option_type: OptionType::Call,
        }
    }

    // ── Pricing ──────────────────────────────────────────────────────────────

    #[test]
    fn test_call_price() {
        let result = black_scholes::price(&default_option());
        println!("call price: {:.6}", result.price);
        assert!((result.price - 9.227).abs() < 0.01);
    }

    #[test]
    fn test_put_price() {
        let option = OptionsData { option_type: OptionType::Put, ..default_option() };
        let result = black_scholes::price(&option);
        println!("put price: {:.6}", result.price);
        assert!((result.price - 6.330).abs() < 0.01);
    }

    #[test]
    fn test_price_positive() {
        let result = black_scholes::price(&default_option());
        assert!(result.price > 0.0);
    }

    // Put-call parity: C - P = S*e^(-qT) - K*e^(-rT)
    #[test]
    fn test_put_call_parity() {
        let call = black_scholes::price(&default_option());
        let put = black_scholes::price(&OptionsData {
            option_type: OptionType::Put,
            ..default_option()
        });
        let opt = default_option();
        let parity = opt.spot * (-opt.dividend * opt.time).exp()
            - opt.strike * (-opt.risk_free_rate * opt.time).exp();
        assert!((call.price - put.price - parity).abs() < 1e-8);
    }

    // ── Greeks ───────────────────────────────────────────────────────────────

    #[test]
    fn test_call_delta_range() {
        let result = black_scholes::price(&default_option());
        // ATM call delta should be roughly 0.5–0.6
        assert!(result.delta > 0.0 && result.delta < 1.0,
            "call delta out of range: {}", result.delta);
    }

    #[test]
    fn test_put_delta_range() {
        let result = black_scholes::price(&OptionsData {
            option_type: OptionType::Put,
            ..default_option()
        });
        assert!(result.delta > -1.0 && result.delta < 0.0,
            "put delta out of range: {}", result.delta);
    }

    // Call delta + |put delta| = e^(-qT) for same strike (delta symmetry)
    #[test]
    fn test_delta_symmetry() {
        let opt = default_option();
        let call = black_scholes::price(&opt);
        let put = black_scholes::price(&OptionsData {
            option_type: OptionType::Put,
            ..opt.clone()
        });
        let expected = (-opt.dividend * opt.time).exp();
        assert!((call.delta + put.delta.abs() - expected).abs() < 1e-8);
    }

    #[test]
    fn test_gamma_positive() {
        let call = black_scholes::price(&default_option());
        let put = black_scholes::price(&OptionsData {
            option_type: OptionType::Put,
            ..default_option()
        });
        assert!(call.gamma > 0.0, "call gamma should be positive");
        assert!(put.gamma > 0.0, "put gamma should be positive");
    }

    // Gamma is identical for call and put with same parameters
    #[test]
    fn test_call_put_gamma_equal() {
        let call = black_scholes::price(&default_option());
        let put = black_scholes::price(&OptionsData {
            option_type: OptionType::Put,
            ..default_option()
        });
        assert!((call.gamma - put.gamma).abs() < 1e-10);
    }

    #[test]
    fn test_vega_positive() {
        let result = black_scholes::price(&default_option());
        assert!(result.vega > 0.0);
    }

    // Vega is identical for call and put
    #[test]
    fn test_call_put_vega_equal() {
        let call = black_scholes::price(&default_option());
        let put = black_scholes::price(&OptionsData {
            option_type: OptionType::Put,
            ..default_option()
        });
        assert!((call.vega - put.vega).abs() < 1e-10);
    }

    #[test]
    fn test_call_theta_negative() {
        // Long call theta is almost always negative
        let result = black_scholes::price(&default_option());
        assert!(result.theta < 0.0, "call theta should be negative: {}", result.theta);
    }

    #[test]
    fn test_call_rho_positive() {
        let result = black_scholes::price(&default_option());
        assert!(result.rho > 0.0, "call rho should be positive: {}", result.rho);
    }

    #[test]
    fn test_put_rho_negative() {
        let result = black_scholes::price(&OptionsData {
            option_type: OptionType::Put,
            ..default_option()
        });
        assert!(result.rho < 0.0, "put rho should be negative: {}", result.rho);
    }

    // ── Implied Volatility ───────────────────────────────────────────────────

    #[test]
    fn test_implied_vol_roundtrip() {
        let opt = default_option();
        let result = black_scholes::price(&opt);
        let iv = black_scholes::implied_volatility(&opt, result.price)
            .expect("implied vol should converge");
        assert!((iv - opt.sigma).abs() < 1e-5,
            "IV roundtrip failed: got {iv:.6}, expected {:.6}", opt.sigma);
    }

    #[test]
    fn test_implied_vol_put_roundtrip() {
        let opt = OptionsData { option_type: OptionType::Put, ..default_option() };
        let result = black_scholes::price(&opt);
        let iv = black_scholes::implied_volatility(&opt, result.price)
            .expect("implied vol should converge");
        assert!((iv - opt.sigma).abs() < 1e-5,
            "Put IV roundtrip failed: got {iv:.6}, expected {:.6}", opt.sigma);
    }
}
