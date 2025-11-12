

pub fn d1(stock_price_s:f64, strike_price_k:f64, time_to_mat_t:f64, risk_free_rate_r:f64, volatility_sigma:f64) -> f64
{
    f64::log10((stock_price_s/strike_price_k) + (risk_free_rate_r + 0.5 * volatility_sigma * volatility_sigma) * time_to_mat_t) / (volatility_sigma * f64::sqrt(time_to_mat_t))
}
pub fn d2(stock_price_s:f64, strike_price_k:f64, time_to_mat_t:f64, risk_free_rate_r:f64, volatility_sigma:f64) -> f64
{
    d1(stock_price_s, strike_price_k, time_to_mat_t, risk_free_rate_r, volatility_sigma) - volatility_sigma * f64::sqrt(time_to_mat_t)
}

pub fn black_scholes_price(stock_price_s:f64, strike_price_k:f64, time_to_mat_t:f64, risk_free_rate_r:f64, volatility_sigma:f64, dividend_returns_q:f64, is_call:bool) -> f64
{
    let d_1 = d1(stock_price_s, strike_price_k, time_to_mat_t, risk_free_rate_r, volatility_sigma);
    let d_2 = d2(stock_price_s, strike_price_k, time_to_mat_t, risk_free_rate_r, volatility_sigma);
    if is_call
    {
        stock_price_s * f64::exp(- dividend_returns_q * time_to_mat_t) * crate::math::distributions::norm_cdf(d_1) - strike_price_k * f64::exp(- risk_free_rate_r * time_to_mat_t) * crate::math::distributions::norm_cdf(d_2)
    }
    else 
    {
        strike_price_k * f64::exp(- risk_free_rate_r * time_to_mat_t) * crate::math::distributions::norm_cdf(-d_2) - stock_price_s * f64::exp(- dividend_returns_q * time_to_mat_t) * crate::math::distributions::norm_cdf(-d_1)
    }
}

pub fn delta(stock_price_s:f64, strike_price_k:f64, time_to_mat:f64, risk_free_rate:f64, sigma:f64, dividend_q:f64, is_call:bool) -> f64
{
    if is_call
    {
        f64::exp(-dividend_q * time_to_mat) * crate::math::distributions::norm_cdf(d1(stock_price_s, strike_price_k, time_to_mat, risk_free_rate, sigma))
    }
    else 
    {
        -f64::exp(-dividend_q * time_to_mat) * crate::math::distributions::norm_cdf(-d1(stock_price_s, strike_price_k, time_to_mat, risk_free_rate, sigma))
    }
}

pub fn vega(stock_price_s:f64, strike_price_k:f64, time_to_mat:f64, risk_free_rate:f64, sigma:f64, dividend_q:f64) -> f64
{
    let d_1:f64 = d1(stock_price_s, strike_price_k, time_to_mat, risk_free_rate, sigma);
    stock_price_s * f64::exp(-dividend_q * time_to_mat) * crate::math::distributions::gaussian_distr(0f64, 1f64, d_1)
}
