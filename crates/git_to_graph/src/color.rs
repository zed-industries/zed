//! Color management for git graph rendering.

use std::collections::HashMap;

/// Default color palette for the graph.
#[allow(dead_code)]
pub const DEFAULT_COLORS: &[&str] = &[
    "#005EBE", "#CD3A00", "#FF9B00", "#007754", "#5247A5", "#009DB5", "#007DFF", "#FF6C3B",
    "#FFB800", "#3EBD90", "#776CCB", "#00C4E0",
];

/// Internal color tracking struct.
#[derive(Debug, Clone)]
pub struct Color {
    pub release_idx: i32,
    pub in_use: bool,
}

impl Color {
    /// Create a new color tracker.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::color::Color;
    ///
    /// let color = Color::new();
    /// assert!(!color.in_use);
    /// assert_eq!(color.release_idx, 0);
    /// ```
    pub fn new() -> Self {
        Color {
            release_idx: 0,
            in_use: false,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for color generators.
pub trait ColorGenerator {
    /// Get a color string for the given index.
    fn get_color(&self, idx: i32) -> String;
}

/// Simple color generator that returns black when out of colors.
#[derive(Debug, Clone)]
pub struct SimpleColorGen {
    colors: Vec<String>,
}

impl SimpleColorGen {
    /// Create a new simple color generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::color::SimpleColorGen;
    ///
    /// let colors = vec!["#FF0000".to_string(), "#00FF00".to_string()];
    /// let color_gen = SimpleColorGen::new(colors);
    /// ```
    pub fn new(colors: Vec<String>) -> Self {
        SimpleColorGen { colors }
    }
}

impl ColorGenerator for SimpleColorGen {
    fn get_color(&self, idx: i32) -> String {
        if idx >= self.colors.len() as i32 {
            "#000".to_string()
        } else {
            self.colors[idx as usize].clone()
        }
    }
}

/// Cycling color generator that wraps around when out of colors.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CycleColorGen {
    colors: Vec<String>,
}

impl CycleColorGen {
    /// Create a new cycling color generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::color::CycleColorGen;
    ///
    /// let colors = vec!["#FF0000".to_string(), "#00FF00".to_string()];
    /// let color_gen = CycleColorGen::new(colors);
    /// ```
    #[allow(dead_code)]
    pub fn new(colors: Vec<String>) -> Self {
        CycleColorGen { colors }
    }
}

impl ColorGenerator for CycleColorGen {
    fn get_color(&self, idx: i32) -> String {
        self.colors[(idx as usize) % self.colors.len()].clone()
    }
}

/// Manager for color allocation and release.
#[derive(Debug)]
pub struct ColorsManager {
    m: HashMap<i32, Color>,
}

impl ColorsManager {
    /// Create a new colors manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::color::ColorsManager;
    ///
    /// let manager = ColorsManager::new();
    /// ```
    pub fn new() -> Self {
        ColorsManager { m: HashMap::new() }
    }

    /// Get an available color index for the given node index.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::color::ColorsManager;
    ///
    /// let mut manager = ColorsManager::new();
    /// let color_idx = manager.get_color(0);
    /// assert_eq!(color_idx, 0);
    /// ```
    pub fn get_color(&mut self, node_idx: i32) -> i32 {
        let mut i = 0;
        loop {
            let clr = self.m.entry(i).or_default();
            if node_idx >= clr.release_idx && !clr.in_use {
                clr.in_use = true;
                return i;
            }
            i += 1;
        }
    }

    /// Release a color at the given index.
    /// We add "2" because we need at least one commit in between two branches to reuse the same color.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::color::ColorsManager;
    ///
    /// let mut manager = ColorsManager::new();
    /// let color_idx = manager.get_color(0);
    /// manager.release_color(color_idx, 5);
    /// ```
    pub fn release_color(&mut self, color_idx: i32, idx: i32) {
        if let Some(clr) = self.m.get_mut(&color_idx) {
            clr.release_idx = idx + 2;
            clr.in_use = false;
        }
    }
}

impl Default for ColorsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_color_gen() {
        let colors = vec!["#FF0000".to_string(), "#00FF00".to_string()];
        let color_gen = SimpleColorGen::new(colors);
        assert_eq!(color_gen.get_color(0), "#FF0000");
        assert_eq!(color_gen.get_color(1), "#00FF00");
        assert_eq!(color_gen.get_color(2), "#000");
    }

    #[test]
    fn test_cycle_color_gen() {
        let colors = vec!["#FF0000".to_string(), "#00FF00".to_string()];
        let color_gen = CycleColorGen::new(colors);
        assert_eq!(color_gen.get_color(0), "#FF0000");
        assert_eq!(color_gen.get_color(1), "#00FF00");
        assert_eq!(color_gen.get_color(2), "#FF0000");
        assert_eq!(color_gen.get_color(3), "#00FF00");
    }

    #[test]
    fn test_colors_manager() {
        let mut manager = ColorsManager::new();
        let c1 = manager.get_color(0);
        assert_eq!(c1, 0);
        let c2 = manager.get_color(0);
        assert_eq!(c2, 1);
        manager.release_color(c1, 5);
        // Color should not be reusable until idx 7 (5 + 2)
        let c3 = manager.get_color(6);
        assert_eq!(c3, 2);
        let c4 = manager.get_color(7);
        assert_eq!(c4, 0); // Now color 0 is reusable
    }
}
