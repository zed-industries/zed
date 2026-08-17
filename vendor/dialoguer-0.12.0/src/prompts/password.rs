use std::{io, sync::Arc};

use console::Term;
use zeroize::Zeroizing;

use crate::{
    theme::{render::TermThemeRenderer, SimpleTheme, Theme},
    validate::PasswordValidator,
    Result,
};

type PasswordValidatorCallback<'a> = Arc<dyn Fn(&String) -> Option<String> + 'a>;

/// Renders a password input prompt.
///
/// ## Example
///
/// ```rust,no_run
/// use dialoguer::Password;
///
/// fn main() {
///     let password = Password::new()
///         .with_prompt("New Password")
///         .with_confirmation("Confirm password", "Passwords mismatching")
///         .interact()
///         .unwrap();
///
///     println!("Your password length is: {}", password.len());
/// }
/// ```
#[derive(Clone)]
pub struct Password<'a> {
    prompt: String,
    report: bool,
    theme: &'a dyn Theme,
    allow_empty_password: bool,
    confirmation_prompt: Option<(String, String)>,
    validator: Option<PasswordValidatorCallback<'a>>,
}

impl Default for Password<'static> {
    fn default() -> Password<'static> {
        Self::new()
    }
}

impl Password<'static> {
    /// Creates a password input prompt with default theme.
    pub fn new() -> Password<'static> {
        Self::with_theme(&SimpleTheme)
    }
}

impl Password<'_> {
    /// Sets the password input prompt.
    pub fn with_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = prompt.into();
        self
    }

    /// Indicates whether to report confirmation after interaction.
    ///
    /// The default is to report.
    pub fn report(mut self, val: bool) -> Self {
        self.report = val;
        self
    }

    /// Enables confirmation prompting.
    pub fn with_confirmation<A, B>(mut self, prompt: A, mismatch_err: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        self.confirmation_prompt = Some((prompt.into(), mismatch_err.into()));
        self
    }

    /// Allows/Disables empty password.
    ///
    /// By default this setting is set to false (i.e. password is not empty).
    pub fn allow_empty_password(mut self, allow_empty_password: bool) -> Self {
        self.allow_empty_password = allow_empty_password;
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// If the user confirms the result is `Ok()`, `Err()` otherwise.
    /// The dialog is rendered on stderr.
    pub fn interact(self) -> Result<String> {
        self.interact_on(&Term::stderr())
    }

    /// Like [`interact`](Self::interact) but allows a specific terminal to be set.
    pub fn interact_on(self, term: &Term) -> Result<String> {
        if !term.is_term() {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "not a terminal").into());
        }

        let mut render = TermThemeRenderer::new(term, self.theme);
        render.set_prompts_reset_height(false);

        loop {
            let password = Zeroizing::new(self.prompt_password(&mut render, &self.prompt)?);

            if let Some(ref validator) = self.validator {
                if let Some(err) = validator(&password) {
                    render.error(&err)?;
                    continue;
                }
            }

            if let Some((ref prompt, ref err)) = self.confirmation_prompt {
                let pw2 = Zeroizing::new(self.prompt_password(&mut render, prompt)?);

                if *password != *pw2 {
                    render.error(err)?;
                    continue;
                }
            }

            render.clear()?;

            if self.report {
                render.password_prompt_selection(&self.prompt)?;
            }
            term.flush()?;

            return Ok((*password).clone());
        }
    }

    fn prompt_password(&self, render: &mut TermThemeRenderer, prompt: &str) -> Result<String> {
        loop {
            render.password_prompt(prompt)?;
            render.term().flush()?;

            let input = render.term().read_secure_line()?;

            render.add_line();

            if !input.is_empty() || self.allow_empty_password {
                return Ok(input);
            }
        }
    }
}

impl<'a> Password<'a> {
    /// Registers a validator.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dialoguer::Password;
    ///
    /// fn main() {
    ///     let password: String = Password::new()
    ///         .with_prompt("Enter password")
    ///         .validate_with(|input: &String| -> Result<(), &str> {
    ///             if input.chars().count() > 8 {
    ///                 Ok(())
    ///             } else {
    ///                 Err("Password must be longer than 8")
    ///             }
    ///         })
    ///         .interact()
    ///         .unwrap();
    /// }
    /// ```
    pub fn validate_with<V>(mut self, validator: V) -> Self
    where
        V: PasswordValidator + 'a,
        V::Err: ToString,
    {
        let old_validator_func = self.validator.take();

        self.validator = Some(Arc::new(move |value: &String| -> Option<String> {
            if let Some(old) = &old_validator_func {
                if let Some(err) = old(value) {
                    return Some(err);
                }
            }

            match validator.validate(value) {
                Ok(()) => None,
                Err(err) => Some(err.to_string()),
            }
        }));

        self
    }

    /// Creates a password input prompt with a specific theme.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use dialoguer::{theme::ColorfulTheme, Password};
    ///
    /// fn main() {
    ///     let password = Password::with_theme(&ColorfulTheme::default())
    ///         .interact()
    ///         .unwrap();
    /// }
    /// ```
    pub fn with_theme(theme: &'a dyn Theme) -> Self {
        Self {
            prompt: "".into(),
            report: true,
            theme,
            allow_empty_password: false,
            confirmation_prompt: None,
            validator: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let password = Password::new().with_prompt("Enter password");

        let _ = password.clone();
    }
}
