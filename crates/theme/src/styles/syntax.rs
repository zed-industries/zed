use gpui::{HighlightStyle, Hsla};

use crate::{
    blue, cyan, gold, indigo, iris, jade, lime, mint, neutral, orange, plum, purple, red, sky,
    tomato, yellow,
};

#[derive(Clone, Default)]
pub struct SyntaxTheme {
    pub highlights: Vec<(String, HighlightStyle)>,
}

impl SyntaxTheme {
    pub fn light() -> Self {
        Self {
            highlights: vec![
                ("attribute".into(), cyan().light().step_11().into()),
                ("boolean".into(), tomato().light().step_11().into()),
                ("comment".into(), neutral().light().step_10().into()),
                ("comment.doc".into(), iris().light().step_11().into()),
                ("constant".into(), red().light().step_9().into()),
                ("constructor".into(), red().light().step_9().into()),
                ("embedded".into(), red().light().step_9().into()),
                ("emphasis".into(), red().light().step_9().into()),
                ("emphasis.strong".into(), red().light().step_9().into()),
                ("enum".into(), red().light().step_9().into()),
                ("function".into(), red().light().step_9().into()),
                ("hint".into(), red().light().step_9().into()),
                ("keyword".into(), orange().light().step_9().into()),
                ("label".into(), red().light().step_9().into()),
                ("link_text".into(), red().light().step_9().into()),
                ("link_uri".into(), red().light().step_9().into()),
                ("number".into(), purple().light().step_10().into()),
                ("operator".into(), red().light().step_9().into()),
                ("predictive".into(), red().light().step_9().into()),
                ("preproc".into(), red().light().step_9().into()),
                ("primary".into(), red().light().step_9().into()),
                ("property".into(), red().light().step_9().into()),
                ("punctuation".into(), neutral().light().step_11().into()),
                (
                    "punctuation.bracket".into(),
                    neutral().light().step_11().into(),
                ),
                (
                    "punctuation.delimiter".into(),
                    neutral().light().step_10().into(),
                ),
                (
                    "punctuation.list_marker".into(),
                    blue().light().step_11().into(),
                ),
                ("punctuation.special".into(), red().light().step_9().into()),
                ("string".into(), jade().light().step_9().into()),
                ("string.escape".into(), red().light().step_9().into()),
                ("string.regex".into(), tomato().light().step_9().into()),
                ("string.special".into(), red().light().step_9().into()),
                (
                    "string.special.symbol".into(),
                    red().light().step_9().into(),
                ),
                ("tag".into(), red().light().step_9().into()),
                ("text.literal".into(), red().light().step_9().into()),
                ("title".into(), red().light().step_9().into()),
                ("type".into(), cyan().light().step_9().into()),
                ("variable".into(), red().light().step_9().into()),
                ("variable.special".into(), red().light().step_9().into()),
                ("variant".into(), red().light().step_9().into()),
            ],
        }
    }

    pub fn dark() -> Self {
        Self {
            highlights: vec![
                ("attribute".into(), tomato().dark().step_11().into()),
                ("boolean".into(), tomato().dark().step_11().into()),
                ("comment".into(), neutral().dark().step_11().into()),
                ("comment.doc".into(), iris().dark().step_12().into()),
                ("constant".into(), orange().dark().step_11().into()),
                ("constructor".into(), gold().dark().step_11().into()),
                ("embedded".into(), red().dark().step_11().into()),
                ("emphasis".into(), red().dark().step_11().into()),
                ("emphasis.strong".into(), red().dark().step_11().into()),
                ("enum".into(), yellow().dark().step_11().into()),
                ("function".into(), blue().dark().step_11().into()),
                ("hint".into(), indigo().dark().step_11().into()),
                ("keyword".into(), plum().dark().step_11().into()),
                ("label".into(), red().dark().step_11().into()),
                ("link_text".into(), red().dark().step_11().into()),
                ("link_uri".into(), red().dark().step_11().into()),
                ("number".into(), red().dark().step_11().into()),
                ("operator".into(), red().dark().step_11().into()),
                ("predictive".into(), red().dark().step_11().into()),
                ("preproc".into(), red().dark().step_11().into()),
                ("primary".into(), red().dark().step_11().into()),
                ("property".into(), red().dark().step_11().into()),
                ("punctuation".into(), neutral().dark().step_11().into()),
                (
                    "punctuation.bracket".into(),
                    neutral().dark().step_11().into(),
                ),
                (
                    "punctuation.delimiter".into(),
                    neutral().dark().step_11().into(),
                ),
                (
                    "punctuation.list_marker".into(),
                    blue().dark().step_11().into(),
                ),
                ("punctuation.special".into(), red().dark().step_11().into()),
                ("string".into(), lime().dark().step_11().into()),
                ("string.escape".into(), orange().dark().step_11().into()),
                ("string.regex".into(), tomato().dark().step_11().into()),
                ("string.special".into(), red().dark().step_11().into()),
                (
                    "string.special.symbol".into(),
                    red().dark().step_11().into(),
                ),
                ("tag".into(), red().dark().step_11().into()),
                ("text.literal".into(), purple().dark().step_11().into()),
                ("title".into(), sky().dark().step_11().into()),
                ("type".into(), mint().dark().step_11().into()),
                ("variable".into(), red().dark().step_11().into()),
                ("variable.special".into(), red().dark().step_11().into()),
                ("variant".into(), red().dark().step_11().into()),
            ],
        }
    }

    // TODO: Get this working with `#[cfg(test)]`. Why isn't it?
    pub fn new_test(colors: impl IntoIterator<Item = (&'static str, Hsla)>) -> Self {
        SyntaxTheme {
            highlights: colors
                .into_iter()
                .map(|(key, color)| {
                    (
                        key.to_owned(),
                        HighlightStyle {
                            color: Some(color),
                            ..Default::default()
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn get(&self, name: &str) -> HighlightStyle {
        self.highlights
            .iter()
            .find_map(|entry| if entry.0 == name { Some(entry.1) } else { None })
            .unwrap_or_default()
    }

    pub fn color(&self, name: &str) -> Hsla {
        self.get(name).color.unwrap_or_default()
    }
}
