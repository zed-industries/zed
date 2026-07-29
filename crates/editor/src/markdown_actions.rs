use super::*;

impl Editor {
    pub fn toggle_markdown_block_quote(
        &mut self,
        _: &ToggleBlockQuote,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.manipulate_mutable_lines_in_markdown(window, cx, |lines| {
            let all_lines_quoted = lines.iter().all(|line| line.starts_with('>'));

            for line in lines.iter_mut() {
                let stripped_line = match line.strip_prefix("> ").or_else(|| line.strip_prefix('>'))
                {
                    Some(rest) => rest.to_string(),
                    None => line.to_string(),
                };

                *line = if all_lines_quoted {
                    Cow::Owned(stripped_line)
                } else if stripped_line.trim().is_empty() {
                    Cow::Borrowed(">")
                } else {
                    Cow::Owned(format!("> {stripped_line}"))
                };
            }
        });
    }

    fn manipulate_mutable_lines_in_markdown<Fn>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        callback: Fn,
    ) where
        Fn: FnMut(&mut Vec<Cow<'_, str>>),
    {
        if !self.is_in_markdown_language(cx) {
            return;
        }

        self.manipulate_mutable_lines(window, cx, callback);
    }

    fn is_in_markdown_language(&self, cx: &mut App) -> bool {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let head = self
            .selections
            .newest::<MultiBufferOffset>(&self.display_snapshot(cx))
            .head();
        snapshot
            .language_at(head)
            .is_some_and(|language| language.name() == "Markdown")
    }
}
