use std::path::Path;
use std::{cmp::Ordering, collections::BTreeMap};

use crate::utils;
use crate::utils::bracket_escape;

use handlebars::{
    Context, Handlebars, Helper, HelperDef, Output, RenderContext, RenderError, RenderErrorReason,
};

// Handlebars helper to construct TOC
#[derive(Clone, Copy)]
pub struct RenderToc {
    pub no_section_label: bool,
}

impl HelperDef for RenderToc {
    fn call<'reg: 'rc, 'rc>(
        &self,
        _h: &Helper<'rc>,
        _r: &'reg Handlebars<'_>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        // get value from context data
        // rc.get_path() is current json parent path, you should always use it like this
        // param is the key of value you want to display
        let chapters = rc.evaluate(ctx, "@root/chapters").and_then(|c| {
            serde_json::value::from_value::<Vec<BTreeMap<String, String>>>(c.as_json().clone())
                .map_err(|_| {
                    RenderErrorReason::Other("Could not decode the JSON data".to_owned()).into()
                })
        })?;
        let current_path = rc
            .evaluate(ctx, "@root/path")?
            .as_json()
            .as_str()
            .ok_or_else(|| {
                RenderErrorReason::Other("Type error for `path`, string expected".to_owned())
            })?
            .replace('\"', "");

        let current_section = rc
            .evaluate(ctx, "@root/section")?
            .as_json()
            .as_str()
            .map(str::to_owned)
            .unwrap_or_default();

        let fold_enable = rc
            .evaluate(ctx, "@root/fold_enable")?
            .as_json()
            .as_bool()
            .ok_or_else(|| {
                RenderErrorReason::Other("Type error for `fold_enable`, bool expected".to_owned())
            })?;

        let fold_level = rc
            .evaluate(ctx, "@root/fold_level")?
            .as_json()
            .as_u64()
            .ok_or_else(|| {
                RenderErrorReason::Other("Type error for `fold_level`, u64 expected".to_owned())
            })?;

        out.write("<ol class=\"chapter\">")?;

        let mut current_level = 1;
        // The "index" page, which has this attribute set, is supposed to alias the first chapter in
        // the book, i.e. the first link. There seems to be no easy way to determine which chapter
        // the "index" is aliasing from within the renderer, so this is used instead to force the
        // first link to be active. See further below.
        let mut is_first_chapter = ctx.data().get("is_index").is_some();

        for item in chapters {
            let (section, level) = if let Some(s) = item.get("section") {
                (s.as_str(), s.matches('.').count())
            } else {
                ("", 1)
            };

            let is_expanded =
                if !fold_enable || (!section.is_empty() && current_section.starts_with(section)) {
                    // Expand if folding is disabled, or if the section is an
                    // ancestor or the current section itself.
                    true
                } else {
                    // Levels that are larger than this would be folded.
                    level - 1 < fold_level as usize
                };

            match level.cmp(&current_level) {
                Ordering::Greater => {
                    while level > current_level {
                        out.write("<li>")?;
                        out.write("<ol class=\"section\">")?;
                        current_level += 1;
                    }
                    write_li_open_tag(out, is_expanded, false)?;
                }
                Ordering::Less => {
                    while level < current_level {
                        out.write("</ol>")?;
                        out.write("</li>")?;
                        current_level -= 1;
                    }
                    write_li_open_tag(out, is_expanded, false)?;
                }
                Ordering::Equal => {
                    write_li_open_tag(out, is_expanded, !item.contains_key("section"))?;
                }
            }

            // Spacer
            if item.contains_key("spacer") {
                out.write("<li class=\"spacer\"></li>")?;
                continue;
            }

            // Part title
            if let Some(title) = item.get("part") {
                out.write("<li class=\"part-title\">")?;
                out.write(&bracket_escape(title))?;
                out.write("</li>")?;
                continue;
            }

            // Link
            let path_exists: bool;
            match item.get("path") {
                Some(path) if !path.is_empty() => {
                    out.write("<a href=\"")?;
                    let tmp = Path::new(path)
                        .with_extension("html")
                        .to_str()
                        .unwrap()
                        // Hack for windows who tends to use `\` as separator instead of `/`
                        .replace('\\', "/");

                    // Add link
                    out.write(&utils::fs::path_to_root(&current_path))?;
                    out.write(&tmp)?;
                    out.write("\"")?;

                    if path == &current_path || is_first_chapter {
                        is_first_chapter = false;
                        out.write(" class=\"active\"")?;
                    }

                    out.write(">")?;
                    path_exists = true;
                }
                _ => {
                    out.write("<div>")?;
                    path_exists = false;
                }
            }

            if !self.no_section_label {
                // Section does not necessarily exist
                if let Some(section) = item.get("section") {
                    out.write("<strong aria-hidden=\"true\">")?;
                    out.write(section)?;
                    out.write("</strong> ")?;
                }
            }

            if let Some(name) = item.get("name") {
                out.write(&bracket_escape(name))?
            }

            if path_exists {
                out.write("</a>")?;
            } else {
                out.write("</div>")?;
            }

            // Render expand/collapse toggle
            if let Some(flag) = item.get("has_sub_items") {
                let has_sub_items = flag.parse::<bool>().unwrap_or_default();
                if fold_enable && has_sub_items {
                    out.write("<a class=\"toggle\"><div>❱</div></a>")?;
                }
            }
            out.write("</li>")?;
        }
        while current_level > 1 {
            out.write("</ol>")?;
            out.write("</li>")?;
            current_level -= 1;
        }

        out.write("</ol>")?;
        Ok(())
    }
}

fn write_li_open_tag(
    out: &mut dyn Output,
    is_expanded: bool,
    is_affix: bool,
) -> Result<(), std::io::Error> {
    let mut li = String::from("<li class=\"chapter-item ");
    if is_expanded {
        li.push_str("expanded ");
    }
    if is_affix {
        li.push_str("affix ");
    }
    li.push_str("\">");
    out.write(&li)
}
