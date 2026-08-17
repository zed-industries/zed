use serde::Serialize;
use serde_json::value::{to_value, Value as Json};

pub(crate) static DEFAULT_VALUE: Json = Json::Null;

/// A JSON wrapper designed for handlebars internal use case
///
/// * Constant: the JSON value hardcoded into template
/// * Context:  the JSON value referenced in your provided data context
/// * Derived:  the owned JSON value computed during rendering process
///
#[derive(Debug)]
pub enum ScopedJson<'reg: 'rc, 'rc> {
    Constant(&'reg Json),
    Derived(Json),
    // represents a json reference to context value, its full path
    Context(&'rc Json, Vec<String>),
    Missing,
}

impl<'reg: 'rc, 'rc> ScopedJson<'reg, 'rc> {
    /// get the JSON reference
    pub fn as_json(&self) -> &Json {
        match self {
            ScopedJson::Constant(j) => j,
            ScopedJson::Derived(ref j) => j,
            ScopedJson::Context(j, _) => j,
            _ => &DEFAULT_VALUE,
        }
    }

    pub fn render(&self) -> String {
        self.as_json().render()
    }

    pub fn is_missing(&self) -> bool {
        matches!(self, ScopedJson::Missing)
    }

    pub fn into_derived(self) -> ScopedJson<'reg, 'rc> {
        let v = self.as_json();
        ScopedJson::Derived(v.clone())
    }

    pub fn context_path(&self) -> Option<&Vec<String>> {
        match self {
            ScopedJson::Context(_, ref p) => Some(p),
            _ => None,
        }
    }
}

impl<'reg: 'rc, 'rc> From<Json> for ScopedJson<'reg, 'rc> {
    fn from(v: Json) -> ScopedJson<'reg, 'rc> {
        ScopedJson::Derived(v)
    }
}

/// Json wrapper that holds the Json value and reference path information
///
#[derive(Debug)]
pub struct PathAndJson<'reg, 'rc> {
    relative_path: Option<String>,
    value: ScopedJson<'reg, 'rc>,
}

impl<'reg: 'rc, 'rc> PathAndJson<'reg, 'rc> {
    pub fn new(
        relative_path: Option<String>,
        value: ScopedJson<'reg, 'rc>,
    ) -> PathAndJson<'reg, 'rc> {
        PathAndJson {
            relative_path,
            value,
        }
    }

    /// Returns relative path when the value is referenced
    /// If the value is from a literal, the path is `None`
    pub fn relative_path(&self) -> Option<&String> {
        self.relative_path.as_ref()
    }

    /// Returns full path to this value if any
    pub fn context_path(&self) -> Option<&Vec<String>> {
        self.value.context_path()
    }

    /// Returns the value
    pub fn value(&self) -> &Json {
        self.value.as_json()
    }

    /// Test if value is missing
    pub fn is_value_missing(&self) -> bool {
        self.value.is_missing()
    }

    pub fn render(&self) -> String {
        self.value.render()
    }
}

/// Render Json data with default format
pub trait JsonRender {
    fn render(&self) -> String;
}

pub trait JsonTruthy {
    fn is_truthy(&self, include_zero: bool) -> bool;
}

impl JsonRender for Json {
    fn render(&self) -> String {
        match *self {
            Json::String(ref s) => s.to_string(),
            Json::Bool(i) => i.to_string(),
            Json::Number(ref n) => n.to_string(),
            Json::Null => "".to_owned(),
            Json::Array(ref a) => {
                let mut buf = String::new();
                buf.push('[');
                for i in a.iter() {
                    buf.push_str(i.render().as_ref());
                    buf.push_str(", ");
                }
                buf.push(']');
                buf
            }
            Json::Object(_) => "[object]".to_owned(),
        }
    }
}

/// Convert any serializable data into Serde Json type
pub fn to_json<T>(src: T) -> Json
where
    T: Serialize,
{
    to_value(src).unwrap_or_default()
}

pub fn as_string(src: &Json) -> Option<&str> {
    src.as_str()
}

impl JsonTruthy for Json {
    fn is_truthy(&self, include_zero: bool) -> bool {
        match *self {
            Json::Bool(ref i) => *i,
            Json::Number(ref n) => {
                if include_zero {
                    n.as_f64().map(|f| !f.is_nan()).unwrap_or(false)
                } else {
                    // there is no inifity in json/serde_json
                    n.as_f64().map(|f| f.is_normal()).unwrap_or(false)
                }
            }
            Json::Null => false,
            Json::String(ref i) => !i.is_empty(),
            Json::Array(ref i) => !i.is_empty(),
            Json::Object(ref i) => !i.is_empty(),
        }
    }
}

#[test]
fn test_json_render() {
    let raw = "<p>Hello world</p>\n<p thing=\"hello\"</p>";
    let thing = Json::String(raw.to_string());

    assert_eq!(raw, thing.render());
}

#[test]
fn test_json_number_truthy() {
    use std::f64;
    assert!(json!(16i16).is_truthy(false));
    assert!(json!(16i16).is_truthy(true));

    assert!(json!(0i16).is_truthy(true));
    assert!(!json!(0i16).is_truthy(false));

    assert!(json!(1.0f64).is_truthy(false));
    assert!(json!(1.0f64).is_truthy(true));

    assert!(json!(Some(16i16)).is_truthy(false));
    assert!(json!(Some(16i16)).is_truthy(true));

    assert!(!json!(None as Option<i16>).is_truthy(false));
    assert!(!json!(None as Option<i16>).is_truthy(true));

    assert!(!json!(f64::NAN).is_truthy(false));
    assert!(!json!(f64::NAN).is_truthy(true));

    // there is no infinity in json/serde_json
    // assert!(json!(f64::INFINITY).is_truthy(false));
    // assert!(json!(f64::INFINITY).is_truthy(true));

    // assert!(json!(f64::NEG_INFINITY).is_truthy(false));
    // assert!(json!(f64::NEG_INFINITY).is_truthy(true));
}
