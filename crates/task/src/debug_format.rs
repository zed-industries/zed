use anyhow::bail;
use collections::HashMap;
use serde::Deserialize;
use util::ResultExt;

use crate::{TaskTemplate, TaskTemplates, VariableName};

struct ZedDebugTaskFile {}

impl ZedDebugTaskFile {
    fn to_zed_format(self) -> anyhow::Result<TaskTemplate> {}
}
impl TryFrom<ZedDebugTaskFile> for TaskTemplates {
    type Error = anyhow::Error;

    fn try_from(value: ZedDebugTaskFile) -> Result<Self, Self::Error> {
        let templates = value
            .tasks
            .into_iter()
            .filter_map(|debug_task_file| debug_task_file.to_zed_format(&replacer).log_err())
            .collect();
        Ok(Self(templates))
    }
}
