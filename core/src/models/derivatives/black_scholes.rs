use crate::types::{OptionType, OptionsData, OptionsResult};
use crate::utils::norm_cdf;

pub fn price(option: &OptionsData) -> OptionsResult {

    let spot = option.spot;
    let strike = option.strike;
    let time = option.time;
    let rate = option.risk_free_rate;
    let sigma = option.sigma;
    let dividend = option.dividend;

    let d1 = ((spot / strike).ln() + (rate - dividend + 0.5 * sigma * sigma) * time)
        / (sigma * time.sqrt());
    let d2 = d1 - sigma * time.sqrt();

    let options_price = match option.option_type {
        OptionType::Call => {
            spot * (-dividend * time).exp() * norm_cdf(d1)
                - strike * (-rate * time).exp() * norm_cdf(d2)
        }
        OptionType::Put => {
            strike * (-rate * time).exp() * norm_cdf(-d2)
                - spot * (-dividend * time).exp() * norm_cdf(-d1)
        }
    };

    OptionsResult {
        price: options_price,
        delta: 0.0, 
        gamma: 0.0,
        vega:  0.0,
        theta: 0.0,
    }
}
