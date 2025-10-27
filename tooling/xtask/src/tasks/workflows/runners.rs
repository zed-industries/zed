pub const LINUX_CHEAP: Runner = Runner("namespace-profile-2x4-ubuntu-2404");
pub const LINUX_DEFAULT: Runner = Runner("namespace-profile-16x32-ubuntu-2204");
pub const MAC_DEFAULT: Runner = Runner("self-mini-macos");

pub struct Runner(&'static str);

impl Into<gh_workflow::RunsOn> for Runner {
    fn into(self) -> gh_workflow::RunsOn {
        self.0.into()
    }
}
