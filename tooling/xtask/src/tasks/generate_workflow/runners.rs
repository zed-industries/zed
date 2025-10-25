pub const LINUX_CHEAP: Runner = Runner("namespace-profile-2x4-ubuntu-2404");

pub struct Runner(&'static str);

impl Into<gh_workflow::RunsOn> for Runner {
    fn into(self) -> gh_workflow::RunsOn {
        self.0.into()
    }
}
