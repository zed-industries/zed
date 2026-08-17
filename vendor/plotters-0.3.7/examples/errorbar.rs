use plotters::prelude::*;

use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rand_xorshift::XorShiftRng;

use itertools::Itertools;

use num_traits::sign::Signed;

const OUT_FILE_NAME: &str = "plotters-doc-data/errorbar.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = generate_random_data();
    let down_sampled = down_sample(&data[..]);

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Linear Function with Noise", ("sans-serif", 60))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(-10f64..10f64, -10f64..10f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(data, &GREEN.mix(0.3)))?
        .label("Raw Data")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

    chart.draw_series(LineSeries::new(
        down_sampled.iter().map(|(x, _, y, _)| (*x, *y)),
        &BLUE,
    ))?;

    chart
        .draw_series(
            down_sampled.iter().map(|(x, yl, ym, yh)| {
                ErrorBar::new_vertical(*x, *yl, *ym, *yh, BLUE.filled(), 20)
            }),
        )?
        .label("Down-sampled")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .configure_series_labels()
        .background_style(WHITE.filled())
        .draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn generate_random_data() -> Vec<(f64, f64)> {
    let norm_dist = Normal::new(0.0, 1.0).unwrap();
    let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
    let x_iter = norm_dist.sample_iter(&mut x_rand);
    x_iter
        .take(20000)
        .filter(|x| x.abs() <= 4.0)
        .zip(-10000..10000)
        .map(|(yn, x)| {
            (
                x as f64 / 1000.0,
                x as f64 / 1000.0 + yn * x as f64 / 10000.0,
            )
        })
        .collect()
}

fn down_sample(data: &[(f64, f64)]) -> Vec<(f64, f64, f64, f64)> {
    let down_sampled: Vec<_> = data
        .iter()
        .group_by(|x| (x.0 * 1.0).round() / 1.0)
        .into_iter()
        .map(|(x, g)| {
            let mut g: Vec<_> = g.map(|(_, y)| *y).collect();
            g.sort_by(|a, b| a.partial_cmp(b).unwrap());
            (
                x,
                g[0],
                g.iter().sum::<f64>() / g.len() as f64,
                g[g.len() - 1],
            )
        })
        .collect();
    down_sampled
}
#[test]
fn entry_point() {
    main().unwrap()
}
