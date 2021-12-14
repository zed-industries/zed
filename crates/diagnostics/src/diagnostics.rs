use collections::HashMap;
use editor::{Editor, ExcerptProperties, MultiBuffer};
use gpui::{elements::*, Entity, ModelHandle, RenderContext, View, ViewContext, ViewHandle};
use language::Point;
use postage::watch;
use project::Project;

struct ProjectDiagnostics {
    editor: ViewHandle<Editor>,
    project: ModelHandle<Project>,
}

impl ProjectDiagnostics {
    fn new(
        project: ModelHandle<Project>,
        settings: watch::Receiver<workspace::Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = cx.add_model(|cx| MultiBuffer::new(project.read(cx).replica_id(cx)));

        let project_paths = project
            .read(cx)
            .diagnostic_summaries(cx)
            .map(|e| e.0)
            .collect::<Vec<_>>();

        cx.spawn(|this, mut cx| {
            let project = project.clone();
            async move {
                let mut excerpts = Vec::new();
                for project_path in project_paths {
                    let buffer = project
                        .update(&mut cx, |project, cx| project.open_buffer(project_path, cx))
                        .await?;
                    let snapshot = buffer.read_with(&cx, |b, _| b.snapshot());

                    let mut grouped_diagnostics = HashMap::default();
                    for entry in snapshot.all_diagnostics() {
                        let mut group = grouped_diagnostics
                            .entry(entry.diagnostic.group_id)
                            .or_insert((Point::zero(), Vec::new()));
                        if entry.diagnostic.is_primary {
                            group.0 = entry.range.start;
                        }
                        group.1.push(entry);
                    }
                    let mut sorted_diagnostic_groups =
                        grouped_diagnostics.into_values().collect::<Vec<_>>();
                    sorted_diagnostic_groups.sort_by_key(|group| group.0);

                    let mut prev_end_row = None;
                    let mut pending_excerpt = None;
                    for diagnostic in snapshot.all_diagnostics::<Point>() {
                        excerpts.push(ExcerptProperties {
                            buffer: &buffer,
                            range: todo!(),
                            header_height: todo!(),
                        });
                    }
                }
                Result::Ok::<_, anyhow::Error>(())
            }
        })
        .detach();

        Self {
            editor: cx.add_view(|cx| {
                Editor::for_buffer(
                    buffer.clone(),
                    editor::settings_builder(buffer.downgrade(), settings),
                    cx,
                )
            }),
            project,
        }
    }
}

impl Entity for ProjectDiagnostics {
    type Event = ();
}

impl View for ProjectDiagnostics {
    fn ui_name() -> &'static str {
        "ProjectDiagnostics"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.editor.id()).boxed()
    }
}
