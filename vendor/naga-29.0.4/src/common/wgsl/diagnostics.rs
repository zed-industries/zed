//! WGSL diagnostic filters and severities.

use core::fmt::{self, Display, Formatter};

use crate::diagnostic_filter::{
    FilterableTriggeringRule, Severity, StandardFilterableTriggeringRule,
};

impl Severity {
    const ERROR: &'static str = "error";
    const WARNING: &'static str = "warning";
    const INFO: &'static str = "info";
    const OFF: &'static str = "off";

    /// Convert from a sentinel word in WGSL into its associated [`Severity`], if possible.
    pub fn from_wgsl_ident(s: &str) -> Option<Self> {
        Some(match s {
            Self::ERROR => Self::Error,
            Self::WARNING => Self::Warning,
            Self::INFO => Self::Info,
            Self::OFF => Self::Off,
            _ => return None,
        })
    }
}

pub struct DisplayFilterableTriggeringRule<'a>(&'a FilterableTriggeringRule);

impl Display for DisplayFilterableTriggeringRule<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let &Self(inner) = self;
        match *inner {
            FilterableTriggeringRule::Standard(rule) => write!(f, "{}", rule.to_wgsl_ident()),
            FilterableTriggeringRule::Unknown(ref rule) => write!(f, "{rule}"),
            FilterableTriggeringRule::User(ref rules) => {
                let &[ref seg1, ref seg2] = rules.as_ref();
                write!(f, "{seg1}.{seg2}")
            }
        }
    }
}

impl FilterableTriggeringRule {
    /// [`Display`] this rule's identifiers in WGSL.
    pub const fn display_wgsl_ident(&self) -> impl Display + '_ {
        DisplayFilterableTriggeringRule(self)
    }
}

impl StandardFilterableTriggeringRule {
    const DERIVATIVE_UNIFORMITY: &'static str = "derivative_uniformity";

    /// Convert from a sentinel word in WGSL into its associated
    /// [`StandardFilterableTriggeringRule`], if possible.
    pub fn from_wgsl_ident(s: &str) -> Option<Self> {
        Some(match s {
            Self::DERIVATIVE_UNIFORMITY => Self::DerivativeUniformity,
            _ => return None,
        })
    }

    /// Maps this [`StandardFilterableTriggeringRule`] into the sentinel word associated with it in
    /// WGSL.
    pub const fn to_wgsl_ident(self) -> &'static str {
        match self {
            Self::DerivativeUniformity => Self::DERIVATIVE_UNIFORMITY,
        }
    }
}
