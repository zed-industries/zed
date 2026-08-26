use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProcessExecCapability {
    /// The command to execute.
    pub command: String,
    /// The arguments to pass to the command. Use `*` for a single wildcard argument.
    /// If the last element is `**`, then any trailing arguments are allowed.
    pub args: Vec<String>,
}

impl ProcessExecCapability {
    /// Returns whether the capability allows the given command and arguments.
    pub fn allows(
        &self,
        desired_command: &str,
        desired_args: &[impl AsRef<str> + std::fmt::Debug],
    ) -> bool {
        if self.command != desired_command && self.command != "*" {
            return false;
        }

        for (ix, arg) in self.args.iter().enumerate() {
            if arg == "**" {
                return true;
            }

            if ix >= desired_args.len() {
                return false;
            }

            if arg != "*" && arg != desired_args[ix].as_ref() {
                return false;
            }
        }

        if self.args.len() < desired_args.len() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_allows_with_exact_match() {
        let capability = ProcessExecCapability {
            command: "ls".to_string(),
            args: vec!["-la".to_string()],
        };

        assert_eq!(capability.allows("ls", &["-la"]), true);
        assert_eq!(capability.allows("ls", &["-l"]), false);
        assert_eq!(capability.allows("pwd", &[] as &[&str]), false);
    }

    #[test]
    fn test_allows_with_wildcard_arg() {
        let capability = ProcessExecCapability {
            command: "git".to_string(),
            args: vec!["*".to_string()],
        };

        assert_eq!(capability.allows("git", &["status"]), true);
        assert_eq!(capability.allows("git", &["commit"]), true);
        // Too many args.
        assert_eq!(capability.allows("git", &["status", "-s"]), false);
        // Wrong command.
        assert_eq!(capability.allows("npm", &["install"]), false);
    }

    #[test]
    fn test_allows_with_double_wildcard() {
        let capability = ProcessExecCapability {
            command: "cargo".to_string(),
            args: vec!["test".to_string(), "**".to_string()],
        };

        assert_eq!(capability.allows("cargo", &["test"]), true);
        assert_eq!(capability.allows("cargo", &["test", "--all"]), true);
        assert_eq!(
            capability.allows("cargo", &["test", "--all", "--no-fail-fast"]),
            true
        );
        // Wrong first arg.
        assert_eq!(capability.allows("cargo", &["build"]), false);
    }

    #[test]
    fn test_allows_with_mixed_wildcards() {
        let capability = ProcessExecCapability {
            command: "docker".to_string(),
            args: vec!["run".to_string(), "*".to_string(), "**".to_string()],
        };

        assert_eq!(capability.allows("docker", &["run", "nginx"]), true);
        assert_eq!(capability.allows("docker", &["run"]), false);
        assert_eq!(
            capability.allows("docker", &["run", "ubuntu", "bash"]),
            true
        );
        assert_eq!(
            capability.allows("docker", &["run", "alpine", "sh", "-c", "echo hello"]),
            true
        );
        // Wrong first arg.
        assert_eq!(capability.allows("docker", &["ps"]), false);
    }
}
