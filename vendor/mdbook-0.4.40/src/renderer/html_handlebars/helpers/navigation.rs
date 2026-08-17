use std::collections::BTreeMap;
use std::path::Path;

use handlebars::{
    Context, Handlebars, Helper, Output, RenderContext, RenderError, RenderErrorReason, Renderable,
};

use crate::utils;
use log::{debug, trace};
use serde_json::json;

type StringMap = BTreeMap<String, String>;

/// Target for `find_chapter`.
enum Target {
    Previous,
    Next,
}

impl Target {
    /// Returns target if found.
    fn find(
        &self,
        base_path: &str,
        current_path: &str,
        current_item: &StringMap,
        previous_item: &StringMap,
    ) -> Result<Option<StringMap>, RenderError> {
        match *self {
            Target::Next => {
                let previous_path = previous_item.get("path").ok_or_else(|| {
                    RenderErrorReason::Other("No path found for chapter in JSON data".to_owned())
                })?;

                if previous_path == base_path {
                    return Ok(Some(current_item.clone()));
                }
            }

            Target::Previous => {
                if current_path == base_path {
                    return Ok(Some(previous_item.clone()));
                }
            }
        }

        Ok(None)
    }
}

fn find_chapter(
    ctx: &Context,
    rc: &mut RenderContext<'_, '_>,
    target: Target,
) -> Result<Option<StringMap>, RenderError> {
    debug!("Get data from context");

    let chapters = rc.evaluate(ctx, "@root/chapters").and_then(|c| {
        serde_json::value::from_value::<Vec<StringMap>>(c.as_json().clone()).map_err(|_| {
            RenderErrorReason::Other("Could not decode the JSON data".to_owned()).into()
        })
    })?;

    let base_path = rc
        .evaluate(ctx, "@root/path")?
        .as_json()
        .as_str()
        .ok_or_else(|| {
            RenderErrorReason::Other("Type error for `path`, string expected".to_owned())
        })?
        .replace('\"', "");

    if !rc.evaluate(ctx, "@root/is_index")?.is_missing() {
        // Special case for index.md which may be a synthetic page.
        // Target::find won't match because there is no page with the path
        // "index.md" (unless there really is an index.md in SUMMARY.md).
        match target {
            Target::Previous => return Ok(None),
            Target::Next => match chapters
                .iter()
                .filter(|chapter| {
                    // Skip things like "spacer"
                    chapter.contains_key("path")
                })
                .nth(1)
            {
                Some(chapter) => return Ok(Some(chapter.clone())),
                None => return Ok(None),
            },
        }
    }

    let mut previous: Option<StringMap> = None;

    debug!("Search for chapter");

    for item in chapters {
        match item.get("path") {
            Some(path) if !path.is_empty() => {
                if let Some(previous) = previous {
                    if let Some(item) = target.find(&base_path, path, &item, &previous)? {
                        return Ok(Some(item));
                    }
                }

                previous = Some(item);
            }
            _ => continue,
        }
    }

    Ok(None)
}

fn render(
    _h: &Helper<'_>,
    r: &Handlebars<'_>,
    ctx: &Context,
    rc: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
    chapter: &StringMap,
) -> Result<(), RenderError> {
    trace!("Creating BTreeMap to inject in context");

    let mut context = BTreeMap::new();
    let base_path = rc
        .evaluate(ctx, "@root/path")?
        .as_json()
        .as_str()
        .ok_or_else(|| {
            RenderErrorReason::Other("Type error for `path`, string expected".to_owned())
        })?
        .replace('\"', "");

    context.insert(
        "path_to_root".to_owned(),
        json!(utils::fs::path_to_root(base_path)),
    );

    chapter
        .get("name")
        .ok_or_else(|| {
            RenderErrorReason::Other("No title found for chapter in JSON data".to_owned())
        })
        .map(|name| context.insert("title".to_owned(), json!(name)))?;

    chapter
        .get("path")
        .ok_or_else(|| {
            RenderErrorReason::Other("No path found for chapter in JSON data".to_owned())
        })
        .and_then(|p| {
            Path::new(p)
                .with_extension("html")
                .to_str()
                .ok_or_else(|| {
                    RenderErrorReason::Other("Link could not be converted to str".to_owned())
                })
                .map(|p| context.insert("link".to_owned(), json!(p.replace('\\', "/"))))
        })?;

    trace!("Render template");

    let t = _h
        .template()
        .ok_or_else(|| RenderErrorReason::Other("Error with the handlebars template".to_owned()))?;
    let local_ctx = Context::wraps(&context)?;
    let mut local_rc = rc.clone();
    t.render(r, &local_ctx, &mut local_rc, out)
}

pub fn previous(
    _h: &Helper<'_>,
    r: &Handlebars<'_>,
    ctx: &Context,
    rc: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    trace!("previous (handlebars helper)");

    if let Some(previous) = find_chapter(ctx, rc, Target::Previous)? {
        render(_h, r, ctx, rc, out, &previous)?;
    }

    Ok(())
}

pub fn next(
    _h: &Helper<'_>,
    r: &Handlebars<'_>,
    ctx: &Context,
    rc: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    trace!("next (handlebars helper)");

    if let Some(next) = find_chapter(ctx, rc, Target::Next)? {
        render(_h, r, ctx, rc, out, &next)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEMPLATE: &str =
        "{{#previous}}{{title}}: {{link}}{{/previous}}|{{#next}}{{title}}: {{link}}{{/next}}";

    #[test]
    fn test_next_previous() {
        let data = json!({
           "name": "two",
           "path": "two.path",
           "chapters": [
              {
                 "name": "one",
                 "path": "one.path"
              },
              {
                 "name": "two",
                 "path": "two.path",
              },
              {
                 "name": "three",
                 "path": "three.path"
              }
           ]
        });

        let mut h = Handlebars::new();
        h.register_helper("previous", Box::new(previous));
        h.register_helper("next", Box::new(next));

        assert_eq!(
            h.render_template(TEMPLATE, &data).unwrap(),
            "one: one.html|three: three.html"
        );
    }

    #[test]
    fn test_first() {
        let data = json!({
           "name": "one",
           "path": "one.path",
           "chapters": [
              {
                 "name": "one",
                 "path": "one.path"
              },
              {
                 "name": "two",
                 "path": "two.path",
              },
              {
                 "name": "three",
                 "path": "three.path"
              }
           ]
        });

        let mut h = Handlebars::new();
        h.register_helper("previous", Box::new(previous));
        h.register_helper("next", Box::new(next));

        assert_eq!(
            h.render_template(TEMPLATE, &data).unwrap(),
            "|two: two.html"
        );
    }
    #[test]
    fn test_last() {
        let data = json!({
           "name": "three",
           "path": "three.path",
           "chapters": [
              {
                 "name": "one",
                 "path": "one.path"
              },
              {
                 "name": "two",
                 "path": "two.path",
              },
              {
                 "name": "three",
                 "path": "three.path"
              }
           ]
        });

        let mut h = Handlebars::new();
        h.register_helper("previous", Box::new(previous));
        h.register_helper("next", Box::new(next));

        assert_eq!(
            h.render_template(TEMPLATE, &data).unwrap(),
            "two: two.html|"
        );
    }
}
