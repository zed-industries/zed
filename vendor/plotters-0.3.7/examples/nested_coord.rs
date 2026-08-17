use plotters::prelude::*;
const OUT_FILE_NAME: &str = "plotters-doc-data/nested_coord.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Nested Coord", ("sans-serif", 50.0))
        .build_cartesian_2d(
            ["Linear", "Quadratic"].nested_coord(|_| 0.0..10.0),
            0.0..10.0,
        )?;

    chart
        .configure_mesh()
        .disable_mesh()
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..10)
            .map(|x| x as f64 / 1.0)
            .map(|x| ((&"Linear", x).into(), x)),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        (0..10)
            .map(|x| x as f64 / 1.0)
            .map(|x| ((&"Quadratic", x).into(), x * x / 10.0)),
        &RED,
    ))?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
#[test]
fn entry_point() {
    main().unwrap()
}
