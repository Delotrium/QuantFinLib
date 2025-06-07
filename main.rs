//Placeholder / Template

fn main()
{
  let list_val = vec![12.5, 12.0];
  let var = vec![9.0, 4.0];
  println!("{:?}", qfl::math::stats::inverse_variance_weighting(&list_val, &var));
}
