pub fn factorial(number: f64) -> f64 {
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

    let n_fact = factorial(n as f64);
    let k_fact = factorial(k as f64);
    let n_minus_k_fact = factorial((n - k) as f64);

    let combination = n_fact / (k_fact * n_minus_k_fact);

    combination * p.powf(k as f64) * (1.0 - p).powf((n - k) as f64)
}

pub fn binom_cdf(observerd_prob: f64, numtrials : u64, mut lower_limit: u64, upper_limit:u64) -> f64
{
    let mut cdf : f64 = 0f64;
    while lower_limit < upper_limit
    {
        cdf = cdf + binom_pdf(observerd_prob, numtrials, lower_limit);
        lower_limit += 1;
    }
    cdf
}

pub fn binom_ev(num_trials :f64, prob_success : f64) -> f64
{
    num_trials * prob_success
}

pub fn binom_var(num_trials:f64, prob_success:f64) -> f64
{
    num_trials * prob_success * (1f64-prob_success)
}

pub fn sum(numbers: &[f64]) -> f64
{
    let mut sum:f64=0f64;
    for n in numbers
    {
        sum += n;
    }
    sum
}

pub fn mean(numbers: &[f64]) -> f64
{
    let total_sum:f64 = sum(numbers);
    total_sum / numbers.len() as f64
}

pub fn expected_value(numbers: &[f64], probabilities: &[f64]) -> f64
{
    let mut i  =1;
    let mut val = 0f64;
    while i<numbers.len()
    {
        val = val + (numbers[i]*probabilities[i]);
        i+=1;
    }
    val
}

pub fn var_sample(numbers: &[f64]) -> f64
{
    let mut i  =1;
    let mut val = 0f64;
    let mean = mean(numbers);
    while i<numbers.len()
    {
        val += f64::powf(numbers[i]-mean, 2f64);
        i+=1;
    }
    val / (numbers.len()-1) as f64
}

pub fn var_pop(numbers: &[f64]) -> f64
{
    let mut i  =1;
    let mut val = 0f64;
    let mean = mean(numbers);
    while i<numbers.len()
    {
        val += f64::powf(numbers[i]-mean, 2f64);
        i+=1;
    }
    val / numbers.len() as f64
}

pub fn standard_deviation_sample(numbers: &[f64]) -> f64
{
    f64::sqrt(var_sample(numbers))
}

pub fn standard_deviation_pop(numbers: &[f64]) -> f64
{
    f64::sqrt(var_pop(numbers))
}

pub fn geometric_mean(numbers: &[f64]) -> f64
{
    let mut i = 0;
    let mut val = 0f64;
    while i < numbers.len()
    {
        val *= numbers[i];
        i+=1;
    }
    f64::powf(val, 1.0/numbers.len() as f64)
}

pub fn downside_deviation(numbers: &[f64], mar: f64) -> f64 {
    if numbers.is_empty() {
        return f64::NAN;
    }
    let mut sum_sq = 0.0;
    let mut count = 0;
    for &x in numbers {
        if x < mar {
            sum_sq += (x - mar).powi(2);
            count += 1;
        }
    }
    if count == 0 {
        return 0.0;
    }
    (sum_sq / count as f64).sqrt()
}

