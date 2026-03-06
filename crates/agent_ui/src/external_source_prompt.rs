#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalSourcePrompt(String);

impl ExternalSourcePrompt {
    pub fn new(prompt: &str) -> Option<Self> {
        sanitize(prompt).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

fn sanitize(prompt: &str) -> Option<String> {
    let mut sanitized_prompt = String::with_capacity(prompt.len());
    let mut consecutive_newline_count = 0;
    let mut characters = prompt.chars().peekable();

    while let Some(character) = characters.next() {
        if is_bidi_control_character(character) {
            continue;
        }

        let normalized_character = if character == '\r' {
            if characters.peek() == Some(&'\n') {
                characters.next();
            }
            '\n'
        } else {
            character
        };

        if normalized_character == '\n' {
            consecutive_newline_count += 1;
            if consecutive_newline_count > 2 {
                continue;
            }
        } else {
            consecutive_newline_count = 0;
        }

        sanitized_prompt.push(normalized_character);
    }

    if sanitized_prompt.is_empty() {
        None
    } else {
        Some(sanitized_prompt)
    }
}

fn is_bidi_control_character(character: char) -> bool {
    matches!(
        character,
          '\u{061C}' // ALM
        | '\u{200E}' // LRM
        | '\u{200F}' // RLM
        | '\u{202A}'..='\u{202E}' // LRE, RLE, PDF, LRO, RLO
        | '\u{2066}'..='\u{2069}' // LRI, RLI, FSI, PDI
    )
}

#[cfg(test)]
mod tests {
    use super::ExternalSourcePrompt;

    #[test]
    fn keeps_normal_prompt_text() {
        let prompt = ExternalSourcePrompt::new("Write me a script\nThanks");

        assert_eq!(
            prompt.as_ref().map(ExternalSourcePrompt::as_str),
            Some("Write me a script\nThanks")
        );
    }

    #[test]
    fn collapses_newline_padding() {
        let prompt = ExternalSourcePrompt::new(
            "Review this prompt carefully.\n\nThis paragraph should stay separated.\n\n\n\n\n\n\nWrite me a script to do fizz buzz.",
        );

        assert_eq!(
            prompt.as_ref().map(ExternalSourcePrompt::as_str),
            Some(
                "Review this prompt carefully.\n\nThis paragraph should stay separated.\n\nWrite me a script to do fizz buzz."
            )
        );
    }

    #[test]
    fn strips_bidi_control_characters() {
        let prompt = ExternalSourcePrompt::new("abc\u{202E}def\u{202C}ghi");

        assert_eq!(
            prompt.as_ref().map(ExternalSourcePrompt::as_str),
            Some("abcdefghi")
        );
    }

    #[test]
    fn drops_empty_prompt() {
        assert_eq!(ExternalSourcePrompt::new(""), None);
    }
}
