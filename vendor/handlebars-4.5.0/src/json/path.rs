use std::iter::Peekable;

use pest::iterators::Pair;
use pest::Parser;

use crate::error::RenderError;
use crate::grammar::{HandlebarsParser, Rule};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PathSeg {
    Named(String),
    Ruled(Rule),
}

/// Represents the Json path in templates.
///
/// It can be either a local variable like `@first`, `../@index`,
/// or a normal relative path like `a/b/c`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Path {
    Relative((Vec<PathSeg>, String)),
    Local((usize, String, String)),
}

impl Path {
    pub(crate) fn new(raw: &str, segs: Vec<PathSeg>) -> Path {
        if let Some((level, name)) = get_local_path_and_level(&segs) {
            Path::Local((level, name, raw.to_owned()))
        } else {
            Path::Relative((segs, raw.to_owned()))
        }
    }

    pub fn parse(raw: &str) -> Result<Path, RenderError> {
        HandlebarsParser::parse(Rule::path, raw)
            .map(|p| {
                let parsed = p.flatten();
                let segs = parse_json_path_from_iter(&mut parsed.peekable(), raw.len());
                Ok(Path::new(raw, segs))
            })
            .map_err(|_| RenderError::new("Invalid JSON path"))?
    }

    pub(crate) fn raw(&self) -> &str {
        match self {
            Path::Relative((_, ref raw)) => raw,
            Path::Local((_, _, ref raw)) => raw,
        }
    }

    pub(crate) fn current() -> Path {
        Path::Relative((Vec::with_capacity(0), "".to_owned()))
    }

    // for test only
    pub(crate) fn with_named_paths(name_segs: &[&str]) -> Path {
        let segs = name_segs
            .iter()
            .map(|n| PathSeg::Named((*n).to_string()))
            .collect();
        Path::Relative((segs, name_segs.join("/")))
    }

    // for test only
    pub(crate) fn segs(&self) -> Option<&[PathSeg]> {
        match self {
            Path::Relative((segs, _)) => Some(segs),
            _ => None,
        }
    }
}

fn get_local_path_and_level(paths: &[PathSeg]) -> Option<(usize, String)> {
    paths.get(0).and_then(|seg| {
        if seg == &PathSeg::Ruled(Rule::path_local) {
            let mut level = 0;
            while paths.get(level + 1)? == &PathSeg::Ruled(Rule::path_up) {
                level += 1;
            }
            if let Some(PathSeg::Named(name)) = paths.get(level + 1) {
                Some((level, name.clone()))
            } else {
                None
            }
        } else {
            None
        }
    })
}

pub(crate) fn parse_json_path_from_iter<'a, I>(it: &mut Peekable<I>, limit: usize) -> Vec<PathSeg>
where
    I: Iterator<Item = Pair<'a, Rule>>,
{
    let mut path_stack = Vec::with_capacity(5);
    while let Some(n) = it.peek() {
        let span = n.as_span();
        if span.end() > limit {
            break;
        }

        match n.as_rule() {
            Rule::path_root => {
                path_stack.push(PathSeg::Ruled(Rule::path_root));
            }
            Rule::path_local => {
                path_stack.push(PathSeg::Ruled(Rule::path_local));
            }
            Rule::path_up => {
                path_stack.push(PathSeg::Ruled(Rule::path_up));
            }
            Rule::path_id | Rule::path_raw_id => {
                let name = n.as_str();
                if name != "this" {
                    path_stack.push(PathSeg::Named(name.to_string()));
                }
            }
            _ => {}
        }

        it.next();
    }

    path_stack
}

pub(crate) fn merge_json_path(path_stack: &mut Vec<String>, relative_path: &[PathSeg]) {
    for seg in relative_path {
        match seg {
            PathSeg::Named(ref s) => {
                path_stack.push(s.to_owned());
            }
            PathSeg::Ruled(Rule::path_root) => {}
            PathSeg::Ruled(Rule::path_up) => {}
            _ => {}
        }
    }
}
