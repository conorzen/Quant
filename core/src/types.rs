#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone)]
pub struct OptionsData {
    pub spot: f64,
    pub strike: f64,
    pub time: f64,
    pub risk_free_rate: f64,
    pub sigma: f64,
    pub dividend: f64,
    pub option_type: OptionType,
}

#[derive(Debug, Clone)]
pub struct OptionsResult {
    pub price: f64,
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
}