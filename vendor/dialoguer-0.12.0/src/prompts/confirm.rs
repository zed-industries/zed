use std::io;

use console::{Key, Term};

use crate::{
    theme::{render::TermThemeRenderer, SimpleTheme, Theme},
    Result,
};

/// Renders a confirm prompt.
///
/// ## Example
///
/// ```rust,no_run
/// use dialoguer::Confirm;
///
/// fn main() {
///     let confirmation = Confirm::new()
///         .with_prompt("Do you want to continue?")
///         .interact()
///         .unwrap();
///
///     if confirmation {
///         println!("Looks like you want to continue");
///     } else {
///         println!("nevermind then :(");
///     }
/// }
/// ```
#[derive(Clone)]
pub struct Confirm<'a> {
    prompt: String,
    report: bool,
    default: Option<bool>,
    show_default: bool,
    wait_for_newline: bool,
    theme: &'a dyn Theme,
}

impl Default for Confirm<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl Confirm<'static> {
    /// Creates a confirm prompt with default theme.
    pub fn new() -> Self {
        Self::with_theme(&SimpleTheme)
    }
}

impl Confirm<'_> {
    /// Sets the confirm prompt.
    pub fn with_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = prompt.into();
        self
    }

    /// Indicates whether or not to report the chosen selection after interaction.
    ///
    /// The default is to report the chosen selection.
    pub fn report(mut self, val: bool) -> Self {
        self.report = val;
        self
    }

    /// Sets when to react to user input.
    ///
    /// When `false` (default), we check on each user keystroke immediately as
    /// it is typed. Valid inputs can be one of 'y', 'n', or a newline to accept
    /// the default.
    ///
    /// When `true`, the user must type their choice and hit the Enter key before
    /// proceeding. Valid inputs can be "yes", "no", "y", "n", or an empty string
    /// to accept the default.
    pub fn wait_for_newline(mut self, wait: bool) -> Self {
        self.wait_for_newline = wait;
        self
    }

    /// Sets a default.
    ///
    /// Out of the box the prompt does not have a default and will continue
    /// to display until the user inputs something and hits enter. If a default is set the user
    /// can instead accept the default with enter.
    pub fn default(mut self, val: bool) -> Self {
        self.default = Some(val);
        self
    }

    /// Disables or enables the default value display.
    ///
    /// The default is to append the default value to the prompt to tell the user.
    pub fn show_default(mut self, val: bool) -> Self {
        self.show_default = val;
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// The dialog is rendered on stderr.
    ///
    /// Result contains `bool` if user answered "yes" or "no" or `default` (configured in [`default`](Self::default) if pushes enter.
    /// This unlike [`interact_opt`](Self::interact_opt) does not allow to quit with 'Esc' or 'q'.
    #[inline]
    pub fn interact(self) -> Result<bool> {
        self.interact_on(&Term::stderr())
    }

    /// Enables user interaction and returns the result.
    ///
    /// The dialog is rendered on stderr.
    ///
    /// Result contains `Some(bool)` if user answered "yes" or "no" or `Some(default)` (configured in [`default`](Self::default)) if pushes enter,
    /// or `None` if user cancelled with 'Esc' or 'q'.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use dialoguer::Confirm;
    ///
    /// fn main() {
    ///     let confirmation = Confirm::new()
    ///         .interact_opt()
    ///         .unwrap();
    ///
    ///     match confirmation {
    ///         Some(answer) => println!("User answered {}", if answer { "yes" } else { "no " }),
    ///         None => println!("User did not answer")
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn interact_opt(self) -> Result<Option<bool>> {
        self.interact_on_opt(&Term::stderr())
    }

    /// Like [`interact`](Self::interact) but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on(self, term: &Term) -> Result<bool> {
        Ok(self
            ._interact_on(term, false)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Quit not allowed in this case"))?)
    }

    /// Like [`interact_opt`](Self::interact_opt) but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on_opt(self, term: &Term) -> Result<Option<bool>> {
        self._interact_on(term, true)
    }

    fn _interact_on(self, term: &Term, allow_quit: bool) -> Result<Option<bool>> {
        if !term.is_term() {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "not a terminal").into());
        }

        let mut render = TermThemeRenderer::new(term, self.theme);

        let default_if_show = if self.show_default {
            self.default
        } else {
            None
        };

        render.confirm_prompt(&self.prompt, default_if_show)?;

        term.hide_cursor()?;
        term.flush()?;

        let rv;

        if self.wait_for_newline {
            // Waits for user input and for the user to hit the Enter key
            // before validation.
            let mut value = default_if_show;

            loop {
                let input = term.read_key()?;

                match input {
                    Key::Char('y') | Key::Char('Y') => {
                        value = Some(true);
                    }
                    Key::Char('n') | Key::Char('N') => {
                        value = Some(false);
                    }
                    Key::Enter => {
                        if !allow_quit {
                            value = value.or(self.default);
                        }

                        if value.is_some() || allow_quit {
                            rv = value;
                            break;
                        }
                        continue;
                    }
                    Key::Escape | Key::Char('q') if allow_quit => {
                        value = None;
                    }
                    _ => {
                        continue;
                    }
                };

                term.clear_line()?;
                render.confirm_prompt(&self.prompt, value)?;
            }
        } else {
            // Default behavior: matches continuously on every keystroke,
            // and does not wait for user to hit the Enter key.
            loop {
                let input = term.read_key()?;
                let value = match input {
                    Key::Char('y') | Key::Char('Y') => Some(true),
                    Key::Char('n') | Key::Char('N') => Some(false),
                    Key::Enter if self.default.is_some() => Some(self.default.unwrap()),
                    Key::Escape | Key::Char('q') if allow_quit => None,
                    _ => {
                        continue;
                    }
                };

                rv = value;
                break;
            }
        }

        term.clear_line()?;
        if self.report {
            render.confirm_prompt_selection(&self.prompt, rv)?;
        }
        term.show_cursor()?;
        term.flush()?;

        Ok(rv)
    }
}

impl<'a> Confirm<'a> {
    /// Creates a confirm prompt with a specific theme.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use dialoguer::{theme::ColorfulTheme, Confirm};
    ///
    /// fn main() {
    ///     let confirmation = Confirm::with_theme(&ColorfulTheme::default())
    ///         .interact()
    ///         .unwrap();
    /// }
    /// ```
    pub fn with_theme(theme: &'a dyn Theme) -> Self {
        Self {
            prompt: "".into(),
            report: true,
            default: None,
            show_default: true,
            wait_for_newline: false,
            theme,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let confirm = Confirm::new().with_prompt("Do you want to continue?");

        let _ = confirm.clone();
    }
}
