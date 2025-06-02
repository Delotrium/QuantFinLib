
pub fn d1(stock_price_s:f64, strike_price_k:f64, time_to_mat_t:f64, risk_free_rate_r:f64, volatility_sigma:f64) -> f64
{
    f64::log10((stock_price_s/strike_price_k) + (risk_free_rate_r + 0.5 * volatility_sigma * volatility_sigma) * time_to_mat_t) / (volatility_sigma * f64::sqrt(time_to_mat_t))
}
pub fn d2(stock_price_s:f64, strike_price_k:f64, time_to_mat_t:f64, risk_free_rate_r:f64, volatility_sigma:f64) -> f64
{
    d1(stock_price_s, strike_price_k, time_to_mat_t, risk_free_rate_r, volatility_sigma) - volatility_sigma * f64::sqrt(time_to_mat_t)
}