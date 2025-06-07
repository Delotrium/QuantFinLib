// Discrete stochastic processes and tools.
use rand::seq::SliceRandom; 

pub fn generate_srw(positive_delta:f64, negative_delta:f64, num_trials:usize) -> Vec<f64>
{
    let mut rng = rand::thread_rng();
    let mut list_nums: Vec<f64> = Vec::new();
    let options = [negative_delta, positive_delta];
    let mut value:f64 = 0f64;
    for _ in 0..num_trials {
        let choice = options.choose(&mut rng).unwrap();
        value+=*choice;
        list_nums.push(value);
    }
    list_nums
}

pub fn donchian_breakout(highs: &[f64], lows: &[f64], period: usize) -> (Vec<f64>, Vec<f64>) {
    let mut upper_band = Vec::new();
    let mut lower_band = Vec::new();

    for i in 0..highs.len() {
        if i < period - 1 {
            upper_band.push(0.0);
            lower_band.push(0.0);
        } else {
            let high_max = highs[i - period + 1..=i].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let low_min = lows[i - period + 1..=i].iter().cloned().fold(f64::INFINITY, f64::min);
            upper_band.push(high_max);
            lower_band.push(low_min);
        }
    }

    (upper_band, lower_band)
}