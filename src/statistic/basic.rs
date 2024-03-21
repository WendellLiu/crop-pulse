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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let data = vec![1, 2, 3, 4, 5];

        let expected_output = vec![0.0, 0.25, 0.5, 0.75, 1.0];

        let result = normalize(&data);

        assert_eq!(result, expected_output);
    }
}
