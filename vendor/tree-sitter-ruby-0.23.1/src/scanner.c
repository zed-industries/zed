#include "tree_sitter/alloc.h"
#include "tree_sitter/array.h"
#include "tree_sitter/parser.h"

#include <string.h>
#include <wctype.h>

typedef enum {
    LINE_BREAK,
    NO_LINE_BREAK,

    // Delimited literals
    SIMPLE_SYMBOL,
    STRING_START,
    SYMBOL_START,
    SUBSHELL_START,
    REGEX_START,
    STRING_ARRAY_START,
    SYMBOL_ARRAY_START,
    HEREDOC_BODY_START,
    STRING_CONTENT,
    HEREDOC_CONTENT,
    STRING_END,
    HEREDOC_BODY_END,
    HEREDOC_START,

    // Whitespace-sensitive tokens
    FORWARD_SLASH,
    BLOCK_AMPERSAND,
    SPLAT_STAR,
    UNARY_MINUS,
    UNARY_MINUS_NUM,
    BINARY_MINUS,
    BINARY_STAR,
    SINGLETON_CLASS_LEFT_ANGLE_LEFT_ANGLE,
    HASH_KEY_SYMBOL,
    IDENTIFIER_SUFFIX,
    CONSTANT_SUFFIX,
    HASH_SPLAT_STAR_STAR,
    BINARY_STAR_STAR,
    ELEMENT_REFERENCE_BRACKET,
    SHORT_INTERPOLATION,

    NONE
} TokenType;

typedef Array(char) String;

typedef struct {
    TokenType type;
    int32_t open_delimiter;
    int32_t close_delimiter;
    int32_t nesting_depth;
    bool allows_interpolation;
} Literal;

typedef struct {
    String word;
    bool end_word_indentation_allowed;
    bool allows_interpolation;
    bool started;
} Heredoc;

typedef struct {
    bool has_leading_whitespace;
    Array(Literal) literal_stack;
    Array(Heredoc) open_heredocs;
} Scanner;

const char NON_IDENTIFIER_CHARS[] = {
    '\0', '\n', '\r', '\t', ' ', ':', ';', '`',  '"', '\'', '@', '$', '#', '.', ',', '|', '^', '&',
    '<',  '=',  '>',  '+',  '-', '*', '/', '\\', '%', '?',  '!', '~', '(', ')', '[', ']', '{', '}',
};

static inline void skip(Scanner *scanner, TSLexer *lexer) {
    scanner->has_leading_whitespace = true;
    lexer->advance(lexer, true);
}

static inline void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static inline void reset(Scanner *scanner) {
    array_delete(&scanner->literal_stack);
    for (uint32_t i = 0; i < scanner->open_heredocs.size; i++) {
        array_delete(&array_get(&scanner->open_heredocs, i)->word);
    }
    array_delete(&scanner->open_heredocs);
}

static inline unsigned serialize(Scanner *scanner, char *buffer) {
    unsigned size = 0;

    if (scanner->literal_stack.size * 5 + 2 >= TREE_SITTER_SERIALIZATION_BUFFER_SIZE) {
        return 0;
    }

    buffer[size++] = (char)scanner->literal_stack.size;
    for (uint32_t i = 0; i < scanner->literal_stack.size; i++) {
        Literal *literal = array_get(&scanner->literal_stack, i);
        buffer[size++] = literal->type;
        buffer[size++] = (char)literal->open_delimiter;
        buffer[size++] = (char)literal->close_delimiter;
        buffer[size++] = (char)literal->nesting_depth;
        buffer[size++] = (char)literal->allows_interpolation;
    }

    buffer[size++] = (char)scanner->open_heredocs.size;
    for (uint32_t i = 0; i < scanner->open_heredocs.size; i++) {
        Heredoc *heredoc = array_get(&scanner->open_heredocs, i);
        if (size + 2 + heredoc->word.size >= TREE_SITTER_SERIALIZATION_BUFFER_SIZE) {
            return 0;
        }
        buffer[size++] = (char)heredoc->end_word_indentation_allowed;
        buffer[size++] = (char)heredoc->allows_interpolation;
        buffer[size++] = (char)heredoc->started;
        buffer[size++] = (char)heredoc->word.size;
        memcpy(&buffer[size], heredoc->word.contents, heredoc->word.size);
        size += heredoc->word.size;
    }

    return size;
}

static inline void deserialize(Scanner *scanner, const char *buffer, unsigned length) {
    unsigned size = 0;
    scanner->has_leading_whitespace = false;
    reset(scanner);

    if (length == 0) {
        return;
    }

    uint8_t literal_depth = buffer[size++];
    for (unsigned j = 0; j < literal_depth; j++) {
        Literal literal = {0};
        literal.type = (TokenType)(buffer[size++]);
        literal.open_delimiter = (unsigned char)buffer[size++];
        literal.close_delimiter = (unsigned char)buffer[size++];
        literal.nesting_depth = (unsigned char)buffer[size++];
        literal.allows_interpolation = buffer[size++];
        array_push(&scanner->literal_stack, literal);
    }

    uint8_t open_heredoc_count = buffer[size++];
    for (unsigned j = 0; j < open_heredoc_count; j++) {
        Heredoc heredoc = {0};
        heredoc.end_word_indentation_allowed = buffer[size++];
        heredoc.allows_interpolation = buffer[size++];
        heredoc.started = buffer[size++];

        heredoc.word = (String)array_new();
        uint8_t word_length = buffer[size++];
        array_reserve(&heredoc.word, word_length);
        memcpy(heredoc.word.contents, &buffer[size], word_length);
        heredoc.word.size = word_length;
        size += word_length;
        array_push(&scanner->open_heredocs, heredoc);
    }

    assert(size == length);
}

static inline bool scan_whitespace(Scanner *scanner, TSLexer *lexer, const bool *valid_symbols) {
    bool heredoc_body_start_is_valid = scanner->open_heredocs.size > 0 && !scanner->open_heredocs.contents[0].started &&
                                       valid_symbols[HEREDOC_BODY_START];
    bool crossed_newline = false;

    for (;;) {
        if (!valid_symbols[NO_LINE_BREAK] && valid_symbols[LINE_BREAK] && lexer->is_at_included_range_start(lexer)) {
            lexer->mark_end(lexer);
            lexer->result_symbol = LINE_BREAK;
            return true;
        }

        switch (lexer->lookahead) {
            case ' ':
            case '\t':
                skip(scanner, lexer);
                break;
            case '\r':
                if (heredoc_body_start_is_valid) {
                    lexer->result_symbol = HEREDOC_BODY_START;
                    scanner->open_heredocs.contents[0].started = true;
                    return true;
                } else {
                    skip(scanner, lexer);
                    break;
                }
            case '\n':
                if (heredoc_body_start_is_valid) {
                    lexer->result_symbol = HEREDOC_BODY_START;
                    scanner->open_heredocs.contents[0].started = true;
                    return true;
                } else if (!valid_symbols[NO_LINE_BREAK] && valid_symbols[LINE_BREAK] && !crossed_newline) {
                    lexer->mark_end(lexer);
                    advance(lexer);
                    crossed_newline = true;
                } else {
                    skip(scanner, lexer);
                }
                break;
            case '\\':
                advance(lexer);
                if (lexer->lookahead == '\r') {
                    skip(scanner, lexer);
                }
                if (iswspace(lexer->lookahead)) {
                    skip(scanner, lexer);
                } else {
                    return false;
                }
                break;
            default:
                if (crossed_newline) {
                    if (lexer->lookahead != '.' && lexer->lookahead != '&' && lexer->lookahead != '#') {
                        lexer->result_symbol = LINE_BREAK;
                    } else if (lexer->lookahead == '.') {
                        // Don't return LINE_BREAK for the call operator (`.`) but do return one for range
                        // operators
                        // (`..` and `...`)
                        advance(lexer);
                        if (!lexer->eof(lexer) && lexer->lookahead == '.') {
                            lexer->result_symbol = LINE_BREAK;
                        } else {
                            return false;
                        }
                    }
                }
                return true;
        }
    }
}

static inline bool scan_operator(TSLexer *lexer) {
    switch (lexer->lookahead) {
        // <, <=, <<, <=>
        case '<':
            advance(lexer);
            if (lexer->lookahead == '<') {
                advance(lexer);
            } else if (lexer->lookahead == '=') {
                advance(lexer);
                if (lexer->lookahead == '>') {
                    advance(lexer);
                }
            }
            return true;

        // >, >=, >>
        case '>':
            advance(lexer);
            if (lexer->lookahead == '>' || lexer->lookahead == '=') {
                advance(lexer);
            }
            return true;

        // ==, ===, =~
        case '=':
            advance(lexer);
            if (lexer->lookahead == '~') {
                advance(lexer);
                return true;
            }
            if (lexer->lookahead == '=') {
                advance(lexer);
                if (lexer->lookahead == '=') {
                    advance(lexer);
                }
                return true;
            }
            return false;

        // +, -, ~, +@, -@, ~@
        case '+':
        case '-':
        case '~':
            advance(lexer);
            if (lexer->lookahead == '@') {
                advance(lexer);
            }
            return true;

        // ..
        case '.':
            advance(lexer);
            if (lexer->lookahead == '.') {
                advance(lexer);
                return true;
            }
            return false;

        // &, ^, |, /, %`
        case '&':
        case '^':
        case '|':
        case '/':
        case '%':
        case '`':
            advance(lexer);
            return true;

        // !, !=, !~
        case '!':
            advance(lexer);
            if (lexer->lookahead == '=' || lexer->lookahead == '~') {
                advance(lexer);
            }
            return true;

        // *, **
        case '*':
            advance(lexer);
            if (lexer->lookahead == '*') {
                advance(lexer);
            }
            return true;

        // [], []=
        case '[':
            advance(lexer);
            if (lexer->lookahead == ']') {
                advance(lexer);
            } else {
                return false;
            }
            if (lexer->lookahead == '=') {
                advance(lexer);
            }
            return true;

        default:
            return false;
    }
}

static inline bool is_iden_char(char c) {
    return memchr(&NON_IDENTIFIER_CHARS, c, sizeof(NON_IDENTIFIER_CHARS)) == NULL;
}

static inline bool scan_symbol_identifier(TSLexer *lexer) {
    if (lexer->lookahead == '@') {
        advance(lexer);
        if (lexer->lookahead == '@') {
            advance(lexer);
        }
    } else if (lexer->lookahead == '$') {
        advance(lexer);
    }

    if (is_iden_char((char)lexer->lookahead)) {
        advance(lexer);
    } else if (!scan_operator(lexer)) {
        return false;
    }

    while (is_iden_char((char)lexer->lookahead)) {
        advance(lexer);
    }

    if (lexer->lookahead == '?' || lexer->lookahead == '!') {
        advance(lexer);
    }

    if (lexer->lookahead == '=') {
        lexer->mark_end(lexer);
        advance(lexer);
        if (lexer->lookahead != '>') {
            lexer->mark_end(lexer);
        }
    }

    return true;
}

static inline bool scan_open_delimiter(Scanner *scanner, TSLexer *lexer, Literal *literal, const bool *valid_symbols) {
    switch (lexer->lookahead) {
        case '"':
            literal->type = STRING_START;
            literal->open_delimiter = literal->close_delimiter = lexer->lookahead;
            literal->allows_interpolation = true;
            advance(lexer);
            return true;

        case '\'':
            literal->type = STRING_START;
            literal->open_delimiter = literal->close_delimiter = lexer->lookahead;
            literal->allows_interpolation = false;
            advance(lexer);
            return true;

        case '`':
            if (!valid_symbols[SUBSHELL_START]) {
                return false;
            }
            literal->type = SUBSHELL_START;
            literal->open_delimiter = literal->close_delimiter = lexer->lookahead;
            literal->allows_interpolation = true;
            advance(lexer);
            return true;

        case '/':
            if (!valid_symbols[REGEX_START]) {
                return false;
            }
            literal->type = REGEX_START;
            literal->open_delimiter = literal->close_delimiter = lexer->lookahead;
            literal->allows_interpolation = true;
            advance(lexer);
            if (valid_symbols[FORWARD_SLASH]) {
                if (!scanner->has_leading_whitespace) {
                    return false;
                }
                if (lexer->lookahead == ' ' || lexer->lookahead == '\t' || lexer->lookahead == '\n' ||
                    lexer->lookahead == '\r') {
                    return false;
                }
                if (lexer->lookahead == '=') {
                    return false;
                }
            }
            return true;

        case '%':
            advance(lexer);

            switch (lexer->lookahead) {
                case 's':
                    if (!valid_symbols[SIMPLE_SYMBOL]) {
                        return false;
                    }
                    literal->type = SYMBOL_START;
                    literal->allows_interpolation = false;
                    advance(lexer);
                    break;

                case 'r':
                    if (!valid_symbols[REGEX_START]) {
                        return false;
                    }
                    literal->type = REGEX_START;
                    literal->allows_interpolation = true;
                    advance(lexer);
                    break;

                case 'x':
                    if (!valid_symbols[SUBSHELL_START]) {
                        return false;
                    }
                    literal->type = SUBSHELL_START;
                    literal->allows_interpolation = true;
                    advance(lexer);
                    break;

                case 'q':
                    if (!valid_symbols[STRING_START]) {
                        return false;
                    }
                    literal->type = STRING_START;
                    literal->allows_interpolation = false;
                    advance(lexer);
                    break;

                case 'Q':
                    if (!valid_symbols[STRING_START]) {
                        return false;
                    }
                    literal->type = STRING_START;
                    literal->allows_interpolation = true;
                    advance(lexer);
                    break;

                case 'w':
                    if (!valid_symbols[STRING_ARRAY_START]) {
                        return false;
                    }
                    literal->type = STRING_ARRAY_START;
                    literal->allows_interpolation = false;
                    advance(lexer);
                    break;

                case 'i':
                    if (!valid_symbols[SYMBOL_ARRAY_START]) {
                        return false;
                    }
                    literal->type = SYMBOL_ARRAY_START;
                    literal->allows_interpolation = false;
                    advance(lexer);
                    break;

                case 'W':
                    if (!valid_symbols[STRING_ARRAY_START]) {
                        return false;
                    }
                    literal->type = STRING_ARRAY_START;
                    literal->allows_interpolation = true;
                    advance(lexer);
                    break;

                case 'I':
                    if (!valid_symbols[SYMBOL_ARRAY_START]) {
                        return false;
                    }
                    literal->type = SYMBOL_ARRAY_START;
                    literal->allows_interpolation = true;
                    advance(lexer);
                    break;

                default:
                    if (!valid_symbols[STRING_START]) {
                        return false;
                    }
                    literal->type = STRING_START;
                    literal->allows_interpolation = true;
                    break;
            }

            switch (lexer->lookahead) {
                case '(':
                    literal->open_delimiter = '(';
                    literal->close_delimiter = ')';
                    break;

                case '[':
                    literal->open_delimiter = '[';
                    literal->close_delimiter = ']';
                    break;

                case '{':
                    literal->open_delimiter = '{';
                    literal->close_delimiter = '}';
                    break;

                case '<':
                    literal->open_delimiter = '<';
                    literal->close_delimiter = '>';
                    break;

                case '\r':
                case '\n':
                case ' ':
                case '\t':
                    // If the `/` operator is valid, then so is the `%` operator, which means
                    // that a `%` followed by whitespace should be considered an operator,
                    // not a percent string.
                    if (valid_symbols[FORWARD_SLASH]) {
                        return false;
                    }
                    break;

                case '|':
                case '!':
                case '#':
                case '/':
                case '\\':
                case '@':
                case '$':
                case '%':
                case '^':
                case '&':
                case '*':
                case ')':
                case ']':
                case '}':
                case '>':
                // TODO: Implement %= as external rule and re-enable = as a valid
                // unbalanced delimiter. That will be necessary due to ambiguity
                // between &= assignment operator and %=...= as string
                // content delimiter.
                // case '=':
                case '+':
                case '-':
                case '~':
                case '`':
                case ',':
                case '.':
                case '?':
                case ':':
                case ';':
                case '_':
                case '"':
                case '\'':
                    literal->open_delimiter = lexer->lookahead;
                    literal->close_delimiter = lexer->lookahead;
                    break;
                default:
                    return false;
            }

            advance(lexer);
            return true;

        default:
            return false;
    }
}

static inline void scan_heredoc_word(TSLexer *lexer, Heredoc *heredoc) {
    String word = array_new();
    int32_t quote = 0;

    switch (lexer->lookahead) {
        case '\'':
        case '"':
        case '`':
            quote = lexer->lookahead;
            advance(lexer);
            while (lexer->lookahead != quote && !lexer->eof(lexer)) {
                array_push(&word, lexer->lookahead);
                advance(lexer);
            }
            advance(lexer);
            break;

        default:
            if (iswalnum(lexer->lookahead) || lexer->lookahead == '_') {
                array_push(&word, lexer->lookahead);
                advance(lexer);
                while (iswalnum(lexer->lookahead) || lexer->lookahead == '_') {
                    array_push(&word, lexer->lookahead);
                    advance(lexer);
                }
            }
            break;
    }

    heredoc->word = word;
    heredoc->allows_interpolation = quote != '\'';
}

static inline bool scan_short_interpolation(TSLexer *lexer, const bool has_content, const TSSymbol content_symbol) {
    char start = (char)lexer->lookahead;
    if (start == '@' || start == '$') {
        if (has_content) {
            lexer->result_symbol = content_symbol;
            return true;
        }
        lexer->mark_end(lexer);
        advance(lexer);
        bool is_short_interpolation = false;
        if (start == '$') {
            if (strchr("!@&`'+~=/\\,;.<>*$?:\"", lexer->lookahead) != NULL) {
                is_short_interpolation = true;
            } else {
                if (lexer->lookahead == '-') {
                    advance(lexer);
                    is_short_interpolation = iswalpha(lexer->lookahead) || lexer->lookahead == '_';
                } else {
                    is_short_interpolation = iswalnum(lexer->lookahead) || lexer->lookahead == '_';
                }
            }
        }
        if (start == '@') {
            if (lexer->lookahead == '@') {
                advance(lexer);
            }
            is_short_interpolation = is_iden_char((char)lexer->lookahead) && !iswdigit(lexer->lookahead);
        }

        if (is_short_interpolation) {
            lexer->result_symbol = SHORT_INTERPOLATION;
            return true;
        }
    }
    return false;
}

static inline bool scan_heredoc_content(Scanner *scanner, TSLexer *lexer) {
    Heredoc *heredoc = array_get(&scanner->open_heredocs, 0);
    size_t position_in_word = 0;
    bool look_for_heredoc_end = true;
    bool has_content = false;

    for (;;) {
        if (position_in_word == heredoc->word.size) {
            if (!has_content) {
                lexer->mark_end(lexer);
            }
            while (lexer->lookahead == ' ' || lexer->lookahead == '\t') {
                advance(lexer);
            }
            if (lexer->lookahead == '\n' || lexer->lookahead == '\r') {
                if (has_content) {
                    lexer->result_symbol = HEREDOC_CONTENT;
                } else {
                    array_delete(&heredoc->word);
                    array_erase(&scanner->open_heredocs, 0);
                    lexer->result_symbol = HEREDOC_BODY_END;
                }
                return true;
            }
            has_content = true;
            position_in_word = 0;
        }

        if (lexer->eof(lexer)) {
            lexer->mark_end(lexer);
            if (has_content) {
                lexer->result_symbol = HEREDOC_CONTENT;
            } else {
                array_delete(&heredoc->word);
                array_erase(&scanner->open_heredocs, 0);
                lexer->result_symbol = HEREDOC_BODY_END;
            }
            return true;
        }

        if (lexer->lookahead == *array_get(&heredoc->word, position_in_word) && look_for_heredoc_end) {
            advance(lexer);
            position_in_word++;
        } else {
            position_in_word = 0;
            look_for_heredoc_end = false;

            if (heredoc->allows_interpolation && lexer->lookahead == '\\') {
                if (has_content) {
                    lexer->result_symbol = HEREDOC_CONTENT;
                    return true;
                }
                return false;
            }

            if (heredoc->allows_interpolation && lexer->lookahead == '#') {
                lexer->mark_end(lexer);
                advance(lexer);
                if (lexer->lookahead == '{') {
                    if (has_content) {
                        lexer->result_symbol = HEREDOC_CONTENT;
                        return true;
                    }
                    return false;
                }
                if (scan_short_interpolation(lexer, has_content, HEREDOC_CONTENT)) {
                    return true;
                }
            } else if (lexer->lookahead == '\r' || lexer->lookahead == '\n') {
                if (lexer->lookahead == '\r') {
                    advance(lexer);
                    if (lexer->lookahead == '\n') {
                        advance(lexer);
                    }
                } else {
                    advance(lexer);
                }
                has_content = true;
                look_for_heredoc_end = true;
                while (lexer->lookahead == ' ' || lexer->lookahead == '\t') {
                    advance(lexer);
                    if (!heredoc->end_word_indentation_allowed) {
                        look_for_heredoc_end = false;
                    }
                }
                lexer->mark_end(lexer);
            } else {
                has_content = true;
                advance(lexer);
                lexer->mark_end(lexer);
            }
        }
    }
}

static inline bool scan_literal_content(Scanner *scanner, TSLexer *lexer) {
    Literal *literal = array_back(&scanner->literal_stack);
    bool has_content = false;
    bool stop_on_space = literal->type == SYMBOL_ARRAY_START || literal->type == STRING_ARRAY_START;

    for (;;) {
        if (stop_on_space && iswspace(lexer->lookahead)) {
            if (has_content) {
                lexer->mark_end(lexer);
                lexer->result_symbol = STRING_CONTENT;
                return true;
            }
            return false;
        }
        if (lexer->lookahead == literal->close_delimiter) {
            lexer->mark_end(lexer);
            if (literal->nesting_depth == 1) {
                if (has_content) {
                    lexer->result_symbol = STRING_CONTENT;
                } else {
                    advance(lexer);
                    if (literal->type == REGEX_START) {
                        while (iswlower(lexer->lookahead)) {
                            advance(lexer);
                        }
                    }
                    array_pop(&scanner->literal_stack);
                    lexer->result_symbol = STRING_END;
                    lexer->mark_end(lexer);
                }
                return true;
            }
            literal->nesting_depth--;
            advance(lexer);

        } else if (lexer->lookahead == literal->open_delimiter) {
            literal->nesting_depth++;
            advance(lexer);
        } else if (literal->allows_interpolation && lexer->lookahead == '#') {
            lexer->mark_end(lexer);
            advance(lexer);
            if (lexer->lookahead == '{') {
                if (has_content) {
                    lexer->result_symbol = STRING_CONTENT;
                    return true;
                }
                return false;
            }
            if (scan_short_interpolation(lexer, has_content, STRING_CONTENT)) {
                return true;
            }
        } else if (lexer->lookahead == '\\') {
            if (literal->allows_interpolation) {
                if (has_content) {
                    lexer->mark_end(lexer);
                    lexer->result_symbol = STRING_CONTENT;
                    return true;
                }
                return false;
            }
            advance(lexer);
            advance(lexer);

        } else if (lexer->eof(lexer)) {
            advance(lexer);
            lexer->mark_end(lexer);
            return false;
        } else {
            advance(lexer);
        }

        has_content = true;
    }
}

static inline bool scan(Scanner *scanner, TSLexer *lexer, const bool *valid_symbols) {
    scanner->has_leading_whitespace = false;

    // Contents of literals, which match any character except for some close delimiter
    if (!valid_symbols[STRING_START]) {
        if ((valid_symbols[STRING_CONTENT] || valid_symbols[STRING_END]) && scanner->literal_stack.size > 0) {
            return scan_literal_content(scanner, lexer);
        }
        if ((valid_symbols[HEREDOC_CONTENT] || valid_symbols[HEREDOC_BODY_END]) && scanner->open_heredocs.size > 0) {
            return scan_heredoc_content(scanner, lexer);
        }
    }

    // Whitespace
    lexer->result_symbol = NONE;
    if (!scan_whitespace(scanner, lexer, valid_symbols)) {
        return false;
    }
    if (lexer->result_symbol != NONE) {
        return true;
    }

    switch (lexer->lookahead) {
        case '&':
            if (valid_symbols[BLOCK_AMPERSAND]) {
                advance(lexer);
                if (lexer->lookahead != '&' && lexer->lookahead != '.' && lexer->lookahead != '=' &&
                    !iswspace(lexer->lookahead)) {
                    lexer->result_symbol = BLOCK_AMPERSAND;
                    return true;
                }
                return false;
            }
            break;

        case '<':
            if (valid_symbols[SINGLETON_CLASS_LEFT_ANGLE_LEFT_ANGLE]) {
                advance(lexer);
                if (lexer->lookahead == '<') {
                    advance(lexer);
                    lexer->result_symbol = SINGLETON_CLASS_LEFT_ANGLE_LEFT_ANGLE;
                    return true;
                }
                return false;
            }
            break;

        case '*':
            if (valid_symbols[SPLAT_STAR] || valid_symbols[BINARY_STAR] || valid_symbols[HASH_SPLAT_STAR_STAR] ||
                valid_symbols[BINARY_STAR_STAR]) {
                advance(lexer);
                if (lexer->lookahead == '=') {
                    return false;
                }
                if (lexer->lookahead == '*') {
                    if (valid_symbols[HASH_SPLAT_STAR_STAR] || valid_symbols[BINARY_STAR_STAR]) {
                        advance(lexer);
                        if (lexer->lookahead == '=') {
                            return false;
                        }
                        if (valid_symbols[BINARY_STAR_STAR] && !scanner->has_leading_whitespace) {
                            lexer->result_symbol = BINARY_STAR_STAR;
                            return true;
                        }
                        if (valid_symbols[HASH_SPLAT_STAR_STAR] && !iswspace(lexer->lookahead)) {
                            lexer->result_symbol = HASH_SPLAT_STAR_STAR;
                            return true;
                        }
                        if (valid_symbols[BINARY_STAR_STAR]) {
                            lexer->result_symbol = BINARY_STAR_STAR;
                            return true;
                        }
                        if (valid_symbols[HASH_SPLAT_STAR_STAR]) {
                            lexer->result_symbol = HASH_SPLAT_STAR_STAR;
                            return true;
                        }
                        return false;
                    }
                    return false;
                }
                if (valid_symbols[BINARY_STAR] && !scanner->has_leading_whitespace) {
                    lexer->result_symbol = BINARY_STAR;
                    return true;
                }
                if (valid_symbols[SPLAT_STAR] && !iswspace(lexer->lookahead)) {
                    lexer->result_symbol = SPLAT_STAR;
                    return true;
                }
                if (valid_symbols[BINARY_STAR]) {
                    lexer->result_symbol = BINARY_STAR;
                    return true;
                }
                if (valid_symbols[SPLAT_STAR]) {
                    lexer->result_symbol = SPLAT_STAR;
                    return true;
                }
                return false;
            }
            break;

        case '-':
            if (valid_symbols[UNARY_MINUS] || valid_symbols[UNARY_MINUS_NUM] || valid_symbols[BINARY_MINUS]) {
                advance(lexer);
                if (lexer->lookahead != '=' && lexer->lookahead != '>') {
                    if (valid_symbols[UNARY_MINUS_NUM] &&
                        (!valid_symbols[BINARY_STAR] || scanner->has_leading_whitespace) &&
                        iswdigit(lexer->lookahead)) {
                        lexer->result_symbol = UNARY_MINUS_NUM;
                        return true;
                    }
                    if (valid_symbols[UNARY_MINUS] && scanner->has_leading_whitespace && !iswspace(lexer->lookahead)) {
                        lexer->result_symbol = UNARY_MINUS;
                    } else if (valid_symbols[BINARY_MINUS]) {
                        lexer->result_symbol = BINARY_MINUS;
                    } else {
                        lexer->result_symbol = UNARY_MINUS;
                    }
                    return true;
                }
                return false;
            }
            break;

        case ':':
            if (valid_symbols[SYMBOL_START]) {
                Literal literal = {0};
                literal.type = SYMBOL_START;
                literal.nesting_depth = 1;
                advance(lexer);

                switch (lexer->lookahead) {
                    case '"':
                        advance(lexer);
                        literal.open_delimiter = '"';
                        literal.close_delimiter = '"';
                        literal.allows_interpolation = true;
                        array_push(&scanner->literal_stack, literal);
                        lexer->result_symbol = SYMBOL_START;
                        return true;

                    case '\'':
                        advance(lexer);
                        literal.open_delimiter = '\'';
                        literal.close_delimiter = '\'';
                        literal.allows_interpolation = false;
                        array_push(&scanner->literal_stack, literal);
                        lexer->result_symbol = SYMBOL_START;
                        return true;

                    default:
                        if (scan_symbol_identifier(lexer)) {
                            lexer->result_symbol = SIMPLE_SYMBOL;
                            return true;
                        }
                }

                return false;
            }
            break;

        case '[':
            // Treat a square bracket as an element reference if either:
            // * the bracket is not preceded by any whitespace
            // * an arbitrary expression is not valid at the current position.
            if (valid_symbols[ELEMENT_REFERENCE_BRACKET] &&
                (!scanner->has_leading_whitespace || !valid_symbols[STRING_START])) {
                advance(lexer);
                lexer->result_symbol = ELEMENT_REFERENCE_BRACKET;
                return true;
            }
            break;

        default:
            break;
    }

    // Open delimiters for literals
    if (((valid_symbols[HASH_KEY_SYMBOL] || valid_symbols[IDENTIFIER_SUFFIX]) &&
         (iswalpha(lexer->lookahead) || lexer->lookahead == '_')) ||
        (valid_symbols[CONSTANT_SUFFIX] && iswupper(lexer->lookahead))) {
        TokenType validIdentifierSymbol = iswupper(lexer->lookahead) ? CONSTANT_SUFFIX : IDENTIFIER_SUFFIX;
        while (iswalnum(lexer->lookahead) || lexer->lookahead == '_') {
            advance(lexer);
        }

        if (valid_symbols[HASH_KEY_SYMBOL] && lexer->lookahead == ':') {
            lexer->mark_end(lexer);
            advance(lexer);
            if (lexer->lookahead != ':') {
                lexer->result_symbol = HASH_KEY_SYMBOL;
                return true;
            }
        } else if (valid_symbols[validIdentifierSymbol] && lexer->lookahead == '!') {
            advance(lexer);
            if (lexer->lookahead != '=') {
                lexer->result_symbol = validIdentifierSymbol;
                return true;
            }
        }

        return false;
    }

    // Open delimiters for literals
    if (valid_symbols[STRING_START]) {
        Literal literal = {0};
        literal.nesting_depth = 1;

        if (lexer->lookahead == '<') {
            advance(lexer);
            if (lexer->lookahead != '<') {
                return false;
            }
            advance(lexer);

            Heredoc heredoc = {0};
            if (lexer->lookahead == '-' || lexer->lookahead == '~') {
                advance(lexer);
                heredoc.end_word_indentation_allowed = true;
            }

            scan_heredoc_word(lexer, &heredoc);
            if (heredoc.word.size == 0) {
                array_delete(&heredoc.word);
                return false;
            }
            array_push(&scanner->open_heredocs, heredoc);
            lexer->result_symbol = HEREDOC_START;
            return true;
        }

        if (scan_open_delimiter(scanner, lexer, &literal, valid_symbols)) {
            array_push(&scanner->literal_stack, literal);
            lexer->result_symbol = literal.type;
            return true;
        }
        return false;
    }

    return false;
}

void *tree_sitter_ruby_external_scanner_create() {
    Scanner *scanner = (Scanner *)ts_calloc(1, sizeof(Scanner));
    return scanner;
}

bool tree_sitter_ruby_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
    Scanner *scanner = (Scanner *)payload;
    return scan(scanner, lexer, valid_symbols);
}

unsigned tree_sitter_ruby_external_scanner_serialize(void *payload, char *buffer) {
    Scanner *scanner = (Scanner *)payload;
    return serialize(scanner, buffer);
}

void tree_sitter_ruby_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
    Scanner *scanner = (Scanner *)payload;
    deserialize(scanner, buffer, length);
}

void tree_sitter_ruby_external_scanner_destroy(void *payload) {
    Scanner *scanner = (Scanner *)payload;
    for (uint32_t i = 0; i < scanner->open_heredocs.size; i++) {
        array_delete(&array_get(&scanner->open_heredocs, i)->word);
    }
    array_delete(&scanner->open_heredocs);
    array_delete(&scanner->literal_stack);
    ts_free(scanner);
}
