#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 95
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 34
#define ALIAS_COUNT 0
#define TOKEN_COUNT 15
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_raw_string_literal = 1,
  anon_sym_DQUOTE = 2,
  aux_sym_interpreted_string_literal_token1 = 3,
  sym_escape_sequence = 4,
  sym__identifier = 5,
  anon_sym_go = 6,
  anon_sym_LF = 7,
  anon_sym_replace = 8,
  anon_sym_LPAREN = 9,
  anon_sym_RPAREN = 10,
  anon_sym_EQ_GT = 11,
  anon_sym_use = 12,
  anon_sym_SLASH_SLASH = 13,
  aux_sym_comment_token1 = 14,
  sym_source_file = 15,
  sym__directive = 16,
  sym__string_literal = 17,
  sym_interpreted_string_literal = 18,
  sym__string_or_ident = 19,
  sym_module_path = 20,
  sym_go_version = 21,
  sym_version = 22,
  sym_go_directive = 23,
  sym_replace_directive = 24,
  sym_replace_spec = 25,
  sym_use_directive = 26,
  sym_use_spec = 27,
  sym_file_path = 28,
  sym_comment = 29,
  aux_sym_source_file_repeat1 = 30,
  aux_sym_interpreted_string_literal_repeat1 = 31,
  aux_sym_replace_directive_repeat1 = 32,
  aux_sym_use_directive_repeat1 = 33,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_raw_string_literal] = "raw_string_literal",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_interpreted_string_literal_token1] = "interpreted_string_literal_token1",
  [sym_escape_sequence] = "escape_sequence",
  [sym__identifier] = "_identifier",
  [anon_sym_go] = "go",
  [anon_sym_LF] = "\n",
  [anon_sym_replace] = "replace",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_EQ_GT] = "=>",
  [anon_sym_use] = "use",
  [anon_sym_SLASH_SLASH] = "//",
  [aux_sym_comment_token1] = "comment_token1",
  [sym_source_file] = "source_file",
  [sym__directive] = "_directive",
  [sym__string_literal] = "_string_literal",
  [sym_interpreted_string_literal] = "interpreted_string_literal",
  [sym__string_or_ident] = "_string_or_ident",
  [sym_module_path] = "module_path",
  [sym_go_version] = "go_version",
  [sym_version] = "version",
  [sym_go_directive] = "go_directive",
  [sym_replace_directive] = "replace_directive",
  [sym_replace_spec] = "replace_spec",
  [sym_use_directive] = "use_directive",
  [sym_use_spec] = "use_spec",
  [sym_file_path] = "file_path",
  [sym_comment] = "comment",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_interpreted_string_literal_repeat1] = "interpreted_string_literal_repeat1",
  [aux_sym_replace_directive_repeat1] = "replace_directive_repeat1",
  [aux_sym_use_directive_repeat1] = "use_directive_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_raw_string_literal] = sym_raw_string_literal,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_interpreted_string_literal_token1] = aux_sym_interpreted_string_literal_token1,
  [sym_escape_sequence] = sym_escape_sequence,
  [sym__identifier] = sym__identifier,
  [anon_sym_go] = anon_sym_go,
  [anon_sym_LF] = anon_sym_LF,
  [anon_sym_replace] = anon_sym_replace,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_EQ_GT] = anon_sym_EQ_GT,
  [anon_sym_use] = anon_sym_use,
  [anon_sym_SLASH_SLASH] = anon_sym_SLASH_SLASH,
  [aux_sym_comment_token1] = aux_sym_comment_token1,
  [sym_source_file] = sym_source_file,
  [sym__directive] = sym__directive,
  [sym__string_literal] = sym__string_literal,
  [sym_interpreted_string_literal] = sym_interpreted_string_literal,
  [sym__string_or_ident] = sym__string_or_ident,
  [sym_module_path] = sym_module_path,
  [sym_go_version] = sym_go_version,
  [sym_version] = sym_version,
  [sym_go_directive] = sym_go_directive,
  [sym_replace_directive] = sym_replace_directive,
  [sym_replace_spec] = sym_replace_spec,
  [sym_use_directive] = sym_use_directive,
  [sym_use_spec] = sym_use_spec,
  [sym_file_path] = sym_file_path,
  [sym_comment] = sym_comment,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_interpreted_string_literal_repeat1] = aux_sym_interpreted_string_literal_repeat1,
  [aux_sym_replace_directive_repeat1] = aux_sym_replace_directive_repeat1,
  [aux_sym_use_directive_repeat1] = aux_sym_use_directive_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_raw_string_literal] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_interpreted_string_literal_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_escape_sequence] = {
    .visible = true,
    .named = true,
  },
  [sym__identifier] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_go] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LF] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_replace] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_use] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_SLASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__directive] = {
    .visible = false,
    .named = true,
  },
  [sym__string_literal] = {
    .visible = false,
    .named = true,
  },
  [sym_interpreted_string_literal] = {
    .visible = true,
    .named = true,
  },
  [sym__string_or_ident] = {
    .visible = false,
    .named = true,
  },
  [sym_module_path] = {
    .visible = true,
    .named = true,
  },
  [sym_go_version] = {
    .visible = true,
    .named = true,
  },
  [sym_version] = {
    .visible = true,
    .named = true,
  },
  [sym_go_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_replace_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_replace_spec] = {
    .visible = true,
    .named = true,
  },
  [sym_use_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_use_spec] = {
    .visible = true,
    .named = true,
  },
  [sym_file_path] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_interpreted_string_literal_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_replace_directive_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_use_directive_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 9,
  [12] = 10,
  [13] = 8,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 14,
  [18] = 16,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 29,
  [33] = 30,
  [34] = 29,
  [35] = 29,
  [36] = 36,
  [37] = 30,
  [38] = 38,
  [39] = 39,
  [40] = 23,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 38,
  [50] = 50,
  [51] = 42,
  [52] = 30,
  [53] = 53,
  [54] = 54,
  [55] = 46,
  [56] = 27,
  [57] = 57,
  [58] = 45,
  [59] = 47,
  [60] = 48,
  [61] = 61,
  [62] = 39,
  [63] = 45,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 45,
  [71] = 48,
  [72] = 47,
  [73] = 46,
  [74] = 74,
  [75] = 48,
  [76] = 76,
  [77] = 47,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 66,
  [82] = 68,
  [83] = 76,
  [84] = 84,
  [85] = 84,
  [86] = 46,
  [87] = 80,
  [88] = 88,
  [89] = 79,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(33);
      ADVANCE_MAP(
        '"', 36,
        '(', 51,
        ')', 53,
        '/', 11,
        '=', 12,
        '\\', 13,
        '`', 14,
        'g', 21,
        'r', 17,
        'u', 23,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(32);
      END_STATE();
    case 1:
      if (lookahead == '\n') SKIP(4);
      if (lookahead == '"') ADVANCE(36);
      if (lookahead == '/') ADVANCE(39);
      if (lookahead == '\\') ADVANCE(13);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(38);
      if (lookahead != 0) ADVANCE(40);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(49);
      if (lookahead == '"') ADVANCE(37);
      if (lookahead == '/') ADVANCE(44);
      if (lookahead == '`') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2);
      if (lookahead != 0 &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(49);
      if (lookahead == '/') ADVANCE(11);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(3);
      END_STATE();
    case 4:
      if (lookahead == '"') ADVANCE(36);
      if (lookahead == '/') ADVANCE(11);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(4);
      END_STATE();
    case 5:
      if (lookahead == '"') ADVANCE(37);
      if (lookahead == '(') ADVANCE(52);
      if (lookahead == '/') ADVANCE(44);
      if (lookahead == '`') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5);
      if (lookahead != 0 &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 6:
      if (lookahead == '"') ADVANCE(37);
      if (lookahead == ')') ADVANCE(54);
      if (lookahead == '/') ADVANCE(44);
      if (lookahead == '`') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(6);
      if (lookahead != 0 &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 7:
      if (lookahead == '"') ADVANCE(37);
      if (lookahead == '/') ADVANCE(44);
      if (lookahead == '=') ADVANCE(45);
      if (lookahead == '`') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(7);
      if (lookahead != 0 &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 8:
      if (lookahead == '"') ADVANCE(37);
      if (lookahead == '/') ADVANCE(44);
      if (lookahead == '`') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(8);
      if (lookahead != 0 &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 9:
      if (lookahead == '(') ADVANCE(52);
      if (lookahead == '/') ADVANCE(44);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(9);
      if (lookahead != 0 &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 10:
      if (lookahead == ')') ADVANCE(54);
      if (lookahead == '/') ADVANCE(44);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(10);
      if (lookahead != 0 &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 11:
      if (lookahead == '/') ADVANCE(58);
      END_STATE();
    case 12:
      if (lookahead == '>') ADVANCE(55);
      END_STATE();
    case 13:
      if (lookahead == 'U') ADVANCE(31);
      if (lookahead == 'u') ADVANCE(27);
      if (lookahead == 'x') ADVANCE(25);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(43);
      if (lookahead != 0) ADVANCE(41);
      END_STATE();
    case 14:
      if (lookahead == '`') ADVANCE(34);
      if (lookahead != 0) ADVANCE(14);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(16);
      END_STATE();
    case 16:
      if (lookahead == 'c') ADVANCE(19);
      END_STATE();
    case 17:
      if (lookahead == 'e') ADVANCE(22);
      END_STATE();
    case 18:
      if (lookahead == 'e') ADVANCE(57);
      END_STATE();
    case 19:
      if (lookahead == 'e') ADVANCE(50);
      END_STATE();
    case 20:
      if (lookahead == 'l') ADVANCE(15);
      END_STATE();
    case 21:
      if (lookahead == 'o') ADVANCE(48);
      END_STATE();
    case 22:
      if (lookahead == 'p') ADVANCE(20);
      END_STATE();
    case 23:
      if (lookahead == 's') ADVANCE(18);
      END_STATE();
    case 24:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(41);
      END_STATE();
    case 25:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(24);
      END_STATE();
    case 26:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(25);
      END_STATE();
    case 27:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(26);
      END_STATE();
    case 28:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(27);
      END_STATE();
    case 29:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(28);
      END_STATE();
    case 30:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(29);
      END_STATE();
    case 31:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(30);
      END_STATE();
    case 32:
      if (eof) ADVANCE(33);
      ADVANCE_MAP(
        '"', 36,
        '(', 51,
        ')', 53,
        '/', 11,
        '=', 12,
        '`', 14,
        'g', 21,
        'r', 17,
        'u', 23,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(32);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_raw_string_literal);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_raw_string_literal);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(aux_sym_interpreted_string_literal_token1);
      if (lookahead == '/') ADVANCE(39);
      if (lookahead == '\t' ||
          (0x0b <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(38);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(40);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(aux_sym_interpreted_string_literal_token1);
      if (lookahead == '/') ADVANCE(40);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(40);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(aux_sym_interpreted_string_literal_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(40);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_escape_sequence);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(41);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(42);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == '/') ADVANCE(59);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == '>') ADVANCE(56);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == '`') ADVANCE(35);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == ',' ||
          lookahead == '[' ||
          lookahead == ']') ADVANCE(14);
      if (lookahead != 0) ADVANCE(46);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_go);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_LF);
      if (lookahead == '\n') ADVANCE(49);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_replace);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(anon_sym_EQ_GT);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_EQ_GT);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != ',' &&
          lookahead != '[' &&
          lookahead != ']') ADVANCE(47);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(63);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (lookahead == '/') ADVANCE(62);
      if (lookahead == '\t' ||
          (0x0b <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(61);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead)) ADVANCE(63);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (lookahead == '/') ADVANCE(60);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(63);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(63);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 6},
  [3] = {.lex_state = 6},
  [4] = {.lex_state = 6},
  [5] = {.lex_state = 5},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 7},
  [9] = {.lex_state = 8},
  [10] = {.lex_state = 8},
  [11] = {.lex_state = 8},
  [12] = {.lex_state = 8},
  [13] = {.lex_state = 7},
  [14] = {.lex_state = 8},
  [15] = {.lex_state = 8},
  [16] = {.lex_state = 8},
  [17] = {.lex_state = 8},
  [18] = {.lex_state = 8},
  [19] = {.lex_state = 10},
  [20] = {.lex_state = 10},
  [21] = {.lex_state = 10},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 9},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 7},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 1},
  [30] = {.lex_state = 1},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 1},
  [33] = {.lex_state = 1},
  [34] = {.lex_state = 1},
  [35] = {.lex_state = 1},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 1},
  [38] = {.lex_state = 6},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 6},
  [41] = {.lex_state = 1},
  [42] = {.lex_state = 6},
  [43] = {.lex_state = 6},
  [44] = {.lex_state = 2},
  [45] = {.lex_state = 7},
  [46] = {.lex_state = 7},
  [47] = {.lex_state = 7},
  [48] = {.lex_state = 7},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 0},
  [52] = {.lex_state = 1},
  [53] = {.lex_state = 0},
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 8},
  [56] = {.lex_state = 8},
  [57] = {.lex_state = 1},
  [58] = {.lex_state = 8},
  [59] = {.lex_state = 8},
  [60] = {.lex_state = 8},
  [61] = {.lex_state = 10},
  [62] = {.lex_state = 10},
  [63] = {.lex_state = 0},
  [64] = {.lex_state = 3},
  [65] = {.lex_state = 3},
  [66] = {.lex_state = 3},
  [67] = {.lex_state = 3},
  [68] = {.lex_state = 3},
  [69] = {.lex_state = 3},
  [70] = {.lex_state = 3},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 3},
  [75] = {.lex_state = 3},
  [76] = {.lex_state = 3},
  [77] = {.lex_state = 3},
  [78] = {.lex_state = 61},
  [79] = {.lex_state = 0},
  [80] = {.lex_state = 3},
  [81] = {.lex_state = 0},
  [82] = {.lex_state = 3},
  [83] = {.lex_state = 3},
  [84] = {.lex_state = 3},
  [85] = {.lex_state = 3},
  [86] = {.lex_state = 3},
  [87] = {.lex_state = 3},
  [88] = {.lex_state = 3},
  [89] = {.lex_state = 0},
  [90] = {.lex_state = 3},
  [91] = {.lex_state = 3},
  [92] = {.lex_state = 0},
  [93] = {.lex_state = 3},
  [94] = {(TSStateId)(-1)},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [sym_comment] = STATE(0),
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_raw_string_literal] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [sym_escape_sequence] = ACTIONS(1),
    [anon_sym_go] = ACTIONS(1),
    [anon_sym_replace] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_EQ_GT] = ACTIONS(1),
    [anon_sym_use] = ACTIONS(1),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(92),
    [sym__directive] = STATE(25),
    [sym_go_directive] = STATE(26),
    [sym_replace_directive] = STATE(26),
    [sym_use_directive] = STATE(26),
    [sym_comment] = STATE(1),
    [aux_sym_source_file_repeat1] = STATE(6),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_go] = ACTIONS(7),
    [anon_sym_replace] = ACTIONS(9),
    [anon_sym_use] = ACTIONS(11),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 11,
    ACTIONS(13), 1,
      sym_raw_string_literal,
    ACTIONS(16), 1,
      anon_sym_DQUOTE,
    ACTIONS(19), 1,
      sym__identifier,
    ACTIONS(22), 1,
      anon_sym_RPAREN,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(8), 1,
      sym_module_path,
    STATE(27), 1,
      sym__string_or_ident,
    STATE(43), 1,
      sym_replace_spec,
    STATE(47), 1,
      sym__string_literal,
    STATE(48), 1,
      sym_interpreted_string_literal,
    STATE(2), 2,
      sym_comment,
      aux_sym_replace_directive_repeat1,
  [35] = 12,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(26), 1,
      sym_raw_string_literal,
    ACTIONS(28), 1,
      anon_sym_DQUOTE,
    ACTIONS(30), 1,
      sym__identifier,
    ACTIONS(32), 1,
      anon_sym_RPAREN,
    STATE(2), 1,
      aux_sym_replace_directive_repeat1,
    STATE(3), 1,
      sym_comment,
    STATE(8), 1,
      sym_module_path,
    STATE(27), 1,
      sym__string_or_ident,
    STATE(43), 1,
      sym_replace_spec,
    STATE(47), 1,
      sym__string_literal,
    STATE(48), 1,
      sym_interpreted_string_literal,
  [72] = 12,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(26), 1,
      sym_raw_string_literal,
    ACTIONS(28), 1,
      anon_sym_DQUOTE,
    ACTIONS(30), 1,
      sym__identifier,
    ACTIONS(34), 1,
      anon_sym_RPAREN,
    STATE(3), 1,
      aux_sym_replace_directive_repeat1,
    STATE(4), 1,
      sym_comment,
    STATE(8), 1,
      sym_module_path,
    STATE(27), 1,
      sym__string_or_ident,
    STATE(43), 1,
      sym_replace_spec,
    STATE(47), 1,
      sym__string_literal,
    STATE(48), 1,
      sym_interpreted_string_literal,
  [109] = 11,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(26), 1,
      sym_raw_string_literal,
    ACTIONS(28), 1,
      anon_sym_DQUOTE,
    ACTIONS(30), 1,
      sym__identifier,
    ACTIONS(36), 1,
      anon_sym_LPAREN,
    STATE(5), 1,
      sym_comment,
    STATE(13), 1,
      sym_module_path,
    STATE(27), 1,
      sym__string_or_ident,
    STATE(28), 1,
      sym_replace_spec,
    STATE(47), 1,
      sym__string_literal,
    STATE(48), 1,
      sym_interpreted_string_literal,
  [143] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(7), 1,
      anon_sym_go,
    ACTIONS(9), 1,
      anon_sym_replace,
    ACTIONS(11), 1,
      anon_sym_use,
    ACTIONS(38), 1,
      ts_builtin_sym_end,
    STATE(6), 1,
      sym_comment,
    STATE(7), 1,
      aux_sym_source_file_repeat1,
    STATE(25), 1,
      sym__directive,
    STATE(26), 3,
      sym_go_directive,
      sym_replace_directive,
      sym_use_directive,
  [173] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(40), 1,
      ts_builtin_sym_end,
    ACTIONS(42), 1,
      anon_sym_go,
    ACTIONS(45), 1,
      anon_sym_replace,
    ACTIONS(48), 1,
      anon_sym_use,
    STATE(25), 1,
      sym__directive,
    STATE(7), 2,
      sym_comment,
      aux_sym_source_file_repeat1,
    STATE(26), 3,
      sym_go_directive,
      sym_replace_directive,
      sym_use_directive,
  [201] = 10,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(51), 1,
      sym_raw_string_literal,
    ACTIONS(53), 1,
      anon_sym_DQUOTE,
    ACTIONS(55), 1,
      sym__identifier,
    ACTIONS(57), 1,
      anon_sym_EQ_GT,
    STATE(8), 1,
      sym_comment,
    STATE(71), 1,
      sym_interpreted_string_literal,
    STATE(72), 1,
      sym__string_literal,
    STATE(81), 1,
      sym__string_or_ident,
    STATE(89), 1,
      sym_version,
  [232] = 10,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(59), 1,
      sym_raw_string_literal,
    ACTIONS(61), 1,
      anon_sym_DQUOTE,
    ACTIONS(63), 1,
      sym__identifier,
    STATE(9), 1,
      sym_comment,
    STATE(16), 1,
      sym_module_path,
    STATE(56), 1,
      sym__string_or_ident,
    STATE(59), 1,
      sym__string_literal,
    STATE(60), 1,
      sym_interpreted_string_literal,
    STATE(83), 1,
      sym_file_path,
  [263] = 10,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(59), 1,
      sym_raw_string_literal,
    ACTIONS(61), 1,
      anon_sym_DQUOTE,
    ACTIONS(63), 1,
      sym__identifier,
    STATE(10), 1,
      sym_comment,
    STATE(17), 1,
      sym_module_path,
    STATE(56), 1,
      sym__string_or_ident,
    STATE(59), 1,
      sym__string_literal,
    STATE(60), 1,
      sym_interpreted_string_literal,
    STATE(82), 1,
      sym_file_path,
  [294] = 10,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(59), 1,
      sym_raw_string_literal,
    ACTIONS(61), 1,
      anon_sym_DQUOTE,
    ACTIONS(63), 1,
      sym__identifier,
    STATE(11), 1,
      sym_comment,
    STATE(18), 1,
      sym_module_path,
    STATE(56), 1,
      sym__string_or_ident,
    STATE(59), 1,
      sym__string_literal,
    STATE(60), 1,
      sym_interpreted_string_literal,
    STATE(76), 1,
      sym_file_path,
  [325] = 10,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(59), 1,
      sym_raw_string_literal,
    ACTIONS(61), 1,
      anon_sym_DQUOTE,
    ACTIONS(63), 1,
      sym__identifier,
    STATE(12), 1,
      sym_comment,
    STATE(14), 1,
      sym_module_path,
    STATE(56), 1,
      sym__string_or_ident,
    STATE(59), 1,
      sym__string_literal,
    STATE(60), 1,
      sym_interpreted_string_literal,
    STATE(68), 1,
      sym_file_path,
  [356] = 10,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(51), 1,
      sym_raw_string_literal,
    ACTIONS(53), 1,
      anon_sym_DQUOTE,
    ACTIONS(55), 1,
      sym__identifier,
    ACTIONS(65), 1,
      anon_sym_EQ_GT,
    STATE(13), 1,
      sym_comment,
    STATE(71), 1,
      sym_interpreted_string_literal,
    STATE(72), 1,
      sym__string_literal,
    STATE(79), 1,
      sym_version,
    STATE(81), 1,
      sym__string_or_ident,
  [387] = 9,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(67), 1,
      sym_raw_string_literal,
    ACTIONS(69), 1,
      anon_sym_DQUOTE,
    ACTIONS(71), 1,
      sym__identifier,
    STATE(14), 1,
      sym_comment,
    STATE(66), 1,
      sym__string_or_ident,
    STATE(75), 1,
      sym_interpreted_string_literal,
    STATE(76), 1,
      sym_version,
    STATE(77), 1,
      sym__string_literal,
  [415] = 9,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(67), 1,
      sym_raw_string_literal,
    ACTIONS(69), 1,
      anon_sym_DQUOTE,
    ACTIONS(71), 1,
      sym__identifier,
    STATE(15), 1,
      sym_comment,
    STATE(64), 1,
      sym__string_or_ident,
    STATE(75), 1,
      sym_interpreted_string_literal,
    STATE(77), 1,
      sym__string_literal,
    STATE(93), 1,
      sym_go_version,
  [443] = 9,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(67), 1,
      sym_raw_string_literal,
    ACTIONS(69), 1,
      anon_sym_DQUOTE,
    ACTIONS(71), 1,
      sym__identifier,
    STATE(16), 1,
      sym_comment,
    STATE(66), 1,
      sym__string_or_ident,
    STATE(75), 1,
      sym_interpreted_string_literal,
    STATE(77), 1,
      sym__string_literal,
    STATE(84), 1,
      sym_version,
  [471] = 9,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(67), 1,
      sym_raw_string_literal,
    ACTIONS(69), 1,
      anon_sym_DQUOTE,
    ACTIONS(71), 1,
      sym__identifier,
    STATE(17), 1,
      sym_comment,
    STATE(66), 1,
      sym__string_or_ident,
    STATE(75), 1,
      sym_interpreted_string_literal,
    STATE(77), 1,
      sym__string_literal,
    STATE(83), 1,
      sym_version,
  [499] = 9,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(67), 1,
      sym_raw_string_literal,
    ACTIONS(69), 1,
      anon_sym_DQUOTE,
    ACTIONS(71), 1,
      sym__identifier,
    STATE(18), 1,
      sym_comment,
    STATE(66), 1,
      sym__string_or_ident,
    STATE(75), 1,
      sym_interpreted_string_literal,
    STATE(77), 1,
      sym__string_literal,
    STATE(85), 1,
      sym_version,
  [527] = 6,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(73), 1,
      sym__identifier,
    ACTIONS(76), 1,
      anon_sym_RPAREN,
    STATE(61), 1,
      sym_use_spec,
    STATE(80), 1,
      sym_file_path,
    STATE(19), 2,
      sym_comment,
      aux_sym_use_directive_repeat1,
  [547] = 7,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(78), 1,
      sym__identifier,
    ACTIONS(80), 1,
      anon_sym_RPAREN,
    STATE(20), 1,
      sym_comment,
    STATE(21), 1,
      aux_sym_use_directive_repeat1,
    STATE(61), 1,
      sym_use_spec,
    STATE(80), 1,
      sym_file_path,
  [569] = 7,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(78), 1,
      sym__identifier,
    ACTIONS(82), 1,
      anon_sym_RPAREN,
    STATE(19), 1,
      aux_sym_use_directive_repeat1,
    STATE(21), 1,
      sym_comment,
    STATE(61), 1,
      sym_use_spec,
    STATE(80), 1,
      sym_file_path,
  [591] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(22), 1,
      sym_comment,
    ACTIONS(84), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [604] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(23), 1,
      sym_comment,
    ACTIONS(86), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [617] = 6,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(78), 1,
      sym__identifier,
    ACTIONS(88), 1,
      anon_sym_LPAREN,
    STATE(24), 1,
      sym_comment,
    STATE(31), 1,
      sym_use_spec,
    STATE(87), 1,
      sym_file_path,
  [636] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(25), 1,
      sym_comment,
    ACTIONS(90), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [649] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(26), 1,
      sym_comment,
    ACTIONS(92), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [662] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(27), 1,
      sym_comment,
    ACTIONS(94), 4,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
      anon_sym_EQ_GT,
  [675] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(28), 1,
      sym_comment,
    ACTIONS(96), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [688] = 5,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(98), 1,
      anon_sym_DQUOTE,
    STATE(29), 1,
      sym_comment,
    STATE(41), 1,
      aux_sym_interpreted_string_literal_repeat1,
    ACTIONS(100), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
  [705] = 5,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(102), 1,
      anon_sym_DQUOTE,
    STATE(29), 1,
      aux_sym_interpreted_string_literal_repeat1,
    STATE(30), 1,
      sym_comment,
    ACTIONS(100), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
  [722] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(31), 1,
      sym_comment,
    ACTIONS(104), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [735] = 5,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(106), 1,
      anon_sym_DQUOTE,
    STATE(32), 1,
      sym_comment,
    STATE(41), 1,
      aux_sym_interpreted_string_literal_repeat1,
    ACTIONS(100), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
  [752] = 5,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(108), 1,
      anon_sym_DQUOTE,
    STATE(32), 1,
      aux_sym_interpreted_string_literal_repeat1,
    STATE(33), 1,
      sym_comment,
    ACTIONS(100), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
  [769] = 5,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(110), 1,
      anon_sym_DQUOTE,
    STATE(34), 1,
      sym_comment,
    STATE(41), 1,
      aux_sym_interpreted_string_literal_repeat1,
    ACTIONS(100), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
  [786] = 5,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(112), 1,
      anon_sym_DQUOTE,
    STATE(35), 1,
      sym_comment,
    STATE(41), 1,
      aux_sym_interpreted_string_literal_repeat1,
    ACTIONS(100), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
  [803] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(36), 1,
      sym_comment,
    ACTIONS(114), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [816] = 5,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(116), 1,
      anon_sym_DQUOTE,
    STATE(34), 1,
      aux_sym_interpreted_string_literal_repeat1,
    STATE(37), 1,
      sym_comment,
    ACTIONS(100), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
  [833] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(38), 1,
      sym_comment,
    ACTIONS(118), 4,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
      anon_sym_RPAREN,
  [846] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(39), 1,
      sym_comment,
    ACTIONS(120), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [859] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(40), 1,
      sym_comment,
    ACTIONS(122), 4,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
      anon_sym_RPAREN,
  [872] = 4,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(124), 1,
      anon_sym_DQUOTE,
    ACTIONS(126), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
    STATE(41), 2,
      sym_comment,
      aux_sym_interpreted_string_literal_repeat1,
  [887] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(42), 1,
      sym_comment,
    ACTIONS(129), 4,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
      anon_sym_RPAREN,
  [900] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(43), 1,
      sym_comment,
    ACTIONS(131), 4,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
      anon_sym_RPAREN,
  [913] = 4,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(135), 1,
      anon_sym_LF,
    STATE(44), 1,
      sym_comment,
    ACTIONS(133), 3,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
  [928] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(45), 1,
      sym_comment,
    ACTIONS(137), 4,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
      anon_sym_EQ_GT,
  [941] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(46), 1,
      sym_comment,
    ACTIONS(139), 4,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
      anon_sym_EQ_GT,
  [954] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(47), 1,
      sym_comment,
    ACTIONS(133), 4,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
      anon_sym_EQ_GT,
  [967] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(48), 1,
      sym_comment,
    ACTIONS(141), 4,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
      anon_sym_EQ_GT,
  [980] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(49), 1,
      sym_comment,
    ACTIONS(143), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [993] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(50), 1,
      sym_comment,
    ACTIONS(145), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [1006] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(51), 1,
      sym_comment,
    ACTIONS(147), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [1019] = 5,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(149), 1,
      anon_sym_DQUOTE,
    STATE(35), 1,
      aux_sym_interpreted_string_literal_repeat1,
    STATE(52), 1,
      sym_comment,
    ACTIONS(100), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
  [1036] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(53), 1,
      sym_comment,
    ACTIONS(151), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [1049] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(54), 1,
      sym_comment,
    ACTIONS(153), 4,
      ts_builtin_sym_end,
      anon_sym_go,
      anon_sym_replace,
      anon_sym_use,
  [1062] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(55), 1,
      sym_comment,
    ACTIONS(139), 3,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
  [1074] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(56), 1,
      sym_comment,
    ACTIONS(94), 3,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
  [1086] = 4,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(155), 1,
      anon_sym_DQUOTE,
    STATE(57), 1,
      sym_comment,
    ACTIONS(157), 2,
      aux_sym_interpreted_string_literal_token1,
      sym_escape_sequence,
  [1100] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(58), 1,
      sym_comment,
    ACTIONS(137), 3,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
  [1112] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(59), 1,
      sym_comment,
    ACTIONS(133), 3,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
  [1124] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(60), 1,
      sym_comment,
    ACTIONS(141), 3,
      sym_raw_string_literal,
      anon_sym_DQUOTE,
      sym__identifier,
  [1136] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(61), 1,
      sym_comment,
    ACTIONS(159), 2,
      sym__identifier,
      anon_sym_RPAREN,
  [1147] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    STATE(62), 1,
      sym_comment,
    ACTIONS(161), 2,
      sym__identifier,
      anon_sym_RPAREN,
  [1158] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(163), 1,
      anon_sym_EQ_GT,
    STATE(63), 1,
      sym_comment,
  [1168] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(165), 1,
      anon_sym_LF,
    STATE(64), 1,
      sym_comment,
  [1178] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(167), 1,
      anon_sym_LF,
    STATE(65), 1,
      sym_comment,
  [1188] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(169), 1,
      anon_sym_LF,
    STATE(66), 1,
      sym_comment,
  [1198] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(171), 1,
      anon_sym_LF,
    STATE(67), 1,
      sym_comment,
  [1208] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(173), 1,
      anon_sym_LF,
    STATE(68), 1,
      sym_comment,
  [1218] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(175), 1,
      anon_sym_LF,
    STATE(69), 1,
      sym_comment,
  [1228] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(163), 1,
      anon_sym_LF,
    STATE(70), 1,
      sym_comment,
  [1238] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(177), 1,
      anon_sym_EQ_GT,
    STATE(71), 1,
      sym_comment,
  [1248] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(179), 1,
      anon_sym_EQ_GT,
    STATE(72), 1,
      sym_comment,
  [1258] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(181), 1,
      anon_sym_EQ_GT,
    STATE(73), 1,
      sym_comment,
  [1268] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(183), 1,
      anon_sym_LF,
    STATE(74), 1,
      sym_comment,
  [1278] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(177), 1,
      anon_sym_LF,
    STATE(75), 1,
      sym_comment,
  [1288] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(185), 1,
      anon_sym_LF,
    STATE(76), 1,
      sym_comment,
  [1298] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(179), 1,
      anon_sym_LF,
    STATE(77), 1,
      sym_comment,
  [1308] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(187), 1,
      aux_sym_comment_token1,
    STATE(78), 1,
      sym_comment,
  [1318] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(189), 1,
      anon_sym_EQ_GT,
    STATE(79), 1,
      sym_comment,
  [1328] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(191), 1,
      anon_sym_LF,
    STATE(80), 1,
      sym_comment,
  [1338] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(169), 1,
      anon_sym_EQ_GT,
    STATE(81), 1,
      sym_comment,
  [1348] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(193), 1,
      anon_sym_LF,
    STATE(82), 1,
      sym_comment,
  [1358] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(195), 1,
      anon_sym_LF,
    STATE(83), 1,
      sym_comment,
  [1368] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(197), 1,
      anon_sym_LF,
    STATE(84), 1,
      sym_comment,
  [1378] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(199), 1,
      anon_sym_LF,
    STATE(85), 1,
      sym_comment,
  [1388] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(181), 1,
      anon_sym_LF,
    STATE(86), 1,
      sym_comment,
  [1398] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(201), 1,
      anon_sym_LF,
    STATE(87), 1,
      sym_comment,
  [1408] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(203), 1,
      anon_sym_LF,
    STATE(88), 1,
      sym_comment,
  [1418] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(205), 1,
      anon_sym_EQ_GT,
    STATE(89), 1,
      sym_comment,
  [1428] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(135), 1,
      anon_sym_LF,
    STATE(90), 1,
      sym_comment,
  [1438] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(207), 1,
      anon_sym_LF,
    STATE(91), 1,
      sym_comment,
  [1448] = 3,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(209), 1,
      ts_builtin_sym_end,
    STATE(92), 1,
      sym_comment,
  [1458] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(211), 1,
      anon_sym_LF,
    STATE(93), 1,
      sym_comment,
  [1468] = 1,
    ACTIONS(213), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 35,
  [SMALL_STATE(4)] = 72,
  [SMALL_STATE(5)] = 109,
  [SMALL_STATE(6)] = 143,
  [SMALL_STATE(7)] = 173,
  [SMALL_STATE(8)] = 201,
  [SMALL_STATE(9)] = 232,
  [SMALL_STATE(10)] = 263,
  [SMALL_STATE(11)] = 294,
  [SMALL_STATE(12)] = 325,
  [SMALL_STATE(13)] = 356,
  [SMALL_STATE(14)] = 387,
  [SMALL_STATE(15)] = 415,
  [SMALL_STATE(16)] = 443,
  [SMALL_STATE(17)] = 471,
  [SMALL_STATE(18)] = 499,
  [SMALL_STATE(19)] = 527,
  [SMALL_STATE(20)] = 547,
  [SMALL_STATE(21)] = 569,
  [SMALL_STATE(22)] = 591,
  [SMALL_STATE(23)] = 604,
  [SMALL_STATE(24)] = 617,
  [SMALL_STATE(25)] = 636,
  [SMALL_STATE(26)] = 649,
  [SMALL_STATE(27)] = 662,
  [SMALL_STATE(28)] = 675,
  [SMALL_STATE(29)] = 688,
  [SMALL_STATE(30)] = 705,
  [SMALL_STATE(31)] = 722,
  [SMALL_STATE(32)] = 735,
  [SMALL_STATE(33)] = 752,
  [SMALL_STATE(34)] = 769,
  [SMALL_STATE(35)] = 786,
  [SMALL_STATE(36)] = 803,
  [SMALL_STATE(37)] = 816,
  [SMALL_STATE(38)] = 833,
  [SMALL_STATE(39)] = 846,
  [SMALL_STATE(40)] = 859,
  [SMALL_STATE(41)] = 872,
  [SMALL_STATE(42)] = 887,
  [SMALL_STATE(43)] = 900,
  [SMALL_STATE(44)] = 913,
  [SMALL_STATE(45)] = 928,
  [SMALL_STATE(46)] = 941,
  [SMALL_STATE(47)] = 954,
  [SMALL_STATE(48)] = 967,
  [SMALL_STATE(49)] = 980,
  [SMALL_STATE(50)] = 993,
  [SMALL_STATE(51)] = 1006,
  [SMALL_STATE(52)] = 1019,
  [SMALL_STATE(53)] = 1036,
  [SMALL_STATE(54)] = 1049,
  [SMALL_STATE(55)] = 1062,
  [SMALL_STATE(56)] = 1074,
  [SMALL_STATE(57)] = 1086,
  [SMALL_STATE(58)] = 1100,
  [SMALL_STATE(59)] = 1112,
  [SMALL_STATE(60)] = 1124,
  [SMALL_STATE(61)] = 1136,
  [SMALL_STATE(62)] = 1147,
  [SMALL_STATE(63)] = 1158,
  [SMALL_STATE(64)] = 1168,
  [SMALL_STATE(65)] = 1178,
  [SMALL_STATE(66)] = 1188,
  [SMALL_STATE(67)] = 1198,
  [SMALL_STATE(68)] = 1208,
  [SMALL_STATE(69)] = 1218,
  [SMALL_STATE(70)] = 1228,
  [SMALL_STATE(71)] = 1238,
  [SMALL_STATE(72)] = 1248,
  [SMALL_STATE(73)] = 1258,
  [SMALL_STATE(74)] = 1268,
  [SMALL_STATE(75)] = 1278,
  [SMALL_STATE(76)] = 1288,
  [SMALL_STATE(77)] = 1298,
  [SMALL_STATE(78)] = 1308,
  [SMALL_STATE(79)] = 1318,
  [SMALL_STATE(80)] = 1328,
  [SMALL_STATE(81)] = 1338,
  [SMALL_STATE(82)] = 1348,
  [SMALL_STATE(83)] = 1358,
  [SMALL_STATE(84)] = 1368,
  [SMALL_STATE(85)] = 1378,
  [SMALL_STATE(86)] = 1388,
  [SMALL_STATE(87)] = 1398,
  [SMALL_STATE(88)] = 1408,
  [SMALL_STATE(89)] = 1418,
  [SMALL_STATE(90)] = 1428,
  [SMALL_STATE(91)] = 1438,
  [SMALL_STATE(92)] = 1448,
  [SMALL_STATE(93)] = 1458,
  [SMALL_STATE(94)] = 1468,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [13] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_replace_directive_repeat1, 2, 0, 0), SHIFT_REPEAT(48),
  [16] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_replace_directive_repeat1, 2, 0, 0), SHIFT_REPEAT(37),
  [19] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_replace_directive_repeat1, 2, 0, 0), SHIFT_REPEAT(47),
  [22] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_replace_directive_repeat1, 2, 0, 0),
  [24] = {.entry = {.count = 1, .reusable = false}}, SHIFT(78),
  [26] = {.entry = {.count = 1, .reusable = false}}, SHIFT(48),
  [28] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [30] = {.entry = {.count = 1, .reusable = false}}, SHIFT(47),
  [32] = {.entry = {.count = 1, .reusable = false}}, SHIFT(65),
  [34] = {.entry = {.count = 1, .reusable = false}}, SHIFT(69),
  [36] = {.entry = {.count = 1, .reusable = false}}, SHIFT(91),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [42] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(15),
  [45] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(5),
  [48] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(24),
  [51] = {.entry = {.count = 1, .reusable = false}}, SHIFT(71),
  [53] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [55] = {.entry = {.count = 1, .reusable = false}}, SHIFT(72),
  [57] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [59] = {.entry = {.count = 1, .reusable = false}}, SHIFT(60),
  [61] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [63] = {.entry = {.count = 1, .reusable = false}}, SHIFT(44),
  [65] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(75),
  [69] = {.entry = {.count = 1, .reusable = false}}, SHIFT(52),
  [71] = {.entry = {.count = 1, .reusable = false}}, SHIFT(77),
  [73] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_use_directive_repeat1, 2, 0, 0), SHIFT_REPEAT(90),
  [76] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_use_directive_repeat1, 2, 0, 0),
  [78] = {.entry = {.count = 1, .reusable = false}}, SHIFT(90),
  [80] = {.entry = {.count = 1, .reusable = false}}, SHIFT(67),
  [82] = {.entry = {.count = 1, .reusable = false}}, SHIFT(74),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_replace_directive, 5, 0, 0),
  [86] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_replace_spec, 5, 0, 0),
  [88] = {.entry = {.count = 1, .reusable = false}}, SHIFT(88),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 1, 0, 0),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__directive, 1, 0, 0),
  [94] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_module_path, 1, 0, 0),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_replace_directive, 2, 0, 0),
  [98] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [102] = {.entry = {.count = 1, .reusable = false}}, SHIFT(55),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_directive, 2, 0, 0),
  [106] = {.entry = {.count = 1, .reusable = false}}, SHIFT(63),
  [108] = {.entry = {.count = 1, .reusable = false}}, SHIFT(73),
  [110] = {.entry = {.count = 1, .reusable = false}}, SHIFT(45),
  [112] = {.entry = {.count = 1, .reusable = false}}, SHIFT(70),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_go_directive, 3, 0, 0),
  [116] = {.entry = {.count = 1, .reusable = false}}, SHIFT(46),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_replace_spec, 6, 0, 0),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_spec, 2, 0, 0),
  [122] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_replace_spec, 5, 0, 0),
  [124] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_interpreted_string_literal_repeat1, 2, 0, 0),
  [126] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_interpreted_string_literal_repeat1, 2, 0, 0), SHIFT_REPEAT(57),
  [129] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_replace_spec, 4, 0, 0),
  [131] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_replace_directive_repeat1, 1, 0, 0),
  [133] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__string_or_ident, 1, 0, 0),
  [135] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_path, 1, 0, 0),
  [137] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interpreted_string_literal, 3, 0, 0),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_interpreted_string_literal, 2, 0, 0),
  [141] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__string_literal, 1, 0, 0),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_replace_spec, 6, 0, 0),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_directive, 6, 0, 0),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_replace_spec, 4, 0, 0),
  [149] = {.entry = {.count = 1, .reusable = false}}, SHIFT(86),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_directive, 5, 0, 0),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_replace_directive, 6, 0, 0),
  [155] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_interpreted_string_literal_repeat1, 1, 0, 0),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_interpreted_string_literal_repeat1, 1, 0, 0),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_use_directive_repeat1, 1, 0, 0),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_use_spec, 2, 0, 0),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interpreted_string_literal, 3, 0, 0),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_go_version, 1, 0, 0),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_version, 1, 0, 0),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__string_literal, 1, 0, 0),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__string_or_ident, 1, 0, 0),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interpreted_string_literal, 2, 0, 0),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [185] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [187] = {.entry = {.count = 1, .reusable = false}}, SHIFT(94),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [195] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [197] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [199] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [201] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [209] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [211] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 2, 0, 0),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_gowork(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
