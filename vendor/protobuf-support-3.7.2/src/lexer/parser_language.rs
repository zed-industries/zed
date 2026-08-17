/// We use the same lexer/tokenizer for all parsers for simplicity
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParserLanguage {
    // `.proto` files
    Proto,
    // Protobuf text format
    TextFormat,
    // JSON
    Json,
}
