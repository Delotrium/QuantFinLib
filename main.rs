//Placeholder / Template


fn main()
{
  println!("Estimated Options Price: ${}", qfl::finance::options::black_scholes_price(100f64, 110f64, 3f64, 0.04, 3f64, 0f64, true));
  
}
