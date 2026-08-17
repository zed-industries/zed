use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::Value;
use std::fmt::Write as _;

#[derive(Debug, Clone, thiserror::Error)]
pub enum IconRegistryError {
    #[error("Iconify JSON error: {0}")]
    Json(String),
    #[error("Iconify JSON is missing a non-empty prefix")]
    MissingPrefix,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconSvg {
    body: String,
    left: f64,
    top: f64,
    width: f64,
    height: f64,
}

impl IconSvg {
    pub fn new(body: impl Into<String>, width: f64, height: f64) -> Self {
        Self {
            body: body.into(),
            left: 0.0,
            top: 0.0,
            width: width.max(1.0),
            height: height.max(1.0),
        }
    }

    pub fn with_viewbox(mut self, left: f64, top: f64, width: f64, height: f64) -> Self {
        self.left = left;
        self.top = top;
        self.width = width.max(1.0);
        self.height = height.max(1.0);
        self
    }

    pub fn to_svg(&self, width_px: f64, height_px: f64, extra_class: Option<&str>) -> String {
        let xmlns_xlink = if self.body.contains("xlink:") {
            r#" xmlns:xlink="http://www.w3.org/1999/xlink""#
        } else {
            ""
        };
        let class_attr = extra_class
            .map(|class| format!(r#" class="{}""#, escape_xml_attr(class)))
            .unwrap_or_default();
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg"{xmlns_xlink}{class_attr} width="{w}" height="{h}" viewBox="{left} {top} {vw} {vh}">{body}</svg>"#,
            w = fmt(width_px.max(1.0)),
            h = fmt(height_px.max(1.0)),
            left = fmt(self.left),
            top = fmt(self.top),
            vw = fmt(self.width),
            vh = fmt(self.height),
            body = self.body
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct IconRegistry {
    icons: FxHashMap<String, IconSvg>,
}

impl IconRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.icons.is_empty()
    }

    pub fn insert(&mut self, name: impl Into<String>, icon: IconSvg) {
        self.icons
            .insert(normalize_icon_key(name.into().as_str()), icon);
    }

    pub fn register_iconify_json_str(
        &mut self,
        json: &str,
        prefix_override: Option<&str>,
    ) -> Result<(), IconRegistryError> {
        let value: Value =
            serde_json::from_str(json).map_err(|err| IconRegistryError::Json(err.to_string()))?;
        self.register_iconify_json_value(&value, prefix_override)
    }

    pub fn register_iconify_json_value(
        &mut self,
        value: &Value,
        prefix_override: Option<&str>,
    ) -> Result<(), IconRegistryError> {
        let prefix = prefix_override
            .map(str::trim)
            .filter(|prefix| !prefix.is_empty())
            .or_else(|| value.get("prefix").and_then(Value::as_str).map(str::trim))
            .filter(|prefix| !prefix.is_empty())
            .ok_or(IconRegistryError::MissingPrefix)?;

        let defaults = IconDefaults {
            left: number_field(value, "left").unwrap_or(0.0),
            top: number_field(value, "top").unwrap_or(0.0),
            width: number_field(value, "width").unwrap_or(16.0),
            height: number_field(value, "height").unwrap_or(16.0),
        };

        let icons = value
            .get("icons")
            .and_then(Value::as_object)
            .ok_or_else(|| IconRegistryError::Json("missing `icons` object".to_string()))?;

        for (name, icon_value) in icons {
            if let Some(icon) = icon_from_value(icon_value, defaults) {
                self.insert(format!("{prefix}:{name}"), icon);
            }
        }

        if let Some(aliases) = value.get("aliases").and_then(Value::as_object) {
            for alias_name in aliases.keys() {
                let mut seen = FxHashSet::default();
                if let Some(icon) =
                    resolve_alias_icon(alias_name, aliases, icons, defaults, &mut seen)
                {
                    self.insert(format!("{prefix}:{alias_name}"), icon);
                }
            }
        }

        Ok(())
    }

    pub fn svg_for(
        &self,
        icon_name: &str,
        width_px: f64,
        height_px: f64,
        fallback_prefix: Option<&str>,
        extra_class: Option<&str>,
    ) -> Option<String> {
        let key = resolve_icon_key(icon_name, fallback_prefix)?;
        self.icons
            .get(&key)
            .map(|icon| icon.to_svg(width_px, height_px, extra_class))
    }
}

#[derive(Debug, Clone, Copy)]
struct IconDefaults {
    left: f64,
    top: f64,
    width: f64,
    height: f64,
}

fn resolve_alias_icon(
    alias_name: &str,
    aliases: &serde_json::Map<String, Value>,
    icons: &serde_json::Map<String, Value>,
    defaults: IconDefaults,
    seen: &mut FxHashSet<String>,
) -> Option<IconSvg> {
    if !seen.insert(alias_name.to_string()) {
        return None;
    }

    let alias = aliases.get(alias_name)?;
    let parent = alias.get("parent").and_then(Value::as_str)?;
    let parent_icon = icons
        .get(parent)
        .and_then(|value| icon_from_value(value, defaults))
        .or_else(|| resolve_alias_icon(parent, aliases, icons, defaults, seen))?;

    Some(merge_icon_overrides(parent_icon, alias, defaults))
}

fn icon_from_value(value: &Value, defaults: IconDefaults) -> Option<IconSvg> {
    let body = value.get("body")?.as_str()?.to_string();
    Some(
        IconSvg::new(
            body,
            number_field(value, "width").unwrap_or(defaults.width),
            number_field(value, "height").unwrap_or(defaults.height),
        )
        .with_viewbox(
            number_field(value, "left").unwrap_or(defaults.left),
            number_field(value, "top").unwrap_or(defaults.top),
            number_field(value, "width").unwrap_or(defaults.width),
            number_field(value, "height").unwrap_or(defaults.height),
        ),
    )
}

fn merge_icon_overrides(mut icon: IconSvg, value: &Value, defaults: IconDefaults) -> IconSvg {
    if let Some(body) = value.get("body").and_then(Value::as_str) {
        icon.body = body.to_string();
    }
    icon.left = number_field(value, "left").unwrap_or(icon.left);
    icon.top = number_field(value, "top").unwrap_or(icon.top);
    icon.width = number_field(value, "width")
        .or(Some(icon.width))
        .unwrap_or(defaults.width)
        .max(1.0);
    icon.height = number_field(value, "height")
        .or(Some(icon.height))
        .unwrap_or(defaults.height)
        .max(1.0);
    icon
}

fn number_field(value: &Value, key: &str) -> Option<f64> {
    value.get(key).and_then(number_value)
}

fn number_value(value: &Value) -> Option<f64> {
    match value {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse::<f64>().ok(),
        _ => None,
    }
}

fn resolve_icon_key(icon_name: &str, fallback_prefix: Option<&str>) -> Option<String> {
    let icon_name = icon_name.trim();
    if icon_name.is_empty() {
        return None;
    }

    if let Some(without_provider) = icon_name.strip_prefix('@') {
        let mut parts = without_provider.split(':');
        let _provider = parts.next()?;
        let prefix = parts.next()?;
        let name = parts.next()?;
        if parts.next().is_none() {
            return Some(normalize_icon_key(&format!("{prefix}:{name}")));
        }
        return None;
    }

    let colon_parts = icon_name.split(':').collect::<Vec<_>>();
    match colon_parts.as_slice() {
        [prefix, name] if !prefix.is_empty() && !name.is_empty() => {
            return Some(normalize_icon_key(&format!("{prefix}:{name}")));
        }
        [provider, prefix, name]
            if !provider.is_empty() && !prefix.is_empty() && !name.is_empty() =>
        {
            return Some(normalize_icon_key(&format!("{prefix}:{name}")));
        }
        _ => {}
    }

    if let Some(prefix) = fallback_prefix
        .map(str::trim)
        .filter(|prefix| !prefix.is_empty())
    {
        return Some(normalize_icon_key(&format!("{prefix}:{icon_name}")));
    }

    let mut parts = icon_name.split('-');
    let prefix = parts.next()?;
    let name = parts.collect::<Vec<_>>().join("-");
    if prefix.is_empty() || name.is_empty() {
        return None;
    }
    Some(normalize_icon_key(&format!("{prefix}:{name}")))
}

fn normalize_icon_key(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
}

fn escape_xml_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn fmt(value: f64) -> String {
    if !value.is_finite() {
        return "0".to_string();
    }
    let mut out = String::new();
    let _ = write!(&mut out, "{value:.6}");
    while out.contains('.') && out.ends_with('0') {
        out.pop();
    }
    if out.ends_with('.') {
        out.pop();
    }
    if out == "-0" { "0".to_string() } else { out }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_iconify_json_icons_and_aliases() {
        let json = r##"{
            "prefix": "test",
            "width": 24,
            "height": 24,
            "icons": {
                "rocket": { "body": "<path id=\"rocket\" d=\"M1 1H23V23H1z\"/>" }
            },
            "aliases": {
                "ship": { "parent": "rocket", "width": 32 }
            }
        }"##;

        let mut registry = IconRegistry::new();
        registry.register_iconify_json_str(json, None).unwrap();

        let rocket = registry
            .svg_for("test:rocket", 48.0, 48.0, None, None)
            .unwrap();
        assert!(rocket.contains(r#"viewBox="0 0 24 24""#));
        assert!(rocket.contains(r#"id="rocket""#));

        let alias = registry
            .svg_for("test:ship", 48.0, 48.0, None, None)
            .unwrap();
        assert!(alias.contains(r#"viewBox="0 0 32 24""#));
    }

    #[test]
    fn resolves_hyphen_icon_names_without_fallback_prefix() {
        let mut registry = IconRegistry::new();
        registry.insert("logos:aws-lambda", IconSvg::new("<path/>", 16.0, 16.0));

        assert!(
            registry
                .svg_for("logos-aws-lambda", 16.0, 16.0, None, None)
                .is_some()
        );
    }
}
