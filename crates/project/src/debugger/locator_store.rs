use anyhow::{Result, anyhow};
use cargo::CargoLocator;
use collections::HashMap;
use gpui::SharedString;
use locators::DapLocator;
use task::{DebugTaskDefinition, DebugTaskTemplate};

mod cargo;
mod locators;

pub(super) struct LocatorStore {
    locators: HashMap<SharedString, Box<dyn DapLocator>>,
}

impl LocatorStore {
    pub(super) fn new() -> Self {
        let locators = HashMap::from_iter([(
            SharedString::new("cargo"),
            Box::new(CargoLocator {}) as Box<dyn DapLocator>,
        )]);
        Self { locators }
    }

    pub(super) async fn resolve_debug_config(
        &self,
        template: DebugTaskTemplate,
    ) -> Result<DebugTaskDefinition> {
        let Some(locator_name) = &template.locator else {
            return Ok(template.definition);
        };

        if let Some(locator) = self.locators.get(locator_name as &str) {
            locator.run_locator(template.definition).await
        } else {
            Err(anyhow!("Couldn't find locator {}", locator_name))
        }
    }
}
