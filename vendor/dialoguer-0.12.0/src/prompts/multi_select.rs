use std::{io, iter::repeat, ops::Rem};

use console::{Key, Term};

use crate::{
    theme::{render::TermThemeRenderer, SimpleTheme, Theme},
    Paging, Result,
};

/// Renders a multi select prompt.
///
/// ## Example
///
/// ```rust,no_run
/// use dialoguer::MultiSelect;
///
/// fn main() {
///     let items = vec!["foo", "bar", "baz"];
///
///     let selection = MultiSelect::new()
///         .with_prompt("What do you choose?")
///         .items(&items)
///         .interact()
///         .unwrap();
///
///     println!("You chose:");
///
///     for i in selection {
///         println!("{}", items[i]);
///     }
/// }
/// ```
#[derive(Clone)]
pub struct MultiSelect<'a> {
    defaults: Vec<bool>,
    items: Vec<String>,
    prompt: Option<String>,
    report: bool,
    clear: bool,
    max_length: Option<usize>,
    theme: &'a dyn Theme,
}

impl Default for MultiSelect<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiSelect<'static> {
    /// Creates a multi select prompt with default theme.
    pub fn new() -> Self {
        Self::with_theme(&SimpleTheme)
    }
}

impl MultiSelect<'_> {
    /// Sets the clear behavior of the menu.
    ///
    /// The default is to clear the menu.
    pub fn clear(mut self, val: bool) -> Self {
        self.clear = val;
        self
    }

    /// Sets a defaults for the menu.
    pub fn defaults(mut self, val: &[bool]) -> Self {
        self.defaults = val
            .to_vec()
            .iter()
            .copied()
            .chain(repeat(false))
            .take(self.items.len())
            .collect();
        self
    }

    /// Sets an optional max length for a page
    ///
    /// Max length is disabled by None
    pub fn max_length(mut self, val: usize) -> Self {
        // Paging subtracts two from the capacity, paging does this to
        // make an offset for the page indicator. So to make sure that
        // we can show the intended amount of items we need to add two
        // to our value.
        self.max_length = Some(val + 2);
        self
    }

    /// Add a single item to the selector.
    #[inline]
    pub fn item<T: ToString>(self, item: T) -> Self {
        self.item_checked(item, false)
    }

    /// Add a single item to the selector with a default checked state.
    pub fn item_checked<T: ToString>(mut self, item: T, checked: bool) -> Self {
        self.items.push(item.to_string());
        self.defaults.push(checked);
        self
    }

    /// Adds multiple items to the selector.
    pub fn items<T, I>(self, items: I) -> Self
    where
        T: ToString,
        I: IntoIterator<Item = T>,
    {
        self.items_checked(items.into_iter().map(|item| (item, false)))
    }

    /// Adds multiple items to the selector with checked state
    pub fn items_checked<T, I>(mut self, items: I) -> Self
    where
        T: ToString,
        I: IntoIterator<Item = (T, bool)>,
    {
        for (item, checked) in items.into_iter() {
            self.items.push(item.to_string());
            self.defaults.push(checked);
        }
        self
    }

    /// Prefaces the menu with a prompt.
    ///
    /// By default, when a prompt is set the system also prints out a confirmation after
    /// the selection. You can opt-out of this with [`report`](Self::report).
    pub fn with_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Indicates whether to report the selected values after interaction.
    ///
    /// The default is to report the selections.
    pub fn report(mut self, val: bool) -> Self {
        self.report = val;
        self
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can select the items with the 'Space' bar and on 'Enter' the indices of selected items will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `Vec<index>` if user hit 'Enter'.
    /// This unlike [`interact_opt`](Self::interact_opt) does not allow to quit with 'Esc' or 'q'.
    #[inline]
    pub fn interact(self) -> Result<Vec<usize>> {
        self.interact_on(&Term::stderr())
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can select the items with the 'Space' bar and on 'Enter' the indices of selected items will be returned.
    /// The dialog is rendered on stderr.
    /// Result contains `Some(Vec<index>)` if user hit 'Enter' or `None` if user cancelled with 'Esc' or 'q'.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use dialoguer::MultiSelect;
    ///
    /// fn main() {
    ///     let items = vec!["foo", "bar", "baz"];
    ///
    ///     let ordered = MultiSelect::new()
    ///         .items(&items)
    ///         .interact_opt()
    ///         .unwrap();
    ///
    ///     match ordered {
    ///         Some(positions) => {
    ///             println!("You chose:");
    ///
    ///             for i in positions {
    ///                 println!("{}", items[i]);
    ///             }
    ///         },
    ///         None => println!("You did not choose anything.")
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn interact_opt(self) -> Result<Option<Vec<usize>>> {
        self.interact_on_opt(&Term::stderr())
    }

    /// Like [`interact`](Self::interact) but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on(self, term: &Term) -> Result<Vec<usize>> {
        Ok(self
            ._interact_on(term, false)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Quit not allowed in this case"))?)
    }

    /// Like [`interact_opt`](Self::interact_opt) but allows a specific terminal to be set.
    #[inline]
    pub fn interact_on_opt(self, term: &Term) -> Result<Option<Vec<usize>>> {
        self._interact_on(term, true)
    }

    fn _interact_on(self, term: &Term, allow_quit: bool) -> Result<Option<Vec<usize>>> {
        if !term.is_term() {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "not a terminal").into());
        }

        if self.items.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Empty list of items given to `MultiSelect`",
            ))?;
        }

        let mut paging = Paging::new(term, self.items.len(), self.max_length);
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = 0;

        let mut size_vec = Vec::new();

        for items in self
            .items
            .iter()
            .flat_map(|i| i.split('\n'))
            .collect::<Vec<_>>()
        {
            let size = &items.len();
            size_vec.push(*size);
        }

        let mut checked: Vec<bool> = self.defaults.clone();

        term.hide_cursor()?;

        loop {
            if let Some(ref prompt) = self.prompt {
                paging
                    .render_prompt(|paging_info| render.multi_select_prompt(prompt, paging_info))?;
            }

            for (idx, item) in self
                .items
                .iter()
                .enumerate()
                .skip(paging.current_page * paging.capacity)
                .take(paging.capacity)
            {
                render.multi_select_prompt_item(item, checked[idx], sel == idx)?;
            }

            term.flush()?;

            match term.read_key()? {
                Key::ArrowDown | Key::Tab | Key::Char('j') => {
                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(self.items.len() as u64) as usize;
                    }
                }
                Key::ArrowUp | Key::BackTab | Key::Char('k') => {
                    if sel == !0 {
                        sel = self.items.len() - 1;
                    } else {
                        sel = ((sel as i64 - 1 + self.items.len() as i64)
                            % (self.items.len() as i64)) as usize;
                    }
                }
                Key::ArrowLeft | Key::Char('h') => {
                    if paging.active {
                        sel = paging.previous_page();
                    }
                }
                Key::ArrowRight | Key::Char('l') => {
                    if paging.active {
                        sel = paging.next_page();
                    }
                }
                Key::Char(' ') => {
                    checked[sel] = !checked[sel];
                }
                Key::Char('a') => {
                    if checked.iter().all(|&item_checked| item_checked) {
                        checked.fill(false);
                    } else {
                        checked.fill(true);
                    }
                }
                Key::Escape | Key::Char('q') => {
                    if allow_quit {
                        if self.clear {
                            render.clear()?;
                        } else {
                            term.clear_last_lines(paging.capacity)?;
                        }

                        term.show_cursor()?;
                        term.flush()?;

                        return Ok(None);
                    }
                }
                Key::Enter => {
                    if self.clear {
                        render.clear()?;
                    }

                    if let Some(ref prompt) = self.prompt {
                        if self.report {
                            let selections: Vec<_> = checked
                                .iter()
                                .enumerate()
                                .filter_map(|(idx, &checked)| {
                                    if checked {
                                        Some(self.items[idx].as_str())
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            render.multi_select_prompt_selection(prompt, &selections[..])?;
                        }
                    }

                    term.show_cursor()?;
                    term.flush()?;

                    return Ok(Some(
                        checked
                            .into_iter()
                            .enumerate()
                            .filter_map(|(idx, checked)| if checked { Some(idx) } else { None })
                            .collect(),
                    ));
                }
                _ => {}
            }

            paging.update(sel)?;

            if paging.active {
                render.clear()?;
            } else {
                render.clear_preserve_prompt(&size_vec)?;
            }
        }
    }
}

impl<'a> MultiSelect<'a> {
    /// Creates a multi select prompt with a specific theme.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use dialoguer::{theme::ColorfulTheme, MultiSelect};
    ///
    /// fn main() {
    ///     let selection = MultiSelect::with_theme(&ColorfulTheme::default())
    ///         .items(&["foo", "bar", "baz"])
    ///         .interact()
    ///         .unwrap();
    /// }
    /// ```
    pub fn with_theme(theme: &'a dyn Theme) -> Self {
        Self {
            items: vec![],
            defaults: vec![],
            clear: true,
            prompt: None,
            report: true,
            max_length: None,
            theme,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let multi_select = MultiSelect::new().with_prompt("Select your favorite(s)");

        let _ = multi_select.clone();
    }

    #[test]
    fn test_iterator() {
        let items = ["First", "Second", "Third"];
        let iterator = items.iter().skip(1);

        assert_eq!(MultiSelect::new().items(iterator).items, &items[1..]);
    }
}
