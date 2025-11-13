
fn main()
{
  let values = vec![1.0, 2.0, 3.0, 4.0, 6.0];
  println!("SD: {}, AM: {}, Sum: {}", qfl::math::stats::standard_deviation_sample(&values), qfl::math::stats::mean(&values), qfl::math::stats::sum(&values));
}
