//Placeholder / Template


fn main()
{
  println!("Estimated Options Price: ${}", qfl::finance::options::black_scholes_price(50f64, 120f64, 3f64, 0.02, 3f64, 0f64, true));
}
