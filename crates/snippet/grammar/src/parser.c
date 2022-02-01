#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 25
#define LARGE_STATE_COUNT 8
#define SYMBOL_COUNT 14
#define ALIAS_COUNT 0
#define TOKEN_COUNT 8
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_DOLLAR = 1,
  anon_sym_DOLLAR_LBRACE = 2,
  anon_sym_RBRACE = 3,
  anon_sym_COLON = 4,
  sym_int = 5,
  sym__raw_curly = 6,
  sym__plain_text = 7,
  sym_snippet = 8,
  sym__any = 9,
  sym_tabstop = 10,
  sym_placeholder = 11,
  sym_text = 12,
  aux_sym_snippet_repeat1 = 13,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_DOLLAR] = "$",
  [anon_sym_DOLLAR_LBRACE] = "${",
  [anon_sym_RBRACE] = "}",
  [anon_sym_COLON] = ":",
  [sym_int] = "int",
  [sym__raw_curly] = "_raw_curly",
  [sym__plain_text] = "_plain_text",
  [sym_snippet] = "snippet",
  [sym__any] = "_any",
  [sym_tabstop] = "tabstop",
  [sym_placeholder] = "placeholder",
  [sym_text] = "text",
  [aux_sym_snippet_repeat1] = "snippet_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_DOLLAR] = anon_sym_DOLLAR,
  [anon_sym_DOLLAR_LBRACE] = anon_sym_DOLLAR_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym_int] = sym_int,
  [sym__raw_curly] = sym__raw_curly,
  [sym__plain_text] = sym__plain_text,
  [sym_snippet] = sym_snippet,
  [sym__any] = sym__any,
  [sym_tabstop] = sym_tabstop,
  [sym_placeholder] = sym_placeholder,
  [sym_text] = sym_text,
  [aux_sym_snippet_repeat1] = aux_sym_snippet_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_DOLLAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOLLAR_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [sym_int] = {
    .visible = true,
    .named = true,
  },
  [sym__raw_curly] = {
    .visible = false,
    .named = true,
  },
  [sym__plain_text] = {
    .visible = false,
    .named = true,
  },
  [sym_snippet] = {
    .visible = true,
    .named = true,
  },
  [sym__any] = {
    .visible = false,
    .named = true,
  },
  [sym_tabstop] = {
    .visible = true,
    .named = true,
  },
  [sym_placeholder] = {
    .visible = true,
    .named = true,
  },
  [sym_text] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_snippet_repeat1] = {
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

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(3);
      if (lookahead == '$') ADVANCE(4);
      if (lookahead == ':') ADVANCE(7);
      if (lookahead == '}') ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(8);
      END_STATE();
    case 1:
      if (lookahead == '$') ADVANCE(4);
      if (lookahead == '\\') ADVANCE(12);
      if (lookahead == '}') ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(10);
      if (lookahead != 0) ADVANCE(11);
      END_STATE();
    case 2:
      if (eof) ADVANCE(3);
      if (lookahead == '$') ADVANCE(4);
      if (lookahead == '\\') ADVANCE(12);
      if (lookahead == '}') ADVANCE(9);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(10);
      if (lookahead != 0) ADVANCE(11);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_DOLLAR);
      if (lookahead == '{') ADVANCE(5);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_DOLLAR_LBRACE);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym_int);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(8);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym__raw_curly);
      if (lookahead == '}') ADVANCE(9);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym__plain_text);
      if (lookahead == '\\') ADVANCE(12);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(10);
      if (lookahead != 0 &&
          lookahead != '$' &&
          lookahead != '}') ADVANCE(11);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym__plain_text);
      if (lookahead == '\\') ADVANCE(12);
      if (lookahead != 0 &&
          lookahead != '$' &&
          lookahead != '}') ADVANCE(11);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym__plain_text);
      if (lookahead == '\\') ADVANCE(12);
      if (lookahead != 0) ADVANCE(11);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 2},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 1},
  [5] = {.lex_state = 1},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 2},
  [8] = {.lex_state = 1},
  [9] = {.lex_state = 2},
  [10] = {.lex_state = 2},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 1},
  [13] = {.lex_state = 1},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 1},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_DOLLAR] = ACTIONS(1),
    [anon_sym_DOLLAR_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [sym_int] = ACTIONS(1),
    [sym__raw_curly] = ACTIONS(1),
  },
  [1] = {
    [sym_snippet] = STATE(20),
    [sym__any] = STATE(6),
    [sym_tabstop] = STATE(6),
    [sym_placeholder] = STATE(6),
    [sym_text] = STATE(6),
    [aux_sym_snippet_repeat1] = STATE(6),
    [anon_sym_DOLLAR] = ACTIONS(3),
    [anon_sym_DOLLAR_LBRACE] = ACTIONS(5),
    [sym__raw_curly] = ACTIONS(7),
    [sym__plain_text] = ACTIONS(9),
  },
  [2] = {
    [sym_snippet] = STATE(18),
    [sym__any] = STATE(5),
    [sym_tabstop] = STATE(5),
    [sym_placeholder] = STATE(5),
    [sym_text] = STATE(5),
    [aux_sym_snippet_repeat1] = STATE(5),
    [anon_sym_DOLLAR] = ACTIONS(11),
    [anon_sym_DOLLAR_LBRACE] = ACTIONS(13),
    [sym__raw_curly] = ACTIONS(15),
    [sym__plain_text] = ACTIONS(17),
  },
  [3] = {
    [sym_snippet] = STATE(22),
    [sym__any] = STATE(5),
    [sym_tabstop] = STATE(5),
    [sym_placeholder] = STATE(5),
    [sym_text] = STATE(5),
    [aux_sym_snippet_repeat1] = STATE(5),
    [anon_sym_DOLLAR] = ACTIONS(11),
    [anon_sym_DOLLAR_LBRACE] = ACTIONS(13),
    [sym__raw_curly] = ACTIONS(15),
    [sym__plain_text] = ACTIONS(17),
  },
  [4] = {
    [sym__any] = STATE(4),
    [sym_tabstop] = STATE(4),
    [sym_placeholder] = STATE(4),
    [sym_text] = STATE(4),
    [aux_sym_snippet_repeat1] = STATE(4),
    [anon_sym_DOLLAR] = ACTIONS(19),
    [anon_sym_DOLLAR_LBRACE] = ACTIONS(22),
    [anon_sym_RBRACE] = ACTIONS(25),
    [sym__raw_curly] = ACTIONS(27),
    [sym__plain_text] = ACTIONS(30),
  },
  [5] = {
    [sym__any] = STATE(4),
    [sym_tabstop] = STATE(4),
    [sym_placeholder] = STATE(4),
    [sym_text] = STATE(4),
    [aux_sym_snippet_repeat1] = STATE(4),
    [anon_sym_DOLLAR] = ACTIONS(11),
    [anon_sym_DOLLAR_LBRACE] = ACTIONS(13),
    [anon_sym_RBRACE] = ACTIONS(33),
    [sym__raw_curly] = ACTIONS(15),
    [sym__plain_text] = ACTIONS(17),
  },
  [6] = {
    [sym__any] = STATE(7),
    [sym_tabstop] = STATE(7),
    [sym_placeholder] = STATE(7),
    [sym_text] = STATE(7),
    [aux_sym_snippet_repeat1] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(35),
    [anon_sym_DOLLAR] = ACTIONS(3),
    [anon_sym_DOLLAR_LBRACE] = ACTIONS(5),
    [sym__raw_curly] = ACTIONS(7),
    [sym__plain_text] = ACTIONS(9),
  },
  [7] = {
    [sym__any] = STATE(7),
    [sym_tabstop] = STATE(7),
    [sym_placeholder] = STATE(7),
    [sym_text] = STATE(7),
    [aux_sym_snippet_repeat1] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(37),
    [anon_sym_DOLLAR] = ACTIONS(39),
    [anon_sym_DOLLAR_LBRACE] = ACTIONS(42),
    [sym__raw_curly] = ACTIONS(45),
    [sym__plain_text] = ACTIONS(48),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(53), 1,
      sym__plain_text,
    ACTIONS(51), 4,
      anon_sym_DOLLAR,
      anon_sym_DOLLAR_LBRACE,
      anon_sym_RBRACE,
      sym__raw_curly,
  [10] = 2,
    ACTIONS(55), 2,
      ts_builtin_sym_end,
      sym__plain_text,
    ACTIONS(57), 3,
      anon_sym_DOLLAR,
      anon_sym_DOLLAR_LBRACE,
      sym__raw_curly,
  [20] = 2,
    ACTIONS(53), 2,
      ts_builtin_sym_end,
      sym__plain_text,
    ACTIONS(51), 3,
      anon_sym_DOLLAR,
      anon_sym_DOLLAR_LBRACE,
      sym__raw_curly,
  [30] = 2,
    ACTIONS(59), 2,
      ts_builtin_sym_end,
      sym__plain_text,
    ACTIONS(61), 3,
      anon_sym_DOLLAR,
      anon_sym_DOLLAR_LBRACE,
      sym__raw_curly,
  [40] = 2,
    ACTIONS(65), 1,
      sym__plain_text,
    ACTIONS(63), 4,
      anon_sym_DOLLAR,
      anon_sym_DOLLAR_LBRACE,
      anon_sym_RBRACE,
      sym__raw_curly,
  [50] = 2,
    ACTIONS(55), 1,
      sym__plain_text,
    ACTIONS(57), 4,
      anon_sym_DOLLAR,
      anon_sym_DOLLAR_LBRACE,
      anon_sym_RBRACE,
      sym__raw_curly,
  [60] = 2,
    ACTIONS(65), 2,
      ts_builtin_sym_end,
      sym__plain_text,
    ACTIONS(63), 3,
      anon_sym_DOLLAR,
      anon_sym_DOLLAR_LBRACE,
      sym__raw_curly,
  [70] = 2,
    ACTIONS(59), 1,
      sym__plain_text,
    ACTIONS(61), 4,
      anon_sym_DOLLAR,
      anon_sym_DOLLAR_LBRACE,
      anon_sym_RBRACE,
      sym__raw_curly,
  [80] = 2,
    ACTIONS(67), 1,
      anon_sym_RBRACE,
    ACTIONS(69), 1,
      anon_sym_COLON,
  [87] = 2,
    ACTIONS(71), 1,
      anon_sym_RBRACE,
    ACTIONS(73), 1,
      anon_sym_COLON,
  [94] = 1,
    ACTIONS(75), 1,
      anon_sym_RBRACE,
  [98] = 1,
    ACTIONS(77), 1,
      sym_int,
  [102] = 1,
    ACTIONS(79), 1,
      ts_builtin_sym_end,
  [106] = 1,
    ACTIONS(81), 1,
      sym_int,
  [110] = 1,
    ACTIONS(83), 1,
      anon_sym_RBRACE,
  [114] = 1,
    ACTIONS(85), 1,
      sym_int,
  [118] = 1,
    ACTIONS(87), 1,
      sym_int,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(8)] = 0,
  [SMALL_STATE(9)] = 10,
  [SMALL_STATE(10)] = 20,
  [SMALL_STATE(11)] = 30,
  [SMALL_STATE(12)] = 40,
  [SMALL_STATE(13)] = 50,
  [SMALL_STATE(14)] = 60,
  [SMALL_STATE(15)] = 70,
  [SMALL_STATE(16)] = 80,
  [SMALL_STATE(17)] = 87,
  [SMALL_STATE(18)] = 94,
  [SMALL_STATE(19)] = 98,
  [SMALL_STATE(20)] = 102,
  [SMALL_STATE(21)] = 106,
  [SMALL_STATE(22)] = 110,
  [SMALL_STATE(23)] = 114,
  [SMALL_STATE(24)] = 118,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [19] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_snippet_repeat1, 2), SHIFT_REPEAT(21),
  [22] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_snippet_repeat1, 2), SHIFT_REPEAT(23),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_snippet_repeat1, 2),
  [27] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_snippet_repeat1, 2), SHIFT_REPEAT(12),
  [30] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_snippet_repeat1, 2), SHIFT_REPEAT(12),
  [33] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_snippet, 1),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_snippet, 1),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_snippet_repeat1, 2),
  [39] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_snippet_repeat1, 2), SHIFT_REPEAT(19),
  [42] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_snippet_repeat1, 2), SHIFT_REPEAT(24),
  [45] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_snippet_repeat1, 2), SHIFT_REPEAT(14),
  [48] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_snippet_repeat1, 2), SHIFT_REPEAT(14),
  [51] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tabstop, 3),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tabstop, 3),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tabstop, 2),
  [57] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tabstop, 2),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_placeholder, 5),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_placeholder, 5),
  [63] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_text, 1),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_text, 1),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [79] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_snippet(void) {
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
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
