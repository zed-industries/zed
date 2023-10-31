pub enum TruncationDirection {
    Start,
    End,
}

pub trait LanguageModel {
    fn name(&self) -> String;
    fn count_tokens(&self, content: &str) -> anyhow::Result<usize>;
    fn truncate(
        &self,
        content: &str,
        length: usize,
        direction: TruncationDirection,
    ) -> anyhow::Result<String>;
    fn capacity(&self) -> anyhow::Result<usize>;
}
