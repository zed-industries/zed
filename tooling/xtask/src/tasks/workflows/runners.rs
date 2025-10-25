pub const LINUX_CHEAP: Runner = Runner::Profile("namespace-profile-2x4-ubuntu-2404");
pub const LINUX_DEFAULT: Runner = Runner::Profile("namespace-profile-16x32-ubuntu-2204");
pub const MAC_DEFAULT: Runner = Runner::Labelled(&["macOS", "ARM64", "test"]);

pub enum Runner {
    Profile(&'static str),
    Labelled(&'static [&'static str]),
}

impl Into<gh_workflow::RunsOn> for Runner {
    fn into(self) -> gh_workflow::RunsOn {
        match self {
            Runner::Profile(profile) => profile.into(),
            Runner::Labelled(items) => items.into(),
        }
    }
}
