#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

    let y = vec![1., 1., 1., 1., 1.];
    let x1 = vec![5., 4., 3., 2., 1.];
    let data = vec![("Y", y), ("X1", x1)];
    let data = RegressionDataBuilder::new().build_from(data)?;
    let formula = "Y ~ X1";
    let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula(formula)
        .fit()?;
    let parameters: Vec<_> = model.iter_parameter_pairs().collect();

    println!("{:?}", parameters);

    Ok(())
}
