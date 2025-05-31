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