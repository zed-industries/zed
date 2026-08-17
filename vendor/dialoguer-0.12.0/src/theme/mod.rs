//! Customizes the rendering of the elements.
use std::fmt;

#[cfg(feature = "fuzzy-select")]
use console::style;
#[cfg(feature = "fuzzy-select")]
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

mod colorful;
pub(crate) mod render;
mod simple;

pub use colorful::ColorfulTheme;
pub use simple::SimpleTheme;

/// Implements a theme for dialoguer.
pub trait Theme {
    /// Formats a prompt.
    #[inline]
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}:", prompt)
    }

    /// Formats out an error.
    #[inline]
    fn format_error(&self, f: &mut dyn fmt::Write, err: &str) -> fmt::Result {
        write!(f, "error: {}", err)
    }

    /// Formats a confirm prompt.
    fn format_confirm_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> fmt::Result {
        if !prompt.is_empty() {
            write!(f, "{} ", &prompt)?;
        }
        match default {
            None => write!(f, "[y/n] ")?,
            Some(true) => write!(f, "[Y/n] ")?,
            Some(false) => write!(f, "[y/N] ")?,
        }
        Ok(())
    }

    /// Formats a confirm prompt after selection.
    fn format_confirm_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selection: Option<bool>,
    ) -> fmt::Result {
        let selection = selection.map(|b| if b { "yes" } else { "no" });

        match selection {
            Some(selection) if prompt.is_empty() => {
                write!(f, "{}", selection)
            }
            Some(selection) => {
                write!(f, "{} {}", &prompt, selection)
            }
            None if prompt.is_empty() => Ok(()),
            None => {
                write!(f, "{}", &prompt)
            }
        }
    }

    /// Formats an input prompt.
    fn format_input_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        match default {
            Some(default) if prompt.is_empty() => write!(f, "[{}]: ", default),
            Some(default) => write!(f, "{} [{}]: ", prompt, default),
            None => write!(f, "{}: ", prompt),
        }
    }

    /// Formats an input prompt after selection.
    #[inline]
    fn format_input_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        write!(f, "{}: {}", prompt, sel)
    }

    /// Formats a password prompt.
    #[inline]
    #[cfg(feature = "password")]
    fn format_password_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        self.format_input_prompt(f, prompt, None)
    }

    /// Formats a password prompt after selection.
    #[inline]
    #[cfg(feature = "password")]
    fn format_password_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
    ) -> fmt::Result {
        self.format_input_prompt_selection(f, prompt, "[hidden]")
    }

    /// Formats a select prompt.
    #[inline]
    fn format_select_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        self.format_prompt(f, prompt)
    }

    /// Formats a select prompt after selection.
    #[inline]
    fn format_select_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        self.format_input_prompt_selection(f, prompt, sel)
    }

    /// Formats a multi select prompt.
    #[inline]
    fn format_multi_select_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        self.format_prompt(f, prompt)
    }

    /// Formats a sort prompt.
    #[inline]
    fn format_sort_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        self.format_prompt(f, prompt)
    }

    /// Formats a multi_select prompt after selection.
    fn format_multi_select_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(f, "{}: ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(f, "{}{}", if idx == 0 { "" } else { ", " }, sel)?;
        }
        Ok(())
    }

    /// Formats a sort prompt after selection.
    #[inline]
    fn format_sort_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        self.format_multi_select_prompt_selection(f, prompt, selections)
    }

    /// Formats a select prompt item.
    fn format_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        active: bool,
    ) -> fmt::Result {
        write!(f, "{} {}", if active { ">" } else { " " }, text)
    }

    /// Formats a multi select prompt item.
    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> fmt::Result {
        write!(
            f,
            "{} {}",
            match (checked, active) {
                (true, true) => "> [x]",
                (true, false) => "  [x]",
                (false, true) => "> [ ]",
                (false, false) => "  [ ]",
            },
            text
        )
    }

    /// Formats a sort prompt item.
    fn format_sort_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        picked: bool,
        active: bool,
    ) -> fmt::Result {
        write!(
            f,
            "{} {}",
            match (picked, active) {
                (true, true) => "> [x]",
                (false, true) => "> [ ]",
                (_, false) => "  [ ]",
            },
            text
        )
    }

    /// Formats a fuzzy select prompt item.
    #[cfg(feature = "fuzzy-select")]
    fn format_fuzzy_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        active: bool,
        highlight_matches: bool,
        matcher: &SkimMatcherV2,
        search_term: &str,
    ) -> fmt::Result {
        write!(f, "{} ", if active { ">" } else { " " })?;

        if highlight_matches {
            if let Some((_score, indices)) = matcher.fuzzy_indices(text, search_term) {
                for (idx, c) in text.chars().enumerate() {
                    if indices.contains(&idx) {
                        write!(f, "{}", style(c).for_stderr().bold())?;
                    } else {
                        write!(f, "{}", c)?;
                    }
                }

                return Ok(());
            }
        }

        write!(f, "{}", text)
    }

    /// Formats a fuzzy select prompt.
    #[cfg(feature = "fuzzy-select")]
    fn format_fuzzy_select_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        search_term: &str,
        bytes_pos: usize,
    ) -> fmt::Result {
        if !prompt.is_empty() {
            write!(f, "{prompt} ")?;
        }

        let (st_head, st_tail) = search_term.split_at(bytes_pos);
        write!(f, "{st_head}|{st_tail}")
    }
}
