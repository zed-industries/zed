mod shells {
    pub use clap_complete::aot::{Bash, Elvish, Fish, PowerShell, Zsh};
    pub use clap_complete_nushell::Nushell;
}

use clap_complete::Generator;

#[derive(Clone, Debug, clap::ValueEnum)]
#[non_exhaustive]
#[value(rename_all = "lower")]
pub(crate) enum Shell {
    Bash,
    Elvish,
    Fish,
    Nushell,
    PowerShell,
    Zsh,
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => self::shells::Bash.file_name(name),
            Shell::Elvish => self::shells::Elvish.file_name(name),
            Shell::Fish => self::shells::Fish.file_name(name),
            Shell::Nushell => self::shells::Nushell.file_name(name),
            Shell::PowerShell => self::shells::PowerShell.file_name(name),
            Shell::Zsh => self::shells::Zsh.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
        match self {
            Shell::Bash => self::shells::Bash.generate(cmd, buf),
            Shell::Elvish => self::shells::Elvish.generate(cmd, buf),
            Shell::Fish => self::shells::Fish.generate(cmd, buf),
            Shell::Nushell => self::shells::Nushell.generate(cmd, buf),
            Shell::PowerShell => self::shells::PowerShell.generate(cmd, buf),
            Shell::Zsh => self::shells::Zsh.generate(cmd, buf),
        }
    }
}

pub(crate) fn main(cmd: &clap::Command, shell: &Shell) {
    let buf = &mut std::io::stdout();
    shell.generate(cmd, buf);
}
