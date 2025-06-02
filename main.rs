//Placeholder / Template


fn main()
{
  let nums = vec![1.0,2.0,3.0,4.0,5.0];
  let var = vec![0.2, 0.1,0.4,0.2,0.0];
 println!("{}", qfl::math::stats::inverse_variance_weighting(&nums, &var));
}
