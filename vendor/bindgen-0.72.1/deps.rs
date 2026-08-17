/// Generating build depfiles from parsed bindings.
use std::{collections::BTreeSet, path::PathBuf};

#[derive(Clone, Debug)]
pub(crate) struct DepfileSpec {
    pub output_module: String,
    pub depfile_path: PathBuf,
}

impl DepfileSpec {
    pub fn write(&self, deps: &BTreeSet<Box<str>>) -> std::io::Result<()> {
        std::fs::write(&self.depfile_path, self.to_string(deps))
    }

    fn to_string(&self, deps: &BTreeSet<Box<str>>) -> String {
        // Transforms a string by escaping spaces and backslashes.
        let escape = |s: &str| s.replace('\\', "\\\\").replace(' ', "\\ ");

        let mut buf = format!("{}:", escape(&self.output_module));
        for file in deps {
            buf = format!("{buf} {}", escape(file));
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escaping_depfile() {
        let spec = DepfileSpec {
            output_module: "Mod Name".to_owned(),
            depfile_path: PathBuf::new(),
        };

        let deps: BTreeSet<_> = vec![
            r"/absolute/path".into(),
            r"C:\win\absolute\path".into(),
            r"../relative/path".into(),
            r"..\win\relative\path".into(),
            r"../path/with spaces/in/it".into(),
            r"..\win\path\with spaces\in\it".into(),
            r"path\with/mixed\separators".into(),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            spec.to_string(&deps),
            "Mod\\ Name: \
            ../path/with\\ spaces/in/it \
            ../relative/path \
            ..\\\\win\\\\path\\\\with\\ spaces\\\\in\\\\it \
            ..\\\\win\\\\relative\\\\path \
            /absolute/path \
            C:\\\\win\\\\absolute\\\\path \
            path\\\\with/mixed\\\\separators"
        );
    }
}
