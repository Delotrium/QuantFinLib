//Placeholder / Template

fn main()
{
  let current_value :f64= 540.82;
  let past_value:f64 = 540.82 - 15.77;
  println!("{}%", qfl::finance::ratios::portfolio_return_rate(current_value, past_value) * 600.0);
}
