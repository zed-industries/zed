use client::User;
use gpui::{ModelHandle, ViewHandle};
use project::Project;
use workspace::Workspace;

pub struct LocalParticipant {
    user: User,
    workspaces: Vec<ViewHandle<Workspace>>,
}

pub struct RemoteParticipant {
    user: User,
    workspaces: Vec<ViewHandle<Workspace>>,
    active_workspace_id: usize,
}
