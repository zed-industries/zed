use full_moon::{
    tokenizer::{Lexer, LexerResult, Token, TokenKind},
    LuaVersion,
};
use mlua::Lua;

#[derive(Default)]
struct LuaRuntime {
    lua: Lua,
    buffer: String,
    tokens: Vec<Token>,
}

impl LuaRuntime {
    fn receive_chunk(&mut self, chunk: &str, run: impl FnOnce(&str)) {
        self.buffer.push_str(chunk);

        let mut lexer = Lexer::new_lazy(&self.buffer, LuaVersion::lua54());

        while let Some(LexerResult::Ok(token)) = lexer.process_next() {
            self.tokens.push(token);
        }

        // If there's an EOF token at the end, drop it.
        // This is the end of a chunk, not the whole file!
        if let Some(TokenKind::Eof) = self.tokens.last().map(|token| token.token_kind()) {
            self.tokens.pop();
        }

        // Drop the last token, because we can't know if it's incomplete.
        // For example, if the last token is `if` that might be because it's
        // a complete `if` conditional token, or if it's the beginning of
        // a variable that happens to start with the letters "if" and the
        // rest of the variable will be coming in the next chunk.
        self.tokens.pop();

        if let Some(token) = self.tokens.last() {
            // Advance the buffer past the last token.
            self.buffer = self.buffer[token.end_position().bytes() + 1..].to_string();

            // TODO turn tokens into parsed statements. Once we're done, let the
            // interpreter proceed running the string through the end of the last
            // complete statement we parsed. Then drop the remaining tokens too, and
            // leave them for next time.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use full_moon::tokenizer::{TokenKind, TokenType};

    #[test]
    fn test_lua_runtime_receive_chunk() {
        let mut runtime = LuaRuntime::default();
        runtime.receive_chunk("foo bar baz");

        assert_eq!(runtime.tokens.len(), 4);
        assert_eq!(runtime.tokens[0].token_kind(), TokenKind::Identifier);
        assert_eq!(
            runtime.tokens[0].token_type(),
            &TokenType::Identifier {
                identifier: "foo".into()
            }
        );

        assert_eq!(runtime.tokens[1].token_kind(), TokenKind::Whitespace);

        assert_eq!(runtime.tokens[2].token_kind(), TokenKind::Identifier);
        assert_eq!(
            runtime.tokens[2].token_type(),
            &TokenType::Identifier {
                identifier: "bar".into()
            }
        );

        assert_eq!(runtime.tokens[3].token_kind(), TokenKind::Whitespace);
    }
}

// pub struct ParserState {
//     errors: Vec<ast::Error>,
//     lexer: Lexer,
//     lua_version: LuaVersion,
// }

// pub fn parse_fallible(code: &str, lua_version: LuaVersion) -> Self {
//     const UNEXPECTED_TOKEN_ERROR: &str = "unexpected token, this needs to be a statement";

//     let lexer = Lexer::new(code, lua_version);
//     let mut parser_state = ParserState::new(lexer);

//     let mut block = match parse_block(&mut parser_state) {
//         ParserResult::Value(block) => block,
//         _ => Block::new(),
//     };

//     let block_has_last_stmt = block.last_stmt().is_some();

//     loop {
//         match parser_state.lexer.current() {
//             Some(LexerResult::Ok(token)) if token.token_kind() == TokenKind::Eof => {
//                 break;
//             }

//             Some(LexerResult::Ok(_)) => {
//                 if let ParserResult::Value(new_block) = parse_block(&mut parser_state) {
//                     if new_block.stmts.is_empty() {
//                         if let Ok(token) = parser_state.current() {
//                             if token.token_kind() == TokenKind::Eof {
//                                 break;
//                             }
//                         }

//                         match parser_state.consume() {
//                             ParserResult::Value(token) => {
//                                 if let Some(crate::Error::AstError(crate::ast::AstError {
//                                     additional,
//                                     ..
//                                 })) = parser_state.errors.last()
//                                 {
//                                     if additional == UNEXPECTED_TOKEN_ERROR {
//                                         continue;
//                                     }
//                                 }

//                                 parser_state.token_error(token, UNEXPECTED_TOKEN_ERROR);
//                             }

//                             ParserResult::LexerMoved => {}

//                             ParserResult::NotFound => unreachable!(),
//                         }

//                         continue;
//                     }

//                     if block_has_last_stmt {
//                         parser_state.token_error(
//                             new_block.tokens().next().unwrap().clone(),
//                             "unexpected statement after last statement",
//                         )
//                     }

//                     block.merge_blocks(new_block);
//                 }
//             }

//             Some(LexerResult::Fatal(_)) => {
//                 for error in parser_state.lexer.consume().unwrap().unwrap_errors() {
//                     parser_state
//                         .errors
//                         .push(crate::Error::TokenizerError(error));
//                 }
//             }

//             None => break,
//         }
//     }

//     let eof = match parser_state.lexer.consume().unwrap() {
//         LexerResult::Ok(token) => token,

//         LexerResult::Recovered(token, errors) => {
//             for error in errors {
//                 parser_state
//                     .errors
//                     .push(crate::Error::TokenizerError(error));
//             }

//             token
//         }

//         LexerResult::Fatal(error) => unreachable!("error: {error:?}"),
//     };

//     debug_assert_eq!(eof.token_kind(), TokenKind::Eof);

//     Self {
//         ast: Ast { nodes: block, eof },
//         errors: parser_state.errors,
//     }
// }

// pub fn parse_block(state: &mut ParserState) -> ParserResult<ast::Block> {
//     let mut stmts = Vec::new();

//     loop {
//         match parse_stmt(state) {
//             ParserResult::Value(StmtVariant::Stmt(stmt)) => {
//                 let semicolon = state.consume_if(Symbol::Semicolon);
//                 stmts.push((stmt, semicolon));
//             }
//             ParserResult::Value(StmtVariant::LastStmt(last_stmt)) => {
//                 let semicolon = state.consume_if(Symbol::Semicolon);
//                 let last_stmt = Some((last_stmt, semicolon));
//                 return ParserResult::Value(
//                     ast::Block::new()
//                         .with_stmts(stmts)
//                         .with_last_stmt(last_stmt),
//                 );
//             }

//             ParserResult::NotFound => break,
//             ParserResult::LexerMoved => {
//                 if stmts.is_empty() {
//                     return ParserResult::LexerMoved;
//                 } else {
//                     break;
//                 }
//             }
//         }
//     }

//     let last_stmt = match parse_last_stmt(state) {
//         ParserResult::Value(stmt) => Some(stmt),
//         ParserResult::LexerMoved | ParserResult::NotFound => None,
//     };

//     ParserResult::Value(
//         ast::Block::new()
//             .with_stmts(stmts)
//             .with_last_stmt(last_stmt),
//     )
// }
