use std::iter;
use std::path::Path;
use std::process::Child;

use crate::stats::univariate::Sample;
use criterion_plot::prelude::*;

mod distributions;
mod iteration_times;
mod pdf;
mod regression;
mod summary;
mod t_test;
use self::distributions::*;
use self::iteration_times::*;
use self::pdf::*;
use self::regression::*;
use self::summary::*;
use self::t_test::*;

use crate::measurement::ValueFormatter;
use crate::report::{BenchmarkId, ValueType};
use crate::stats::bivariate::Data;

use super::{PlotContext, PlotData, Plotter};
use crate::format;

fn gnuplot_escape(string: &str) -> String {
    string.replace('_', "\\_").replace('\'', "''")
}

static DEFAULT_FONT: &str = "Helvetica";
static KDE_POINTS: usize = 500;
static SIZE: Size = Size(1280, 720);

const LINEWIDTH: LineWidth = LineWidth(2.);
const POINT_SIZE: PointSize = PointSize(0.75);

const DARK_BLUE: Color = Color::Rgb(31, 120, 180);
const DARK_ORANGE: Color = Color::Rgb(255, 127, 0);
const DARK_RED: Color = Color::Rgb(227, 26, 28);

fn debug_script(path: &Path, figure: &Figure) {
    if crate::debug_enabled() {
        let mut script_path = path.to_path_buf();
        script_path.set_extension("gnuplot");
        info!("Writing gnuplot script to {:?}", script_path);
        let result = figure.save(script_path.as_path());
        if let Err(e) = result {
            error!("Failed to write debug output: {}", e);
        }
    }
}

/// Private
trait Append<T> {
    /// Private
    fn append_(self, item: T) -> Self;
}

// NB I wish this was in the standard library
impl<T> Append<T> for Vec<T> {
    fn append_(mut self, item: T) -> Vec<T> {
        self.push(item);
        self
    }
}

#[derive(Default)]
pub(crate) struct Gnuplot {
    process_list: Vec<Child>,
}

impl Plotter for Gnuplot {
    fn pdf(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        let size = ctx.size.map(|(w, h)| Size(w, h));
        self.process_list.push(if ctx.is_thumbnail {
            if let Some(cmp) = data.comparison {
                pdf_comparison_small(
                    ctx.id,
                    ctx.context,
                    data.formatter,
                    data.measurements,
                    cmp,
                    size,
                )
            } else {
                pdf_small(ctx.id, ctx.context, data.formatter, data.measurements, size)
            }
        } else if let Some(cmp) = data.comparison {
            pdf_comparison(
                ctx.id,
                ctx.context,
                data.formatter,
                data.measurements,
                cmp,
                size,
            )
        } else {
            pdf(ctx.id, ctx.context, data.formatter, data.measurements, size)
        });
    }

    fn regression(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        let size = ctx.size.map(|(w, h)| Size(w, h));
        self.process_list.push(if ctx.is_thumbnail {
            if let Some(cmp) = data.comparison {
                let base_data = Data::new(&cmp.base_iter_counts, &cmp.base_sample_times);
                regression_comparison_small(
                    ctx.id,
                    ctx.context,
                    data.formatter,
                    data.measurements,
                    cmp,
                    &base_data,
                    size,
                )
            } else {
                regression_small(ctx.id, ctx.context, data.formatter, data.measurements, size)
            }
        } else if let Some(cmp) = data.comparison {
            let base_data = Data::new(&cmp.base_iter_counts, &cmp.base_sample_times);
            regression_comparison(
                ctx.id,
                ctx.context,
                data.formatter,
                data.measurements,
                cmp,
                &base_data,
                size,
            )
        } else {
            regression(ctx.id, ctx.context, data.formatter, data.measurements, size)
        });
    }

    fn iteration_times(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        let size = ctx.size.map(|(w, h)| Size(w, h));
        self.process_list.push(if ctx.is_thumbnail {
            if let Some(cmp) = data.comparison {
                iteration_times_comparison_small(
                    ctx.id,
                    ctx.context,
                    data.formatter,
                    data.measurements,
                    cmp,
                    size,
                )
            } else {
                iteration_times_small(ctx.id, ctx.context, data.formatter, data.measurements, size)
            }
        } else if let Some(cmp) = data.comparison {
            iteration_times_comparison(
                ctx.id,
                ctx.context,
                data.formatter,
                data.measurements,
                cmp,
                size,
            )
        } else {
            iteration_times(ctx.id, ctx.context, data.formatter, data.measurements, size)
        });
    }

    fn abs_distributions(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        let size = ctx.size.map(|(w, h)| Size(w, h));
        self.process_list.extend(abs_distributions(
            ctx.id,
            ctx.context,
            data.formatter,
            data.measurements,
            size,
        ));
    }

    fn rel_distributions(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        let size = ctx.size.map(|(w, h)| Size(w, h));
        if let Some(cmp) = data.comparison {
            self.process_list.extend(rel_distributions(
                ctx.id,
                ctx.context,
                data.measurements,
                cmp,
                size,
            ));
        } else {
            error!("Comparison data is not provided for a relative distribution figure");
        }
    }

    fn t_test(&mut self, ctx: PlotContext<'_>, data: PlotData<'_>) {
        let size = ctx.size.map(|(w, h)| Size(w, h));
        if let Some(cmp) = data.comparison {
            self.process_list
                .push(t_test(ctx.id, ctx.context, data.measurements, cmp, size));
        } else {
            error!("Comparison data is not provided for t_test plot");
        }
    }

    fn line_comparison(
        &mut self,
        ctx: PlotContext<'_>,
        formatter: &dyn ValueFormatter,
        all_curves: &[&(&BenchmarkId, Vec<f64>)],
        value_type: ValueType,
    ) {
        let path = ctx.line_comparison_path();
        self.process_list.push(line_comparison(
            formatter,
            ctx.id.as_title(),
            all_curves,
            &path,
            value_type,
            ctx.context.plot_config.summary_scale,
        ));
    }

    fn violin(
        &mut self,
        ctx: PlotContext<'_>,
        formatter: &dyn ValueFormatter,
        all_curves: &[&(&BenchmarkId, Vec<f64>)],
    ) {
        let violin_path = ctx.violin_path();

        self.process_list.push(violin(
            formatter,
            ctx.id.as_title(),
            all_curves,
            &violin_path,
            ctx.context.plot_config.summary_scale,
        ));
    }

    fn wait(&mut self) {
        let start = std::time::Instant::now();
        let child_count = self.process_list.len();
        for child in self.process_list.drain(..) {
            match child.wait_with_output() {
                Ok(ref out) if out.status.success() => {}
                Ok(out) => error!("Error in Gnuplot: {}", String::from_utf8_lossy(&out.stderr)),
                Err(e) => error!("Got IO error while waiting for Gnuplot to complete: {}", e),
            }
        }
        let elapsed = &start.elapsed();
        info!(
            "Waiting for {} gnuplot processes took {}",
            child_count,
            format::time(elapsed.as_nanos() as f64)
        );
    }
}
