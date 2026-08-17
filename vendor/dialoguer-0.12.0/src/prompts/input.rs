use std::{
    cmp::Ordering,
    io, iter,
    str::FromStr,
    sync::{Arc, Mutex},
};

use console::{Key, Term};

#[cfg(feature = "completion")]
use crate::completion::Completion;
#[cfg(feature = "history")]
use crate::history::History;
use crate::{
    theme::{render::TermThemeRenderer, SimpleTheme, Theme},
    validate::InputValidator,
    Result,
};

type InputValidatorCallback<'a, T> = Arc<Mutex<dyn FnMut(&T) -> Option<String> + 'a>>;

/// Renders an input prompt.
///
/// ## Example
///
/// ```rust,no_run
/// use dialoguer::Input;
///
/// fn main() {
///     let name: String = Input::new()
///         .with_prompt("Your name?")
///         .interact_text()
///         .unwrap();
///
///     println!("Your name is: {}", name);
/// }
/// ```
///
/// It can also be used with turbofish notation:
///
/// ```rust,no_run
/// use dialoguer::Input;
///
/// fn main() {
///     let name = Input::<String>::new()
///         .with_prompt("Your name?")
///         .interact_text()
///         .unwrap();
///
///     println!("Your name is: {}", name);
/// }
/// ```
#[derive(Clone)]
pub struct Input<'a, T> {
    prompt: String,
    post_completion_text: Option<String>,
    report: bool,
    default: Option<T>,
    show_default: bool,
    initial_text: Option<String>,
    theme: &'a dyn Theme,
    permit_empty: bool,
    validator: Option<InputValidatorCallback<'a, T>>,
    #[cfg(feature = "history")]
    history: Option<Arc<Mutex<&'a mut dyn History<T>>>>,
    #[cfg(feature = "completion")]
    completion: Option<&'a dyn Completion>,
}

impl<T> Default for Input<'static, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Input<'_, T> {
    /// Creates an input prompt with default theme.
    pub fn new() -> Self {
        Self::with_theme(&SimpleTheme)
    }

    /// Sets the input prompt.
    pub fn with_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = prompt.into();
        self
    }

    /// Changes the prompt text to the post completion text after input is complete
    pub fn with_post_completion_text<S: Into<String>>(mut self, post_completion_text: S) -> Self {
        self.post_completion_text = Some(post_completion_text.into());
        self
    }

    /// Indicates whether to report the input value after interaction.
    ///
    /// The default is to report the input value.
    pub fn report(mut self, val: bool) -> Self {
        self.report = val;
        self
    }

    /// Sets initial text that user can accept or erase.
    pub fn with_initial_text<S: Into<String>>(mut self, val: S) -> Self {
        self.initial_text = Some(val.into());
        self
    }

    /// Sets a default.
    ///
    /// Out of the box the prompt does not have a default and will continue
    /// to display until the user inputs something and hits enter. If a default is set the user
    /// can instead accept the default with enter.
    pub fn default(mut self, value: T) -> Self {
        self.default = Some(value);
        self
    }

    /// Enables or disables an empty input
    ///
    /// By default, if there is no default value set for the input, the user must input a non-empty string.
    pub fn allow_empty(mut self, val: bool) -> Self {
        self.permit_empty = val;
        self
    }

    /// Disables or enables the default value display.
    ///
    /// The default behaviour is to append [`default`](#method.default) to the prompt to tell the
    /// user what is the default value.
    ///
    /// This method does not affect existence of default value, only its display in the prompt!
    pub fn show_default(mut self, val: bool) -> Self {
        self.show_default = val;
        self
    }
}

impl<'a, T> Input<'a, T> {
    /// Creates an input prompt with a specific theme.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use dialoguer::{theme::ColorfulTheme, Input};
    ///
    /// fn main() {
    ///     let name: String = Input::with_theme(&ColorfulTheme::default())
    ///         .interact()
    ///         .unwrap();
    /// }
    /// ```
    pub fn with_theme(theme: &'a dyn Theme) -> Self {
        Self {
            prompt: "".into(),
            post_completion_text: None,
            report: true,
            default: None,
            show_default: true,
            initial_text: None,
            theme,
            permit_empty: false,
            validator: None,
            #[cfg(feature = "history")]
            history: None,
            #[cfg(feature = "completion")]
            completion: None,
        }
    }

    /// Enable history processing
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use std::{collections::VecDeque, fmt::Display};
    /// use dialoguer::{History, Input};
    ///
    /// struct MyHistory {
    ///     history: VecDeque<String>,
    /// }
    ///
    /// impl Default for MyHistory {
    ///     fn default() -> Self {
    ///         MyHistory {
    ///             history: VecDeque::new(),
    ///         }
    ///     }
    /// }
    ///
    /// impl<T: ToString> History<T> for MyHistory {
    ///     fn read(&self, pos: usize) -> Option<String> {
    ///         self.history.get(pos).cloned()
    ///     }
    ///
    ///     fn write(&mut self, val: &T)
    ///     where
    ///     {
    ///         self.history.push_front(val.to_string());
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let mut history = MyHistory::default();
    ///
    ///     let input = Input::<String>::new()
    ///         .history_with(&mut history)
    ///         .interact_text()
    ///         .unwrap();
    /// }
    /// ```
    #[cfg(feature = "history")]
    pub fn history_with<H>(mut self, history: &'a mut H) -> Self
    where
        H: History<T>,
    {
        self.history = Some(Arc::new(Mutex::new(history)));
        self
    }

    /// Enable completion
    #[cfg(feature = "completion")]
    pub fn completion_with<C>(mut self, completion: &'a C) -> Self
    where
        C: Completion,
    {
        self.completion = Some(completion);
        self
    }
}

impl<'a, T> Input<'a, T>
where
    T: 'a,
{
    /// Registers a validator.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dialoguer::Input;
    ///
    /// fn main() {
    ///     let mail: String = Input::new()
    ///         .with_prompt("Enter email")
    ///         .validate_with(|input: &String| -> Result<(), &str> {
    ///             if input.contains('@') {
    ///                 Ok(())
    ///             } else {
    ///                 Err("This is not a mail address")
    ///             }
    ///         })
    ///         .interact()
    ///         .unwrap();
    /// }
    /// ```
    pub fn validate_with<V>(mut self, mut validator: V) -> Self
    where
        V: InputValidator<T> + 'a,
        V::Err: ToString,
    {
        let mut old_validator_func = self.validator.take();

        self.validator = Some(Arc::new(Mutex::new(move |value: &T| -> Option<String> {
            if let Some(old) = old_validator_func.as_mut() {
                if let Some(err) = old.lock().unwrap()(value) {
                    return Some(err);
                }
            }

            match validator.validate(value) {
                Ok(()) => None,
                Err(err) => Some(err.to_string()),
            }
        })));

        self
    }
}

impl<T> Input<'_, T>
where
    T: Clone + ToString + FromStr,
    <T as FromStr>::Err: ToString,
{
    /// Enables the user to enter a printable ascii sequence and returns the result.
    ///
    /// Its difference from [`interact`](Self::interact) is that it only allows ascii characters for string,
    /// while [`interact`](Self::interact) allows virtually any character to be used e.g arrow keys.
    ///
    /// The dialog is rendered on stderr.
    pub fn interact_text(self) -> Result<T> {
        self.interact_text_on(&Term::stderr())
    }

    /// Like [`interact_text`](Self::interact_text) but allows a specific terminal to be set.
    pub fn interact_text_on(mut self, term: &Term) -> Result<T> {
        if !term.is_term() {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "not a terminal").into());
        }

        let mut render = TermThemeRenderer::new(term, self.theme);

        loop {
            let default_string = self.default.as_ref().map(ToString::to_string);

            let prompt_len = render.input_prompt(
                &self.prompt,
                if self.show_default {
                    default_string.as_deref()
                } else {
                    None
                },
            )?;

            let mut chars: Vec<char> = Vec::new();
            let mut position = 0;
            #[cfg(feature = "history")]
            let mut hist_pos = 0;

            if let Some(initial) = self.initial_text.as_ref() {
                term.write_str(initial)?;
                chars = initial.chars().collect();
                position = chars.len();
            }
            term.flush()?;

            loop {
                match term.read_key()? {
                    Key::Backspace if position > 0 => {
                        position -= 1;
                        chars.remove(position);
                        let line_size = term.size().1 as usize;
                        // Case we want to delete last char of a line so the cursor is at the beginning of the next line
                        if (position + prompt_len) % (line_size - 1) == 0 {
                            term.clear_line()?;
                            term.move_cursor_up(1)?;
                            term.move_cursor_right(line_size + 1)?;
                        } else {
                            term.clear_chars(1)?;
                        }

                        let tail: String = chars[position..].iter().collect();

                        if !tail.is_empty() {
                            term.write_str(&tail)?;

                            let total = position + prompt_len + tail.chars().count();
                            let total_line = total / line_size;
                            let line_cursor = (position + prompt_len) / line_size;
                            term.move_cursor_up(total_line - line_cursor)?;

                            term.move_cursor_left(line_size)?;
                            term.move_cursor_right((position + prompt_len) % line_size)?;
                        }

                        term.flush()?;
                    }
                    Key::Char(chr) if !chr.is_ascii_control() => {
                        chars.insert(position, chr);
                        position += 1;
                        let tail: String =
                            iter::once(&chr).chain(chars[position..].iter()).collect();
                        term.write_str(&tail)?;
                        term.move_cursor_left(tail.chars().count() - 1)?;
                        term.flush()?;
                    }
                    Key::ArrowLeft if position > 0 => {
                        if (position + prompt_len) % term.size().1 as usize == 0 {
                            term.move_cursor_up(1)?;
                            term.move_cursor_right(term.size().1 as usize)?;
                        } else {
                            term.move_cursor_left(1)?;
                        }
                        position -= 1;
                        term.flush()?;
                    }
                    Key::ArrowRight if position < chars.len() => {
                        if (position + prompt_len) % (term.size().1 as usize - 1) == 0 {
                            term.move_cursor_down(1)?;
                            term.move_cursor_left(term.size().1 as usize)?;
                        } else {
                            term.move_cursor_right(1)?;
                        }
                        position += 1;
                        term.flush()?;
                    }
                    Key::UnknownEscSeq(seq) if seq == vec!['b'] => {
                        let line_size = term.size().1 as usize;
                        let nb_space = chars[..position]
                            .iter()
                            .rev()
                            .take_while(|c| c.is_whitespace())
                            .count();
                        let find_last_space = chars[..position - nb_space]
                            .iter()
                            .rposition(|c| c.is_whitespace());

                        // If we find a space we set the cursor to the next char else we set it to the beginning of the input
                        if let Some(mut last_space) = find_last_space {
                            if last_space < position {
                                last_space += 1;
                                let new_line = (prompt_len + last_space) / line_size;
                                let old_line = (prompt_len + position) / line_size;
                                let diff_line = old_line - new_line;
                                if diff_line != 0 {
                                    term.move_cursor_up(old_line - new_line)?;
                                }

                                let new_pos_x = (prompt_len + last_space) % line_size;
                                let old_pos_x = (prompt_len + position) % line_size;
                                let diff_pos_x = new_pos_x as i64 - old_pos_x as i64;
                                //println!("new_pos_x = {}, old_pos_x = {}, diff = {}", new_pos_x, old_pos_x, diff_pos_x);
                                if diff_pos_x < 0 {
                                    term.move_cursor_left(-diff_pos_x as usize)?;
                                } else {
                                    term.move_cursor_right((diff_pos_x) as usize)?;
                                }
                                position = last_space;
                            }
                        } else {
                            term.move_cursor_left(position)?;
                            position = 0;
                        }

                        term.flush()?;
                    }
                    Key::UnknownEscSeq(seq) if seq == vec!['f'] => {
                        let line_size = term.size().1 as usize;
                        let find_next_space =
                            chars[position..].iter().position(|c| c.is_whitespace());

                        // If we find a space we set the cursor to the next char else we set it to the beginning of the input
                        if let Some(mut next_space) = find_next_space {
                            let nb_space = chars[position + next_space..]
                                .iter()
                                .take_while(|c| c.is_whitespace())
                                .count();
                            next_space += nb_space;
                            let new_line = (prompt_len + position + next_space) / line_size;
                            let old_line = (prompt_len + position) / line_size;
                            term.move_cursor_down(new_line - old_line)?;

                            let new_pos_x = (prompt_len + position + next_space) % line_size;
                            let old_pos_x = (prompt_len + position) % line_size;
                            let diff_pos_x = new_pos_x as i64 - old_pos_x as i64;
                            if diff_pos_x < 0 {
                                term.move_cursor_left(-diff_pos_x as usize)?;
                            } else {
                                term.move_cursor_right((diff_pos_x) as usize)?;
                            }
                            position += next_space;
                        } else {
                            let new_line = (prompt_len + chars.len()) / line_size;
                            let old_line = (prompt_len + position) / line_size;
                            term.move_cursor_down(new_line - old_line)?;

                            let new_pos_x = (prompt_len + chars.len()) % line_size;
                            let old_pos_x = (prompt_len + position) % line_size;
                            let diff_pos_x = new_pos_x as i64 - old_pos_x as i64;
                            match diff_pos_x.cmp(&0) {
                                Ordering::Less => {
                                    term.move_cursor_left((-diff_pos_x - 1) as usize)?;
                                }
                                Ordering::Equal => {}
                                Ordering::Greater => {
                                    term.move_cursor_right((diff_pos_x) as usize)?;
                                }
                            }
                            position = chars.len();
                        }

                        term.flush()?;
                    }
                    #[cfg(feature = "completion")]
                    Key::ArrowRight | Key::Tab => {
                        if let Some(completion) = &self.completion {
                            let input: String = chars.clone().into_iter().collect();
                            if let Some(x) = completion.get(&input) {
                                term.clear_chars(chars.len())?;
                                chars.clear();
                                position = 0;
                                for ch in x.chars() {
                                    chars.insert(position, ch);
                                    position += 1;
                                }
                                term.write_str(&x)?;
                                term.flush()?;
                            }
                        }
                    }
                    #[cfg(feature = "history")]
                    Key::ArrowUp => {
                        let line_size = term.size().1 as usize;
                        if let Some(history) = &self.history {
                            if let Some(previous) = history.lock().unwrap().read(hist_pos) {
                                hist_pos += 1;
                                let mut chars_len = chars.len();
                                while ((prompt_len + chars_len) / line_size) > 0 {
                                    term.clear_chars(chars_len)?;
                                    if (prompt_len + chars_len) % line_size == 0 {
                                        chars_len -= std::cmp::min(chars_len, line_size);
                                    } else {
                                        chars_len -= std::cmp::min(
                                            chars_len,
                                            (prompt_len + chars_len + 1) % line_size,
                                        );
                                    }
                                    if chars_len > 0 {
                                        term.move_cursor_up(1)?;
                                        term.move_cursor_right(line_size)?;
                                    }
                                }
                                term.clear_chars(chars_len)?;
                                chars.clear();
                                position = 0;
                                for ch in previous.chars() {
                                    chars.insert(position, ch);
                                    position += 1;
                                }
                                term.write_str(&previous)?;
                                term.flush()?;
                            }
                        }
                    }
                    #[cfg(feature = "history")]
                    Key::ArrowDown => {
                        let line_size = term.size().1 as usize;
                        if let Some(history) = &self.history {
                            let mut chars_len = chars.len();
                            while ((prompt_len + chars_len) / line_size) > 0 {
                                term.clear_chars(chars_len)?;
                                if (prompt_len + chars_len) % line_size == 0 {
                                    chars_len -= std::cmp::min(chars_len, line_size);
                                } else {
                                    chars_len -= std::cmp::min(
                                        chars_len,
                                        (prompt_len + chars_len + 1) % line_size,
                                    );
                                }
                                if chars_len > 0 {
                                    term.move_cursor_up(1)?;
                                    term.move_cursor_right(line_size)?;
                                }
                            }
                            term.clear_chars(chars_len)?;
                            chars.clear();
                            position = 0;
                            // Move the history position back one in case we have up arrowed into it
                            // and the position is sitting on the next to read
                            if let Some(pos) = hist_pos.checked_sub(1) {
                                hist_pos = pos;
                                // Move it back again to get the previous history entry
                                if let Some(pos) = pos.checked_sub(1) {
                                    if let Some(previous) = history.lock().unwrap().read(pos) {
                                        for ch in previous.chars() {
                                            chars.insert(position, ch);
                                            position += 1;
                                        }
                                        term.write_str(&previous)?;
                                    }
                                }
                            }
                            term.flush()?;
                        }
                    }
                    Key::Enter => break,
                    _ => (),
                }
            }
            let input = chars.iter().collect::<String>();

            term.clear_line()?;
            render.clear()?;

            if chars.is_empty() {
                if let Some(ref default) = self.default {
                    if let Some(ref mut validator) = self.validator {
                        if let Some(err) = validator.lock().unwrap()(default) {
                            render.error(&err)?;
                            continue;
                        }
                    }

                    if self.report {
                        render.input_prompt_selection(&self.prompt, &default.to_string())?;
                    }
                    term.flush()?;
                    return Ok(default.clone());
                } else if !self.permit_empty {
                    continue;
                }
            }

            match input.parse::<T>() {
                Ok(value) => {
                    #[cfg(feature = "history")]
                    if let Some(history) = &mut self.history {
                        history.lock().unwrap().write(&value);
                    }

                    if let Some(ref mut validator) = self.validator {
                        if let Some(err) = validator.lock().unwrap()(&value) {
                            render.error(&err)?;
                            continue;
                        }
                    }

                    if self.report {
                        if let Some(post_completion_text) = &self.post_completion_text {
                            render.input_prompt_selection(post_completion_text, &input)?;
                        } else {
                            render.input_prompt_selection(&self.prompt, &input)?;
                        }
                    }
                    term.flush()?;

                    return Ok(value);
                }
                Err(err) => {
                    render.error(&err.to_string())?;
                    continue;
                }
            }
        }
    }

    /// Enables user interaction and returns the result.
    ///
    /// Allows any characters as input, including e.g arrow keys.
    /// Some of the keys might have undesired behavior.
    /// For more limited version, see [`interact_text`](Self::interact_text).
    ///
    /// If the user confirms the result is `true`, `false` otherwise.
    /// The dialog is rendered on stderr.
    pub fn interact(self) -> Result<T> {
        self.interact_on(&Term::stderr())
    }

    /// Like [`interact`](Self::interact) but allows a specific terminal to be set.
    pub fn interact_on(mut self, term: &Term) -> Result<T> {
        if !term.is_term() {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "not a terminal").into());
        }

        let mut render = TermThemeRenderer::new(term, self.theme);

        loop {
            let default_string = self.default.as_ref().map(ToString::to_string);

            render.input_prompt(
                &self.prompt,
                if self.show_default {
                    default_string.as_deref()
                } else {
                    None
                },
            )?;
            term.flush()?;

            let input = if let Some(initial_text) = self.initial_text.as_ref() {
                term.read_line_initial_text(initial_text)?
            } else {
                term.read_line()?
            };

            render.add_line();
            term.clear_line()?;
            render.clear()?;

            if input.is_empty() {
                if let Some(ref default) = self.default {
                    if let Some(ref mut validator) = self.validator {
                        if let Some(err) = validator.lock().unwrap()(default) {
                            render.error(&err)?;
                            continue;
                        }
                    }

                    if self.report {
                        render.input_prompt_selection(&self.prompt, &default.to_string())?;
                    }
                    term.flush()?;
                    return Ok(default.clone());
                } else if !self.permit_empty {
                    continue;
                }
            }

            match input.parse::<T>() {
                Ok(value) => {
                    if let Some(ref mut validator) = self.validator {
                        if let Some(err) = validator.lock().unwrap()(&value) {
                            render.error(&err)?;
                            continue;
                        }
                    }

                    if self.report {
                        render.input_prompt_selection(&self.prompt, &input)?;
                    }
                    term.flush()?;

                    return Ok(value);
                }
                Err(err) => {
                    render.error(&err.to_string())?;
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let input = Input::<String>::new().with_prompt("Your name");

        let _ = input.clone();
    }
}
