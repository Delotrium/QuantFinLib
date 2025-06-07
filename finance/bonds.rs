//Functions relating to bond and bond valuing

pub fn zcb(facevalue:f64,rate:f64,time:f64,maturity:f64) -> f64
{
    facevalue/f64::powf(1f64+rate,maturity-time)
}

pub fn annuity_value(annuity_ammount:f64, amount_of_periods:f64, rate:f64, periods_per_year:f64) -> f64
{
    let geo_r:f64 = f64::powf(1f64+(rate/periods_per_year), -periods_per_year);
    let sum:f64 = &geo_r * (1.0-f64::powf(geo_r, amount_of_periods)) / (1.0-&geo_r);
    annuity_ammount * sum
}

pub fn market_value(principal:f64, _yield:f64, current_time:f64, redepmtion_time:f64, coupon:f64, coupons_remaining:f64) -> f64
{
    let inintal_term:f64 = principal * f64::exp(- _yield * (current_time - redepmtion_time));
    let mut sum:f64 = 0.0;
    for i in 0..(coupons_remaining as usize) {
        sum += coupon * f64::exp(- _yield * (current_time + i as f64 - redepmtion_time));
    }
    inintal_term + sum
}

pub fn yield_to_maturity(face_value:f64, coupon_rate:f64, current_price:f64, time_to_maturity:f64) -> f64
{
    let mut ytm = 0.05; // Initial guess
    let mut diff:f64 = 1.0;
    let tolerance = 1e-6;

    while diff.abs() > tolerance {
        let price = market_value(face_value, ytm, 0.0, time_to_maturity, face_value * coupon_rate, time_to_maturity);
        diff = current_price - price;
        ytm += diff / (face_value * coupon_rate * time_to_maturity); // Adjust ytm based on the difference
    }
    ytm
}