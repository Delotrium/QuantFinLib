pub fn compound_interest_cont(principal:f64, rate:f64, time:f64)-> f64
{
    principal * f64::exp(rate*time)
}

pub fn compound_interest_periodic(principal:f64, rate:f64, periods:f64, periods_pa:f64) -> f64
{
    principal * f64::powf(1f64+(rate/periods_pa), periods*periods_pa)
}

pub fn simple_interest(principal:f64, rate:f64, time:f64) ->f64
{
    principal * rate * time + principal
}