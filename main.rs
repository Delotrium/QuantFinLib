//Placeholder / Template


fn main()
{
  let list_nums:Vec<f64> = vec![1.0,2.0,3.0,4.0,7.0];
  println!("The Sharpe ratio from list is; {}", qfl::finance::ratios::sharpe_list(&list_nums, 0.02));
  println!("The Sortino ratio from list is; {}", qfl::finance::ratios::sortino_list(&list_nums, 0.02));
  println!("{}", qfl::math::stats::downside_deviation(&list_nums,0.02));
}
