use super::*;

use std::path::Path;

pub(crate) fn iteration_times_figure(
    title: Option<&str>,
    path: &Path,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    size: Option<(u32, u32)>,
) {
    let data = &measurements.avg_times;
    let max_avg_time = data.max();
    let mut scaled_y: Vec<_> = data.iter().map(|(f, _)| f).collect();
    let unit = formatter.scale_values(max_avg_time, &mut scaled_y);
    let scaled_y = Sample::new(&scaled_y);

    let size = size.unwrap_or(SIZE);
    let root_area = SVGBackend::new(path, size).into_drawing_area();

    let mut cb = ChartBuilder::on(&root_area);
    if let Some(title) = title {
        cb.caption(title, (DEFAULT_FONT, 20));
    }

    let x_range = (1.0)..((data.len() + 1) as f64);
    let y_range = plotters::data::fitting_range(scaled_y.iter());

    let mut chart = cb
        .margin((5).percent())
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(x_range, y_range)
        .unwrap();

    chart
        .configure_mesh()
        .y_desc(format!("Average Iteration Time ({})", unit))
        .x_label_formatter(&|x| pretty_print_float(*x, true))
        .light_line_style(TRANSPARENT)
        .draw()
        .unwrap();

    chart
        .draw_series(
            (1..=data.len())
                .zip(scaled_y.iter())
                .map(|(x, y)| Circle::new((x as f64, *y), POINT_SIZE, DARK_BLUE.filled())),
        )
        .unwrap()
        .label("Sample")
        .legend(|(x, y)| Circle::new((x + 10, y), POINT_SIZE, DARK_BLUE.filled()));

    if title.is_some() {
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .draw()
            .unwrap();
    }
}

pub(crate) fn iteration_times_comparison_figure(
    title: Option<&str>,
    path: &Path,
    formatter: &dyn ValueFormatter,
    measurements: &MeasurementData<'_>,
    comparison: &ComparisonData,
    size: Option<(u32, u32)>,
) {
    let current_data = &measurements.avg_times;
    let base_data = &comparison.base_avg_times;

    let mut all_data: Vec<f64> = current_data.iter().map(|(f, _)| f).collect();
    all_data.extend_from_slice(base_data);

    let typical_value = Sample::new(&all_data).max();
    let unit = formatter.scale_values(typical_value, &mut all_data);

    let (scaled_current_y, scaled_base_y) = all_data.split_at(current_data.len());
    let scaled_current_y = Sample::new(scaled_current_y);
    let scaled_base_y = Sample::new(scaled_base_y);

    let size = size.unwrap_or(SIZE);
    let root_area = SVGBackend::new(path, size).into_drawing_area();

    let mut cb = ChartBuilder::on(&root_area);
    if let Some(title) = title {
        cb.caption(title, (DEFAULT_FONT, 20));
    }

    let max_samples = current_data.len().max(base_data.len()) as f64;

    let y_range = plotters::data::fitting_range(all_data.iter());

    let mut chart = cb
        .margin((5).percent())
        .set_label_area_size(LabelAreaPosition::Left, (5).percent_width().min(60))
        .set_label_area_size(LabelAreaPosition::Bottom, (5).percent_height().min(40))
        .build_cartesian_2d(0.0..max_samples, y_range)
        .unwrap();

    chart
        .configure_mesh()
        .y_desc(format!("Average Iteration Time ({})", unit))
        .x_label_formatter(&|x| pretty_print_float(*x, true))
        .light_line_style(TRANSPARENT)
        .draw()
        .unwrap();

    chart
        .draw_series(
            (1..=current_data.len())
                .zip(scaled_current_y.iter())
                .map(|(x, y)| Circle::new((x as f64, *y), POINT_SIZE, DARK_BLUE.filled())),
        )
        .unwrap()
        .label("Current")
        .legend(|(x, y)| Circle::new((x + 10, y), POINT_SIZE, DARK_BLUE.filled()));

    chart
        .draw_series(
            (1..=base_data.len())
                .zip(scaled_base_y.iter())
                .map(|(x, y)| Circle::new((x as f64, *y), POINT_SIZE, DARK_RED.filled())),
        )
        .unwrap()
        .label("Base")
        .legend(|(x, y)| Circle::new((x + 10, y), POINT_SIZE, DARK_RED.filled()));

    if title.is_some() {
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .draw()
            .unwrap();
    }
}
