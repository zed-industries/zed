//! SVG renderers for Mermaid-parity diagrams.
//!
//! Public API is re-exported from the parity-focused renderer implementation.
//!
//! This module is named `parity` to reflect intent: upstream Mermaid is treated as the spec, and
//! SVG output is gated by DOM parity checks.

#![forbid(unsafe_code)]

mod fallback;
mod icon_registry;
mod parity;
mod pipeline;
mod theme_profile;

pub(crate) use parity::theme as render_theme;

pub use fallback::foreign_object_label_fallback_svg_text;
pub use icon_registry::{IconRegistry, IconRegistryError, IconSvg};
pub use parity::*;
pub use pipeline::{
    CssOverridePolicy, CssOverridePostprocessor, DropNativeDuplicateFallbacksPostprocessor,
    ForeignObjectFallbackPostprocessor, RootBackgroundPostprocessor, SanitizeCssPostprocessor,
    SanitizeSvgAttributesPostprocessor, ScopedCssPostprocessor, StripForeignObjectPostprocessor,
    SvgPipeline, SvgPipelinePreset, SvgPostprocessContext, SvgPostprocessMetadata,
    SvgPostprocessor, resvg_safe_svg,
};
pub use theme_profile::{
    CompiledHostTheme, CompiledHostThemeOutput, HostThemeAppearance, HostThemeOutput,
    HostThemePipelinePreset, HostThemePreset, HostThemeProfile, HostThemeProfileBuilder,
    HostThemeRoles, HostThemeRootBackground, supported_host_theme_presets,
};
