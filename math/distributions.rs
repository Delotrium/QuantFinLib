// Modeling and calculations for mathematic distributions

const PI : f64 = std::f64::consts::PI;

pub fn poisson(lambda:f64, x :u64)->f64
{
    (f64::exp(-lambda) * f64::powf(lambda, x as f64))/ crate::math::stats::factorial(x as f64)
}

pub fn gaussian_distr(mean: f64, sd: f64, x: f64) -> f64 {
    if sd <= 0.0 {
        return f64::NAN;
    }
    let coeff = 1.0 / (sd * f64::sqrt(2.0 * PI));
    let exponent = -((x - mean) * (x - mean)) / (2.0 * sd * sd);
    coeff * f64::exp(exponent)
}

pub fn skew(numbers: &[f64]) -> f64
{
    let mean = crate::math::stats::mean(numbers);
    let sd = crate::math::stats::standard_deviation_sample(numbers);
    let coeff:f64 = 1.0/((numbers.len() as f64) * f64::powf(sd,3.0));
    let mut skew:f64 =0f64;
    let mut i = 1;
    while i< numbers.len()
    {
        skew += f64::powf(numbers[i] - mean, 3.0);
        i+=1;
    }
    skew * coeff
}

pub fn kurtosis(numbers: &[f64]) -> f64
{
    let mean = crate::math::stats::mean(numbers);
    let sd = crate::math::stats::standard_deviation_sample(numbers);
    let coeff:f64 = 1.0/((numbers.len() as f64) * f64::powf(sd,4.0));
    let mut kurt:f64 =0f64;
    let mut i = 1;
    while i < numbers.len() - 1
    {
        kurt += f64::powf(numbers[i]-mean, 4.0);
        i+=1;
    }
    kurt * coeff - 3.0
}

pub fn erf(x: f64) -> f64 {
    let a = [
        1.061405429, -1.453152027, 1.421413741,
        -0.284496736, 0.254829592,
    ];
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = (((((a[4] * t + a[3]) * t + a[2]) * t + a[1]) * t + a[0]) * t)
        * (-x * x).exp();

    sign * (1.0 - y)
}

pub fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}