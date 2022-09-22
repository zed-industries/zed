use client::User;
use gpui::ModelHandle;
use project::Project;

pub enum Location {
    Project { project_id: usize },
    External,
}

pub struct LocalParticipant {
    user: User,
    projects: Vec<ModelHandle<Project>>,
}

pub struct RemoteParticipant {
    user: User,
    projects: Vec<ModelHandle<Project>>,
    location: Location,
}
