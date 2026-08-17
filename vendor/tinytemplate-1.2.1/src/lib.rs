//! ## TinyTemplate
//!
//! TinyTemplate is a minimal templating library originally designed for use in [Criterion.rs].
//! It deliberately does not provide all of the features of a full-power template engine, but in
//! return it provides a simple API, clear templating syntax, decent performance and very few
//! dependencies.
//!
//! ## Features
//!
//! The most important features are as follows (see the [syntax](syntax/index.html) module for full
//! details on the template syntax):
//!
//! * Rendering values - `{ myvalue }`
//! * Conditionals - `{{ if foo }}Foo is true{{ else }}Foo is false{{ endif }}`
//! * Loops - `{{ for value in row }}{value}{{ endfor }}`
//! * Customizable value formatters `{ value | my_formatter }`
//! * Macros `{{ call my_template with foo }}`
//!
//! ## Restrictions
//!
//! TinyTemplate was designed with the assumption that the templates are available as static strings,
//! either using string literals or the `include_str!` macro. Thus, it borrows `&str` slices from the
//! template text itself and uses them during the rendering process. Although it is possible to use
//! TinyTemplate with template strings loaded at runtime, this is not recommended.
//!
//! Additionally, TinyTemplate can only render templates into Strings. If you need to render a
//! template directly to a socket or file, TinyTemplate may not be right for you.
//!
//! ## Example
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate tinytemplate;
//!
//! use tinytemplate::TinyTemplate;
//! use std::error::Error;
//!
//! #[derive(Serialize)]
//! struct Context {
//!     name: String,
//! }
//!
//! static TEMPLATE : &'static str = "Hello {name}!";
//!
//! pub fn main() -> Result<(), Box<Error>> {
//!     let mut tt = TinyTemplate::new();
//!     tt.add_template("hello", TEMPLATE)?;
//!
//!     let context = Context {
//!         name: "World".to_string(),
//!     };
//!
//!     let rendered = tt.render("hello", &context)?;
//! #   assert_eq!("Hello World!", &rendered);
//!     println!("{}", rendered);
//!
//!     Ok(())
//! }
//! ```
//!
//! [Criterion.rs]: https://github.com/bheisler/criterion.rs
//!

extern crate serde;
extern crate serde_json;

#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate serde_derive;

mod compiler;
pub mod error;
mod instruction;
pub mod syntax;
mod template;

use error::*;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Write;
use template::Template;

/// Type alias for closures which can be used as value formatters.
pub type ValueFormatter = dyn Fn(&Value, &mut String) -> Result<()>;

/// Appends `value` to `output`, performing HTML-escaping in the process.
pub fn escape(value: &str, output: &mut String) {
    // Algorithm taken from the rustdoc source code.
    let value_str = value;
    let mut last_emitted = 0;
    for (i, ch) in value.bytes().enumerate() {
        match ch as char {
            '<' | '>' | '&' | '\'' | '"' => {
                output.push_str(&value_str[last_emitted..i]);
                let s = match ch as char {
                    '>' => "&gt;",
                    '<' => "&lt;",
                    '&' => "&amp;",
                    '\'' => "&#39;",
                    '"' => "&quot;",
                    _ => unreachable!(),
                };
                output.push_str(s);
                last_emitted = i + 1;
            }
            _ => {}
        }
    }

    if last_emitted < value_str.len() {
        output.push_str(&value_str[last_emitted..]);
    }
}

/// The format function is used as the default value formatter for all values unless the user
/// specifies another. It is provided publicly so that it can be called as part of custom formatters.
/// Values are formatted as follows:
///
/// * `Value::Null` => the empty string
/// * `Value::Bool` => true|false
/// * `Value::Number` => the number, as formatted by `serde_json`.
/// * `Value::String` => the string, HTML-escaped
///
/// Arrays and objects are not formatted, and attempting to do so will result in a rendering error.
pub fn format(value: &Value, output: &mut String) -> Result<()> {
    match value {
        Value::Null => Ok(()),
        Value::Bool(b) => {
            write!(output, "{}", b)?;
            Ok(())
        }
        Value::Number(n) => {
            write!(output, "{}", n)?;
            Ok(())
        }
        Value::String(s) => {
            escape(s, output);
            Ok(())
        }
        _ => Err(unprintable_error()),
    }
}

/// Identical to [`format`](fn.format.html) except that this does not perform HTML escaping.
pub fn format_unescaped(value: &Value, output: &mut String) -> Result<()> {
    match value {
        Value::Null => Ok(()),
        Value::Bool(b) => {
            write!(output, "{}", b)?;
            Ok(())
        }
        Value::Number(n) => {
            write!(output, "{}", n)?;
            Ok(())
        }
        Value::String(s) => {
            output.push_str(s);
            Ok(())
        }
        _ => Err(unprintable_error()),
    }
}

/// The TinyTemplate struct is the entry point for the TinyTemplate library. It contains the
/// template and formatter registries and provides functions to render templates as well as to
/// register templates and formatters.
pub struct TinyTemplate<'template> {
    templates: HashMap<&'template str, Template<'template>>,
    formatters: HashMap<&'template str, Box<ValueFormatter>>,
    default_formatter: &'template ValueFormatter,
}
impl<'template> TinyTemplate<'template> {
    /// Create a new TinyTemplate registry. The returned registry contains no templates, and has
    /// [`format_unescaped`](fn.format_unescaped.html) registered as a formatter named "unescaped".
    pub fn new() -> TinyTemplate<'template> {
        let mut tt = TinyTemplate {
            templates: HashMap::default(),
            formatters: HashMap::default(),
            default_formatter: &format,
        };
        tt.add_formatter("unescaped", format_unescaped);
        tt
    }

    /// Parse and compile the given template, then register it under the given name.
    pub fn add_template(&mut self, name: &'template str, text: &'template str) -> Result<()> {
        let template = Template::compile(text)?;
        self.templates.insert(name, template);
        Ok(())
    }

    /// Changes the default formatter from [`format`](fn.format.html) to `formatter`. Usefull in combination with [`format_unescaped`](fn.format_unescaped.html) to deactivate HTML-escaping
    pub fn set_default_formatter<F>(&mut self, formatter: &'template F)
    where
        F: 'static + Fn(&Value, &mut String) -> Result<()>,
    {
        self.default_formatter = formatter;
    }

    /// Register the given formatter function under the given name.
    pub fn add_formatter<F>(&mut self, name: &'template str, formatter: F)
    where
        F: 'static + Fn(&Value, &mut String) -> Result<()>,
    {
        self.formatters.insert(name, Box::new(formatter));
    }

    /// Render the template with the given name using the given context object. The context
    /// object must implement `serde::Serialize` as it will be converted to `serde_json::Value`.
    pub fn render<C>(&self, template: &str, context: &C) -> Result<String>
    where
        C: Serialize,
    {
        let value = serde_json::to_value(context)?;
        match self.templates.get(template) {
            Some(tmpl) => tmpl.render(
                &value,
                &self.templates,
                &self.formatters,
                self.default_formatter,
            ),
            None => Err(Error::GenericError {
                msg: format!("Unknown template '{}'", template),
            }),
        }
    }
}
impl<'template> Default for TinyTemplate<'template> {
    fn default() -> TinyTemplate<'template> {
        TinyTemplate::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Serialize)]
    struct Context {
        name: String,
    }

    static TEMPLATE: &'static str = "Hello {name}!";

    #[test]
    pub fn test_set_default_formatter() {
        let mut tt = TinyTemplate::new();
        tt.add_template("hello", TEMPLATE).unwrap();
        tt.set_default_formatter(&format_unescaped);

        let context = Context {
            name: "<World>".to_string(),
        };

        let rendered = tt.render("hello", &context).unwrap();
        assert_eq!(rendered, "Hello <World>!")
    }
}
