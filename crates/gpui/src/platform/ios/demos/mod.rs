//! iOS Demo modules for showcasing GPUI capabilities.
//!
//! This module contains interactive demos that demonstrate GPUI's
//! rendering and input handling on iOS.

mod animation_playground;
mod menu;
mod shader_showcase;

pub use animation_playground::AnimationPlayground;
pub use menu::{DemoApp, back_button};
pub use shader_showcase::ShaderShowcase;

// Color palette - Catppuccin Mocha theme
pub const BACKGROUND: u32 = 0x1e1e2e;
pub const SURFACE: u32 = 0x313244;
pub const OVERLAY: u32 = 0x45475a;
pub const TEXT: u32 = 0xcdd6f4;
pub const SUBTEXT: u32 = 0xa6adc8;
pub const RED: u32 = 0xf38ba8;
pub const GREEN: u32 = 0xa6e3a1;
pub const BLUE: u32 = 0x89b4fa;
pub const YELLOW: u32 = 0xf9e2af;
pub const PINK: u32 = 0xf5c2e7;
pub const MAUVE: u32 = 0xcba6f7;
pub const PEACH: u32 = 0xfab387;
pub const TEAL: u32 = 0x94e2d5;
pub const SKY: u32 = 0x89dceb;
pub const LAVENDER: u32 = 0xb4befe;

/// Get a random color from the vibrant palette
pub fn random_color(seed: usize) -> u32 {
    const COLORS: [u32; 10] = [
        RED, GREEN, BLUE, YELLOW, PINK, MAUVE, PEACH, TEAL, SKY, LAVENDER,
    ];
    COLORS[seed % COLORS.len()]
}
