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