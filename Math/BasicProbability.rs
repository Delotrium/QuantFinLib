pub fn Factorial(number: f64) -> f64 {
    if number < 0.0 {
        return 0.0; // Factorial is not defined for negative numbers
    }
    if number == 0.0 || number == 1.0 {
        return 1.0; // Base case: 0! = 1! = 1
    }
    let mut result = 1.0;
    for i in 2..=number as u64 {
        result *= i as f64;
    }
    result
}
pub fn binom_pdf(p: f64, n: u64, k: u64) -> f64 {
    if k > n {
        return 0.0;
    }

    let n_fact = factorial(n);
    let k_fact = factorial(k);
    let n_minus_k_fact = factorial(n - k);

    let combination = n_fact / (k_fact * n_minus_k_fact);

    combination * p.powf(k as f64) * (1.0 - p).powf((n - k) as f64)
}
