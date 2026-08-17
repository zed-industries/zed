use crate::context::Context;
#[cfg(not(feature = "no_logging"))]
use crate::error::RenderError;
use crate::helpers::{HelperDef, HelperResult};
#[cfg(not(feature = "no_logging"))]
use crate::json::value::JsonRender;
use crate::output::Output;
use crate::registry::Registry;
use crate::render::{Helper, RenderContext};
#[cfg(not(feature = "no_logging"))]
use log::Level;
#[cfg(not(feature = "no_logging"))]
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct LogHelper;

#[cfg(not(feature = "no_logging"))]
impl HelperDef for LogHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Registry<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        _: &mut dyn Output,
    ) -> HelperResult {
        let param_to_log = h
            .params()
            .iter()
            .map(|p| {
                if let Some(ref relative_path) = p.relative_path() {
                    format!("{}: {}", relative_path, p.value().render())
                } else {
                    p.value().render()
                }
            })
            .collect::<Vec<String>>()
            .join(", ");

        let level = h
            .hash_get("level")
            .and_then(|v| v.value().as_str())
            .unwrap_or("info");

        if let Ok(log_level) = Level::from_str(level) {
            log!(log_level, "{}", param_to_log)
        } else {
            return Err(RenderError::new(&format!(
                "Unsupported logging level {}",
                level
            )));
        }
        Ok(())
    }
}

#[cfg(feature = "no_logging")]
impl HelperDef for LogHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        _: &Helper<'reg, 'rc>,
        _: &Registry<'reg>,
        _: &Context,
        _: &mut RenderContext<'reg, 'rc>,
        _: &mut dyn Output,
    ) -> HelperResult {
        Ok(())
    }
}

pub static LOG_HELPER: LogHelper = LogHelper;
