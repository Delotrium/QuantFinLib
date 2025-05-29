const PI : f64 = 3.14159265358979323846;

pub fn poisson(lambda:f64, x :u64)->f64
{
    (f64::exp(-lambda) * f64::powf(lambda, x as f64))/ crate::Math::BasicProbability::Factorial(x as f64)
}

pub fn gaussian_distr(mean: f64, sd: f64, x: f64) -> f64 {
    if sd <= 0.0 {
        return f64::NAN; // or panic!("Standard deviation must be positive");
    }
    let coeff = 1.0 / (sd * f64::sqrt(2.0 * PI));
    let exponent = -((x - mean) * (x - mean)) / (2.0 * sd * sd);
    coeff * f64::exp(exponent)
}
