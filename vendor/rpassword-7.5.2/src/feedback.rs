use crate::config::PasswordFeedback;
use rtoolbox::safe_string::SafeString;
use std::cmp::min;

pub struct FeedbackState {
    password: SafeString,
    needs_terminal_configuration: bool,
    displayed_count: usize,
    feedback: PasswordFeedback,
}

impl FeedbackState {
    pub fn new(feedback: PasswordFeedback, needs_terminal_configuration: bool) -> Self {
        FeedbackState {
            password: SafeString::new(),
            needs_terminal_configuration,
            displayed_count: 0,
            feedback,
        }
    }

    pub fn push_char(&mut self, c: char) -> String {
        self.password.push(c);

        if !self.needs_terminal_configuration {
            return String::new();
        }

        match self.feedback {
            PasswordFeedback::Hide => String::new(),
            PasswordFeedback::Mask(mask) => {
                self.displayed_count += 1;
                mask.to_string()
            }
            PasswordFeedback::PartialMask(mask, n) => {
                self.displayed_count += 1;
                if self.displayed_count <= n {
                    c.to_string()
                } else {
                    mask.to_string()
                }
            }
        }
    }

    pub fn pop_char(&mut self) -> String {
        let last_char = self.password.chars().last();
        if let Some(c) = last_char {
            let new_len = self.password.len() - c.len_utf8();
            self.password.truncate(new_len);

            if !self.needs_terminal_configuration {
                return String::new();
            }

            if self.displayed_count > 0 {
                self.displayed_count -= 1;
                "\x08 \x08".to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    pub fn clear(&mut self) -> String {
        self.password = SafeString::new();

        if !self.needs_terminal_configuration {
            return String::new();
        }

        let count = self.displayed_count;
        self.displayed_count = 0;
        "\x08 \x08".repeat(count).to_string()
    }

    pub fn clear_til_last_space(&mut self) -> String {
        let mut trimmed = self.password.as_str().trim_end();

        match trimmed.rfind(' ') {
            Some(last_space_position) => {
                trimmed = &trimmed[..=last_space_position];
            }
            None => {
                trimmed = "";
            }
        }

        let new_displayed_count = trimmed.chars().count();
        let removed_chars = self.password.chars().count() - trimmed.chars().count();
        self.password = trimmed.to_string().into();

        if !self.needs_terminal_configuration {
            return String::new();
        }

        let count = self.displayed_count;
        self.displayed_count = new_displayed_count;
        "\x08 \x08".repeat(min(removed_chars, count)).to_string()
    }

    pub fn abort(&mut self) -> String {
        self.password = SafeString::new();

        if !self.needs_terminal_configuration {
            return String::new();
        }

        self.displayed_count = 0;
        '\n'.to_string()
    }

    pub fn finish(&mut self) -> String {
        if !self.needs_terminal_configuration {
            return String::new();
        }

        '\n'.to_string()
    }

    pub fn is_empty(&self) -> bool {
        self.password.is_empty()
    }

    pub fn into_password(self) -> String {
        self.password.into_inner()
    }
}

#[cfg(test)]
mod tests {
    mod with_terminal_configuration {
        use crate::config::PasswordFeedback;
        use crate::feedback::FeedbackState;

        #[test]
        fn feedback_state_mask_star() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), true);
            assert_eq!(state.push_char('a'), "*");
            assert_eq!(state.push_char('b'), "*");
            assert_eq!(state.push_char('🚲'), "*");
            assert_eq!(state.push_char('🚲'), "*");
            assert_eq!(state.pop_char(), "\x08 \x08");
            assert_eq!(state.into_password(), "ab🚲");
        }

        #[test]
        fn feedback_state_mask_hash() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('#'), true);
            assert_eq!(state.push_char('x'), "#");
            assert_eq!(state.push_char('y'), "#");
            assert_eq!(state.push_char('🚲'), "#");
            assert_eq!(state.push_char('🚲'), "#");
            assert_eq!(state.into_password(), "xy🚲🚲");
        }

        #[test]
        fn feedback_state_hide() {
            let mut state = FeedbackState::new(PasswordFeedback::Hide, true);
            assert!(state.push_char('a').is_empty());
            assert!(state.push_char('b').is_empty());
            assert!(state.push_char('🚲').is_empty());
            assert!(state.pop_char().is_empty());
            assert_eq!(state.into_password(), "ab");
        }

        #[test]
        fn feedback_state_partial_mask() {
            let mut state = FeedbackState::new(PasswordFeedback::PartialMask('*', 3), true);
            assert_eq!(state.push_char('a'), "a");
            assert_eq!(state.push_char('b'), "b");
            assert_eq!(state.push_char('c'), "c");
            assert_eq!(state.push_char('🚲'), "*");
            assert_eq!(state.push_char('🚲'), "*");
            assert_eq!(state.into_password(), "abc🚲🚲");
        }

        #[test]
        fn feedback_state_backspace_empty() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), true);
            assert!(state.pop_char().is_empty());
        }

        #[test]
        fn feedback_state_clear() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), true);
            state.push_char('a');
            state.push_char('b');
            state.push_char('c');
            state.push_char('🚲');
            assert_eq!(state.clear(), "\x08 \x08\x08 \x08\x08 \x08\x08 \x08");
            assert!(state.is_empty());
        }

        #[test]
        fn feedback_state_clear_til_last_space() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), true);
            state.push_char('a');
            state.push_char('b');
            state.push_char('c');
            state.push_char(' ');
            state.push_char('d');
            state.push_char('🚲');
            state.push_char(' ');
            state.push_char(' ');
            state.push_char(' ');
            assert_eq!(
                state.clear_til_last_space(),
                "\x08 \x08\x08 \x08\x08 \x08\x08 \x08\x08 \x08"
            );
            assert_eq!(state.into_password(), "abc ");
        }

        #[test]
        fn feedback_state_abort() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), true);
            state.push_char('a');
            state.push_char('b');
            state.push_char('c');
            assert_eq!(state.abort(), "\n");
            assert!(state.is_empty());
        }

        #[test]
        fn feedback_state_finish() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), true);
            state.push_char('a');
            state.push_char('b');
            state.push_char('c');
            assert_eq!(state.finish(), "\n");
            assert_eq!(state.into_password(), "abc");
        }

        #[test]
        fn feedback_state_partial_mask_zero() {
            let mut state = FeedbackState::new(PasswordFeedback::PartialMask('*', 0), true);
            assert_eq!(state.push_char('a'), "*");
            assert_eq!(state.push_char('b'), "*");
            assert_eq!(state.into_password(), "ab");
        }
    }

    mod without_terminal_configuration {
        use crate::config::PasswordFeedback;
        use crate::feedback::FeedbackState;

        #[test]
        fn feedback_state_mask_star() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), false);
            assert_eq!(state.push_char('a'), "");
            assert_eq!(state.push_char('b'), "");
            assert_eq!(state.push_char('c'), "");
            assert_eq!(state.pop_char(), "");
            assert_eq!(state.into_password(), "ab");
        }

        #[test]
        fn feedback_state_mask_hash() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('#'), false);
            assert_eq!(state.push_char('x'), "");
            assert_eq!(state.push_char('y'), "");
            assert_eq!(state.into_password(), "xy");
        }

        #[test]
        fn feedback_state_hide() {
            let mut state = FeedbackState::new(PasswordFeedback::Hide, false);
            assert!(state.push_char('a').is_empty());
            assert!(state.push_char('b').is_empty());
            assert!(state.pop_char().is_empty());
            assert_eq!(state.into_password(), "a");
        }

        #[test]
        fn feedback_state_partial_mask() {
            let mut state = FeedbackState::new(PasswordFeedback::PartialMask('*', 3), false);
            assert_eq!(state.push_char('a'), "");
            assert_eq!(state.push_char('b'), "");
            assert_eq!(state.push_char('c'), "");
            assert_eq!(state.push_char('d'), "");
            assert_eq!(state.push_char('e'), "");
            assert_eq!(state.into_password(), "abcde");
        }

        #[test]
        fn feedback_state_backspace_empty() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), false);
            assert!(state.pop_char().is_empty());
        }

        #[test]
        fn feedback_state_clear() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), false);
            state.push_char('a');
            state.push_char('b');
            state.push_char('c');
            assert_eq!(state.clear(), "");
            assert!(state.is_empty());
        }

        #[test]
        fn feedback_state_clear_til_last_space() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), false);
            state.push_char('a');
            state.push_char('b');
            state.push_char('c');
            state.push_char(' ');
            state.push_char('d');
            state.push_char('🚲');
            state.push_char(' ');
            state.push_char(' ');
            state.push_char(' ');
            assert_eq!(state.clear_til_last_space(), "");
            assert_eq!(state.into_password(), "abc ");
        }

        #[test]
        fn feedback_state_abort() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), false);
            state.push_char('a');
            state.push_char('b');
            state.push_char('c');
            assert_eq!(state.abort(), "");
            assert!(state.is_empty());
        }

        #[test]
        fn feedback_state_finish() {
            let mut state = FeedbackState::new(PasswordFeedback::Mask('*'), false);
            state.push_char('a');
            state.push_char('b');
            state.push_char('c');
            assert_eq!(state.finish(), "");
            assert_eq!(state.into_password(), "abc");
        }

        #[test]
        fn feedback_state_partial_mask_zero() {
            let mut state = FeedbackState::new(PasswordFeedback::PartialMask('*', 0), false);
            assert_eq!(state.push_char('a'), "");
            assert_eq!(state.push_char('b'), "");
            assert_eq!(state.into_password(), "ab");
        }
    }
}
