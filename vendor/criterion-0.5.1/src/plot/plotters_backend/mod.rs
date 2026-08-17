use super::{PlotContext, PlotData, Plotter};
use crate::measurement::ValueFormatter;
use crate::report::{BenchmarkId, ComparisonData, MeasurementData, ValueType};
use plotters::data::float::pretty_print_float;
use plotters::prelude::*;

use crate::kde;
use crate::stats::bivariate::Data;
use crate::stats::univariate::Sample;

static DEFAULT_FONT: FontFamily = FontFamily::SansSerif;
static KDE_POINTS: usize = 500;
static SIZE: (u32, u32) = (960, 540);
static POINT_SIZE: u32 = 3;

const DARK_BLUE: RGBColor = RGBColor(31, 120, 180);
const DARK_ORANGE: RGBColor = RGBColor(255, 127, 0);
const DARK_RED: RGBColor = RGBColor(227, 26, 28);

mod distributions;
mod iteration_times;
mod pdf;
mod regression;
mod summary;
mod t_test;

fn convert_size(size: Option<(usize, usize)>) -> Option<(u32, u32)> {
    if let Some((w, h)) = size {
        return Some((w as u32, h as u32));
    }
    None
}
#[derive(Default)]
pub struct PlottersBackend;

#[allow(unused_variables)]
impl Plotter for PlottersBackend {
    fn pdf(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        if let Some(cmp) = data.comparison {
            let (path, title) = if ctx.is_thumbnail {
                (
                    ctx.context.report_path(ctx.id, "relative_pdf_small.svg"),
                    None,
                )
            } else {
                (
                    ctx.context.report_path(ctx.id, "both/pdf.svg"),
                    Some(ctx.id.as_title()),
                )
            };
            pdf::pdf_comparison_figure(
                path.as_ref(),
                title,
                data.formatter,
                data.measurements,
                cmp,
                convert_size(ctx.size),
            );
            return;
        }
        if ctx.is_thumbnail {
            pdf::pdf_small(
                ctx.id,
                ctx.context,
                data.formatter,
                data.measurements,
                convert_size(ctx.size),
            );
        } else {
            pdf::pdf(
                ctx.id,
                ctx.context,
                data.formatter,
                data.measurements,
                convert_size(ctx.size),
            );
        }
    }

    fn regression(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        let (title, path) = match (data.comparison.is_some(), ctx.is_thumbnail) {
            (true, true) => (
                None,
                ctx.context
                    .report_path(ctx.id, "relative_regression_small.svg"),
            ),
            (true, false) => (
                Some(ctx.id.as_title()),
                ctx.context.report_path(ctx.id, "both/regression.svg"),
            ),
            (false, true) => (
                None,
                ctx.context.report_path(ctx.id, "regression_small.svg"),
            ),
            (false, false) => (
                Some(ctx.id.as_title()),
                ctx.context.report_path(ctx.id, "regression.svg"),
            ),
        };

        if let Some(cmp) = data.comparison {
            let base_data = Data::new(&cmp.base_iter_counts, &cmp.base_sample_times);
            regression::regression_comparison_figure(
                title,
                path.as_path(),
                data.formatter,
                data.measurements,
                cmp,
                &base_data,
                convert_size(ctx.size),
            );
        } else {
            regression::regression_figure(
                title,
                path.as_path(),
                data.formatter,
                data.measurements,
                convert_size(ctx.size),
            );
        }
    }

    fn iteration_times(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        let (title, path) = match (data.comparison.is_some(), ctx.is_thumbnail) {
            (true, true) => (
                None,
                ctx.context
                    .report_path(ctx.id, "relative_iteration_times_small.svg"),
            ),
            (true, false) => (
                Some(ctx.id.as_title()),
                ctx.context.report_path(ctx.id, "both/iteration_times.svg"),
            ),
            (false, true) => (
                None,
                ctx.context.report_path(ctx.id, "iteration_times_small.svg"),
            ),
            (false, false) => (
                Some(ctx.id.as_title()),
                ctx.context.report_path(ctx.id, "iteration_times.svg"),
            ),
        };

        if let Some(cmp) = data.comparison {
            let base_data = Data::new(&cmp.base_iter_counts, &cmp.base_sample_times);
            iteration_times::iteration_times_comparison_figure(
                title,
                path.as_path(),
                data.formatter,
                data.measurements,
                cmp,
                convert_size(ctx.size),
            );
        } else {
            iteration_times::iteration_times_figure(
                title,
                path.as_path(),
                data.formatter,
                data.measurements,
                convert_size(ctx.size),
            );
        }
    }

    fn abs_distributions(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        distributions::abs_distributions(
            ctx.id,
            ctx.context,
            data.formatter,
            data.measurements,
            convert_size(ctx.size),
        );
    }

    fn rel_distributions(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        distributions::rel_distributions(
            ctx.id,
            ctx.context,
            data.measurements,
            data.comparison.unwrap(),
            convert_size(ctx.size),
        );
    }

    fn line_comparison(
        &mut self,
        ctx: PlotContext<'_>,
        formatter: &dyn ValueFormatter,
        all_curves: &[&(&BenchmarkId, Vec<f64>)],
        value_type: ValueType,
    ) {
        let path = ctx.line_comparison_path();
        summary::line_comparison(
            formatter,
            ctx.id.as_title(),
            all_curves,
            &path,
            value_type,
            ctx.context.plot_config.summary_scale,
        );
    }

    fn violin(
        &mut self,
        ctx: PlotContext<'_>,
        formatter: &dyn ValueFormatter,
        all_curves: &[&(&BenchmarkId, Vec<f64>)],
    ) {
        let violin_path = ctx.violin_path();

        summary::violin(
            formatter,
            ctx.id.as_title(),
            all_curves,
            &violin_path,
            ctx.context.plot_config.summary_scale,
        );
    }

    fn t_test(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        let title = ctx.id.as_title();
        let path = ctx.context.report_path(ctx.id, "change/t-test.svg");
        t_test::t_test(
            path.as_path(),
            title,
            data.comparison.unwrap(),
            convert_size(ctx.size),
        );
    }

    fn wait(&mut self) {}
}
