pub fn normalize<T>(data: &[T]) -> Vec<f64>
where
    T: Into<f64> + Clone,
{
    let converted_data: Vec<f64> = data.iter().cloned().map(|x| x.into()).collect();

    let min = converted_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = converted_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    converted_data
        .iter()
        .map(|x| (x - min) / (max - min))
        .collect()
}
