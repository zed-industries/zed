use crate::context::Context;
use crate::helpers::{HelperDef, HelperResult};
#[cfg(not(feature = "no_logging"))]
use crate::json::value::JsonRender;
use crate::output::Output;
use crate::registry::Registry;
use crate::render::{Helper, RenderContext};
#[cfg(not(feature = "no_logging"))]
use crate::RenderErrorReason;
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
        h: &Helper<'rc>,
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
            return Err(RenderErrorReason::InvalidLoggingLevel(level.to_string()).into());
        }
        Ok(())
    }
}

#[cfg(feature = "no_logging")]
impl HelperDef for LogHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        _: &Helper<'rc>,
        _: &Registry<'reg>,
        _: &Context,
        _: &mut RenderContext<'reg, 'rc>,
        _: &mut dyn Output,
    ) -> HelperResult {
        Ok(())
    }
}

pub static LOG_HELPER: LogHelper = LogHelper;

#[cfg(test)]
mod test {
    use crate::registry::Registry;

    #[test]
    #[cfg(not(feature = "no_logging"))]
    fn test_log_helper() {
        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t0", "{{log this level=\"warn\"}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t1", "{{log this level=\"hello\"}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t2", "{{log this}}")
            .is_ok());

        let r0 = handlebars.render("t0", &true);
        assert!(r0.is_ok());

        let r1 = handlebars.render("t1", &true);
        assert!(r1.is_err());

        let r2 = handlebars.render("t2", &true);
        assert!(r2.is_ok());
    }

    #[test]
    #[cfg(feature = "no_logging")]
    fn test_log_helper() {
        let mut handlebars = Registry::new();
        assert!(handlebars
            .register_template_string("t0", "{{log this level=\"warn\"}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t1", "{{log this level=\"hello\"}}")
            .is_ok());
        assert!(handlebars
            .register_template_string("t2", "{{log this}}")
            .is_ok());

        let r0 = handlebars.render("t0", &true);
        assert!(r0.is_ok());

        let r1 = handlebars.render("t1", &true);
        assert!(r1.is_ok());

        let r2 = handlebars.render("t2", &true);
        assert!(r2.is_ok());
    }
}
