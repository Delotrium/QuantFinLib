const PI : f64 = 3.14159265358979323846;

pub fn poisson(lambda:f64, x :u64)->f64
{
    (f64::exp(-lambda) * f64::powf(lambda, x as f64))/ crate::Math::basic_probability::factorial(x as f64)
}

pub fn gaussian_distr(mean: f64, sd: f64, x: f64) -> f64 {
    if sd <= 0.0 {
        return f64::NAN; // or panic!("Standard deviation must be positive");
    }
    let coeff = 1.0 / (sd * f64::sqrt(2.0 * PI));
    let exponent = -((x - mean) * (x - mean)) / (2.0 * sd * sd);
    coeff * f64::exp(exponent)
}

pub fn skew(numbers: &[f64]) -> f64
{
    let mean = crate::Math::basic_probability::mean(numbers);
    let sd = crate::Math::basic_probability::standard_deviation_sample(numbers);
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
    let mean = crate::Math::basic_probability::mean(numbers);
    let sd = crate::Math::basic_probability::standard_deviation_sample(numbers);
    let coeff:f64 = 1.0/((numbers.len() as f64) * f64::powf(sd,4.0));
    let mut kurt:f64 =0f64;
    let mut i = 1;
    while i < numbers.len()
    {
        kurt += f64::powf(numbers[i]-mean, 4.0);
        i+=1;
    }
    kurt * coeff - 3.0
}