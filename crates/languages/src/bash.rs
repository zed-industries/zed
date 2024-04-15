use language::ContextProviderWithTasks;
use task::{TaskTemplate, TaskTemplates, VariableName};

pub(super) fn bash_task_context() -> ContextProviderWithTasks {
    ContextProviderWithTasks::new(TaskTemplates(vec![
        TaskTemplate {
            label: format!("execute '{}'", VariableName::SelectedText.template_value()),
            command: VariableName::SelectedText.template_value(),
            ..TaskTemplate::default()
        },
        TaskTemplate {
            label: format!("run '{}'", VariableName::File.template_value()),
            command: VariableName::File.template_value(),
            ..TaskTemplate::default()
        },
    ]))
}
