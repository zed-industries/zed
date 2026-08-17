use std::backtrace::BacktraceStatus;

use egui::{Label, Response, Sense, Ui, WidgetText};
use egui_extras::{Column, TableBuilder};

use crate::allocator::{fmt_bytes, AllocationReport};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AllocationReportVisualizeSorting {
    #[default]
    None,
    Idx,
    Name,
    Size,
}

#[derive(Debug, Default)]
pub(crate) struct AllocationReportVisualizeSettings {
    pub filter: String,
    pub sorting: AllocationReportVisualizeSorting,
    pub ascending: bool,
}

pub(crate) fn render_allocation_reports_ui(
    ui: &mut Ui,
    settings: &mut AllocationReportVisualizeSettings,
    allocations: impl IntoIterator<Item = AllocationReport>,
) {
    ui.horizontal(|ui| {
        ui.label("Filter");
        ui.text_edit_singleline(&mut settings.filter);
    });
    let breakdown_filter = settings.filter.to_lowercase();

    let mut allocations = allocations
        .into_iter()
        .enumerate()
        .filter(|(_, report)| report.name.to_lowercase().contains(&breakdown_filter))
        .collect::<Vec<_>>();
    let total_size_under_filter: u64 = allocations.iter().map(|a| a.1.size).sum();

    ui.label(format!("Total: {}", fmt_bytes(total_size_under_filter)));

    let row_height = ui.text_style_height(&egui::TextStyle::Body);
    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .column(Column::exact(30.0))
        .column(Column::initial(300.0).at_least(200.0).clip(true))
        .column(Column::exact(70.0));

    fn header_button(ui: &mut Ui, label: &str) -> Response {
        let label = WidgetText::from(label).strong();
        let label = Label::new(label).sense(Sense::click());
        ui.add(label)
    }

    let table = table.header(row_height, |mut row| {
        row.col(|ui| {
            if header_button(ui, "Idx").clicked() {
                if settings.sorting == AllocationReportVisualizeSorting::Idx {
                    settings.ascending = !settings.ascending;
                } else {
                    settings.sorting = AllocationReportVisualizeSorting::Idx;
                    settings.ascending = false;
                }
            }
        });
        row.col(|ui| {
            if header_button(ui, "Name").clicked() {
                if settings.sorting == AllocationReportVisualizeSorting::Name {
                    settings.ascending = !settings.ascending;
                } else {
                    settings.sorting = AllocationReportVisualizeSorting::Name;
                    settings.ascending = false;
                }
            }
        });
        row.col(|ui| {
            if header_button(ui, "Size").clicked() {
                if settings.sorting == AllocationReportVisualizeSorting::Size {
                    settings.ascending = !settings.ascending;
                } else {
                    settings.sorting = AllocationReportVisualizeSorting::Size;
                    settings.ascending = false;
                }
            }
        });
    });

    match (settings.sorting, settings.ascending) {
        (AllocationReportVisualizeSorting::None, _) => {}
        (AllocationReportVisualizeSorting::Idx, true) => allocations.sort_by_key(|(idx, _)| *idx),
        (AllocationReportVisualizeSorting::Idx, false) => {
            allocations.sort_by_key(|(idx, _)| core::cmp::Reverse(*idx))
        }
        (AllocationReportVisualizeSorting::Name, true) => {
            allocations.sort_by(|(_, alloc1), (_, alloc2)| alloc1.name.cmp(&alloc2.name))
        }
        (AllocationReportVisualizeSorting::Name, false) => {
            allocations.sort_by(|(_, alloc1), (_, alloc2)| alloc1.name.cmp(&alloc2.name).reverse())
        }
        (AllocationReportVisualizeSorting::Size, true) => {
            allocations.sort_by_key(|(_, alloc)| alloc.size)
        }
        (AllocationReportVisualizeSorting::Size, false) => {
            allocations.sort_by_key(|(_, alloc)| core::cmp::Reverse(alloc.size))
        }
    }

    table.body(|mut body| {
        for (idx, alloc) in allocations {
            body.row(row_height, |mut row| {
                let AllocationReport {
                    name,
                    size,
                    backtrace,
                    ..
                } = alloc;

                row.col(|ui| {
                    ui.label(idx.to_string());
                });

                let resp = row.col(|ui| {
                    ui.label(name);
                });

                if backtrace.status() == BacktraceStatus::Captured {
                    resp.1.on_hover_ui(|ui| {
                        ui.label(backtrace.to_string());
                    });
                }

                row.col(|ui| {
                    ui.label(fmt_bytes(size));
                });
            });
        }
    });
}
