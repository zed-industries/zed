use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NpmInstallPackageCapability {
    pub package: String,
}

impl NpmInstallPackageCapability {
    /// Returns whether the capability allows installing the given NPM package.
    pub fn allows(&self, package: &str) -> bool {
        self.package == "*" || self.package == package
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_allows() {
        let capability = NpmInstallPackageCapability {
            package: "*".to_string(),
        };
        assert_eq!(capability.allows("package"), true);

        let capability = NpmInstallPackageCapability {
            package: "react".to_string(),
        };
        assert_eq!(capability.allows("react"), true);

        let capability = NpmInstallPackageCapability {
            package: "react".to_string(),
        };
        assert_eq!(capability.allows("malicious-package"), false);
    }
}
