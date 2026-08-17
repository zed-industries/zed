use super::*;
use std::sync::RwLock;

#[derive(Default)]
pub(crate) struct WarningBuilder(RwLock<Vec<String>>);

impl WarningBuilder {
    pub fn build(self) -> Warnings {
        Warnings(self.0.write().unwrap().split_off(0))
    }

    pub fn add(&self, message: String) {
        self.0.write().unwrap().push(message);
    }

    pub fn skip_method(&self, method: MethodDef, dependencies: &TypeMap, config: &Config<'_>) {
        let mut message = String::new();
        writeln!(
            &mut message,
            "skipping `{}.{}` due to missing dependencies:",
            method.parent().type_name(),
            method.name()
        )
        .unwrap();

        for tn in dependencies.keys() {
            if !config.types.contains_key(tn) && config.references.contains(*tn).is_none() {
                writeln!(&mut message, "  {tn}").unwrap();
            }
        }

        self.add(message);
    }
}

/// Contains warnings collected during code generation.
#[derive(Debug)]
pub struct Warnings(Vec<String>);

impl Warnings {
    /// Panics if any warnings are present.
    #[track_caller]
    pub fn unwrap(&self) {
        if !self.is_empty() {
            panic!("{self}");
        }
    }
}

impl std::fmt::Display for Warnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for message in &self.0 {
            write!(f, "{message}")?;
        }
        Ok(())
    }
}

impl std::ops::Deref for Warnings {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
