
fn main()
{
  qfl::engine::init_stocks::write_csv_to_bin("NVDA_stock_data.csv", "NVDA.bin").expect("ERROR LOADING CSV TO BIN");
  let values = qfl::engine::init_stocks::bin_to_vec("NVDA.bin").expect("ERROR LOADING BIN TO VEC");
  let prices: Vec<f64> = values.iter().map(|p| p.price).collect();
  let prices: &[f64] = &prices;
  println!("Mean: {}, SD: {}, GM: {}, AR: {}%, ALR: {}%", 
    qfl::math::stats::mean(prices), 
    qfl::math::stats::standard_deviation_sample(prices), 
    qfl::math::stats::geometric_mean(prices),
    qfl::finance::ratios::average_returns(prices),
    qfl::finance::ratios::average_log_returns(prices));
}
