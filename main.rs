//Placeholder / Template
use qfl::math;

fn main()
{
  let mut list_nums:Vec<f64> = vec![1.0,2.0,3.0,4.0,5.0];
  println!("The total sum is; {}", math::basic_probability::sum(&list_nums));
  println!("The mean value is; {}", math::basic_probability::mean(&list_nums));
  list_nums.push(13f64);
  println!("The total sum is; {}", math::basic_probability::sum(&list_nums));
  println!("The mean value is; {}", math::basic_probability::mean(&list_nums));
}
