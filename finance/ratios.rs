// Basic financial ratios

pub fn return_amount(current_price:f64, past_price:f64)->f64
{
    (current_price-past_price)/(current_price)
}

pub fn sharpe(returns:f64, risk_free_rate:f64, volatility:f64) -> f64
{
    (returns-risk_free_rate)/volatility
}

pub fn sharpe_list(values : &[f64], risk_free_rate:f64) -> f64
{
    let mean = crate::math::stats::mean(values);
    let sd = crate::math::stats::standard_deviation_pop(values);
    (mean-risk_free_rate)/sd
}

pub fn sortino(returns:f64, risk_free_rate:f64, volatility:f64) -> f64
{
    (returns-risk_free_rate)/(f64::sqrt(volatility))
}

pub fn sortino_list(values: &[f64], risk_free_rate:f64) -> f64
{
    let mean = crate::math::stats::mean(values);
    let dd = crate::math::stats::downside_deviation(values, risk_free_rate);
    (mean-risk_free_rate)/(dd)
}

pub fn price_earnings_ratio(market_price:f64, earnings_per_share:f64) -> f64
{
    market_price/earnings_per_share
}
