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

    #[test]
    fn test_call_price() {
        let result = black_scholes::price(&default_option());
        println!("call price: {:.6}", result.price);
        assert!((result.price - 9.227).abs() < 0.01);
    }
    
    #[test]
    fn test_put_price() {
        let option = OptionsData {
            option_type: OptionType::Put,
            ..default_option()
        };
        let result = black_scholes::price(&option);
        println!("put price: {:.6}", result.price);
        assert!((result.price - 6.330).abs() < 0.01);
    }

    #[test]
    fn test_price_positive() {
        let result = black_scholes::price(&default_option());
        assert!(result.price > 0.0);
    }
}