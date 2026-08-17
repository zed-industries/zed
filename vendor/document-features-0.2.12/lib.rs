// Copyright © SixtyFPS GmbH <info@sixtyfps.io>
// SPDX-License-Identifier: MIT OR Apache-2.0

/*!
Document your crate's feature flags.

This crates provides a macro that extracts "documentation" comments from Cargo.toml

To use this crate, add `#![doc = document_features::document_features!()]` in your crate documentation.
The `document_features!()` macro reads your `Cargo.toml` file, extracts feature comments and generates
a markdown string for your documentation.

Basic example:

```rust
//! Normal crate documentation goes here.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]

// rest of the crate goes here.
```

## Documentation format:

The documentation of your crate features goes into `Cargo.toml`, where they are defined.

The `document_features!()` macro analyzes the contents of `Cargo.toml`.
Similar to Rust's documentation comments `///` and `//!`, the macro understands
comments that start with `## ` and `#! `. Note the required trailing space.
Lines starting with `###` will not be understood as doc comment.

`## ` comments are meant to be *above* the feature they document.
There can be several `## ` comments, but they must always be followed by a
feature name or an optional dependency.
There should not be `#! ` comments between the comment and the feature they document.

`#! ` comments are not associated with a particular feature, and will be printed
in where they occur. Use them to group features, for example.

## Examples:

*/
#![doc = self_test!(/**
[package]
name = "..."
# ...

[features]
default = ["foo"]
#! This comments goes on top

## The foo feature enables the `foo` functions
foo = []

## The bar feature enables the bar module
bar = []

#! ### Experimental features
#! The following features are experimental

## Enable the fusion reactor
##
## ⚠️ Can lead to explosions
fusion = []

[dependencies]
document-features = "0.2"

#! ### Optional dependencies

## Enable this feature to implement the trait for the types from the genial crate
genial = { version = "0.2", optional = true }

## This awesome dependency is specified in its own table
[dependencies.awesome]
version = "1.3.5"
optional = true
*/
=>
    /**
This comments goes on top
* **`foo`** *(enabled by default)* —  The foo feature enables the `foo` functions
* **`bar`** —  The bar feature enables the bar module

#### Experimental features
The following features are experimental
* **`fusion`** —  Enable the fusion reactor

  ⚠️ Can lead to explosions

#### Optional dependencies
* **`genial`** —  Enable this feature to implement the trait for the types from the genial crate
* **`awesome`** —  This awesome dependency is specified in its own table
*/
)]
/*!

## Customization

You can customize the formatting of the features in the generated documentation by setting
the key **`feature_label=`** to a given format string. This format string must be either
a [string literal](https://doc.rust-lang.org/reference/tokens.html#string-literals) or
a [raw string literal](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals).
Every occurrence of `{feature}` inside the format string will be substituted with the name of the feature.

For instance, to emulate the HTML formatting used by `rustdoc` one can use the following:

```rust
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]
```

The default formatting is equivalent to:

```rust
#![doc = document_features::document_features!(feature_label = "**`{feature}`**")]
```

## Compatibility

The minimum Rust version required to use this crate is Rust 1.56.
You can make this crate optional and use `#[cfg_attr()]` statements to enable it only when building the documentation:
You need to have two levels of `cfg_attr` because Rust < 1.54 doesn't parse the attribute
otherwise.

```rust,ignore
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
```

In your Cargo.toml, enable this feature while generating the documentation on docs.rs:

```toml
[dependencies]
document-features = { version = "0.2", optional = true }

[package.metadata.docs.rs]
features = ["document-features"]
## Alternative: enable all features so they are all documented
## all-features = true
```
 */

#[cfg(not(feature = "default"))]
compile_error!(
    "The feature `default` must be enabled to ensure \
    forward compatibility with future version of this crate"
);

extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt::Write;
use std::path::Path;
use std::str::FromStr;

fn error(e: &str) -> TokenStream {
    TokenStream::from_str(&format!("::core::compile_error!{{\"{}\"}}", e.escape_default())).unwrap()
}

fn compile_error(msg: &str, tt: Option<TokenTree>) -> TokenStream {
    let span = tt.as_ref().map_or_else(proc_macro::Span::call_site, TokenTree::span);
    use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing};
    use std::iter::FromIterator;
    TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct({
            let mut punct = Punct::new('!', Spacing::Alone);
            punct.set_span(span);
            punct
        }),
        TokenTree::Group({
            let mut group = Group::new(Delimiter::Brace, {
                TokenStream::from_iter([TokenTree::Literal({
                    let mut string = Literal::string(msg);
                    string.set_span(span);
                    string
                })])
            });
            group.set_span(span);
            group
        }),
    ])
}

#[derive(Default)]
struct Args {
    feature_label: Option<String>,
}

fn parse_args(input: TokenStream) -> Result<Args, TokenStream> {
    let mut token_trees = input.into_iter().fuse();

    // parse the key, ensuring that it is the identifier `feature_label`
    match token_trees.next() {
        None => return Ok(Args::default()),
        Some(TokenTree::Ident(ident)) if ident.to_string() == "feature_label" => (),
        tt => return Err(compile_error("expected `feature_label`", tt)),
    }

    // parse a single equal sign `=`
    match token_trees.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == '=' => (),
        tt => return Err(compile_error("expected `=`", tt)),
    }

    // parse the value, ensuring that it is a string literal containing the substring `"{feature}"`
    let feature_label;
    if let Some(tt) = token_trees.next() {
        match litrs::StringLit::<String>::try_from(&tt) {
            Ok(string_lit) if string_lit.value().contains("{feature}") => {
                feature_label = string_lit.into_value()
            }
            _ => {
                return Err(compile_error(
                    "expected a string literal containing the substring \"{feature}\"",
                    Some(tt),
                ))
            }
        }
    } else {
        return Err(compile_error(
            "expected a string literal containing the substring \"{feature}\"",
            None,
        ));
    }

    // ensure there is nothing left after the format string
    if let tt @ Some(_) = token_trees.next() {
        return Err(compile_error("unexpected token after the format string", tt));
    }

    Ok(Args { feature_label: Some(feature_label) })
}

/// Produce a literal string containing documentation extracted from Cargo.toml
///
/// See the [crate] documentation for details
#[proc_macro]
pub fn document_features(tokens: TokenStream) -> TokenStream {
    parse_args(tokens)
        .and_then(|args| document_features_impl(&args))
        .unwrap_or_else(std::convert::identity)
}

fn document_features_impl(args: &Args) -> Result<TokenStream, TokenStream> {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut cargo_toml = std::fs::read_to_string(Path::new(&path).join("Cargo.toml"))
        .map_err(|e| error(&format!("Can't open Cargo.toml: {:?}", e)))?;

    if !has_doc_comments(&cargo_toml) {
        // On crates.io, Cargo.toml is usually "normalized" and stripped of all comments.
        // The original Cargo.toml has been renamed Cargo.toml.orig
        if let Ok(orig) = std::fs::read_to_string(Path::new(&path).join("Cargo.toml.orig")) {
            if has_doc_comments(&orig) {
                cargo_toml = orig;
            }
        }
    }

    let result = process_toml(&cargo_toml, args).map_err(|e| error(&e))?;
    Ok(std::iter::once(proc_macro::TokenTree::from(proc_macro::Literal::string(&result))).collect())
}

/// Check if the Cargo.toml has comments that looks like doc comments.
fn has_doc_comments(cargo_toml: &str) -> bool {
    let mut lines = cargo_toml.lines().map(str::trim);
    while let Some(line) = lines.next() {
        if line.starts_with("## ") || line.starts_with("#! ") {
            return true;
        }
        let before_coment = line.split_once('#').map_or(line, |(before, _)| before);
        if line.starts_with("#") {
            continue;
        }
        if let Some((_, mut quote)) = before_coment.split_once("\"\"\"") {
            loop {
                // skip slashes.
                if let Some((_, s)) = quote.split_once('\\') {
                    quote = s.strip_prefix('\\').or_else(|| s.strip_prefix('"')).unwrap_or(s);
                    continue;
                }
                // skip quotes.
                if let Some((_, out_quote)) = quote.split_once("\"\"\"") {
                    let out_quote = out_quote.trim_start_matches('"');
                    let out_quote =
                        out_quote.split_once('#').map_or(out_quote, |(before, _)| before);
                    if let Some((_, q)) = out_quote.split_once("\"\"\"") {
                        quote = q;
                        continue;
                    }
                    break;
                };
                match lines.next() {
                    Some(l) => quote = l,
                    None => return false,
                }
            }
        }
    }
    false
}

#[test]
fn test_has_doc_comments() {
    assert!(has_doc_comments("foo\nbar\n## comment\nddd"));
    assert!(!has_doc_comments("foo\nbar\n#comment\nddd"));
    assert!(!has_doc_comments(
        r#"
[[package.metadata.release.pre-release-replacements]]
exactly = 1 # not a doc comment
file = "CHANGELOG.md"
replace = """
<!-- next-header -->
## [Unreleased] - ReleaseDate
"""
search = "<!-- next-header -->"
array = ["""foo""", """
bar""", """eee
## not a comment
"""]
    "#
    ));
    assert!(has_doc_comments(
        r#"
[[package.metadata.release.pre-release-replacements]]
exactly = 1 # """
file = "CHANGELOG.md"
replace = """
<!-- next-header -->
## [Unreleased] - ReleaseDate
"""
search = "<!-- next-header -->"
array = ["""foo""", """
bar""", """eee
## not a comment
"""]
## This is a comment
feature = "45"
        "#
    ));

    assert!(!has_doc_comments(
        r#"
[[package.metadata.release.pre-release-replacements]]
value = """" string \"""
## within the string
\""""
another_string = """"" # """
## also within"""
"#
    ));

    assert!(has_doc_comments(
        r#"
[[package.metadata.release.pre-release-replacements]]
value = """" string \"""
## within the string
\""""
another_string = """"" # """
## also within"""
## out of the string
foo = bar
        "#
    ));
}

fn dependents(
    feature_dependencies: &HashMap<String, Vec<String>>,
    feature: &str,
    collected: &mut HashSet<String>,
) {
    if collected.contains(feature) {
        return;
    }
    collected.insert(feature.to_string());
    if let Some(dependencies) = feature_dependencies.get(feature) {
        for dependency in dependencies {
            dependents(feature_dependencies, dependency, collected);
        }
    }
}

fn parse_feature_deps<'a>(
    s: &'a str,
    dep: &str,
) -> Result<impl Iterator<Item = String> + 'a, String> {
    Ok(s.trim()
        .strip_prefix('[')
        .and_then(|r| r.strip_suffix(']'))
        .ok_or_else(|| format!("Parse error while parsing dependency {}", dep))?
        .split(',')
        .map(|d| d.trim().trim_matches(|c| c == '"' || c == '\'').trim().to_string())
        .filter(|d: &String| !d.is_empty()))
}

fn process_toml(cargo_toml: &str, args: &Args) -> Result<String, String> {
    // Get all lines between the "[features]" and the next block
    let mut lines = cargo_toml
        .lines()
        .map(str::trim)
        // and skip empty lines and comments that are not docs comments
        .filter(|l| {
            !l.is_empty() && (!l.starts_with('#') || l.starts_with("##") || l.starts_with("#!"))
        });
    let mut top_comment = String::new();
    let mut current_comment = String::new();
    let mut features = vec![];
    let mut default_features = HashSet::new();
    let mut current_table = "";
    let mut dependencies = HashMap::new();
    while let Some(line) = lines.next() {
        if let Some(x) = line.strip_prefix("#!") {
            if !x.is_empty() && !x.starts_with(' ') {
                continue; // it's not a doc comment
            }
            if !current_comment.is_empty() {
                return Err("Cannot mix ## and #! comments between features.".into());
            }
            if top_comment.is_empty() && !features.is_empty() {
                top_comment = "\n".into();
            }
            writeln!(top_comment, "{}", x).unwrap();
        } else if let Some(x) = line.strip_prefix("##") {
            if !x.is_empty() && !x.starts_with(' ') {
                continue; // it's not a doc comment
            }
            writeln!(current_comment, " {}", x).unwrap();
        } else if let Some(table) = line.strip_prefix('[') {
            current_table = table
                .split_once(']')
                .map(|(t, _)| t.trim())
                .ok_or_else(|| format!("Parse error while parsing line: {}", line))?;
            if !current_comment.is_empty() {
                #[allow(clippy::unnecessary_lazy_evaluations)]
                let dep = current_table
                    .rsplit_once('.')
                    .and_then(|(table, dep)| table.trim().ends_with("dependencies").then(|| dep))
                    .ok_or_else(|| format!("Not a feature: `{}`", line))?;
                features.push((
                    dep.trim(),
                    std::mem::take(&mut top_comment),
                    std::mem::take(&mut current_comment),
                ));
            }
        } else if let Some((dep, rest)) = line.split_once('=') {
            let dep = dep.trim().trim_matches('"');
            let rest = get_balanced(rest, &mut lines)
                .map_err(|e| format!("Parse error while parsing value {}: {}", dep, e))?;
            if current_table == "features" {
                if dep == "default" {
                    default_features.extend(parse_feature_deps(&rest, dep)?);
                } else {
                    for d in parse_feature_deps(&rest, dep)? {
                        dependencies
                            .entry(dep.to_string())
                            .or_insert_with(Vec::new)
                            .push(d.clone());
                    }
                }
            }
            if !current_comment.is_empty() {
                if current_table.ends_with("dependencies") {
                    if !rest
                        .split_once("optional")
                        .and_then(|(_, r)| r.trim().strip_prefix('='))
                        .map_or(false, |r| r.trim().starts_with("true"))
                    {
                        return Err(format!("Dependency {} is not an optional dependency", dep));
                    }
                } else if current_table != "features" {
                    return Err(format!(
                        r#"Comment cannot be associated with a feature: "{}""#,
                        current_comment.trim()
                    ));
                }
                features.push((
                    dep,
                    std::mem::take(&mut top_comment),
                    std::mem::take(&mut current_comment),
                ));
            }
        }
    }
    let df = default_features.iter().cloned().collect::<Vec<_>>();
    for feature in df {
        let mut resolved = HashSet::new();
        dependents(&dependencies, &feature, &mut resolved);
        default_features.extend(resolved.into_iter());
    }
    if !current_comment.is_empty() {
        return Err("Found comment not associated with a feature".into());
    }
    if features.is_empty() {
        return Ok("*No documented features in Cargo.toml*".into());
    }
    let mut result = String::new();
    for (f, top, comment) in features {
        let default = if default_features.contains(f) { " *(enabled by default)*" } else { "" };
        let feature_label = args.feature_label.as_deref().unwrap_or("**`{feature}`**");
        let comment = if comment.trim().is_empty() {
            String::new()
        } else {
            format!(" —{}", comment.trim_end())
        };

        writeln!(
            result,
            "{}* {}{}{}",
            top,
            feature_label.replace("{feature}", f),
            default,
            comment,
        )
        .unwrap();
    }
    result += &top_comment;
    Ok(result)
}

fn get_balanced<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = &'a str>,
) -> Result<Cow<'a, str>, String> {
    let mut line = first_line;
    let mut result = Cow::from("");

    let mut in_quote = false;
    let mut level = 0;
    loop {
        let mut last_slash = false;
        for (idx, b) in line.as_bytes().iter().enumerate() {
            if last_slash {
                last_slash = false
            } else if in_quote {
                match b {
                    b'\\' => last_slash = true,
                    b'"' | b'\'' => in_quote = false,
                    _ => (),
                }
            } else {
                match b {
                    b'\\' => last_slash = true,
                    b'"' => in_quote = true,
                    b'{' | b'[' => level += 1,
                    b'}' | b']' if level == 0 => return Err("unbalanced source".into()),
                    b'}' | b']' => level -= 1,
                    b'#' => {
                        line = &line[..idx];
                        break;
                    }
                    _ => (),
                }
            }
        }
        if result.len() == 0 {
            result = Cow::from(line);
        } else {
            *result.to_mut() += line;
        }
        if level == 0 {
            return Ok(result);
        }
        line = if let Some(l) = lines.next() {
            l
        } else {
            return Err("unbalanced source".into());
        };
    }
}

#[test]
fn test_get_balanced() {
    assert_eq!(
        get_balanced(
            "{",
            &mut IntoIterator::into_iter(["a", "{ abc[], #ignore", " def }", "}", "xxx"])
        ),
        Ok("{a{ abc[],  def }}".into())
    );
    assert_eq!(
        get_balanced("{ foo = \"{#\" } #ignore", &mut IntoIterator::into_iter(["xxx"])),
        Ok("{ foo = \"{#\" } ".into())
    );
    assert_eq!(
        get_balanced("]", &mut IntoIterator::into_iter(["["])),
        Err("unbalanced source".into())
    );
}

#[cfg(feature = "self-test")]
#[proc_macro]
#[doc(hidden)]
/// Helper macro for the tests. Do not use
pub fn self_test_helper(input: TokenStream) -> TokenStream {
    let mut code = String::new();
    for line in (&input).to_string().trim_matches(|c| c == '"' || c == '#').lines() {
        // Rustdoc removes the lines that starts with `# ` and removes one `#` from lines that starts with # followed by space.
        // We need to re-add the `#` that was removed by rustdoc to get the original.
        if line.strip_prefix('#').map_or(false, |x| x.is_empty() || x.starts_with(' ')) {
            code += "#";
        }
        code += line;
        code += "\n";
    }
    process_toml(&code, &Args::default()).map_or_else(
        |e| error(&e),
        |r| std::iter::once(proc_macro::TokenTree::from(proc_macro::Literal::string(&r))).collect(),
    )
}

#[cfg(feature = "self-test")]
macro_rules! self_test {
    (#[doc = $toml:literal] => #[doc = $md:literal]) => {
        concat!(
            "\n`````rust\n\
            fn normalize_md(md : &str) -> String {
               md.lines().skip_while(|l| l.is_empty()).map(|l| l.trim())
                .collect::<Vec<_>>().join(\"\\n\")
            }
            assert_eq!(normalize_md(document_features::self_test_helper!(",
            stringify!($toml),
            ")), normalize_md(",
            stringify!($md),
            "));\n`````\n\n"
        )
    };
}

#[cfg(not(feature = "self-test"))]
macro_rules! self_test {
    (#[doc = $toml:literal] => #[doc = $md:literal]) => {
        concat!(
            "This contents in Cargo.toml:\n`````toml",
            $toml,
            "\n`````\n Generates the following:\n\
            <table><tr><th>Preview</th></tr><tr><td>\n\n",
            $md,
            "\n</td></tr></table>\n\n&nbsp;\n",
        )
    };
}

#[allow(unused)] // Workaround until https://github.com/rust-lang/rust/pull/147914 is merged
use self_test;

// The following struct is inserted only during generation of the documentation in order to exploit doc-tests.
// These doc-tests are used to check that invalid arguments to the `document_features!` macro cause a compile time error.
// For a more principled way of testing compilation error, maybe investigate <https://docs.rs/trybuild>.
//
/// ```rust
/// #![doc = document_features::document_features!()]
/// #![doc = document_features::document_features!(feature_label = "**`{feature}`**")]
/// #![doc = document_features::document_features!(feature_label = r"**`{feature}`**")]
/// #![doc = document_features::document_features!(feature_label = r#"**`{feature}`**"#)]
/// #![doc = document_features::document_features!(feature_label = "<span class=\"stab portability\"><code>{feature}</code></span>")]
/// #![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]
/// ```
/// ```compile_fail
/// #![doc = document_features::document_features!(feature_label > "<span>{feature}</span>")]
/// ```
/// ```compile_fail
/// #![doc = document_features::document_features!(label = "<span>{feature}</span>")]
/// ```
/// ```compile_fail
/// #![doc = document_features::document_features!(feature_label = "{feat}")]
/// ```
/// ```compile_fail
/// #![doc = document_features::document_features!(feature_label = 3.14)]
/// ```
/// ```compile_fail
/// #![doc = document_features::document_features!(feature_label = )]
/// ```
/// ```compile_fail
/// #![doc = document_features::document_features!(feature_label = "**`{feature}`**" extra)]
/// ```
#[cfg(doc)]
struct FeatureLabelCompilationTest;

#[cfg(test)]
mod tests {
    use super::{process_toml, Args};

    #[track_caller]
    fn test_error(toml: &str, expected: &str) {
        let err = process_toml(toml, &Args::default()).unwrap_err();
        assert!(err.contains(expected), "{:?} does not contain {:?}", err, expected)
    }

    #[test]
    fn only_get_balanced_in_correct_table() {
        process_toml(
            r#"

[package.metadata.release]
pre-release-replacements = [
  {test=\"\#\# \"},
]
[abcd]
[features]#xyz
#! abc
#
###
#! def
#!
## 123
## 456
feat1 = ["plop"]
#! ghi
no_doc = []
##
feat2 = ["momo"]
#! klm
default = ["feat1", "something_else"]
#! end
            "#,
            &Args::default(),
        )
        .unwrap();
    }

    #[test]
    fn no_features() {
        let r = process_toml(
            r#"
[features]
[dependencies]
foo = 4;
"#,
            &Args::default(),
        )
        .unwrap();
        assert_eq!(r, "*No documented features in Cargo.toml*");
    }

    #[test]
    fn no_features2() {
        let r = process_toml(
            r#"
[packages]
[dependencies]
"#,
            &Args::default(),
        )
        .unwrap();
        assert_eq!(r, "*No documented features in Cargo.toml*");
    }

    #[test]
    fn parse_error3() {
        test_error(
            r#"
[features]
ff = []
[abcd
efgh
[dependencies]
"#,
            "Parse error while parsing line: [abcd",
        );
    }

    #[test]
    fn parse_error4() {
        test_error(
            r#"
[features]
## dd
## ff
#! ee
## ff
"#,
            "Cannot mix",
        );
    }

    #[test]
    fn parse_error5() {
        test_error(
            r#"
[features]
## dd
"#,
            "not associated with a feature",
        );
    }

    #[test]
    fn parse_error6() {
        test_error(
            r#"
[features]
# ff
foo = []
default = [
#ffff
# ff
"#,
            "Parse error while parsing value default",
        );
    }

    #[test]
    fn parse_error7() {
        test_error(
            r#"
[features]
# f
foo = [ x = { ]
bar = []
"#,
            "Parse error while parsing value foo",
        );
    }

    #[test]
    fn not_a_feature1() {
        test_error(
            r#"
## hallo
[features]
"#,
            "Not a feature: `[features]`",
        );
    }

    #[test]
    fn not_a_feature2() {
        test_error(
            r#"
[package]
## hallo
foo = []
"#,
            "Comment cannot be associated with a feature: \"hallo\"",
        );
    }

    #[test]
    fn non_optional_dep1() {
        test_error(
            r#"
[dev-dependencies]
## Not optional
foo = { version = "1.2", optional = false }
"#,
            "Dependency foo is not an optional dependency",
        );
    }

    #[test]
    fn non_optional_dep2() {
        test_error(
            r#"
[dev-dependencies]
## Not optional
foo = { version = "1.2" }
"#,
            "Dependency foo is not an optional dependency",
        );
    }

    #[test]
    fn basic() {
        let toml = r#"
[abcd]
[features]#xyz
#! abc
#
###
#! def
#!
## 123
## 456
feat1 = ["plop"]
#! ghi
no_doc = []
##
feat2 = ["momo"]
#! klm
default = ["feat1", "something_else"]
#! end
        "#;
        let parsed = process_toml(toml, &Args::default()).unwrap();
        assert_eq!(
            parsed,
            " abc\n def\n\n* **`feat1`** *(enabled by default)* —  123\n  456\n\n ghi\n* **`feat2`**\n\n klm\n end\n"
        );
        let parsed = process_toml(
            toml,
            &Args {
                feature_label: Some(
                    "<span class=\"stab portability\"><code>{feature}</code></span>".into(),
                ),
            },
        )
        .unwrap();
        assert_eq!(
            parsed,
            " abc\n def\n\n* <span class=\"stab portability\"><code>feat1</code></span> *(enabled by default)* —  123\n  456\n\n ghi\n* <span class=\"stab portability\"><code>feat2</code></span>\n\n klm\n end\n"
        );
    }

    #[test]
    fn dependencies() {
        let toml = r#"
#! top
[dev-dependencies] #yo
## dep1
dep1 = { version="1.2", optional=true}
#! yo
dep2 = "1.3"
## dep3
[target.'cfg(unix)'.build-dependencies.dep3]
version = "42"
optional = true
        "#;
        let parsed = process_toml(toml, &Args::default()).unwrap();
        assert_eq!(parsed, " top\n* **`dep1`** —  dep1\n\n yo\n* **`dep3`** —  dep3\n");
        let parsed = process_toml(
            toml,
            &Args {
                feature_label: Some(
                    "<span class=\"stab portability\"><code>{feature}</code></span>".into(),
                ),
            },
        )
        .unwrap();
        assert_eq!(parsed, " top\n* <span class=\"stab portability\"><code>dep1</code></span> —  dep1\n\n yo\n* <span class=\"stab portability\"><code>dep3</code></span> —  dep3\n");
    }

    #[test]
    fn multi_lines() {
        let toml = r#"
[package.metadata.foo]
ixyz = [
    ["array"],
    [
        "of",
        "arrays"
    ]
]
[dev-dependencies]
## dep1
dep1 = {
    version="1.2-}",
    optional=true
}
[features]
default = [
    "goo",
    "\"]",
    "bar",
]
## foo
foo = [
   "bar"
]
## bar
bar = [

]
        "#;
        let parsed = process_toml(toml, &Args::default()).unwrap();
        assert_eq!(
            parsed,
            "* **`dep1`** —  dep1\n* **`foo`** —  foo\n* **`bar`** *(enabled by default)* —  bar\n"
        );
        let parsed = process_toml(
            toml,
            &Args {
                feature_label: Some(
                    "<span class=\"stab portability\"><code>{feature}</code></span>".into(),
                ),
            },
        )
        .unwrap();
        assert_eq!(
            parsed,
            "* <span class=\"stab portability\"><code>dep1</code></span> —  dep1\n* <span class=\"stab portability\"><code>foo</code></span> —  foo\n* <span class=\"stab portability\"><code>bar</code></span> *(enabled by default)* —  bar\n"
        );
    }

    #[test]
    fn dots_in_feature() {
        let toml = r#"
[features]
## This is a test
"teßt." = []
default = ["teßt."]
[dependencies]
## A dep
"dep" = { version = "123", optional = true }
        "#;
        let parsed = process_toml(toml, &Args::default()).unwrap();
        assert_eq!(
            parsed,
            "* **`teßt.`** *(enabled by default)* —  This is a test\n* **`dep`** —  A dep\n"
        );
        let parsed = process_toml(
            toml,
            &Args {
                feature_label: Some(
                    "<span class=\"stab portability\"><code>{feature}</code></span>".into(),
                ),
            },
        )
        .unwrap();
        assert_eq!(
            parsed,
            "* <span class=\"stab portability\"><code>teßt.</code></span> *(enabled by default)* —  This is a test\n* <span class=\"stab portability\"><code>dep</code></span> —  A dep\n"
        );
    }

    #[test]
    fn recursive_default() {
        let toml = r#"
[features]
default=["qqq"]

## Qqq
qqq=["www"]

## Www
www=[]
        "#;
        let parsed = process_toml(toml, &Args::default()).unwrap();
        assert_eq!(parsed, "* **`qqq`** *(enabled by default)* —  Qqq\n* **`www`** *(enabled by default)* —  Www\n");
    }
}
