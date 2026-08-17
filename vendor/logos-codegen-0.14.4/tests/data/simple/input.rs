#[derive(Logos, Debug, Clone, Copy, PartialEq)]
enum Token {
    #[regex("a-z")]
    Letter,
}
