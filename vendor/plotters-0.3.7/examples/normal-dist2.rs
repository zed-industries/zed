use plotters::prelude::*;

use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rand_xorshift::XorShiftRng;

use num_traits::sign::Signed;

const OUT_FILE_NAME: &str = "plotters-doc-data/normal-dist2.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sd = 0.60;

    let random_points: Vec<f64> = {
        let norm_dist = Normal::new(0.0, sd).unwrap();
        let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
        let x_iter = norm_dist.sample_iter(&mut x_rand);
        x_iter.take(5000).filter(|x| x.abs() <= 4.0).collect()
    };

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .caption("1D Gaussian Distribution Demo", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .set_label_area_size(LabelAreaPosition::Right, 60)
        .build_cartesian_2d(-4f64..4f64, 0f64..0.1)?
        .set_secondary_coord(
            (-4f64..4f64).step(0.1).use_round().into_segmented(),
            0u32..500u32,
        );

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_label_formatter(&|y| format!("{:.0}%", *y * 100.0))
        .y_desc("Percentage")
        .draw()?;

    chart.configure_secondary_axes().y_desc("Count").draw()?;

    let actual = Histogram::vertical(chart.borrow_secondary())
        .style(GREEN.filled())
        .margin(3)
        .data(random_points.iter().map(|x| (*x, 1)));

    chart
        .draw_secondary_series(actual)?
        .label("Observed")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], GREEN.filled()));

    let pdf = LineSeries::new(
        (-400..400).map(|x| x as f64 / 100.0).map(|x| {
            (
                x,
                (-x * x / 2.0 / sd / sd).exp() / (2.0 * std::f64::consts::PI * sd * sd).sqrt()
                    * 0.1,
            )
        }),
        &RED,
    );

    chart
        .draw_series(pdf)?
        .label("PDF")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.filled()));

    chart.configure_series_labels().draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
#[test]
fn entry_point() {
    main().unwrap()
}
