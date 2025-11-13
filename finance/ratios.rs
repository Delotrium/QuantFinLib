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

pub fn portfolio_return_rate(
    current_value: f64,
    past_value: f64,
) -> f64 {
    (current_value - past_value) / past_value
}

pub fn average_returns(prices: &[f64]) -> f64 {
    if prices.len() < 2 {
        return f64::NAN; 
    }
    let mut sum = 0.0;
    let mut count = 0;
    for i in 1..prices.len() {
        let r = (prices[i] - prices[i - 1]) / prices[i - 1];
        sum += r;
        count += 1;
    }
    sum / count as f64
}

pub fn average_log_returns(prices: &[f64]) -> f64 {
    if prices.len() < 2 {
        return f64::NAN;
    }
    let mut sum = 0.0;
    for i in 1..prices.len() {
        sum += (prices[i] / prices[i - 1]).ln();
    }
    sum 
}