
pub fn avg(vals: &Vec<i32>) -> f32 {
    return vals.iter().sum::<i32>() as f32 / vals.len() as f32;
}

pub fn float_avg(vals: &Vec<f32>) -> f32 {
    return vals.iter().sum::<f32>() / vals.len() as f32;
}

pub fn variance(vals: &Vec<i32>) -> f32 {
    let avg_vals = avg(vals);
    let squared_diffs = vals
        .iter()
        .map(|v| (*v as f32 - avg_vals) * (*v as f32 - avg_vals))
        .collect::<Vec<f32>>();
    return float_avg(&squared_diffs);
}

pub fn sd(vals: &Vec<i32>) -> f32 {
    return variance(vals).sqrt();
}

pub fn summarize(vals: &Vec<i32>) -> String {
    return format!("{} ± {}", avg(vals), sd(vals));
}