use std::{error::Error, fmt, path::Path, rc::Rc};

use agent_client_protocol::{self as acp};
use anyhow::Result;
use gpui::{AsyncApp, Entity, Task};
use project::Project;
use ui::App;

use crate::AcpThread;

pub trait AgentConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>>;

    fn auth_methods(&self) -> &[acp::AuthMethod];

    fn authenticate(&self, method: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>>;

    fn prompt(&self, params: acp::PromptRequest, cx: &mut App) -> Task<Result<()>>;

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App);
}

#[derive(Debug)]
pub struct AuthRequired;

impl Error for AuthRequired {}
impl fmt::Display for AuthRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthRequired")
    }
}
