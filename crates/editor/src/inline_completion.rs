pub trait InlineCompletionProvider {
    fn refresh();
    fn text_for_active_completion() -> Option<&str>;
    fn accept();
    fn discard();
    fn cycle();
}
