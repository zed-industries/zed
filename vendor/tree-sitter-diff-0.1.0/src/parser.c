#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 89
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 63
#define ALIAS_COUNT 1
#define TOKEN_COUNT 38
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 2
#define MAX_ALIAS_SEQUENCE_LENGTH 8
#define PRODUCTION_ID_COUNT 4

enum ts_symbol_identifiers {
  aux_sym_source_token1 = 1,
  anon_sym_diff = 2,
  aux_sym_command_token1 = 3,
  anon_sym_new = 4,
  anon_sym_deleted = 5,
  anon_sym_file = 6,
  anon_sym_mode = 7,
  anon_sym_old = 8,
  anon_sym_rename = 9,
  anon_sym_from = 10,
  anon_sym_to = 11,
  anon_sym_Binary = 12,
  anon_sym_files = 13,
  anon_sym_and = 14,
  anon_sym_differ = 15,
  anon_sym_index = 16,
  anon_sym_DOT_DOT = 17,
  anon_sym_similarity = 18,
  anon_sym_index2 = 19,
  aux_sym_similarity_token1 = 20,
  anon_sym_PERCENT = 21,
  anon_sym_DASH_DASH_DASH = 22,
  anon_sym_PLUS_PLUS_PLUS = 23,
  anon_sym_AT_AT = 24,
  anon_sym_AT_AT2 = 25,
  aux_sym_location_token1 = 26,
  anon_sym_PLUS = 27,
  anon_sym_PLUS_PLUS = 28,
  anon_sym_PLUS_PLUS_PLUS_PLUS = 29,
  anon_sym_DASH = 30,
  anon_sym_DASH_DASH = 31,
  anon_sym_DASH_DASH_DASH_DASH = 32,
  sym_context = 33,
  anon_sym_POUND = 34,
  sym_linerange = 35,
  aux_sym_filename_token1 = 36,
  sym_commit = 37,
  sym_source = 38,
  sym__line = 39,
  sym_block = 40,
  sym_hunks = 41,
  sym_hunk = 42,
  sym_changes = 43,
  sym_command = 44,
  sym_file_change = 45,
  sym_binary_change = 46,
  sym_index = 47,
  sym_similarity = 48,
  sym_old_file = 49,
  sym_new_file = 50,
  sym_location = 51,
  sym_addition = 52,
  sym_deletion = 53,
  sym_comment = 54,
  sym_filename = 55,
  sym_mode = 56,
  aux_sym_source_repeat1 = 57,
  aux_sym_block_repeat1 = 58,
  aux_sym_hunks_repeat1 = 59,
  aux_sym_changes_repeat1 = 60,
  aux_sym_changes_repeat2 = 61,
  aux_sym_filename_repeat1 = 62,
  alias_sym_score = 63,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_source_token1] = "source_token1",
  [anon_sym_diff] = "diff",
  [aux_sym_command_token1] = "argument",
  [anon_sym_new] = "new",
  [anon_sym_deleted] = "deleted",
  [anon_sym_file] = "file",
  [anon_sym_mode] = "mode",
  [anon_sym_old] = "old",
  [anon_sym_rename] = "rename",
  [anon_sym_from] = "from",
  [anon_sym_to] = "to",
  [anon_sym_Binary] = "Binary",
  [anon_sym_files] = "files",
  [anon_sym_and] = "and",
  [anon_sym_differ] = "differ",
  [anon_sym_index] = "index",
  [anon_sym_DOT_DOT] = "..",
  [anon_sym_similarity] = "similarity",
  [anon_sym_index2] = "index",
  [aux_sym_similarity_token1] = "similarity_token1",
  [anon_sym_PERCENT] = "%",
  [anon_sym_DASH_DASH_DASH] = "---",
  [anon_sym_PLUS_PLUS_PLUS] = "+++",
  [anon_sym_AT_AT] = "@@",
  [anon_sym_AT_AT2] = "@@",
  [aux_sym_location_token1] = "location_token1",
  [anon_sym_PLUS] = "+",
  [anon_sym_PLUS_PLUS] = "++",
  [anon_sym_PLUS_PLUS_PLUS_PLUS] = "++++",
  [anon_sym_DASH] = "-",
  [anon_sym_DASH_DASH] = "--",
  [anon_sym_DASH_DASH_DASH_DASH] = "----",
  [sym_context] = "context",
  [anon_sym_POUND] = "#",
  [sym_linerange] = "linerange",
  [aux_sym_filename_token1] = "filename_token1",
  [sym_commit] = "commit",
  [sym_source] = "source",
  [sym__line] = "_line",
  [sym_block] = "block",
  [sym_hunks] = "hunks",
  [sym_hunk] = "hunk",
  [sym_changes] = "changes",
  [sym_command] = "command",
  [sym_file_change] = "file_change",
  [sym_binary_change] = "binary_change",
  [sym_index] = "index",
  [sym_similarity] = "similarity",
  [sym_old_file] = "old_file",
  [sym_new_file] = "new_file",
  [sym_location] = "location",
  [sym_addition] = "addition",
  [sym_deletion] = "deletion",
  [sym_comment] = "comment",
  [sym_filename] = "filename",
  [sym_mode] = "mode",
  [aux_sym_source_repeat1] = "source_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym_hunks_repeat1] = "hunks_repeat1",
  [aux_sym_changes_repeat1] = "changes_repeat1",
  [aux_sym_changes_repeat2] = "changes_repeat2",
  [aux_sym_filename_repeat1] = "filename_repeat1",
  [alias_sym_score] = "score",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_source_token1] = aux_sym_source_token1,
  [anon_sym_diff] = anon_sym_diff,
  [aux_sym_command_token1] = aux_sym_command_token1,
  [anon_sym_new] = anon_sym_new,
  [anon_sym_deleted] = anon_sym_deleted,
  [anon_sym_file] = anon_sym_file,
  [anon_sym_mode] = anon_sym_mode,
  [anon_sym_old] = anon_sym_old,
  [anon_sym_rename] = anon_sym_rename,
  [anon_sym_from] = anon_sym_from,
  [anon_sym_to] = anon_sym_to,
  [anon_sym_Binary] = anon_sym_Binary,
  [anon_sym_files] = anon_sym_files,
  [anon_sym_and] = anon_sym_and,
  [anon_sym_differ] = anon_sym_differ,
  [anon_sym_index] = anon_sym_index,
  [anon_sym_DOT_DOT] = anon_sym_DOT_DOT,
  [anon_sym_similarity] = anon_sym_similarity,
  [anon_sym_index2] = anon_sym_index,
  [aux_sym_similarity_token1] = aux_sym_similarity_token1,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_DASH_DASH_DASH] = anon_sym_DASH_DASH_DASH,
  [anon_sym_PLUS_PLUS_PLUS] = anon_sym_PLUS_PLUS_PLUS,
  [anon_sym_AT_AT] = anon_sym_AT_AT,
  [anon_sym_AT_AT2] = anon_sym_AT_AT,
  [aux_sym_location_token1] = aux_sym_location_token1,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_PLUS_PLUS] = anon_sym_PLUS_PLUS,
  [anon_sym_PLUS_PLUS_PLUS_PLUS] = anon_sym_PLUS_PLUS_PLUS_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_DASH_DASH] = anon_sym_DASH_DASH,
  [anon_sym_DASH_DASH_DASH_DASH] = anon_sym_DASH_DASH_DASH_DASH,
  [sym_context] = sym_context,
  [anon_sym_POUND] = anon_sym_POUND,
  [sym_linerange] = sym_linerange,
  [aux_sym_filename_token1] = aux_sym_filename_token1,
  [sym_commit] = sym_commit,
  [sym_source] = sym_source,
  [sym__line] = sym__line,
  [sym_block] = sym_block,
  [sym_hunks] = sym_hunks,
  [sym_hunk] = sym_hunk,
  [sym_changes] = sym_changes,
  [sym_command] = sym_command,
  [sym_file_change] = sym_file_change,
  [sym_binary_change] = sym_binary_change,
  [sym_index] = sym_index,
  [sym_similarity] = sym_similarity,
  [sym_old_file] = sym_old_file,
  [sym_new_file] = sym_new_file,
  [sym_location] = sym_location,
  [sym_addition] = sym_addition,
  [sym_deletion] = sym_deletion,
  [sym_comment] = sym_comment,
  [sym_filename] = sym_filename,
  [sym_mode] = sym_mode,
  [aux_sym_source_repeat1] = aux_sym_source_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym_hunks_repeat1] = aux_sym_hunks_repeat1,
  [aux_sym_changes_repeat1] = aux_sym_changes_repeat1,
  [aux_sym_changes_repeat2] = aux_sym_changes_repeat2,
  [aux_sym_filename_repeat1] = aux_sym_filename_repeat1,
  [alias_sym_score] = alias_sym_score,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_diff] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_command_token1] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_new] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_deleted] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_file] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mode] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_old] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rename] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_from] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_to] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Binary] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_files] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_and] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_differ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_index] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_similarity] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_index2] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_similarity_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_DASH_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS_PLUS_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT_AT2] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_location_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS_PLUS_PLUS_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_DASH_DASH_DASH] = {
    .visible = true,
    .named = false,
  },
  [sym_context] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [sym_linerange] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_filename_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_commit] = {
    .visible = true,
    .named = true,
  },
  [sym_source] = {
    .visible = true,
    .named = true,
  },
  [sym__line] = {
    .visible = false,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_hunks] = {
    .visible = true,
    .named = true,
  },
  [sym_hunk] = {
    .visible = true,
    .named = true,
  },
  [sym_changes] = {
    .visible = true,
    .named = true,
  },
  [sym_command] = {
    .visible = true,
    .named = true,
  },
  [sym_file_change] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_change] = {
    .visible = true,
    .named = true,
  },
  [sym_index] = {
    .visible = true,
    .named = true,
  },
  [sym_similarity] = {
    .visible = true,
    .named = true,
  },
  [sym_old_file] = {
    .visible = true,
    .named = true,
  },
  [sym_new_file] = {
    .visible = true,
    .named = true,
  },
  [sym_location] = {
    .visible = true,
    .named = true,
  },
  [sym_addition] = {
    .visible = true,
    .named = true,
  },
  [sym_deletion] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_filename] = {
    .visible = true,
    .named = true,
  },
  [sym_mode] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_hunks_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_changes_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_changes_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_filename_repeat1] = {
    .visible = false,
    .named = false,
  },
  [alias_sym_score] = {
    .visible = true,
    .named = true,
  },
};

enum ts_field_identifiers {
  field_changes = 1,
  field_location = 2,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_changes] = "changes",
  [field_location] = "location",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [2] = {.index = 0, .length = 1},
  [3] = {.index = 1, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_location, 0},
  [1] =
    {field_changes, 2},
    {field_location, 0},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
  [1] = {
    [2] = alias_sym_score,
  },
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
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
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
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 24,
  [37] = 25,
  [38] = 24,
  [39] = 25,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(88);
      ADVANCE_MAP(
        '\n', 89,
        '\r', 1,
        '#', 204,
        '%', 151,
        '+', 159,
        '-', 162,
        '.', 4,
        '@', 5,
        'B', 38,
        'a', 54,
        'd', 18,
        'f', 39,
        'i', 56,
        'm', 61,
        'n', 20,
        'o', 46,
        'r', 29,
        's', 37,
        't', 59,
        'b', 81,
        'c', 81,
        'e', 81,
      );
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') SKIP(83);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(121);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(89);
      END_STATE();
    case 2:
      if (lookahead == '+') ADVANCE(3);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(205);
      END_STATE();
    case 3:
      if (lookahead == '+') ADVANCE(153);
      END_STATE();
    case 4:
      if (lookahead == '.') ADVANCE(108);
      END_STATE();
    case 5:
      if (lookahead == '@') ADVANCE(155);
      END_STATE();
    case 6:
      if (lookahead == '@') ADVANCE(156);
      END_STATE();
    case 7:
      if (lookahead == 'a') ADVANCE(62);
      END_STATE();
    case 8:
      if (lookahead == 'a') ADVANCE(212);
      if (lookahead == '\t' ||
          lookahead == 0x0b ||
          lookahead == '\f' ||
          lookahead == ' ') SKIP(8);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead)) ADVANCE(214);
      END_STATE();
    case 9:
      if (lookahead == 'a') ADVANCE(64);
      END_STATE();
    case 10:
      if (lookahead == 'a') ADVANCE(53);
      END_STATE();
    case 11:
      if (lookahead == 'd') ADVANCE(103);
      END_STATE();
    case 12:
      if (lookahead == 'd') ADVANCE(97);
      END_STATE();
    case 13:
      if (lookahead == 'd') ADVANCE(93);
      END_STATE();
    case 14:
      if (lookahead == 'd') ADVANCE(211);
      if (lookahead == '\t' ||
          lookahead == 0x0b ||
          lookahead == '\f' ||
          lookahead == ' ') SKIP(14);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead)) ADVANCE(214);
      END_STATE();
    case 15:
      if (lookahead == 'd') ADVANCE(23);
      END_STATE();
    case 16:
      if (lookahead == 'd') ADVANCE(24);
      END_STATE();
    case 17:
      if (lookahead == 'd') ADVANCE(28);
      END_STATE();
    case 18:
      if (lookahead == 'e') ADVANCE(45);
      if (lookahead == 'i') ADVANCE(34);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(80);
      END_STATE();
    case 19:
      if (lookahead == 'e') ADVANCE(45);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(80);
      END_STATE();
    case 20:
      if (lookahead == 'e') ADVANCE(68);
      END_STATE();
    case 21:
      if (lookahead == 'e') ADVANCE(67);
      END_STATE();
    case 22:
      if (lookahead == 'e') ADVANCE(95);
      END_STATE();
    case 23:
      if (lookahead == 'e') ADVANCE(69);
      END_STATE();
    case 24:
      if (lookahead == 'e') ADVANCE(96);
      END_STATE();
    case 25:
      if (lookahead == 'e') ADVANCE(98);
      END_STATE();
    case 26:
      if (lookahead == 'e') ADVANCE(94);
      END_STATE();
    case 27:
      if (lookahead == 'e') ADVANCE(65);
      END_STATE();
    case 28:
      if (lookahead == 'e') ADVANCE(70);
      END_STATE();
    case 29:
      if (lookahead == 'e') ADVANCE(57);
      END_STATE();
    case 30:
      if (lookahead == 'e') ADVANCE(63);
      END_STATE();
    case 31:
      if (lookahead == 'e') ADVANCE(13);
      END_STATE();
    case 32:
      if (lookahead == 'f') ADVANCE(90);
      END_STATE();
    case 33:
      if (lookahead == 'f') ADVANCE(44);
      if (lookahead == '\t' ||
          lookahead == 0x0b ||
          lookahead == '\f' ||
          lookahead == ' ') SKIP(33);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'e')) ADVANCE(81);
      END_STATE();
    case 34:
      if (lookahead == 'f') ADVANCE(32);
      END_STATE();
    case 35:
      if (lookahead == 'f') ADVANCE(36);
      END_STATE();
    case 36:
      if (lookahead == 'f') ADVANCE(30);
      END_STATE();
    case 37:
      if (lookahead == 'i') ADVANCE(52);
      END_STATE();
    case 38:
      if (lookahead == 'i') ADVANCE(55);
      END_STATE();
    case 39:
      if (lookahead == 'i') ADVANCE(47);
      if (lookahead == 'r') ADVANCE(60);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(80);
      END_STATE();
    case 40:
      if (lookahead == 'i') ADVANCE(66);
      END_STATE();
    case 41:
      if (lookahead == 'i') ADVANCE(50);
      END_STATE();
    case 42:
      if (lookahead == 'i') ADVANCE(35);
      END_STATE();
    case 43:
      if (lookahead == 'i') ADVANCE(48);
      END_STATE();
    case 44:
      if (lookahead == 'i') ADVANCE(49);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(80);
      END_STATE();
    case 45:
      if (lookahead == 'l') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(79);
      END_STATE();
    case 46:
      if (lookahead == 'l') ADVANCE(12);
      END_STATE();
    case 47:
      if (lookahead == 'l') ADVANCE(22);
      END_STATE();
    case 48:
      if (lookahead == 'l') ADVANCE(26);
      END_STATE();
    case 49:
      if (lookahead == 'l') ADVANCE(27);
      END_STATE();
    case 50:
      if (lookahead == 'l') ADVANCE(9);
      END_STATE();
    case 51:
      if (lookahead == 'm') ADVANCE(99);
      END_STATE();
    case 52:
      if (lookahead == 'm') ADVANCE(41);
      END_STATE();
    case 53:
      if (lookahead == 'm') ADVANCE(25);
      END_STATE();
    case 54:
      if (lookahead == 'n') ADVANCE(11);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(80);
      END_STATE();
    case 55:
      if (lookahead == 'n') ADVANCE(7);
      END_STATE();
    case 56:
      if (lookahead == 'n') ADVANCE(15);
      END_STATE();
    case 57:
      if (lookahead == 'n') ADVANCE(10);
      END_STATE();
    case 58:
      if (lookahead == 'n') ADVANCE(17);
      END_STATE();
    case 59:
      if (lookahead == 'o') ADVANCE(100);
      END_STATE();
    case 60:
      if (lookahead == 'o') ADVANCE(51);
      END_STATE();
    case 61:
      if (lookahead == 'o') ADVANCE(16);
      END_STATE();
    case 62:
      if (lookahead == 'r') ADVANCE(71);
      END_STATE();
    case 63:
      if (lookahead == 'r') ADVANCE(105);
      END_STATE();
    case 64:
      if (lookahead == 'r') ADVANCE(40);
      END_STATE();
    case 65:
      if (lookahead == 's') ADVANCE(102);
      END_STATE();
    case 66:
      if (lookahead == 't') ADVANCE(72);
      END_STATE();
    case 67:
      if (lookahead == 't') ADVANCE(31);
      END_STATE();
    case 68:
      if (lookahead == 'w') ADVANCE(92);
      END_STATE();
    case 69:
      if (lookahead == 'x') ADVANCE(107);
      END_STATE();
    case 70:
      if (lookahead == 'x') ADVANCE(110);
      END_STATE();
    case 71:
      if (lookahead == 'y') ADVANCE(101);
      END_STATE();
    case 72:
      if (lookahead == 'y') ADVANCE(109);
      END_STATE();
    case 73:
      if (lookahead == '\t' ||
          lookahead == 0x0b ||
          lookahead == '\f' ||
          lookahead == ' ') SKIP(73);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(91);
      END_STATE();
    case 74:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(205);
      END_STATE();
    case 75:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(206);
      END_STATE();
    case 76:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(248);
      END_STATE();
    case 77:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(76);
      END_STATE();
    case 78:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(77);
      END_STATE();
    case 79:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(78);
      END_STATE();
    case 80:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(79);
      END_STATE();
    case 81:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(80);
      END_STATE();
    case 82:
      if (eof) ADVANCE(88);
      ADVANCE_MAP(
        '\n', 89,
        '\r', 1,
        '#', 204,
        '+', 159,
        '-', 162,
        '@', 166,
        'B', 184,
        'd', 177,
        'i', 192,
        'n', 174,
        'o', 187,
        'r', 179,
        's', 183,
        '\t', 165,
        0x0b, 165,
        '\f', 165,
        ' ', 165,
      );
      if (lookahead != 0) ADVANCE(203);
      END_STATE();
    case 83:
      if (eof) ADVANCE(88);
      ADVANCE_MAP(
        '\n', 89,
        '\r', 1,
        '%', 151,
        '.', 4,
        '@', 6,
        'a', 54,
        'd', 19,
        'f', 39,
        'i', 58,
        'm', 61,
        'n', 20,
        'o', 46,
        'r', 29,
        't', 59,
        'b', 81,
        'c', 81,
        'e', 81,
      );
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') SKIP(83);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(121);
      END_STATE();
    case 84:
      if (eof) ADVANCE(88);
      ADVANCE_MAP(
        '\n', 89,
        '\r', 1,
        '+', 2,
        '-', 74,
        '@', 6,
        'd', 42,
        'f', 43,
        'i', 58,
        'm', 61,
      );
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') SKIP(85);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(150);
      END_STATE();
    case 85:
      if (eof) ADVANCE(88);
      ADVANCE_MAP(
        '\n', 89,
        '\r', 1,
        '@', 6,
        'd', 42,
        'f', 43,
        'i', 58,
        'm', 61,
        '+', 74,
        '-', 74,
      );
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') SKIP(85);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(150);
      END_STATE();
    case 86:
      if (eof) ADVANCE(88);
      if (lookahead == '\n') ADVANCE(89);
      if (lookahead == '\r') ADVANCE(1);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') SKIP(86);
      if (lookahead != 0) ADVANCE(214);
      END_STATE();
    case 87:
      if (eof) ADVANCE(88);
      if (lookahead == '\n') ADVANCE(89);
      if (lookahead == '\r') ADVANCE(1);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(157);
      if (lookahead != 0) ADVANCE(158);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(aux_sym_source_token1);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_diff);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(aux_sym_command_token1);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(91);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_new);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_deleted);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_file);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_file);
      if (lookahead == 's') ADVANCE(102);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_mode);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_old);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_rename);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(anon_sym_from);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_to);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_Binary);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_files);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_and);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_and);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_differ);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_differ);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_index);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_similarity);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_index2);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(248);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(149);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(215);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(150);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(76);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(111);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(216);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(112);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(77);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(113);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(217);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(114);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(78);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(115);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(218);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(116);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(79);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(117);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(219);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(118);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(80);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(119);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(220);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(120);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(221);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(122);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(222);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(123);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(223);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(124);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(224);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(125);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(225);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(126);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(226);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(127);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(227);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(128);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(228);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(129);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(229);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(130);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(230);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(131);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(231);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(132);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(232);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(133);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(233);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(134);
      END_STATE();
    case 136:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(234);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(135);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(235);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(136);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(236);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(137);
      END_STATE();
    case 139:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(237);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(138);
      END_STATE();
    case 140:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(238);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(139);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(239);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(140);
      END_STATE();
    case 142:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(240);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(141);
      END_STATE();
    case 143:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(241);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(142);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(242);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(143);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(243);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(144);
      END_STATE();
    case 146:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(244);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(145);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(245);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(146);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(246);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(147);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('a' <= lookahead && lookahead <= 'f')) ADVANCE(247);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(148);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(aux_sym_similarity_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(150);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(anon_sym_DASH_DASH_DASH);
      if (lookahead == '-') ADVANCE(164);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_PLUS_PLUS_PLUS);
      END_STATE();
    case 154:
      ACCEPT_TOKEN(anon_sym_PLUS_PLUS_PLUS);
      if (lookahead == '+') ADVANCE(161);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(anon_sym_AT_AT);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(anon_sym_AT_AT2);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(aux_sym_location_token1);
      if (lookahead == '\t' ||
          lookahead == 0x0b ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(157);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead)) ADVANCE(158);
      END_STATE();
    case 158:
      ACCEPT_TOKEN(aux_sym_location_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(158);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(anon_sym_PLUS);
      if (lookahead == '+') ADVANCE(160);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(anon_sym_PLUS_PLUS);
      if (lookahead == '+') ADVANCE(154);
      END_STATE();
    case 161:
      ACCEPT_TOKEN(anon_sym_PLUS_PLUS_PLUS_PLUS);
      END_STATE();
    case 162:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '-') ADVANCE(163);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(anon_sym_DASH_DASH);
      if (lookahead == '-') ADVANCE(152);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(anon_sym_DASH_DASH_DASH_DASH);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(sym_context);
      ADVANCE_MAP(
        '\n', 89,
        '\r', 1,
        'd', 178,
        'n', 174,
        'o', 187,
        'r', 179,
        '\t', 165,
        0x0b, 165,
        '\f', 165,
        ' ', 165,
      );
      if (lookahead != 0) ADVANCE(203);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == '@') ADVANCE(155);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'a') ADVANCE(195);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'a') ADVANCE(191);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'a') ADVANCE(196);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'd') ADVANCE(97);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'd') ADVANCE(93);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'd') ADVANCE(176);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'e') ADVANCE(98);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'e') ADVANCE(199);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'e') ADVANCE(198);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'e') ADVANCE(200);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'e') ADVANCE(188);
      if (lookahead == 'i') ADVANCE(182);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'e') ADVANCE(188);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'e') ADVANCE(194);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'e') ADVANCE(171);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'f') ADVANCE(90);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'f') ADVANCE(181);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'i') ADVANCE(190);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'i') ADVANCE(193);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'i') ADVANCE(197);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 186:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'i') ADVANCE(189);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'l') ADVANCE(170);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'l') ADVANCE(175);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 189:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'l') ADVANCE(169);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 190:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'm') ADVANCE(186);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'm') ADVANCE(173);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'n') ADVANCE(172);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'n') ADVANCE(167);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'n') ADVANCE(168);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'r') ADVANCE(201);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'r') ADVANCE(185);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 't') ADVANCE(202);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 198:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 't') ADVANCE(180);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 199:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'w') ADVANCE(92);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 200:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'x') ADVANCE(107);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'y') ADVANCE(101);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 202:
      ACCEPT_TOKEN(sym_context);
      if (lookahead == 'y') ADVANCE(109);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 203:
      ACCEPT_TOKEN(sym_context);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(203);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(sym_linerange);
      if (lookahead == ',') ADVANCE(75);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(205);
      END_STATE();
    case 206:
      ACCEPT_TOKEN(sym_linerange);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(206);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(aux_sym_filename_token1);
      if (lookahead == 'd') ADVANCE(104);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(aux_sym_filename_token1);
      if (lookahead == 'e') ADVANCE(213);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 209:
      ACCEPT_TOKEN(aux_sym_filename_token1);
      if (lookahead == 'f') ADVANCE(208);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(aux_sym_filename_token1);
      if (lookahead == 'f') ADVANCE(209);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(aux_sym_filename_token1);
      if (lookahead == 'i') ADVANCE(210);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 212:
      ACCEPT_TOKEN(aux_sym_filename_token1);
      if (lookahead == 'n') ADVANCE(207);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(aux_sym_filename_token1);
      if (lookahead == 'r') ADVANCE(106);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(aux_sym_filename_token1);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(214);
      END_STATE();
    case 215:
      ACCEPT_TOKEN(sym_commit);
      END_STATE();
    case 216:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(215);
      END_STATE();
    case 217:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(216);
      END_STATE();
    case 218:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(217);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(218);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(219);
      END_STATE();
    case 221:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(220);
      END_STATE();
    case 222:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(221);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(222);
      END_STATE();
    case 224:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(223);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(224);
      END_STATE();
    case 226:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(225);
      END_STATE();
    case 227:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(226);
      END_STATE();
    case 228:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(227);
      END_STATE();
    case 229:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(228);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(229);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(230);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(231);
      END_STATE();
    case 233:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(232);
      END_STATE();
    case 234:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(233);
      END_STATE();
    case 235:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(234);
      END_STATE();
    case 236:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(235);
      END_STATE();
    case 237:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(236);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(237);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(238);
      END_STATE();
    case 240:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(239);
      END_STATE();
    case 241:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(240);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(241);
      END_STATE();
    case 243:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(242);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(243);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(244);
      END_STATE();
    case 246:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(245);
      END_STATE();
    case 247:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(246);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(247);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 82},
  [2] = {.lex_state = 82},
  [3] = {.lex_state = 82},
  [4] = {.lex_state = 82},
  [5] = {.lex_state = 82},
  [6] = {.lex_state = 82},
  [7] = {.lex_state = 82},
  [8] = {.lex_state = 82},
  [9] = {.lex_state = 82},
  [10] = {.lex_state = 82},
  [11] = {.lex_state = 82},
  [12] = {.lex_state = 82},
  [13] = {.lex_state = 82},
  [14] = {.lex_state = 82},
  [15] = {.lex_state = 82},
  [16] = {.lex_state = 82},
  [17] = {.lex_state = 82},
  [18] = {.lex_state = 82},
  [19] = {.lex_state = 86},
  [20] = {.lex_state = 86},
  [21] = {.lex_state = 82},
  [22] = {.lex_state = 82},
  [23] = {.lex_state = 84},
  [24] = {.lex_state = 86},
  [25] = {.lex_state = 86},
  [26] = {.lex_state = 86},
  [27] = {.lex_state = 86},
  [28] = {.lex_state = 86},
  [29] = {.lex_state = 87},
  [30] = {.lex_state = 87},
  [31] = {.lex_state = 86},
  [32] = {.lex_state = 86},
  [33] = {.lex_state = 87},
  [34] = {.lex_state = 86},
  [35] = {.lex_state = 87},
  [36] = {.lex_state = 8},
  [37] = {.lex_state = 8},
  [38] = {.lex_state = 14},
  [39] = {.lex_state = 14},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 84},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 84},
  [52] = {.lex_state = 84},
  [53] = {.lex_state = 0},
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 0},
  [56] = {.lex_state = 84},
  [57] = {.lex_state = 84},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 84},
  [63] = {.lex_state = 33},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 33},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 0},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 84},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 84},
  [73] = {.lex_state = 84},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 0},
  [77] = {.lex_state = 0},
  [78] = {.lex_state = 0},
  [79] = {.lex_state = 84},
  [80] = {.lex_state = 0},
  [81] = {.lex_state = 0},
  [82] = {.lex_state = 84},
  [83] = {.lex_state = 73},
  [84] = {.lex_state = 33},
  [85] = {.lex_state = 0},
  [86] = {.lex_state = 0},
  [87] = {.lex_state = 0},
  [88] = {.lex_state = 84},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_source_token1] = ACTIONS(1),
    [anon_sym_diff] = ACTIONS(1),
    [anon_sym_new] = ACTIONS(1),
    [anon_sym_deleted] = ACTIONS(1),
    [anon_sym_file] = ACTIONS(1),
    [anon_sym_mode] = ACTIONS(1),
    [anon_sym_old] = ACTIONS(1),
    [anon_sym_rename] = ACTIONS(1),
    [anon_sym_from] = ACTIONS(1),
    [anon_sym_to] = ACTIONS(1),
    [anon_sym_Binary] = ACTIONS(1),
    [anon_sym_files] = ACTIONS(1),
    [anon_sym_and] = ACTIONS(1),
    [anon_sym_index] = ACTIONS(1),
    [anon_sym_DOT_DOT] = ACTIONS(1),
    [anon_sym_similarity] = ACTIONS(1),
    [anon_sym_index2] = ACTIONS(1),
    [aux_sym_similarity_token1] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_DASH_DASH_DASH] = ACTIONS(1),
    [anon_sym_PLUS_PLUS_PLUS] = ACTIONS(1),
    [anon_sym_AT_AT] = ACTIONS(1),
    [anon_sym_AT_AT2] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_PLUS_PLUS] = ACTIONS(1),
    [anon_sym_PLUS_PLUS_PLUS_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_DASH_DASH] = ACTIONS(1),
    [anon_sym_DASH_DASH_DASH_DASH] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(1),
    [sym_commit] = ACTIONS(1),
  },
  [1] = {
    [sym_source] = STATE(68),
    [sym__line] = STATE(48),
    [sym_block] = STATE(2),
    [sym_command] = STATE(75),
    [sym_file_change] = STATE(48),
    [sym_binary_change] = STATE(48),
    [sym_index] = STATE(48),
    [sym_similarity] = STATE(48),
    [sym_old_file] = STATE(48),
    [sym_new_file] = STATE(48),
    [sym_location] = STATE(48),
    [sym_addition] = STATE(48),
    [sym_deletion] = STATE(48),
    [sym_comment] = STATE(48),
    [aux_sym_source_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [aux_sym_source_token1] = ACTIONS(5),
    [anon_sym_diff] = ACTIONS(7),
    [anon_sym_new] = ACTIONS(9),
    [anon_sym_deleted] = ACTIONS(11),
    [anon_sym_old] = ACTIONS(13),
    [anon_sym_rename] = ACTIONS(15),
    [anon_sym_Binary] = ACTIONS(17),
    [anon_sym_index] = ACTIONS(19),
    [anon_sym_similarity] = ACTIONS(21),
    [anon_sym_DASH_DASH_DASH] = ACTIONS(23),
    [anon_sym_PLUS_PLUS_PLUS] = ACTIONS(25),
    [anon_sym_AT_AT] = ACTIONS(27),
    [anon_sym_PLUS] = ACTIONS(29),
    [anon_sym_PLUS_PLUS] = ACTIONS(29),
    [anon_sym_PLUS_PLUS_PLUS_PLUS] = ACTIONS(31),
    [anon_sym_DASH] = ACTIONS(33),
    [anon_sym_DASH_DASH] = ACTIONS(33),
    [anon_sym_DASH_DASH_DASH_DASH] = ACTIONS(35),
    [sym_context] = ACTIONS(37),
    [anon_sym_POUND] = ACTIONS(39),
  },
  [2] = {
    [sym__line] = STATE(53),
    [sym_block] = STATE(3),
    [sym_command] = STATE(75),
    [sym_file_change] = STATE(53),
    [sym_binary_change] = STATE(53),
    [sym_index] = STATE(53),
    [sym_similarity] = STATE(53),
    [sym_old_file] = STATE(53),
    [sym_new_file] = STATE(53),
    [sym_location] = STATE(53),
    [sym_addition] = STATE(53),
    [sym_deletion] = STATE(53),
    [sym_comment] = STATE(53),
    [aux_sym_source_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(41),
    [aux_sym_source_token1] = ACTIONS(43),
    [anon_sym_diff] = ACTIONS(7),
    [anon_sym_new] = ACTIONS(9),
    [anon_sym_deleted] = ACTIONS(11),
    [anon_sym_old] = ACTIONS(13),
    [anon_sym_rename] = ACTIONS(15),
    [anon_sym_Binary] = ACTIONS(17),
    [anon_sym_index] = ACTIONS(19),
    [anon_sym_similarity] = ACTIONS(21),
    [anon_sym_DASH_DASH_DASH] = ACTIONS(23),
    [anon_sym_PLUS_PLUS_PLUS] = ACTIONS(25),
    [anon_sym_AT_AT] = ACTIONS(27),
    [anon_sym_PLUS] = ACTIONS(29),
    [anon_sym_PLUS_PLUS] = ACTIONS(29),
    [anon_sym_PLUS_PLUS_PLUS_PLUS] = ACTIONS(31),
    [anon_sym_DASH] = ACTIONS(33),
    [anon_sym_DASH_DASH] = ACTIONS(33),
    [anon_sym_DASH_DASH_DASH_DASH] = ACTIONS(35),
    [sym_context] = ACTIONS(45),
    [anon_sym_POUND] = ACTIONS(39),
  },
  [3] = {
    [sym__line] = STATE(76),
    [sym_block] = STATE(3),
    [sym_command] = STATE(75),
    [sym_file_change] = STATE(76),
    [sym_binary_change] = STATE(76),
    [sym_index] = STATE(76),
    [sym_similarity] = STATE(76),
    [sym_old_file] = STATE(76),
    [sym_new_file] = STATE(76),
    [sym_location] = STATE(76),
    [sym_addition] = STATE(76),
    [sym_deletion] = STATE(76),
    [sym_comment] = STATE(76),
    [aux_sym_source_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(47),
    [aux_sym_source_token1] = ACTIONS(49),
    [anon_sym_diff] = ACTIONS(52),
    [anon_sym_new] = ACTIONS(55),
    [anon_sym_deleted] = ACTIONS(58),
    [anon_sym_old] = ACTIONS(61),
    [anon_sym_rename] = ACTIONS(64),
    [anon_sym_Binary] = ACTIONS(67),
    [anon_sym_index] = ACTIONS(70),
    [anon_sym_similarity] = ACTIONS(73),
    [anon_sym_DASH_DASH_DASH] = ACTIONS(76),
    [anon_sym_PLUS_PLUS_PLUS] = ACTIONS(79),
    [anon_sym_AT_AT] = ACTIONS(82),
    [anon_sym_PLUS] = ACTIONS(85),
    [anon_sym_PLUS_PLUS] = ACTIONS(85),
    [anon_sym_PLUS_PLUS_PLUS_PLUS] = ACTIONS(88),
    [anon_sym_DASH] = ACTIONS(91),
    [anon_sym_DASH_DASH] = ACTIONS(91),
    [anon_sym_DASH_DASH_DASH_DASH] = ACTIONS(94),
    [sym_context] = ACTIONS(97),
    [anon_sym_POUND] = ACTIONS(100),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 13,
    ACTIONS(9), 1,
      anon_sym_new,
    ACTIONS(11), 1,
      anon_sym_deleted,
    ACTIONS(13), 1,
      anon_sym_old,
    ACTIONS(15), 1,
      anon_sym_rename,
    ACTIONS(17), 1,
      anon_sym_Binary,
    ACTIONS(19), 1,
      anon_sym_index,
    ACTIONS(21), 1,
      anon_sym_similarity,
    ACTIONS(107), 1,
      anon_sym_DASH_DASH_DASH,
    STATE(5), 1,
      aux_sym_block_repeat1,
    STATE(74), 1,
      sym_old_file,
    STATE(61), 4,
      sym_file_change,
      sym_binary_change,
      sym_index,
      sym_similarity,
    ACTIONS(103), 6,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(105), 7,
      aux_sym_source_token1,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [54] = 13,
    ACTIONS(9), 1,
      anon_sym_new,
    ACTIONS(11), 1,
      anon_sym_deleted,
    ACTIONS(13), 1,
      anon_sym_old,
    ACTIONS(15), 1,
      anon_sym_rename,
    ACTIONS(17), 1,
      anon_sym_Binary,
    ACTIONS(19), 1,
      anon_sym_index,
    ACTIONS(21), 1,
      anon_sym_similarity,
    ACTIONS(107), 1,
      anon_sym_DASH_DASH_DASH,
    STATE(6), 1,
      aux_sym_block_repeat1,
    STATE(86), 1,
      sym_old_file,
    STATE(61), 4,
      sym_file_change,
      sym_binary_change,
      sym_index,
      sym_similarity,
    ACTIONS(109), 6,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(111), 7,
      aux_sym_source_token1,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [108] = 11,
    ACTIONS(117), 1,
      anon_sym_new,
    ACTIONS(120), 1,
      anon_sym_deleted,
    ACTIONS(123), 1,
      anon_sym_old,
    ACTIONS(126), 1,
      anon_sym_rename,
    ACTIONS(129), 1,
      anon_sym_Binary,
    ACTIONS(132), 1,
      anon_sym_index,
    ACTIONS(135), 1,
      anon_sym_similarity,
    STATE(6), 1,
      aux_sym_block_repeat1,
    STATE(61), 4,
      sym_file_change,
      sym_binary_change,
      sym_index,
      sym_similarity,
    ACTIONS(113), 6,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(115), 8,
      aux_sym_source_token1,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [157] = 12,
    ACTIONS(31), 1,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
    ACTIONS(35), 1,
      anon_sym_DASH_DASH_DASH_DASH,
    ACTIONS(142), 1,
      anon_sym_DASH_DASH_DASH,
    ACTIONS(144), 1,
      anon_sym_PLUS_PLUS_PLUS,
    ACTIONS(146), 1,
      sym_context,
    STATE(11), 1,
      aux_sym_changes_repeat2,
    STATE(18), 1,
      sym_changes,
    ACTIONS(29), 2,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
    ACTIONS(33), 2,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
    STATE(40), 2,
      sym_addition,
      sym_deletion,
    ACTIONS(140), 5,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
    ACTIONS(138), 7,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_POUND,
  [207] = 5,
    ACTIONS(152), 1,
      anon_sym_AT_AT,
    STATE(71), 1,
      sym_location,
    STATE(8), 2,
      sym_hunk,
      aux_sym_hunks_repeat1,
    ACTIONS(148), 8,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(150), 12,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [242] = 5,
    ACTIONS(27), 1,
      anon_sym_AT_AT,
    STATE(71), 1,
      sym_location,
    STATE(8), 2,
      sym_hunk,
      aux_sym_hunks_repeat1,
    ACTIONS(155), 8,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(157), 12,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [277] = 11,
    ACTIONS(163), 1,
      anon_sym_DASH_DASH_DASH,
    ACTIONS(166), 1,
      anon_sym_PLUS_PLUS_PLUS,
    ACTIONS(172), 1,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
    ACTIONS(178), 1,
      anon_sym_DASH_DASH_DASH_DASH,
    ACTIONS(181), 1,
      sym_context,
    STATE(10), 1,
      aux_sym_changes_repeat2,
    ACTIONS(169), 2,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
    ACTIONS(175), 2,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
    STATE(40), 2,
      sym_addition,
      sym_deletion,
    ACTIONS(161), 5,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
    ACTIONS(159), 7,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_POUND,
  [324] = 11,
    ACTIONS(31), 1,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
    ACTIONS(35), 1,
      anon_sym_DASH_DASH_DASH_DASH,
    ACTIONS(142), 1,
      anon_sym_DASH_DASH_DASH,
    ACTIONS(144), 1,
      anon_sym_PLUS_PLUS_PLUS,
    ACTIONS(146), 1,
      sym_context,
    STATE(10), 1,
      aux_sym_changes_repeat2,
    ACTIONS(29), 2,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
    ACTIONS(33), 2,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
    STATE(40), 2,
      sym_addition,
      sym_deletion,
    ACTIONS(186), 5,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
    ACTIONS(184), 7,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_POUND,
  [371] = 4,
    ACTIONS(188), 1,
      aux_sym_source_token1,
    STATE(13), 1,
      aux_sym_changes_repeat1,
    ACTIONS(159), 9,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(161), 11,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [402] = 4,
    ACTIONS(192), 1,
      aux_sym_source_token1,
    STATE(13), 1,
      aux_sym_changes_repeat1,
    ACTIONS(190), 9,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(195), 11,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [433] = 2,
    ACTIONS(47), 9,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(197), 12,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [459] = 2,
    ACTIONS(113), 9,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(115), 12,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [485] = 2,
    ACTIONS(199), 9,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(201), 12,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [511] = 2,
    ACTIONS(203), 9,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(205), 12,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [537] = 2,
    ACTIONS(207), 9,
      ts_builtin_sym_end,
      anon_sym_diff,
      anon_sym_Binary,
      anon_sym_index,
      anon_sym_similarity,
      anon_sym_AT_AT,
      anon_sym_PLUS_PLUS_PLUS_PLUS,
      anon_sym_DASH_DASH_DASH_DASH,
      anon_sym_POUND,
    ACTIONS(209), 12,
      aux_sym_source_token1,
      anon_sym_new,
      anon_sym_deleted,
      anon_sym_old,
      anon_sym_rename,
      anon_sym_DASH_DASH_DASH,
      anon_sym_PLUS_PLUS_PLUS,
      anon_sym_PLUS,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH,
      anon_sym_DASH_DASH,
      sym_context,
  [563] = 4,
    ACTIONS(213), 1,
      aux_sym_filename_token1,
    STATE(24), 1,
      aux_sym_filename_repeat1,
    STATE(55), 1,
      sym_filename,
    ACTIONS(211), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [577] = 4,
    ACTIONS(213), 1,
      aux_sym_filename_token1,
    STATE(24), 1,
      aux_sym_filename_repeat1,
    STATE(47), 1,
      sym_filename,
    ACTIONS(215), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [591] = 4,
    ACTIONS(27), 1,
      anon_sym_AT_AT,
    STATE(17), 1,
      sym_hunks,
    STATE(71), 1,
      sym_location,
    STATE(9), 2,
      sym_hunk,
      aux_sym_hunks_repeat1,
  [605] = 4,
    ACTIONS(27), 1,
      anon_sym_AT_AT,
    STATE(16), 1,
      sym_hunks,
    STATE(71), 1,
      sym_location,
    STATE(9), 2,
      sym_hunk,
      aux_sym_hunks_repeat1,
  [619] = 3,
    ACTIONS(219), 1,
      aux_sym_similarity_token1,
    STATE(59), 1,
      sym_mode,
    ACTIONS(217), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [630] = 3,
    ACTIONS(223), 1,
      aux_sym_filename_token1,
    STATE(25), 1,
      aux_sym_filename_repeat1,
    ACTIONS(221), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [641] = 3,
    ACTIONS(227), 1,
      aux_sym_filename_token1,
    STATE(25), 1,
      aux_sym_filename_repeat1,
    ACTIONS(225), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [652] = 3,
    ACTIONS(230), 1,
      aux_sym_filename_token1,
    STATE(36), 1,
      aux_sym_filename_repeat1,
    STATE(64), 1,
      sym_filename,
  [662] = 3,
    ACTIONS(213), 1,
      aux_sym_filename_token1,
    STATE(24), 1,
      aux_sym_filename_repeat1,
    STATE(41), 1,
      sym_filename,
  [672] = 3,
    ACTIONS(213), 1,
      aux_sym_filename_token1,
    STATE(24), 1,
      aux_sym_filename_repeat1,
    STATE(47), 1,
      sym_filename,
  [682] = 3,
    ACTIONS(211), 1,
      ts_builtin_sym_end,
    ACTIONS(232), 1,
      aux_sym_source_token1,
    ACTIONS(234), 1,
      aux_sym_location_token1,
  [692] = 3,
    ACTIONS(215), 1,
      ts_builtin_sym_end,
    ACTIONS(236), 1,
      aux_sym_source_token1,
    ACTIONS(238), 1,
      aux_sym_location_token1,
  [702] = 3,
    ACTIONS(240), 1,
      aux_sym_filename_token1,
    STATE(38), 1,
      aux_sym_filename_repeat1,
    STATE(88), 1,
      sym_filename,
  [712] = 3,
    ACTIONS(213), 1,
      aux_sym_filename_token1,
    STATE(24), 1,
      aux_sym_filename_repeat1,
    STATE(85), 1,
      sym_filename,
  [722] = 3,
    ACTIONS(242), 1,
      ts_builtin_sym_end,
    ACTIONS(244), 1,
      aux_sym_source_token1,
    ACTIONS(246), 1,
      aux_sym_location_token1,
  [732] = 3,
    ACTIONS(213), 1,
      aux_sym_filename_token1,
    STATE(24), 1,
      aux_sym_filename_repeat1,
    STATE(55), 1,
      sym_filename,
  [742] = 3,
    ACTIONS(248), 1,
      ts_builtin_sym_end,
    ACTIONS(250), 1,
      aux_sym_source_token1,
    ACTIONS(252), 1,
      aux_sym_location_token1,
  [752] = 3,
    ACTIONS(254), 1,
      anon_sym_and,
    ACTIONS(256), 1,
      aux_sym_filename_token1,
    STATE(37), 1,
      aux_sym_filename_repeat1,
  [762] = 3,
    ACTIONS(258), 1,
      anon_sym_and,
    ACTIONS(260), 1,
      aux_sym_filename_token1,
    STATE(37), 1,
      aux_sym_filename_repeat1,
  [772] = 3,
    ACTIONS(254), 1,
      anon_sym_differ,
    ACTIONS(263), 1,
      aux_sym_filename_token1,
    STATE(39), 1,
      aux_sym_filename_repeat1,
  [782] = 3,
    ACTIONS(258), 1,
      anon_sym_differ,
    ACTIONS(265), 1,
      aux_sym_filename_token1,
    STATE(39), 1,
      aux_sym_filename_repeat1,
  [792] = 2,
    ACTIONS(268), 1,
      aux_sym_source_token1,
    STATE(12), 1,
      aux_sym_changes_repeat1,
  [799] = 1,
    ACTIONS(270), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [804] = 2,
    ACTIONS(272), 1,
      anon_sym_PLUS_PLUS_PLUS,
    STATE(69), 1,
      sym_new_file,
  [811] = 1,
    ACTIONS(274), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [816] = 1,
    ACTIONS(276), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [821] = 1,
    ACTIONS(278), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [826] = 1,
    ACTIONS(280), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [831] = 1,
    ACTIONS(282), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [836] = 2,
    ACTIONS(41), 1,
      ts_builtin_sym_end,
    ACTIONS(284), 1,
      aux_sym_source_token1,
  [843] = 1,
    ACTIONS(286), 2,
      anon_sym_from,
      anon_sym_to,
  [848] = 1,
    ACTIONS(288), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [853] = 2,
    ACTIONS(290), 1,
      anon_sym_file,
    ACTIONS(292), 1,
      anon_sym_mode,
  [860] = 2,
    ACTIONS(219), 1,
      aux_sym_similarity_token1,
    STATE(41), 1,
      sym_mode,
  [867] = 2,
    ACTIONS(284), 1,
      aux_sym_source_token1,
    ACTIONS(294), 1,
      ts_builtin_sym_end,
  [874] = 1,
    ACTIONS(296), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [879] = 1,
    ACTIONS(298), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [884] = 2,
    ACTIONS(219), 1,
      aux_sym_similarity_token1,
    STATE(50), 1,
      sym_mode,
  [891] = 2,
    ACTIONS(272), 1,
      anon_sym_PLUS_PLUS_PLUS,
    STATE(65), 1,
      sym_new_file,
  [898] = 1,
    ACTIONS(300), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [903] = 1,
    ACTIONS(302), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [908] = 1,
    ACTIONS(304), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [913] = 1,
    ACTIONS(306), 1,
      aux_sym_source_token1,
  [917] = 1,
    ACTIONS(308), 1,
      aux_sym_similarity_token1,
  [921] = 1,
    ACTIONS(310), 1,
      sym_commit,
  [925] = 1,
    ACTIONS(312), 1,
      anon_sym_and,
  [929] = 1,
    ACTIONS(314), 1,
      aux_sym_source_token1,
  [933] = 1,
    ACTIONS(316), 1,
      sym_commit,
  [937] = 1,
    ACTIONS(318), 1,
      anon_sym_PERCENT,
  [941] = 1,
    ACTIONS(320), 1,
      ts_builtin_sym_end,
  [945] = 1,
    ACTIONS(322), 1,
      aux_sym_source_token1,
  [949] = 1,
    ACTIONS(324), 1,
      anon_sym_AT_AT2,
  [953] = 1,
    ACTIONS(326), 1,
      aux_sym_source_token1,
  [957] = 1,
    ACTIONS(328), 1,
      anon_sym_index2,
  [961] = 1,
    ACTIONS(330), 1,
      sym_linerange,
  [965] = 1,
    ACTIONS(332), 1,
      aux_sym_source_token1,
  [969] = 1,
    ACTIONS(334), 1,
      aux_sym_source_token1,
  [973] = 1,
    ACTIONS(284), 1,
      aux_sym_source_token1,
  [977] = 1,
    ACTIONS(215), 1,
      aux_sym_source_token1,
  [981] = 1,
    ACTIONS(211), 1,
      aux_sym_source_token1,
  [985] = 1,
    ACTIONS(290), 1,
      anon_sym_file,
  [989] = 1,
    ACTIONS(292), 1,
      anon_sym_mode,
  [993] = 1,
    ACTIONS(336), 1,
      anon_sym_mode,
  [997] = 1,
    ACTIONS(338), 1,
      sym_linerange,
  [1001] = 1,
    ACTIONS(340), 1,
      aux_sym_command_token1,
  [1005] = 1,
    ACTIONS(342), 1,
      anon_sym_files,
  [1009] = 1,
    ACTIONS(344), 1,
      aux_sym_source_token1,
  [1013] = 1,
    ACTIONS(346), 1,
      aux_sym_source_token1,
  [1017] = 1,
    ACTIONS(348), 1,
      anon_sym_DOT_DOT,
  [1021] = 1,
    ACTIONS(350), 1,
      anon_sym_differ,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 54,
  [SMALL_STATE(6)] = 108,
  [SMALL_STATE(7)] = 157,
  [SMALL_STATE(8)] = 207,
  [SMALL_STATE(9)] = 242,
  [SMALL_STATE(10)] = 277,
  [SMALL_STATE(11)] = 324,
  [SMALL_STATE(12)] = 371,
  [SMALL_STATE(13)] = 402,
  [SMALL_STATE(14)] = 433,
  [SMALL_STATE(15)] = 459,
  [SMALL_STATE(16)] = 485,
  [SMALL_STATE(17)] = 511,
  [SMALL_STATE(18)] = 537,
  [SMALL_STATE(19)] = 563,
  [SMALL_STATE(20)] = 577,
  [SMALL_STATE(21)] = 591,
  [SMALL_STATE(22)] = 605,
  [SMALL_STATE(23)] = 619,
  [SMALL_STATE(24)] = 630,
  [SMALL_STATE(25)] = 641,
  [SMALL_STATE(26)] = 652,
  [SMALL_STATE(27)] = 662,
  [SMALL_STATE(28)] = 672,
  [SMALL_STATE(29)] = 682,
  [SMALL_STATE(30)] = 692,
  [SMALL_STATE(31)] = 702,
  [SMALL_STATE(32)] = 712,
  [SMALL_STATE(33)] = 722,
  [SMALL_STATE(34)] = 732,
  [SMALL_STATE(35)] = 742,
  [SMALL_STATE(36)] = 752,
  [SMALL_STATE(37)] = 762,
  [SMALL_STATE(38)] = 772,
  [SMALL_STATE(39)] = 782,
  [SMALL_STATE(40)] = 792,
  [SMALL_STATE(41)] = 799,
  [SMALL_STATE(42)] = 804,
  [SMALL_STATE(43)] = 811,
  [SMALL_STATE(44)] = 816,
  [SMALL_STATE(45)] = 821,
  [SMALL_STATE(46)] = 826,
  [SMALL_STATE(47)] = 831,
  [SMALL_STATE(48)] = 836,
  [SMALL_STATE(49)] = 843,
  [SMALL_STATE(50)] = 848,
  [SMALL_STATE(51)] = 853,
  [SMALL_STATE(52)] = 860,
  [SMALL_STATE(53)] = 867,
  [SMALL_STATE(54)] = 874,
  [SMALL_STATE(55)] = 879,
  [SMALL_STATE(56)] = 884,
  [SMALL_STATE(57)] = 891,
  [SMALL_STATE(58)] = 898,
  [SMALL_STATE(59)] = 903,
  [SMALL_STATE(60)] = 908,
  [SMALL_STATE(61)] = 913,
  [SMALL_STATE(62)] = 917,
  [SMALL_STATE(63)] = 921,
  [SMALL_STATE(64)] = 925,
  [SMALL_STATE(65)] = 929,
  [SMALL_STATE(66)] = 933,
  [SMALL_STATE(67)] = 937,
  [SMALL_STATE(68)] = 941,
  [SMALL_STATE(69)] = 945,
  [SMALL_STATE(70)] = 949,
  [SMALL_STATE(71)] = 953,
  [SMALL_STATE(72)] = 957,
  [SMALL_STATE(73)] = 961,
  [SMALL_STATE(74)] = 965,
  [SMALL_STATE(75)] = 969,
  [SMALL_STATE(76)] = 973,
  [SMALL_STATE(77)] = 977,
  [SMALL_STATE(78)] = 981,
  [SMALL_STATE(79)] = 985,
  [SMALL_STATE(80)] = 989,
  [SMALL_STATE(81)] = 993,
  [SMALL_STATE(82)] = 997,
  [SMALL_STATE(83)] = 1001,
  [SMALL_STATE(84)] = 1005,
  [SMALL_STATE(85)] = 1009,
  [SMALL_STATE(86)] = 1013,
  [SMALL_STATE(87)] = 1017,
  [SMALL_STATE(88)] = 1021,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(51),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(79),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(80),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(49),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(48),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 1, 0, 0),
  [43] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [45] = {.entry = {.count = 1, .reusable = false}}, SHIFT(53),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0),
  [49] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(3),
  [52] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(83),
  [55] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(51),
  [58] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(79),
  [61] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(80),
  [64] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(49),
  [67] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(84),
  [70] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(63),
  [73] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(72),
  [76] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(20),
  [79] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(19),
  [82] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(82),
  [85] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(29),
  [88] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(29),
  [91] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(30),
  [94] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(30),
  [97] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(76),
  [100] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0), SHIFT_REPEAT(35),
  [103] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2, 0, 0),
  [105] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2, 0, 0),
  [107] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, 0, 0),
  [111] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3, 0, 0),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0),
  [115] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0),
  [117] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(51),
  [120] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(79),
  [123] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(80),
  [126] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(49),
  [129] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(84),
  [132] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(63),
  [135] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(72),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hunk, 2, 0, 2),
  [140] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hunk, 2, 0, 2),
  [142] = {.entry = {.count = 1, .reusable = false}}, SHIFT(77),
  [144] = {.entry = {.count = 1, .reusable = false}}, SHIFT(78),
  [146] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [148] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_hunks_repeat1, 2, 0, 0),
  [150] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_hunks_repeat1, 2, 0, 0),
  [152] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_hunks_repeat1, 2, 0, 0), SHIFT_REPEAT(82),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hunks, 1, 0, 0),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hunks, 1, 0, 0),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_changes_repeat2, 2, 0, 0),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_changes_repeat2, 2, 0, 0),
  [163] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_changes_repeat2, 2, 0, 0), SHIFT_REPEAT(77),
  [166] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_changes_repeat2, 2, 0, 0), SHIFT_REPEAT(78),
  [169] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_changes_repeat2, 2, 0, 0), SHIFT_REPEAT(29),
  [172] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_changes_repeat2, 2, 0, 0), SHIFT_REPEAT(29),
  [175] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_changes_repeat2, 2, 0, 0), SHIFT_REPEAT(30),
  [178] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_changes_repeat2, 2, 0, 0), SHIFT_REPEAT(30),
  [181] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_changes_repeat2, 2, 0, 0), SHIFT_REPEAT(40),
  [184] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_changes, 1, 0, 0),
  [186] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_changes, 1, 0, 0),
  [188] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [190] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_changes_repeat1, 2, 0, 0),
  [192] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_changes_repeat1, 2, 0, 0), SHIFT_REPEAT(13),
  [195] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_changes_repeat1, 2, 0, 0),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_source_repeat1, 2, 0, 0),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 7, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 7, 0, 0),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 8, 0, 0),
  [205] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 8, 0, 0),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hunk, 3, 0, 3),
  [209] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hunk, 3, 0, 3),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_addition, 1, 0, 0),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_deletion, 1, 0, 0),
  [217] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_index, 4, 0, 0),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [221] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_filename, 1, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [225] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_filename_repeat1, 2, 0, 0),
  [227] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_filename_repeat1, 2, 0, 0), SHIFT_REPEAT(25),
  [230] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [232] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_addition, 1, 0, 0),
  [234] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [236] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_deletion, 1, 0, 0),
  [238] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [240] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [242] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_location, 4, 0, 0),
  [244] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_location, 4, 0, 0),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [248] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 1, 0, 0),
  [250] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_comment, 1, 0, 0),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [254] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_filename, 1, 0, 0),
  [256] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [258] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_filename_repeat1, 2, 0, 0),
  [260] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_filename_repeat1, 2, 0, 0), SHIFT_REPEAT(37),
  [263] = {.entry = {.count = 1, .reusable = false}}, SHIFT(39),
  [265] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_filename_repeat1, 2, 0, 0), SHIFT_REPEAT(39),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [270] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_change, 3, 0, 0),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [274] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_change, 6, 0, 0),
  [276] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_addition, 2, 0, 0),
  [278] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_deletion, 2, 0, 0),
  [280] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 2, 0, 0),
  [282] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_old_file, 2, 0, 0),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [288] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file_change, 4, 0, 0),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [292] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [294] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 2, 0, 0),
  [296] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_similarity, 4, 0, 1),
  [298] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_new_file, 2, 0, 0),
  [300] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mode, 1, 0, 0),
  [302] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_index, 5, 0, 0),
  [304] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_location, 5, 0, 0),
  [306] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [308] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [310] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [312] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [314] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [316] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [320] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [322] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [324] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [326] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [328] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [340] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [344] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_command, 3, 0, 0),
  [346] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [348] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
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

TS_PUBLIC const TSLanguage *tree_sitter_diff(void) {
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
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
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
