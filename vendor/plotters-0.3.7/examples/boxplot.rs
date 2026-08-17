use itertools::Itertools;
use plotters::data::fitting_range;
use plotters::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};

fn read_data<BR: BufRead>(reader: BR) -> HashMap<(String, String), Vec<f64>> {
    let mut ds = HashMap::new();
    for l in reader.lines() {
        let line = l.unwrap();
        let tuple: Vec<&str> = line.split('\t').collect();
        if tuple.len() == 3 {
            let key = (String::from(tuple[0]), String::from(tuple[1]));
            let entry = ds.entry(key).or_insert_with(Vec::new);
            entry.push(tuple[2].parse::<f64>().unwrap());
        }
    }
    ds
}

const OUT_FILE_NAME: &str = "plotters-doc-data/boxplot.svg";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let root = root.margin(5, 5, 5, 5);

    let (upper, lower) = root.split_vertically(512);

    let args: Vec<String> = env::args().collect();

    let ds = if args.len() < 2 {
        read_data(io::Cursor::new(get_data()))
    } else {
        let file = fs::File::open(&args[1])?;
        read_data(BufReader::new(file))
    };
    let dataset: Vec<(String, String, Quartiles)> = ds
        .iter()
        .map(|(k, v)| (k.0.clone(), k.1.clone(), Quartiles::new(v)))
        .collect();

    let host_list: Vec<_> = dataset
        .iter()
        .unique_by(|x| x.0.clone())
        .sorted_by(|a, b| b.2.median().partial_cmp(&a.2.median()).unwrap())
        .map(|x| x.0.clone())
        .collect();

    let mut colors = (0..).map(Palette99::pick);
    let mut offsets = (-12..).step_by(24);
    let mut series = BTreeMap::new();
    for x in dataset.iter() {
        let entry = series
            .entry(x.1.clone())
            .or_insert_with(|| (Vec::new(), colors.next().unwrap(), offsets.next().unwrap()));
        entry.0.push((x.0.clone(), &x.2));
    }

    let values: Vec<f32> = dataset.iter().flat_map(|x| x.2.values().to_vec()).collect();
    let values_range = fitting_range(values.iter());

    let mut chart = ChartBuilder::on(&upper)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .caption("Ping Boxplot", ("sans-serif", 20))
        .build_cartesian_2d(
            values_range.start - 1.0..values_range.end + 1.0,
            host_list[..].into_segmented(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Ping, ms")
        .y_desc("Host")
        .y_labels(host_list.len())
        .light_line_style(WHITE)
        .draw()?;

    for (label, (values, style, offset)) in &series {
        chart
            .draw_series(values.iter().map(|x| {
                Boxplot::new_horizontal(SegmentValue::CenterOf(&x.0), x.1)
                    .width(20)
                    .whisker_width(0.5)
                    .style(style)
                    .offset(*offset)
            }))?
            .label(label)
            .legend(move |(x, y)| Rectangle::new([(x, y - 6), (x + 12, y + 6)], style.filled()));
    }
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .border_style(BLACK.mix(0.5))
        .legend_area_size(22)
        .draw()?;

    let drawing_areas = lower.split_evenly((1, 2));
    let (left, right) = (&drawing_areas[0], &drawing_areas[1]);

    let quartiles_a = Quartiles::new(&[
        6.0, 7.0, 15.9, 36.9, 39.0, 40.0, 41.0, 42.0, 43.0, 47.0, 49.0,
    ]);
    let quartiles_b = Quartiles::new(&[16.0, 17.0, 50.0, 60.0, 40.2, 41.3, 42.7, 43.3, 47.0]);

    let ab_axis = ["a", "b"];

    let values_range = fitting_range(
        quartiles_a
            .values()
            .iter()
            .chain(quartiles_b.values().iter()),
    );
    let mut chart = ChartBuilder::on(left)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Vertical Boxplot", ("sans-serif", 20))
        .build_cartesian_2d(
            ab_axis[..].into_segmented(),
            values_range.start - 10.0..values_range.end + 10.0,
        )?;

    chart.configure_mesh().light_line_style(WHITE).draw()?;
    chart.draw_series(vec![
        Boxplot::new_vertical(SegmentValue::CenterOf(&"a"), &quartiles_a),
        Boxplot::new_vertical(SegmentValue::CenterOf(&"b"), &quartiles_b),
    ])?;

    let mut chart = ChartBuilder::on(right)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Horizontal Boxplot", ("sans-serif", 20))
        .build_cartesian_2d(-30f32..90f32, 0..3)?;

    chart.configure_mesh().light_line_style(WHITE).draw()?;
    chart.draw_series(vec![
        Boxplot::new_horizontal(1, &quartiles_a),
        Boxplot::new_horizontal(2, &Quartiles::new(&[30])),
    ])?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}

fn get_data() -> String {
    String::from(
        "
 1.1.1.1	wireless	41.6
 1.1.1.1	wireless	32.5
 1.1.1.1	wireless	33.1
 1.1.1.1	wireless	32.3
 1.1.1.1	wireless	36.7
 1.1.1.1	wireless	32.0
 1.1.1.1	wireless	33.1
 1.1.1.1	wireless	32.0
 1.1.1.1	wireless	32.9
 1.1.1.1	wireless	32.7
 1.1.1.1	wireless	34.5
 1.1.1.1	wireless	36.5
 1.1.1.1	wireless	31.9
 1.1.1.1	wireless	33.7
 1.1.1.1	wireless	32.6
 1.1.1.1	wireless	35.1
 8.8.8.8	wireless	42.3
 8.8.8.8	wireless	32.9
 8.8.8.8	wireless	32.9
 8.8.8.8	wireless	34.3
 8.8.8.8	wireless	32.0
 8.8.8.8	wireless	33.3
 8.8.8.8	wireless	31.5
 8.8.8.8	wireless	33.1
 8.8.8.8	wireless	33.2
 8.8.8.8	wireless	35.9
 8.8.8.8	wireless	42.3
 8.8.8.8	wireless	34.1
 8.8.8.8	wireless	34.2
 8.8.8.8	wireless	34.2
 8.8.8.8	wireless	32.4
 8.8.8.8	wireless	33.0
 1.1.1.1	wired	31.8
 1.1.1.1	wired	28.6
 1.1.1.1	wired	29.4
 1.1.1.1	wired	28.8
 1.1.1.1	wired	28.2
 1.1.1.1	wired	28.8
 1.1.1.1	wired	28.4
 1.1.1.1	wired	28.6
 1.1.1.1	wired	28.3
 1.1.1.1	wired	28.5
 1.1.1.1	wired	28.5
 1.1.1.1	wired	28.5
 1.1.1.1	wired	28.4
 1.1.1.1	wired	28.6
 1.1.1.1	wired	28.4
 1.1.1.1	wired	28.9
 8.8.8.8	wired	33.3
 8.8.8.8	wired	28.4
 8.8.8.8	wired	28.7
 8.8.8.8	wired	29.1
 8.8.8.8	wired	29.6
 8.8.8.8	wired	28.9
 8.8.8.8	wired	28.6
 8.8.8.8	wired	29.3
 8.8.8.8	wired	28.6
 8.8.8.8	wired	29.1
 8.8.8.8	wired	28.7
 8.8.8.8	wired	28.3
 8.8.8.8	wired	28.3
 8.8.8.8	wired	28.6
 8.8.8.8	wired	29.4
 8.8.8.8	wired	33.1
",
    )
}
#[test]
fn entry_point() {
    main().unwrap()
}
