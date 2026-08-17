use plotters::prelude::*;

const OUT_FILE_NAME: &str = "plotters-doc-data/colormaps.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let colormaps_rgb: [(Box<dyn ColorMap<RGBColor>>, &str); 4] = [
        (Box::new(ViridisRGB {}), "Viridis"),
        (Box::new(BlackWhite {}), "BlackWhite"),
        (Box::new(Bone {}), "Bone"),
        (Box::new(Copper {}), "Copper"),
    ];

    let colormaps_hsl: [(Box<dyn ColorMap<HSLColor>>, &str); 2] = [
        (Box::new(MandelbrotHSL {}), "MandelbrotHSL"),
        (Box::new(VulcanoHSL {}), "VulcanoHSL"),
    ];

    let size_x: i32 = 800;
    let n_colormaps = colormaps_rgb.len() + colormaps_hsl.len();
    let size_y = 200 + n_colormaps as u32 * 100;
    let root = BitMapBackend::new(OUT_FILE_NAME, (size_x as u32, size_y)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Demonstration of predefined colormaps", ("sans-serif", 20))
        .build_cartesian_2d(
            -150.0..size_x as f32 + 50.0,
            0.0..3.0 * (n_colormaps as f32),
        )?;

    use plotters::style::text_anchor::*;
    let centered = Pos::new(HPos::Center, VPos::Center);
    let label_style = TextStyle::from(("monospace", 14.0).into_font()).pos(centered);

    let mut colormap_counter = 0;
    macro_rules! plot_colormaps(
        ($colormap:expr) => {
            for (colormap, colormap_name) in $colormap.iter() {
                chart.draw_series(
                    (0..size_x as i32).map(|x| {
                        Rectangle::new([
                            (x as f32,     3.0*(n_colormaps - 1 - colormap_counter) as f32 + 0.5),
                            (x as f32+1.0, 3.0*(n_colormaps - 1 - colormap_counter) as f32 + 2.5)
                        ],
                    colormap.get_color_normalized(x as f32, 0.0, size_x as f32).filled())
                    })
                )?;
                chart.draw_series(
                    [Text::new(colormap_name.to_owned(), (-75.0, 3.0*(n_colormaps-1-colormap_counter) as f32 + 1.5), &label_style)]
                )?;
                colormap_counter+=1;
            }
        }
    );

    plot_colormaps!(colormaps_rgb);
    plot_colormaps!(colormaps_hsl);

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
#[test]
fn entry_point() {
    main().unwrap()
}
