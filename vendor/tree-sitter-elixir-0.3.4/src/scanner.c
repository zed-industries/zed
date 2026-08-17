#include "tree_sitter/parser.h"

// See references in grammar.externals
enum TokenType {
  QUOTED_CONTENT_I_SINGLE,
  QUOTED_CONTENT_I_DOUBLE,
  QUOTED_CONTENT_I_HEREDOC_SINGLE,
  QUOTED_CONTENT_I_HEREDOC_DOUBLE,
  QUOTED_CONTENT_I_PARENTHESIS,
  QUOTED_CONTENT_I_CURLY,
  QUOTED_CONTENT_I_SQUARE,
  QUOTED_CONTENT_I_ANGLE,
  QUOTED_CONTENT_I_BAR,
  QUOTED_CONTENT_I_SLASH,
  QUOTED_CONTENT_SINGLE,
  QUOTED_CONTENT_DOUBLE,
  QUOTED_CONTENT_HEREDOC_SINGLE,
  QUOTED_CONTENT_HEREDOC_DOUBLE,
  QUOTED_CONTENT_PARENTHESIS,
  QUOTED_CONTENT_CURLY,
  QUOTED_CONTENT_SQUARE,
  QUOTED_CONTENT_ANGLE,
  QUOTED_CONTENT_BAR,
  QUOTED_CONTENT_SLASH,

  NEWLINE_BEFORE_DO,
  NEWLINE_BEFORE_BINARY_OPERATOR,
  NEWLINE_BEFORE_COMMENT,

  BEFORE_UNARY_OPERATOR,

  NOT_IN,

  QUOTED_ATOM_START
};

static inline void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static inline void skip(TSLexer *lexer) { lexer->advance(lexer, true); }

// Note: some checks require several lexer steps of lookahead
// and alter its state, for these we use names check_*

static inline bool is_whitespace(int32_t c) {
  return c == ' ' || c == '\t' || c == '\n' || c == '\r';
}

static inline bool is_inline_whitespace(int32_t c) {
  return c == ' ' || c == '\t';
}

static inline bool is_newline(int32_t c) {
  // Note: this implies \r\n is treated as two line breaks,
  // but in our case it's fine, since multiple line breaks
  // make no difference
  return c == '\n' || c == '\r';
}

static inline bool is_digit(int32_t c) { return '0' <= c && c <= '9'; }

static inline bool check_keyword_end(TSLexer *lexer) {
  if (lexer->lookahead == ':') {
    advance(lexer);
    return is_whitespace(lexer->lookahead);
  }
  return false;
}

static bool check_operator_end(TSLexer *lexer) {
  // Keyword
  if (lexer->lookahead == ':') {
    return !check_keyword_end(lexer);
  }
  while (is_inline_whitespace(lexer->lookahead)) {
    advance(lexer);
  }
  // Operator identifier with arity
  if (lexer->lookahead == '/') {
    advance(lexer);
    while (is_whitespace(lexer->lookahead)) {
      advance(lexer);
    }
    if (is_digit(lexer->lookahead)) {
      return false;
    }
  }

  return true;
}

const char token_terminators[] = {
    // Operator starts
    '@', '.', '+', '-', '^', '-', '*', '/', '<', '>', '|', '~', '=', '&', '\\',
    '%',
    // Delimiters
    '{', '}', '[', ']', '(', ')', '"', '\'',
    // Separators
    ',', ';',
    // Comment
    '#'};

const uint8_t token_terminators_length =
    sizeof(token_terminators) / sizeof(char);

// Note: this is a heuristic as we only use this to distinguish word
// operators and we don't want to include complex Unicode ranges
static inline bool is_token_end(int32_t c) {
  for (uint8_t i = 0; i < token_terminators_length; i++) {
    if (c == token_terminators[i]) {
      return true;
    }
  }

  return is_whitespace(c);
}

typedef struct {
  const enum TokenType token_type;
  const bool supports_interpol;
  const int32_t end_delimiter;
  const uint8_t delimiter_length;
} QuotedContentInfo;

const QuotedContentInfo quoted_content_infos[] = {
    {QUOTED_CONTENT_I_SINGLE,         true,  '\'', 1},
    {QUOTED_CONTENT_I_DOUBLE,         true,  '"',  1},
    {QUOTED_CONTENT_I_HEREDOC_SINGLE, true,  '\'', 3},
    {QUOTED_CONTENT_I_HEREDOC_DOUBLE, true,  '"',  3},
    {QUOTED_CONTENT_I_PARENTHESIS,    true,  ')',  1},
    {QUOTED_CONTENT_I_CURLY,          true,  '}',  1},
    {QUOTED_CONTENT_I_SQUARE,         true,  ']',  1},
    {QUOTED_CONTENT_I_ANGLE,          true,  '>',  1},
    {QUOTED_CONTENT_I_BAR,            true,  '|',  1},
    {QUOTED_CONTENT_I_SLASH,          true,  '/',  1},
    {QUOTED_CONTENT_SINGLE,           false, '\'', 1},
    {QUOTED_CONTENT_DOUBLE,           false, '"',  1},
    {QUOTED_CONTENT_HEREDOC_SINGLE,   false, '\'', 3},
    {QUOTED_CONTENT_HEREDOC_DOUBLE,   false, '"',  3},
    {QUOTED_CONTENT_PARENTHESIS,      false, ')',  1},
    {QUOTED_CONTENT_CURLY,            false, '}',  1},
    {QUOTED_CONTENT_SQUARE,           false, ']',  1},
    {QUOTED_CONTENT_ANGLE,            false, '>',  1},
    {QUOTED_CONTENT_BAR,              false, '|',  1},
    {QUOTED_CONTENT_SLASH,            false, '/',  1},
};

const uint8_t quoted_content_infos_length =
    sizeof(quoted_content_infos) / sizeof(QuotedContentInfo);

static inline int8_t find_quoted_token_info(const bool *valid_symbols) {
  // Quoted tokens are mutually exclusive and only one should be valid
  // at a time. If multiple are valid it means we parse an arbitrary
  // code outside quotes, in which case we don't want to tokenize it as
  // quoted content.
  if (valid_symbols[QUOTED_CONTENT_I_SINGLE] &&
      valid_symbols[QUOTED_CONTENT_I_DOUBLE]) {
    return -1;
  }

  for (uint8_t i = 0; i < quoted_content_infos_length; i++) {
    if (valid_symbols[quoted_content_infos[i].token_type]) {
      return i;
    }
  }

  return -1;
}

static bool scan_quoted_content(TSLexer *lexer, const QuotedContentInfo *info) {
  lexer->result_symbol = info->token_type;

  bool is_heredoc = (info->delimiter_length == 3);

  for (bool has_content = false; true; has_content = true) {
    bool newline = false;

    if (is_newline(lexer->lookahead)) {
      advance(lexer);

      has_content = true;
      newline = true;

      while (is_whitespace(lexer->lookahead)) {
        advance(lexer);
      }
    }

    lexer->mark_end(lexer);

    if (lexer->lookahead == info->end_delimiter) {
      uint8_t length = 1;

      while (length < info->delimiter_length) {
        advance(lexer);
        if (lexer->lookahead == info->end_delimiter) {
          length++;
        } else {
          break;
        }
      }

      if (length == info->delimiter_length && (!is_heredoc || newline)) {
        return has_content;
      }
    } else {
      if (lexer->lookahead == '#') {
        advance(lexer);
        if (info->supports_interpol && lexer->lookahead == '{') {
          return has_content;
        }
      } else if (lexer->lookahead == '\\') {
        advance(lexer);
        if (is_heredoc && lexer->lookahead == '\n') {
          // We need to know about the newline to correctly recognise
          // heredoc end delimiter, so we intentionally ignore
          // escaping
        } else if (info->supports_interpol ||
                   lexer->lookahead == info->end_delimiter) {
          return has_content;
        }
      } else if (lexer->lookahead == '\0') {
        // If we reached the end of the file, this means there is no
        // end delimiter, so the syntax is invalid. In that case we
        // want to treat all the scanned content as quoted content.
        return has_content;
      } else {
        advance(lexer);
      }
    }
  }

  return false;
}

static bool scan_newline(TSLexer *lexer, const bool *valid_symbols) {
  advance(lexer);

  while (is_whitespace(lexer->lookahead)) {
    advance(lexer);
  }

  // Note we include all the whitespace after newline, so that the
  // parser doesn't have to go through it again
  lexer->mark_end(lexer);

  if (lexer->lookahead == '#') {
    lexer->result_symbol = NEWLINE_BEFORE_COMMENT;
    return true;
  }

  if (lexer->lookahead == 'd' && valid_symbols[NEWLINE_BEFORE_DO]) {
    lexer->result_symbol = NEWLINE_BEFORE_DO;
    advance(lexer);
    if (lexer->lookahead == 'o') {
      advance(lexer);
      return is_token_end(lexer->lookahead);
    }
    return false;
  }

  if (valid_symbols[NEWLINE_BEFORE_BINARY_OPERATOR]) {
    lexer->result_symbol = NEWLINE_BEFORE_BINARY_OPERATOR;

    // &&, &&&
    if (lexer->lookahead == '&') {
      advance(lexer);
      if (lexer->lookahead == '&') {
        advance(lexer);
        if (lexer->lookahead == '&') {
          advance(lexer);
          return check_operator_end(lexer);
        } else {
          return check_operator_end(lexer);
        }
      }
      // =, ==, ===, =~, =>
    } else if (lexer->lookahead == '=') {
      advance(lexer);
      if (lexer->lookahead == '=') {
        advance(lexer);
        if (lexer->lookahead == '=') {
          advance(lexer);
          return check_operator_end(lexer);
        } else {
          return check_operator_end(lexer);
        }
      } else if (lexer->lookahead == '~') {
        advance(lexer);
        return check_operator_end(lexer);
      } else if (lexer->lookahead == '>') {
        advance(lexer);
        return check_operator_end(lexer);
      } else {
        return check_operator_end(lexer);
      }
      // ::
    } else if (lexer->lookahead == ':') {
      advance(lexer);
      if (lexer->lookahead == ':') {
        advance(lexer);
        // Ignore ::: atom
        if (lexer->lookahead == ':')
          return false;
        return check_operator_end(lexer);
      }
      // ++, +++
    } else if (lexer->lookahead == '+') {
      advance(lexer);
      if (lexer->lookahead == '+') {
        advance(lexer);
        if (lexer->lookahead == '+') {
          advance(lexer);
          return check_operator_end(lexer);
        } else {
          return check_operator_end(lexer);
        }
      }
      // --, ---, ->
    } else if (lexer->lookahead == '-') {
      advance(lexer);
      if (lexer->lookahead == '-') {
        advance(lexer);
        if (lexer->lookahead == '-') {
          advance(lexer);
          return check_operator_end(lexer);
        } else {
          return check_operator_end(lexer);
        }
      } else if (lexer->lookahead == '>') {
        advance(lexer);
        return check_operator_end(lexer);
      }
      // <, <=, <-, <>, <~, <~>, <|>, <<<, <<~
    } else if (lexer->lookahead == '<') {
      advance(lexer);
      if (lexer->lookahead == '=' || lexer->lookahead == '-' ||
          lexer->lookahead == '>') {
        advance(lexer);
        return check_operator_end(lexer);
      } else if (lexer->lookahead == '~') {
        advance(lexer);
        if (lexer->lookahead == '>') {
          advance(lexer);
          return check_operator_end(lexer);
        } else {
          return check_operator_end(lexer);
        }
      } else if (lexer->lookahead == '|') {
        advance(lexer);
        if (lexer->lookahead == '>') {
          advance(lexer);
          return check_operator_end(lexer);
        }
      } else if (lexer->lookahead == '<') {
        advance(lexer);
        if (lexer->lookahead == '<' || lexer->lookahead == '~') {
          advance(lexer);
          return check_operator_end(lexer);
        }
      } else {
        return check_operator_end(lexer);
      }
      // >, >=, >>>
    } else if (lexer->lookahead == '>') {
      advance(lexer);
      if (lexer->lookahead == '=') {
        advance(lexer);
        return check_operator_end(lexer);
      } else if (lexer->lookahead == '>') {
        advance(lexer);
        if (lexer->lookahead == '>') {
          advance(lexer);
          return check_operator_end(lexer);
        }
      } else {
        return check_operator_end(lexer);
      }
      // ^^^
    } else if (lexer->lookahead == '^') {
      advance(lexer);
      if (lexer->lookahead == '^') {
        advance(lexer);
        if (lexer->lookahead == '^') {
          advance(lexer);
          return check_operator_end(lexer);
        }
      }
      // !=, !==
    } else if (lexer->lookahead == '!') {
      advance(lexer);
      if (lexer->lookahead == '=') {
        advance(lexer);
        if (lexer->lookahead == '=') {
          advance(lexer);
          return check_operator_end(lexer);
        } else {
          return check_operator_end(lexer);
        }
      }
      // ~>, ~>>
    } else if (lexer->lookahead == '~') {
      advance(lexer);
      if (lexer->lookahead == '>') {
        advance(lexer);
        if (lexer->lookahead == '>') {
          advance(lexer);
          return check_operator_end(lexer);
        } else {
          return check_operator_end(lexer);
        }
      }
      // |, ||, |||, |>
    } else if (lexer->lookahead == '|') {
      advance(lexer);
      if (lexer->lookahead == '|') {
        advance(lexer);
        if (lexer->lookahead == '|') {
          advance(lexer);
          return check_operator_end(lexer);
        } else {
          return check_operator_end(lexer);
        }
      } else if (lexer->lookahead == '>') {
        advance(lexer);
        return check_operator_end(lexer);
      } else {
        return check_operator_end(lexer);
      }
      // *, **
    } else if (lexer->lookahead == '*') {
      advance(lexer);
      if (lexer->lookahead == '*') {
        advance(lexer);
        return check_operator_end(lexer);
      } else {
        return check_operator_end(lexer);
      }
      // / //
    } else if (lexer->lookahead == '/') {
      advance(lexer);
      if (lexer->lookahead == '/') {
        advance(lexer);
        return check_operator_end(lexer);
      } else {
        return check_operator_end(lexer);
      }
      // ., ..
    } else if (lexer->lookahead == '.') {
      advance(lexer);
      if (lexer->lookahead == '.') {
        advance(lexer);
        // Ignore ... identifier
        if (lexer->lookahead == '.')
          return false;
        return check_operator_end(lexer);
      } else {
        return check_operator_end(lexer);
      }
      // double slash
    } else if (lexer->lookahead == '\\') {
      advance(lexer);
      if (lexer->lookahead == '\\') {
        advance(lexer);
        return check_operator_end(lexer);
      }
      // when
    } else if (lexer->lookahead == 'w') {
      advance(lexer);
      if (lexer->lookahead == 'h') {
        advance(lexer);
        if (lexer->lookahead == 'e') {
          advance(lexer);
          if (lexer->lookahead == 'n') {
            advance(lexer);
            return is_token_end(lexer->lookahead) && check_operator_end(lexer);
          }
        }
      }
      // and
    } else if (lexer->lookahead == 'a') {
      advance(lexer);
      if (lexer->lookahead == 'n') {
        advance(lexer);
        if (lexer->lookahead == 'd') {
          advance(lexer);
          return is_token_end(lexer->lookahead) && check_operator_end(lexer);
        }
      }
      // or
    } else if (lexer->lookahead == 'o') {
      advance(lexer);
      if (lexer->lookahead == 'r') {
        advance(lexer);
        return is_token_end(lexer->lookahead) && check_operator_end(lexer);
      }
      // in
    } else if (lexer->lookahead == 'i') {
      advance(lexer);
      if (lexer->lookahead == 'n') {
        advance(lexer);
        return is_token_end(lexer->lookahead) && check_operator_end(lexer);
      }
      // not in
    } else if (lexer->lookahead == 'n') {
      advance(lexer);
      if (lexer->lookahead == 'o') {
        advance(lexer);
        if (lexer->lookahead == 't') {
          advance(lexer);
          while (is_inline_whitespace(lexer->lookahead)) {
            advance(lexer);
          }
          if (lexer->lookahead == 'i') {
            advance(lexer);
            if (lexer->lookahead == 'n') {
              advance(lexer);
              return is_token_end(lexer->lookahead) &&
                     check_operator_end(lexer);
            }
          }
        }
      }
    }
  }

  return false;
}

static bool scan(TSLexer *lexer, const bool *valid_symbols) {
  int8_t quoted_content_info_idx = find_quoted_token_info(valid_symbols);

  // Quoted content, which matches any character except for close
  // delimiters, escapes and interpolations
  if (quoted_content_info_idx != -1) {
    const QuotedContentInfo info =
        quoted_content_infos[quoted_content_info_idx];
    return scan_quoted_content(lexer, &info);
  }

  bool skipped_whitespace = false;

  while (is_inline_whitespace(lexer->lookahead)) {
    skipped_whitespace = true;
    skip(lexer);
  }

  // Newline, which is either tokenized as a special newline or ignored
  if (is_newline(lexer->lookahead) &&
      (valid_symbols[NEWLINE_BEFORE_DO] ||
       valid_symbols[NEWLINE_BEFORE_BINARY_OPERATOR] ||
       valid_symbols[NEWLINE_BEFORE_COMMENT])) {
    return scan_newline(lexer, valid_symbols);
  }

  // before unary +
  if (lexer->lookahead == '+') {
    if (skipped_whitespace && valid_symbols[BEFORE_UNARY_OPERATOR]) {
      lexer->mark_end(lexer);
      advance(lexer);
      if (lexer->lookahead == '+' || lexer->lookahead == ':' ||
          lexer->lookahead == '/') {
        return false;
      }
      if (is_whitespace(lexer->lookahead)) {
        return false;
      }
      lexer->result_symbol = BEFORE_UNARY_OPERATOR;
      return true;
    }
    // before unary -
  } else if (lexer->lookahead == '-') {
    if (skipped_whitespace && valid_symbols[BEFORE_UNARY_OPERATOR]) {
      lexer->mark_end(lexer);
      lexer->result_symbol = BEFORE_UNARY_OPERATOR;
      advance(lexer);
      if (lexer->lookahead == '-' || lexer->lookahead == '>' ||
          lexer->lookahead == ':' || lexer->lookahead == '/') {
        return false;
      }
      if (is_whitespace(lexer->lookahead)) {
        return false;
      }
      return true;
    }
    // not in
  } else if (lexer->lookahead == 'n') {
    if (valid_symbols[NOT_IN]) {
      lexer->result_symbol = NOT_IN;
      advance(lexer);
      if (lexer->lookahead == 'o') {
        advance(lexer);
        if (lexer->lookahead == 't') {
          advance(lexer);
          while (is_inline_whitespace(lexer->lookahead)) {
            advance(lexer);
          }
          if (lexer->lookahead == 'i') {
            advance(lexer);
            if (lexer->lookahead == 'n') {
              advance(lexer);
              return is_token_end(lexer->lookahead);
            }
          }
        }
      }
    }
    // quoted atom start
  } else if (lexer->lookahead == ':') {
    if (valid_symbols[QUOTED_ATOM_START]) {
      advance(lexer);
      lexer->mark_end(lexer);
      lexer->result_symbol = QUOTED_ATOM_START;
      if (lexer->lookahead == '"' || lexer->lookahead == '\'') {
        return true;
      }
    }
  }

  return false;
}

void *tree_sitter_elixir_external_scanner_create() { return NULL; }

bool tree_sitter_elixir_external_scanner_scan(void *payload, TSLexer *lexer,
                                              const bool *valid_symbols) {
  return scan(lexer, valid_symbols);
}

unsigned tree_sitter_elixir_external_scanner_serialize(void *payload,
                                                       char *buffer) {
  return 0;
}

void tree_sitter_elixir_external_scanner_deserialize(void *payload,
                                                     const char *buffer,
                                                     unsigned length) {}

void tree_sitter_elixir_external_scanner_destroy(void *payload) {}
