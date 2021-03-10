#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 12
#define STATE_COUNT 18
#define LARGE_STATE_COUNT 6
#define SYMBOL_COUNT 17
#define ALIAS_COUNT 0
#define TOKEN_COUNT 9
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 3
#define MAX_ALIAS_SEQUENCE_LENGTH 3

enum {
  sym_identifier = 1,
  anon_sym_BANG = 2,
  anon_sym_AMP_AMP = 3,
  anon_sym_PIPE_PIPE = 4,
  anon_sym_EQ_EQ = 5,
  anon_sym_BANG_EQ = 6,
  anon_sym_LPAREN = 7,
  anon_sym_RPAREN = 8,
  sym_source = 9,
  sym__expression = 10,
  sym_not = 11,
  sym_and = 12,
  sym_or = 13,
  sym_equal = 14,
  sym_not_equal = 15,
  sym_parenthesized = 16,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_BANG] = "!",
  [anon_sym_AMP_AMP] = "&&",
  [anon_sym_PIPE_PIPE] = "||",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_source] = "source",
  [sym__expression] = "_expression",
  [sym_not] = "not",
  [sym_and] = "and",
  [sym_or] = "or",
  [sym_equal] = "equal",
  [sym_not_equal] = "not_equal",
  [sym_parenthesized] = "parenthesized",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym_BANG] = anon_sym_BANG,
  [anon_sym_AMP_AMP] = anon_sym_AMP_AMP,
  [anon_sym_PIPE_PIPE] = anon_sym_PIPE_PIPE,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_source] = sym_source,
  [sym__expression] = sym__expression,
  [sym_not] = sym_not,
  [sym_and] = sym_and,
  [sym_or] = sym_or,
  [sym_equal] = sym_equal,
  [sym_not_equal] = sym_not_equal,
  [sym_parenthesized] = sym_parenthesized,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
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
  [sym_source] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym_not] = {
    .visible = true,
    .named = true,
  },
  [sym_and] = {
    .visible = true,
    .named = true,
  },
  [sym_or] = {
    .visible = true,
    .named = true,
  },
  [sym_equal] = {
    .visible = true,
    .named = true,
  },
  [sym_not_equal] = {
    .visible = true,
    .named = true,
  },
  [sym_parenthesized] = {
    .visible = true,
    .named = true,
  },
};

enum {
  field_expression = 1,
  field_left = 2,
  field_right = 3,
};

static const char *ts_field_names[] = {
  [0] = NULL,
  [field_expression] = "expression",
  [field_left] = "left",
  [field_right] = "right",
};

static const TSFieldMapSlice ts_field_map_slices[3] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_expression, 1},
  [1] =
    {field_left, 0},
    {field_right, 2},
};

static TSSymbol ts_alias_sequences[3][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(7);
      if (lookahead == '!') ADVANCE(10);
      if (lookahead == '&') ADVANCE(2);
      if (lookahead == '(') ADVANCE(15);
      if (lookahead == ')') ADVANCE(16);
      if (lookahead == '=') ADVANCE(4);
      if (lookahead == '|') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(8);
      END_STATE();
    case 1:
      if (lookahead == '!') ADVANCE(9);
      if (lookahead == '(') ADVANCE(15);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(8);
      END_STATE();
    case 2:
      if (lookahead == '&') ADVANCE(11);
      END_STATE();
    case 3:
      if (lookahead == '=') ADVANCE(14);
      END_STATE();
    case 4:
      if (lookahead == '=') ADVANCE(13);
      END_STATE();
    case 5:
      if (lookahead == '|') ADVANCE(12);
      END_STATE();
    case 6:
      if (eof) ADVANCE(7);
      if (lookahead == '!') ADVANCE(3);
      if (lookahead == '&') ADVANCE(2);
      if (lookahead == ')') ADVANCE(16);
      if (lookahead == '=') ADVANCE(4);
      if (lookahead == '|') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(6)
      END_STATE();
    case 7:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(8);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(14);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_AMP_AMP);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_PIPE_PIPE);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 1},
  [2] = {.lex_state = 1},
  [3] = {.lex_state = 1},
  [4] = {.lex_state = 1},
  [5] = {.lex_state = 1},
  [6] = {.lex_state = 6},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [anon_sym_AMP_AMP] = ACTIONS(1),
    [anon_sym_PIPE_PIPE] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
  },
  [1] = {
    [sym_source] = STATE(15),
    [sym__expression] = STATE(13),
    [sym_not] = STATE(13),
    [sym_and] = STATE(13),
    [sym_or] = STATE(13),
    [sym_equal] = STATE(13),
    [sym_not_equal] = STATE(13),
    [sym_parenthesized] = STATE(13),
    [sym_identifier] = ACTIONS(3),
    [anon_sym_BANG] = ACTIONS(5),
    [anon_sym_LPAREN] = ACTIONS(7),
  },
  [2] = {
    [sym__expression] = STATE(7),
    [sym_not] = STATE(7),
    [sym_and] = STATE(7),
    [sym_or] = STATE(7),
    [sym_equal] = STATE(7),
    [sym_not_equal] = STATE(7),
    [sym_parenthesized] = STATE(7),
    [sym_identifier] = ACTIONS(3),
    [anon_sym_BANG] = ACTIONS(5),
    [anon_sym_LPAREN] = ACTIONS(7),
  },
  [3] = {
    [sym__expression] = STATE(14),
    [sym_not] = STATE(14),
    [sym_and] = STATE(14),
    [sym_or] = STATE(14),
    [sym_equal] = STATE(14),
    [sym_not_equal] = STATE(14),
    [sym_parenthesized] = STATE(14),
    [sym_identifier] = ACTIONS(3),
    [anon_sym_BANG] = ACTIONS(5),
    [anon_sym_LPAREN] = ACTIONS(7),
  },
  [4] = {
    [sym__expression] = STATE(11),
    [sym_not] = STATE(11),
    [sym_and] = STATE(11),
    [sym_or] = STATE(11),
    [sym_equal] = STATE(11),
    [sym_not_equal] = STATE(11),
    [sym_parenthesized] = STATE(11),
    [sym_identifier] = ACTIONS(3),
    [anon_sym_BANG] = ACTIONS(5),
    [anon_sym_LPAREN] = ACTIONS(7),
  },
  [5] = {
    [sym__expression] = STATE(12),
    [sym_not] = STATE(12),
    [sym_and] = STATE(12),
    [sym_or] = STATE(12),
    [sym_equal] = STATE(12),
    [sym_not_equal] = STATE(12),
    [sym_parenthesized] = STATE(12),
    [sym_identifier] = ACTIONS(3),
    [anon_sym_BANG] = ACTIONS(5),
    [anon_sym_LPAREN] = ACTIONS(7),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 3,
    ACTIONS(11), 1,
      anon_sym_EQ_EQ,
    ACTIONS(13), 1,
      anon_sym_BANG_EQ,
    ACTIONS(9), 4,
      ts_builtin_sym_end,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_RPAREN,
  [13] = 1,
    ACTIONS(15), 4,
      ts_builtin_sym_end,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_RPAREN,
  [20] = 1,
    ACTIONS(17), 4,
      ts_builtin_sym_end,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_RPAREN,
  [27] = 1,
    ACTIONS(19), 4,
      ts_builtin_sym_end,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_RPAREN,
  [34] = 1,
    ACTIONS(21), 4,
      ts_builtin_sym_end,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_RPAREN,
  [41] = 1,
    ACTIONS(23), 4,
      ts_builtin_sym_end,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_RPAREN,
  [48] = 2,
    ACTIONS(27), 1,
      anon_sym_AMP_AMP,
    ACTIONS(25), 3,
      ts_builtin_sym_end,
      anon_sym_PIPE_PIPE,
      anon_sym_RPAREN,
  [57] = 3,
    ACTIONS(27), 1,
      anon_sym_AMP_AMP,
    ACTIONS(29), 1,
      ts_builtin_sym_end,
    ACTIONS(31), 1,
      anon_sym_PIPE_PIPE,
  [67] = 3,
    ACTIONS(27), 1,
      anon_sym_AMP_AMP,
    ACTIONS(31), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(33), 1,
      anon_sym_RPAREN,
  [77] = 1,
    ACTIONS(35), 1,
      ts_builtin_sym_end,
  [81] = 1,
    ACTIONS(37), 1,
      sym_identifier,
  [85] = 1,
    ACTIONS(39), 1,
      sym_identifier,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(6)] = 0,
  [SMALL_STATE(7)] = 13,
  [SMALL_STATE(8)] = 20,
  [SMALL_STATE(9)] = 27,
  [SMALL_STATE(10)] = 34,
  [SMALL_STATE(11)] = 41,
  [SMALL_STATE(12)] = 48,
  [SMALL_STATE(13)] = 57,
  [SMALL_STATE(14)] = 67,
  [SMALL_STATE(15)] = 77,
  [SMALL_STATE(16)] = 81,
  [SMALL_STATE(17)] = 85,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_not, 2, .production_id = 1),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_equal, 3, .production_id = 2),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_not_equal, 3, .production_id = 2),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesized, 3, .production_id = 1),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_and, 3, .production_id = 2),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_or, 3, .production_id = 2),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 1),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [35] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_context_predicate(void) {
  static TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .parse_table = (const uint16_t *)ts_parse_table,
    .parse_actions = ts_parse_actions,
    .lex_modes = ts_lex_modes,
    .alias_sequences = (const TSSymbol *)ts_alias_sequences,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .lex_fn = ts_lex,
    .field_count = FIELD_COUNT,
    .field_map_slices = (const TSFieldMapSlice *)ts_field_map_slices,
    .field_map_entries = (const TSFieldMapEntry *)ts_field_map_entries,
    .field_names = ts_field_names,
    .large_state_count = LARGE_STATE_COUNT,
    .small_parse_table = (const uint16_t *)ts_small_parse_table,
    .small_parse_table_map = (const uint32_t *)ts_small_parse_table_map,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .state_count = STATE_COUNT,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
