use clap::Args;
use clap::ValueEnum;

#[derive(Args, Debug)]
pub(crate) struct ImplicitParsers {
    /// Implicitly using `std::str::FromStr`
    #[arg(short = 'O')]
    optimization: Option<usize>,

    /// Allow invalid UTF-8 paths
    #[arg(short = 'I', value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    include: Option<std::path::PathBuf>,

    /// Handle IP addresses
    #[arg(long)]
    bind: Option<std::net::IpAddr>,

    /// Allow human-readable durations
    #[arg(long)]
    sleep: Option<jiff::SignedDuration>,

    /// Custom enums
    #[arg(long)]
    bump_level: Option<BumpLevel>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub(crate) enum BumpLevel {
    /// Increase the major version (x.0.0)
    Major,
    /// Increase the minor version (x.y.0)
    Minor,
    /// Increase the patch version (x.y.z)
    Patch,
}

impl std::fmt::Display for BumpLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use clap::ValueEnum;

        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for BumpLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use clap::ValueEnum;

        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {s}"))
    }
}
