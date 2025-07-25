use std::{path::Path, rc::Rc};

use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{AsyncApp, Entity, Task};
use project::Project;
use ui::App;

use crate::AcpThread;

pub trait AgentConnection {
    fn name(&self) -> &'static str;

    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>>;

    fn authenticate(&self, cx: &mut App) -> Task<Result<()>>;

    fn prompt(&self, params: acp::PromptToolArguments, cx: &mut App) -> Task<Result<()>>;

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App);
}
