use super::core::display_width;

#[derive(Debug)]
pub(crate) struct LineWrapper<'w> {
    hard_width: usize,
    line_width: usize,
    indentation: Option<&'w str>,
}

impl<'w> LineWrapper<'w> {
    pub(crate) fn new(hard_width: usize) -> Self {
        Self {
            hard_width,
            line_width: 0,
            indentation: None,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.line_width = 0;
        self.indentation = None;
    }

    pub(crate) fn wrap(&mut self, mut words: Vec<&'w str>) -> Vec<&'w str> {
        let mut first_word = false;
        if self.indentation.is_none() {
            first_word = true;
            if let Some(word) = words.first() {
                if word.trim().is_empty() {
                    self.indentation = Some(*word);
                } else {
                    self.indentation = Some("");
                }
            }
        }

        let mut i = 0;
        while i < words.len() {
            let word = &words[i];
            let trimmed = word.trim_end();
            let word_width = display_width(trimmed);
            let trimmed_delta = word.len() - trimmed.len();
            if first_word && 0 < word_width {
                // Never try to wrap the first word
                first_word = false;
            } else if self.hard_width < self.line_width + word_width {
                if 0 < i {
                    let prev = i - 1;
                    let trimmed = words[prev].trim_end();
                    words[prev] = trimmed;
                }

                self.line_width = 0;
                words.insert(i, "\n");
                i += 1;
                if let Some(indentation) = self.indentation {
                    words.insert(i, indentation);
                    self.line_width += indentation.len();
                    i += 1;
                }
            }
            self.line_width += word_width + trimmed_delta;

            i += 1;
        }
        words
    }
}
