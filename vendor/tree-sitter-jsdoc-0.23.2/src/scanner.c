#include "tree_sitter/parser.h"

enum TokenType { TYPE_TOKEN };

void *tree_sitter_jsdoc_external_scanner_create() { return NULL; }

void tree_sitter_jsdoc_external_scanner_destroy(void *payload) {}

unsigned tree_sitter_jsdoc_external_scanner_serialize(void *payload, char *buffer) { return 0; }

void tree_sitter_jsdoc_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {}

// Scan to the next balanced `}` character.
static bool scan_for_type(TSLexer *lexer) {
    int stack = 0;
    while (true) {
        if (lexer->eof(lexer)) {
            return false;
        }
        switch (lexer->lookahead) {
            case '{':
                stack++;
                break;
            case '}':
                stack--;
                if (stack == -1) {
                    return true;
                }
                break;
            case '\n':
            case '\0': // fallthrough
                // Something's gone wrong.
                return false;
            default:;
        }
        lexer->advance(lexer, false);
    }
}

bool tree_sitter_jsdoc_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
    if (valid_symbols[TYPE_TOKEN] && scan_for_type(lexer)) {
        lexer->result_symbol = TYPE_TOKEN;
        lexer->mark_end(lexer);
        return true;
    }

    return false;
}
