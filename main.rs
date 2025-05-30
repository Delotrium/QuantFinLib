//Placeholder / Template
use QuantFinLib::Math;

fn main()
{
  let mut list_nums:Vec<f64> = vec![1.0,2.0,3.0,4.0,5.0];
  println!("The total sum is; {}", Math::basic_probability::sum(&list_nums));
  println!("The mean value is; {}", Math::basic_probability::mean(&list_nums));
  list_nums.push(13f64);
  println!("The total sum is; {}", Math::basic_probability::sum(&list_nums));
  println!("The mean value is; {}", Math::basic_probability::mean(&list_nums));
}
