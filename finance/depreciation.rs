pub fn straight_line_depr_ammount(init_value:f64, final_value:f64, useful_life:f64) -> f64
{
    (init_value-final_value)/useful_life
}

pub fn straight_line_depr_table(init_value:f64, final_value:f64, useful_life:f64) -> Vec<f64>
{
    let amount:f64=straight_line_depr_ammount(init_value, final_value, useful_life);
    let mut value_list = Vec::new();
    let mut i = 0f64;
    while i < useful_life + 1.0
    {
        value_list.push(init_value - (i*amount));
        i += 1.0
    }
    value_list
}
