use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

use crate::statistic::basic::normalize;

pub fn calculate_serial_beta_coefficient(serial: &Vec<f64>) -> f64 {
    let x = (1..serial.len() + 1).map(|x| x as f64).collect();

    let y = normalize(serial);

    let material = vec![("Y", y), ("X", x)];
    let data = RegressionDataBuilder::new().build_from(material).unwrap();
    let formula = "Y ~ X";
    let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula(formula)
        .fit()
        .unwrap();
    let parameters: Vec<_> = model.iter_parameter_pairs().collect();

    parameters[0].1
}
