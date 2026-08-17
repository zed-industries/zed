#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#ifdef _MSC_VER
#pragma optimize("", off)
#elif defined(__clang__)
#pragma clang optimize off
#elif defined(__GNUC__)
#pragma GCC optimize ("O0")
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 249
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 123
#define ALIAS_COUNT 0
#define TOKEN_COUNT 89
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 4
#define MAX_ALIAS_SEQUENCE_LENGTH 33
#define PRODUCTION_ID_COUNT 8

enum {
  aux_sym_source_token1 = 1,
  aux_sym_subject_token1 = 2,
  aux_sym_subject_token2 = 3,
  aux_sym_message_token1 = 4,
  anon_sym_POUND = 5,
  anon_sym_COLON = 6,
  anon_sym_EQ = 7,
  anon_sym_interactive = 8,
  anon_sym_rebase = 9,
  anon_sym_in = 10,
  anon_sym_progress = 11,
  anon_sym_SEMI = 12,
  anon_sym_onto = 13,
  anon_sym_You = 14,
  anon_sym_are = 15,
  anon_sym_currently = 16,
  aux_sym__rebase_summary_token1 = 17,
  anon_sym_rebasing = 18,
  anon_sym_branch = 19,
  anon_sym_SQUOTE = 20,
  anon_sym_on = 21,
  anon_sym_DOT = 22,
  anon_sym_Last = 23,
  aux_sym__rebase_header_token1 = 24,
  anon_sym_done = 25,
  anon_sym_LPAREN = 26,
  aux_sym__rebase_header_token2 = 27,
  anon_sym_RPAREN = 28,
  anon_sym_Next = 29,
  anon_sym_to = 30,
  anon_sym_do = 31,
  anon_sym_remaining = 32,
  anon_sym_No = 33,
  anon_sym_commands = 34,
  anon_sym_Changes = 35,
  anon_sym_be = 36,
  anon_sym_committed = 37,
  anon_sym_not = 38,
  anon_sym_staged = 39,
  anon_sym_for = 40,
  anon_sym_commit = 41,
  anon_sym_On = 42,
  anon_sym_Your = 43,
  anon_sym_is = 44,
  anon_sym_up = 45,
  anon_sym_date = 46,
  anon_sym_with = 47,
  anon_sym_SQUOTE_DOT = 48,
  anon_sym_ahead = 49,
  anon_sym_of = 50,
  anon_sym_behind = 51,
  anon_sym_by = 52,
  aux_sym__branch_declaration_token1 = 53,
  anon_sym_and = 54,
  anon_sym_have = 55,
  anon_sym_diverged = 56,
  anon_sym_COMMA = 57,
  anon_sym_HEAD = 58,
  anon_sym_detached = 59,
  anon_sym_at = 60,
  anon_sym_Conflicts = 61,
  anon_sym_Untracked = 62,
  anon_sym_files = 63,
  anon_sym_newfile = 64,
  anon_sym_modified = 65,
  anon_sym_renamed = 66,
  anon_sym_deleted = 67,
  anon_sym_DASH_GT = 68,
  sym_commit = 69,
  sym__non_punctuated_word = 70,
  sym__non_comment = 71,
  sym__any_word = 72,
  sym_branch = 73,
  anon_sym_pick = 74,
  anon_sym_edit = 75,
  anon_sym_squash = 76,
  anon_sym_merge = 77,
  anon_sym_fixup = 78,
  anon_sym_drop = 79,
  anon_sym_reword = 80,
  anon_sym_exec = 81,
  anon_sym_label = 82,
  anon_sym_reset = 83,
  aux_sym_path_token1 = 84,
  sym_item = 85,
  sym_user = 86,
  aux_sym__rest_token1 = 87,
  sym__newline = 88,
  sym_source = 89,
  sym_subject = 90,
  sym__body_line = 91,
  sym__trailer = 92,
  sym_message = 93,
  sym__text = 94,
  sym_comment = 95,
  sym__comment_body = 96,
  sym_trailer = 97,
  sym_trailer_key = 98,
  sym_trailer_value = 99,
  sym__rebase_summary = 100,
  sym__rebase_header = 101,
  sym_summary = 102,
  sym__change_header = 103,
  sym__branch_declaration = 104,
  sym_header = 105,
  sym_change = 106,
  sym__word = 107,
  sym_rebase_command = 108,
  sym_path = 109,
  aux_sym__rest = 110,
  aux_sym_source_repeat1 = 111,
  aux_sym_source_repeat2 = 112,
  aux_sym_source_repeat3 = 113,
  aux_sym_subject_repeat1 = 114,
  aux_sym_message_repeat1 = 115,
  aux_sym__comment_body_repeat1 = 116,
  aux_sym_trailer_value_repeat1 = 117,
  aux_sym__rebase_summary_repeat1 = 118,
  aux_sym__rebase_summary_repeat2 = 119,
  aux_sym_summary_repeat1 = 120,
  aux_sym_summary_repeat2 = 121,
  aux_sym_path_repeat1 = 122,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_source_token1] = "scissors",
  [aux_sym_subject_token1] = "subject_token1",
  [aux_sym_subject_token2] = "subject_token2",
  [aux_sym_message_token1] = "message_token1",
  [anon_sym_POUND] = "#",
  [anon_sym_COLON] = ":",
  [anon_sym_EQ] = "=",
  [anon_sym_interactive] = "interactive",
  [anon_sym_rebase] = "rebase",
  [anon_sym_in] = "in",
  [anon_sym_progress] = "progress",
  [anon_sym_SEMI] = ";",
  [anon_sym_onto] = "onto",
  [anon_sym_You] = "You",
  [anon_sym_are] = "are",
  [anon_sym_currently] = "currently",
  [aux_sym__rebase_summary_token1] = "_rebase_summary_token1",
  [anon_sym_rebasing] = "rebasing",
  [anon_sym_branch] = "branch",
  [anon_sym_SQUOTE] = "'",
  [anon_sym_on] = "on",
  [anon_sym_DOT] = ".",
  [anon_sym_Last] = "Last",
  [aux_sym__rebase_header_token1] = "_rebase_header_token1",
  [anon_sym_done] = "done",
  [anon_sym_LPAREN] = "(",
  [aux_sym__rebase_header_token2] = "_rebase_header_token2",
  [anon_sym_RPAREN] = ")",
  [anon_sym_Next] = "Next",
  [anon_sym_to] = "to",
  [anon_sym_do] = "do",
  [anon_sym_remaining] = "remaining",
  [anon_sym_No] = "No",
  [anon_sym_commands] = "commands",
  [anon_sym_Changes] = "Changes",
  [anon_sym_be] = "be",
  [anon_sym_committed] = "committed",
  [anon_sym_not] = "not",
  [anon_sym_staged] = "staged",
  [anon_sym_for] = "for",
  [anon_sym_commit] = "commit",
  [anon_sym_On] = "On",
  [anon_sym_Your] = "Your",
  [anon_sym_is] = "is",
  [anon_sym_up] = "up",
  [anon_sym_date] = "date",
  [anon_sym_with] = "with",
  [anon_sym_SQUOTE_DOT] = "'.",
  [anon_sym_ahead] = "ahead",
  [anon_sym_of] = "of",
  [anon_sym_behind] = "behind",
  [anon_sym_by] = "by",
  [aux_sym__branch_declaration_token1] = "_branch_declaration_token1",
  [anon_sym_and] = "and",
  [anon_sym_have] = "have",
  [anon_sym_diverged] = "diverged",
  [anon_sym_COMMA] = ",",
  [anon_sym_HEAD] = "HEAD",
  [anon_sym_detached] = "detached",
  [anon_sym_at] = "at",
  [anon_sym_Conflicts] = "Conflicts",
  [anon_sym_Untracked] = "Untracked",
  [anon_sym_files] = "files",
  [anon_sym_newfile] = "new file",
  [anon_sym_modified] = "modified",
  [anon_sym_renamed] = "renamed",
  [anon_sym_deleted] = "deleted",
  [anon_sym_DASH_GT] = "->",
  [sym_commit] = "commit",
  [sym__non_punctuated_word] = "_non_punctuated_word",
  [sym__non_comment] = "_non_comment",
  [sym__any_word] = "_any_word",
  [sym_branch] = "branch",
  [anon_sym_pick] = "pick",
  [anon_sym_edit] = "edit",
  [anon_sym_squash] = "squash",
  [anon_sym_merge] = "merge",
  [anon_sym_fixup] = "fixup",
  [anon_sym_drop] = "drop",
  [anon_sym_reword] = "reword",
  [anon_sym_exec] = "exec",
  [anon_sym_label] = "label",
  [anon_sym_reset] = "reset",
  [aux_sym_path_token1] = "path_token1",
  [sym_item] = "item",
  [sym_user] = "user",
  [aux_sym__rest_token1] = "_rest_token1",
  [sym__newline] = "_newline",
  [sym_source] = "source",
  [sym_subject] = "subject",
  [sym__body_line] = "_body_line",
  [sym__trailer] = "_trailer",
  [sym_message] = "message",
  [sym__text] = "_text",
  [sym_comment] = "comment",
  [sym__comment_body] = "_comment_body",
  [sym_trailer] = "trailer",
  [sym_trailer_key] = "trailer_key",
  [sym_trailer_value] = "trailer_value",
  [sym__rebase_summary] = "summary",
  [sym__rebase_header] = "header",
  [sym_summary] = "summary",
  [sym__change_header] = "header",
  [sym__branch_declaration] = "_branch_declaration",
  [sym_header] = "header",
  [sym_change] = "change",
  [sym__word] = "_word",
  [sym_rebase_command] = "rebase_command",
  [sym_path] = "path",
  [aux_sym__rest] = "_rest",
  [aux_sym_source_repeat1] = "source_repeat1",
  [aux_sym_source_repeat2] = "source_repeat2",
  [aux_sym_source_repeat3] = "source_repeat3",
  [aux_sym_subject_repeat1] = "subject_repeat1",
  [aux_sym_message_repeat1] = "message_repeat1",
  [aux_sym__comment_body_repeat1] = "_comment_body_repeat1",
  [aux_sym_trailer_value_repeat1] = "trailer_value_repeat1",
  [aux_sym__rebase_summary_repeat1] = "_rebase_summary_repeat1",
  [aux_sym__rebase_summary_repeat2] = "_rebase_summary_repeat2",
  [aux_sym_summary_repeat1] = "summary_repeat1",
  [aux_sym_summary_repeat2] = "summary_repeat2",
  [aux_sym_path_repeat1] = "path_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_source_token1] = aux_sym_source_token1,
  [aux_sym_subject_token1] = aux_sym_subject_token1,
  [aux_sym_subject_token2] = aux_sym_subject_token2,
  [aux_sym_message_token1] = aux_sym_message_token1,
  [anon_sym_POUND] = anon_sym_POUND,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_interactive] = anon_sym_interactive,
  [anon_sym_rebase] = anon_sym_rebase,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_progress] = anon_sym_progress,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_onto] = anon_sym_onto,
  [anon_sym_You] = anon_sym_You,
  [anon_sym_are] = anon_sym_are,
  [anon_sym_currently] = anon_sym_currently,
  [aux_sym__rebase_summary_token1] = aux_sym__rebase_summary_token1,
  [anon_sym_rebasing] = anon_sym_rebasing,
  [anon_sym_branch] = anon_sym_branch,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [anon_sym_on] = anon_sym_on,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_Last] = anon_sym_Last,
  [aux_sym__rebase_header_token1] = aux_sym__rebase_header_token1,
  [anon_sym_done] = anon_sym_done,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [aux_sym__rebase_header_token2] = aux_sym__rebase_header_token2,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_Next] = anon_sym_Next,
  [anon_sym_to] = anon_sym_to,
  [anon_sym_do] = anon_sym_do,
  [anon_sym_remaining] = anon_sym_remaining,
  [anon_sym_No] = anon_sym_No,
  [anon_sym_commands] = anon_sym_commands,
  [anon_sym_Changes] = anon_sym_Changes,
  [anon_sym_be] = anon_sym_be,
  [anon_sym_committed] = anon_sym_committed,
  [anon_sym_not] = anon_sym_not,
  [anon_sym_staged] = anon_sym_staged,
  [anon_sym_for] = anon_sym_for,
  [anon_sym_commit] = anon_sym_commit,
  [anon_sym_On] = anon_sym_On,
  [anon_sym_Your] = anon_sym_Your,
  [anon_sym_is] = anon_sym_is,
  [anon_sym_up] = anon_sym_up,
  [anon_sym_date] = anon_sym_date,
  [anon_sym_with] = anon_sym_with,
  [anon_sym_SQUOTE_DOT] = anon_sym_SQUOTE_DOT,
  [anon_sym_ahead] = anon_sym_ahead,
  [anon_sym_of] = anon_sym_of,
  [anon_sym_behind] = anon_sym_behind,
  [anon_sym_by] = anon_sym_by,
  [aux_sym__branch_declaration_token1] = aux_sym__branch_declaration_token1,
  [anon_sym_and] = anon_sym_and,
  [anon_sym_have] = anon_sym_have,
  [anon_sym_diverged] = anon_sym_diverged,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_HEAD] = anon_sym_HEAD,
  [anon_sym_detached] = anon_sym_detached,
  [anon_sym_at] = anon_sym_at,
  [anon_sym_Conflicts] = anon_sym_Conflicts,
  [anon_sym_Untracked] = anon_sym_Untracked,
  [anon_sym_files] = anon_sym_files,
  [anon_sym_newfile] = anon_sym_newfile,
  [anon_sym_modified] = anon_sym_modified,
  [anon_sym_renamed] = anon_sym_renamed,
  [anon_sym_deleted] = anon_sym_deleted,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [sym_commit] = sym_commit,
  [sym__non_punctuated_word] = sym__non_punctuated_word,
  [sym__non_comment] = sym__non_comment,
  [sym__any_word] = sym__any_word,
  [sym_branch] = sym_branch,
  [anon_sym_pick] = anon_sym_pick,
  [anon_sym_edit] = anon_sym_edit,
  [anon_sym_squash] = anon_sym_squash,
  [anon_sym_merge] = anon_sym_merge,
  [anon_sym_fixup] = anon_sym_fixup,
  [anon_sym_drop] = anon_sym_drop,
  [anon_sym_reword] = anon_sym_reword,
  [anon_sym_exec] = anon_sym_exec,
  [anon_sym_label] = anon_sym_label,
  [anon_sym_reset] = anon_sym_reset,
  [aux_sym_path_token1] = aux_sym_path_token1,
  [sym_item] = sym_item,
  [sym_user] = sym_user,
  [aux_sym__rest_token1] = aux_sym__rest_token1,
  [sym__newline] = sym__newline,
  [sym_source] = sym_source,
  [sym_subject] = sym_subject,
  [sym__body_line] = sym__body_line,
  [sym__trailer] = sym__trailer,
  [sym_message] = sym_message,
  [sym__text] = sym__text,
  [sym_comment] = sym_comment,
  [sym__comment_body] = sym__comment_body,
  [sym_trailer] = sym_trailer,
  [sym_trailer_key] = sym_trailer_key,
  [sym_trailer_value] = sym_trailer_value,
  [sym__rebase_summary] = sym_summary,
  [sym__rebase_header] = sym_header,
  [sym_summary] = sym_summary,
  [sym__change_header] = sym_header,
  [sym__branch_declaration] = sym__branch_declaration,
  [sym_header] = sym_header,
  [sym_change] = sym_change,
  [sym__word] = sym__word,
  [sym_rebase_command] = sym_rebase_command,
  [sym_path] = sym_path,
  [aux_sym__rest] = aux_sym__rest,
  [aux_sym_source_repeat1] = aux_sym_source_repeat1,
  [aux_sym_source_repeat2] = aux_sym_source_repeat2,
  [aux_sym_source_repeat3] = aux_sym_source_repeat3,
  [aux_sym_subject_repeat1] = aux_sym_subject_repeat1,
  [aux_sym_message_repeat1] = aux_sym_message_repeat1,
  [aux_sym__comment_body_repeat1] = aux_sym__comment_body_repeat1,
  [aux_sym_trailer_value_repeat1] = aux_sym_trailer_value_repeat1,
  [aux_sym__rebase_summary_repeat1] = aux_sym__rebase_summary_repeat1,
  [aux_sym__rebase_summary_repeat2] = aux_sym__rebase_summary_repeat2,
  [aux_sym_summary_repeat1] = aux_sym_summary_repeat1,
  [aux_sym_summary_repeat2] = aux_sym_summary_repeat2,
  [aux_sym_path_repeat1] = aux_sym_path_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_token1] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_subject_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_subject_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_message_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_interactive] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rebase] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_in] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_progress] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_onto] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_You] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_are] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_currently] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__rebase_summary_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_rebasing] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_branch] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_on] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Last] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__rebase_header_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_done] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__rebase_header_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Next] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_to] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_do] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_remaining] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_No] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_commands] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Changes] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_be] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_committed] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_not] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_staged] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_for] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_commit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_On] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Your] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_is] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_up] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_date] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_with] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SQUOTE_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ahead] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_of] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_behind] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_by] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__branch_declaration_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_and] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_have] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_diverged] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_HEAD] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_detached] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_at] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Conflicts] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Untracked] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_files] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_newfile] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_modified] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_renamed] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_deleted] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [sym_commit] = {
    .visible = true,
    .named = true,
  },
  [sym__non_punctuated_word] = {
    .visible = false,
    .named = true,
  },
  [sym__non_comment] = {
    .visible = false,
    .named = true,
  },
  [sym__any_word] = {
    .visible = false,
    .named = true,
  },
  [sym_branch] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_pick] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_edit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_squash] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_merge] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fixup] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_drop] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_reword] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_exec] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_label] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_reset] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_path_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_item] = {
    .visible = true,
    .named = true,
  },
  [sym_user] = {
    .visible = true,
    .named = true,
  },
  [aux_sym__rest_token1] = {
    .visible = false,
    .named = false,
  },
  [sym__newline] = {
    .visible = false,
    .named = true,
  },
  [sym_source] = {
    .visible = true,
    .named = true,
  },
  [sym_subject] = {
    .visible = true,
    .named = true,
  },
  [sym__body_line] = {
    .visible = false,
    .named = true,
  },
  [sym__trailer] = {
    .visible = false,
    .named = true,
  },
  [sym_message] = {
    .visible = true,
    .named = true,
  },
  [sym__text] = {
    .visible = false,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym__comment_body] = {
    .visible = false,
    .named = true,
  },
  [sym_trailer] = {
    .visible = true,
    .named = true,
  },
  [sym_trailer_key] = {
    .visible = true,
    .named = true,
  },
  [sym_trailer_value] = {
    .visible = true,
    .named = true,
  },
  [sym__rebase_summary] = {
    .visible = true,
    .named = true,
  },
  [sym__rebase_header] = {
    .visible = true,
    .named = true,
  },
  [sym_summary] = {
    .visible = true,
    .named = true,
  },
  [sym__change_header] = {
    .visible = true,
    .named = true,
  },
  [sym__branch_declaration] = {
    .visible = false,
    .named = true,
  },
  [sym_header] = {
    .visible = true,
    .named = true,
  },
  [sym_change] = {
    .visible = true,
    .named = true,
  },
  [sym__word] = {
    .visible = false,
    .named = true,
  },
  [sym_rebase_command] = {
    .visible = true,
    .named = true,
  },
  [sym_path] = {
    .visible = true,
    .named = true,
  },
  [aux_sym__rest] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_source_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_source_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_source_repeat3] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_subject_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_message_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__comment_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_trailer_value_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__rebase_summary_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__rebase_summary_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_summary_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_summary_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_path_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_key = 1,
  field_kind = 2,
  field_separator = 3,
  field_value = 4,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_key] = "key",
  [field_kind] = "kind",
  [field_separator] = "separator",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [3] = {.index = 0, .length = 3},
  [7] = {.index = 3, .length = 1},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_key, 0},
    {field_separator, 1},
    {field_value, 2},
  [3] =
    {field_kind, 0},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
  [1] = {
    [1] = sym_message,
  },
  [2] = {
    [2] = sym_message,
  },
  [4] = {
    [3] = sym_message,
  },
  [5] = {
    [4] = sym_message,
  },
  [6] = {
    [5] = sym_message,
  },
};

static const uint16_t ts_non_terminal_alias_map[] = {
  aux_sym__rest, 2,
    aux_sym__rest,
    sym_message,
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
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
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
  [70] = 62,
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
  [83] = 75,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 41,
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 149,
  [150] = 150,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 157,
  [158] = 158,
  [159] = 159,
  [160] = 160,
  [161] = 161,
  [162] = 162,
  [163] = 163,
  [164] = 164,
  [165] = 165,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 169,
  [170] = 170,
  [171] = 171,
  [172] = 172,
  [173] = 173,
  [174] = 174,
  [175] = 175,
  [176] = 176,
  [177] = 177,
  [178] = 178,
  [179] = 179,
  [180] = 180,
  [181] = 181,
  [182] = 182,
  [183] = 183,
  [184] = 184,
  [185] = 185,
  [186] = 186,
  [187] = 187,
  [188] = 188,
  [189] = 189,
  [190] = 190,
  [191] = 191,
  [192] = 192,
  [193] = 193,
  [194] = 194,
  [195] = 195,
  [196] = 196,
  [197] = 197,
  [198] = 198,
  [199] = 199,
  [200] = 200,
  [201] = 201,
  [202] = 202,
  [203] = 203,
  [204] = 204,
  [205] = 205,
  [206] = 206,
  [207] = 207,
  [208] = 208,
  [209] = 209,
  [210] = 210,
  [211] = 211,
  [212] = 212,
  [213] = 213,
  [214] = 214,
  [215] = 215,
  [216] = 216,
  [217] = 217,
  [218] = 218,
  [219] = 219,
  [220] = 220,
  [221] = 221,
  [222] = 222,
  [223] = 223,
  [224] = 224,
  [225] = 225,
  [226] = 226,
  [227] = 227,
  [228] = 228,
  [229] = 229,
  [230] = 230,
  [231] = 231,
  [232] = 232,
  [233] = 233,
  [234] = 234,
  [235] = 235,
  [236] = 236,
  [237] = 237,
  [238] = 238,
  [239] = 239,
  [240] = 240,
  [241] = 241,
  [242] = 242,
  [243] = 243,
  [244] = 244,
  [245] = 245,
  [246] = 246,
  [247] = 247,
  [248] = 248,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(511);
      if (lookahead == '\'') ADVANCE(508);
      if (lookahead == '(') ADVANCE(508);
      if (lookahead == ')') ADVANCE(508);
      if (lookahead == ',') ADVANCE(508);
      if (lookahead == '.') ADVANCE(508);
      if (lookahead == ':') ADVANCE(508);
      if (lookahead == ';') ADVANCE(508);
      if (lookahead == '=') ADVANCE(508);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(280);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(506);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(507);
      if (lookahead != 0) ADVANCE(508);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(517);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '-') ADVANCE(509);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(510);
      if (lookahead != 0) ADVANCE(511);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(292);
      if (lookahead != 0) ADVANCE(316);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(28);
      END_STATE();
    case 5:
      if (lookahead == '\n') ADVANCE(28);
      if (lookahead == '\r') ADVANCE(4);
      if (lookahead == '-') ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == '\n') ADVANCE(29);
      END_STATE();
    case 7:
      if (lookahead == '\n') ADVANCE(29);
      if (lookahead == '\r') ADVANCE(6);
      END_STATE();
    case 8:
      if (lookahead == ' ') ADVANCE(34);
      if (lookahead == '-') ADVANCE(8);
      END_STATE();
    case 9:
      if (lookahead == ' ') ADVANCE(36);
      END_STATE();
    case 10:
      if (lookahead == ' ') ADVANCE(176);
      END_STATE();
    case 11:
      if (lookahead == ' ') ADVANCE(165);
      END_STATE();
    case 12:
      if (lookahead == ' ') ADVANCE(156);
      END_STATE();
    case 13:
      if (lookahead == ' ') ADVANCE(38);
      END_STATE();
    case 14:
      if (lookahead == ' ') ADVANCE(37);
      END_STATE();
    case 15:
      if (lookahead == ' ') ADVANCE(31);
      END_STATE();
    case 16:
      if (lookahead == ' ') ADVANCE(228);
      END_STATE();
    case 17:
      if (lookahead == ' ') ADVANCE(209);
      END_STATE();
    case 18:
      if (lookahead == ' ') ADVANCE(53);
      END_STATE();
    case 19:
      if (lookahead == ' ') ADVANCE(254);
      END_STATE();
    case 20:
      if (lookahead == ' ') ADVANCE(57);
      END_STATE();
    case 21:
      if (lookahead == ' ') ADVANCE(189);
      END_STATE();
    case 22:
      if (lookahead == ' ') ADVANCE(153);
      END_STATE();
    case 23:
      if (lookahead == ' ') ADVANCE(142);
      END_STATE();
    case 24:
      if (lookahead == ' ') ADVANCE(121);
      END_STATE();
    case 25:
      if (lookahead == '#') ADVANCE(488);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(291);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead)) ADVANCE(484);
      END_STATE();
    case 26:
      if (lookahead == '#') ADVANCE(487);
      if (lookahead == '@') ADVANCE(483);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(291);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(481);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead)) ADVANCE(484);
      END_STATE();
    case 27:
      if (lookahead == '#') ADVANCE(296);
      if (lookahead == '\'') ADVANCE(32);
      if (lookahead == 'b') ADVANCE(90);
      if (lookahead == 'c') ADVANCE(204);
      if (lookahead == 'd') ADVANCE(195);
      if (lookahead == 'o') ADVANCE(175);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(294);
      END_STATE();
    case 28:
      if (lookahead == '#') ADVANCE(9);
      END_STATE();
    case 29:
      if (lookahead == '#') ADVANCE(14);
      END_STATE();
    case 30:
      if (lookahead == '-') ADVANCE(8);
      END_STATE();
    case 31:
      if (lookahead == '-') ADVANCE(5);
      END_STATE();
    case 32:
      if (lookahead == '.') ADVANCE(348);
      END_STATE();
    case 33:
      if (lookahead == '8') ADVANCE(15);
      END_STATE();
    case 34:
      if (lookahead == '>') ADVANCE(33);
      END_STATE();
    case 35:
      if (lookahead == '>') ADVANCE(369);
      END_STATE();
    case 36:
      if (lookahead == 'D') ADVANCE(198);
      END_STATE();
    case 37:
      if (lookahead == 'E') ADVANCE(251);
      END_STATE();
    case 38:
      if (lookahead == 'a') ADVANCE(56);
      END_STATE();
    case 39:
      if (lookahead == 'a') ADVANCE(226);
      END_STATE();
    case 40:
      if (lookahead == 'a') ADVANCE(227);
      END_STATE();
    case 41:
      if (lookahead == 'a') ADVANCE(177);
      END_STATE();
    case 42:
      if (lookahead == 'a') ADVANCE(240);
      if (lookahead == 'e') ADVANCE(160);
      if (lookahead == 'i') ADVANCE(252);
      if (lookahead == 'o') ADVANCE(185);
      if (lookahead == 'r') ADVANCE(197);
      END_STATE();
    case 43:
      if (lookahead == 'a') ADVANCE(169);
      END_STATE();
    case 44:
      if (lookahead == 'a') ADVANCE(65);
      END_STATE();
    case 45:
      if (lookahead == 'a') ADVANCE(224);
      END_STATE();
    case 46:
      if (lookahead == 'a') ADVANCE(61);
      END_STATE();
    case 47:
      if (lookahead == 'a') ADVANCE(147);
      END_STATE();
    case 48:
      if (lookahead == 'a') ADVANCE(55);
      END_STATE();
    case 49:
      if (lookahead == 'a') ADVANCE(125);
      END_STATE();
    case 50:
      if (lookahead == 'a') ADVANCE(181);
      if (lookahead == 'i') ADVANCE(234);
      END_STATE();
    case 51:
      if (lookahead == 'a') ADVANCE(184);
      if (lookahead == 'i') ADVANCE(235);
      END_STATE();
    case 52:
      if (lookahead == 'a') ADVANCE(253);
      END_STATE();
    case 53:
      if (lookahead == 'b') ADVANCE(92);
      END_STATE();
    case 54:
      if (lookahead == 'b') ADVANCE(40);
      if (lookahead == 'm') ADVANCE(47);
      if (lookahead == 'n') ADVANCE(43);
      if (lookahead == 's') ADVANCE(107);
      if (lookahead == 'w') ADVANCE(201);
      END_STATE();
    case 55:
      if (lookahead == 'b') ADVANCE(96);
      END_STATE();
    case 56:
      if (lookahead == 'b') ADVANCE(202);
      END_STATE();
    case 57:
      if (lookahead == 'b') ADVANCE(111);
      END_STATE();
    case 58:
      if (lookahead == 'c') ADVANCE(154);
      END_STATE();
    case 59:
      if (lookahead == 'c') ADVANCE(503);
      END_STATE();
    case 60:
      if (lookahead == 'c') ADVANCE(130);
      END_STATE();
    case 61:
      if (lookahead == 'c') ADVANCE(135);
      END_STATE();
    case 62:
      if (lookahead == 'c') ADVANCE(203);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(294);
      END_STATE();
    case 63:
      if (lookahead == 'd') ADVANCE(137);
      END_STATE();
    case 64:
      if (lookahead == 'd') ADVANCE(355);
      END_STATE();
    case 65:
      if (lookahead == 'd') ADVANCE(349);
      END_STATE();
    case 66:
      if (lookahead == 'd') ADVANCE(351);
      END_STATE();
    case 67:
      if (lookahead == 'd') ADVANCE(502);
      END_STATE();
    case 68:
      if (lookahead == 'd') ADVANCE(339);
      END_STATE();
    case 69:
      if (lookahead == 'd') ADVANCE(324);
      END_STATE();
    case 70:
      if (lookahead == 'd') ADVANCE(368);
      END_STATE();
    case 71:
      if (lookahead == 'd') ADVANCE(367);
      END_STATE();
    case 72:
      if (lookahead == 'd') ADVANCE(360);
      END_STATE();
    case 73:
      if (lookahead == 'd') ADVANCE(357);
      END_STATE();
    case 74:
      if (lookahead == 'd') ADVANCE(366);
      END_STATE();
    case 75:
      if (lookahead == 'd') ADVANCE(337);
      END_STATE();
    case 76:
      if (lookahead == 'd') ADVANCE(264);
      END_STATE();
    case 77:
      if (lookahead == 'd') ADVANCE(223);
      END_STATE();
    case 78:
      if (lookahead == 'd') ADVANCE(150);
      if (lookahead == 'x') ADVANCE(94);
      END_STATE();
    case 79:
      if (lookahead == 'd') ADVANCE(141);
      END_STATE();
    case 80:
      if (lookahead == 'e') ADVANCE(263);
      END_STATE();
    case 81:
      if (lookahead == 'e') ADVANCE(257);
      if (lookahead == 'o') ADVANCE(333);
      END_STATE();
    case 82:
      if (lookahead == 'e') ADVANCE(54);
      END_STATE();
    case 83:
      if (lookahead == 'e') ADVANCE(307);
      END_STATE();
    case 84:
      if (lookahead == 'e') ADVANCE(346);
      END_STATE();
    case 85:
      if (lookahead == 'e') ADVANCE(325);
      END_STATE();
    case 86:
      if (lookahead == 'e') ADVANCE(356);
      END_STATE();
    case 87:
      if (lookahead == 'e') ADVANCE(499);
      END_STATE();
    case 88:
      if (lookahead == 'e') ADVANCE(301);
      END_STATE();
    case 89:
      if (lookahead == 'e') ADVANCE(365);
      END_STATE();
    case 90:
      if (lookahead == 'e') ADVANCE(336);
      END_STATE();
    case 91:
      if (lookahead == 'e') ADVANCE(76);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(159);
      END_STATE();
    case 93:
      if (lookahead == 'e') ADVANCE(221);
      END_STATE();
    case 94:
      if (lookahead == 'e') ADVANCE(59);
      END_STATE();
    case 95:
      if (lookahead == 'e') ADVANCE(212);
      END_STATE();
    case 96:
      if (lookahead == 'e') ADVANCE(155);
      END_STATE();
    case 97:
      if (lookahead == 'e') ADVANCE(225);
      END_STATE();
    case 98:
      if (lookahead == 'e') ADVANCE(182);
      END_STATE();
    case 99:
      if (lookahead == 'e') ADVANCE(44);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(16);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(68);
      END_STATE();
    case 102:
      if (lookahead == 'e') ADVANCE(213);
      if (lookahead == 'o') ADVANCE(79);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(12);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(13);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(70);
      END_STATE();
    case 106:
      if (lookahead == 'e') ADVANCE(71);
      END_STATE();
    case 107:
      if (lookahead == 'e') ADVANCE(233);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(72);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(73);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(74);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(23);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(75);
      END_STATE();
    case 113:
      if (lookahead == 'e') ADVANCE(167);
      END_STATE();
    case 114:
      if (lookahead == 'e') ADVANCE(256);
      if (lookahead == 'o') ADVANCE(229);
      END_STATE();
    case 115:
      if (lookahead == 'e') ADVANCE(134);
      if (lookahead == 'r') ADVANCE(41);
      if (lookahead == 'y') ADVANCE(352);
      END_STATE();
    case 116:
      if (lookahead == 'e') ADVANCE(243);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(220);
      END_STATE();
    case 118:
      if (lookahead == 'f') ADVANCE(260);
      END_STATE();
    case 119:
      if (lookahead == 'f') ADVANCE(350);
      if (lookahead == 'n') ADVANCE(241);
      END_STATE();
    case 120:
      if (lookahead == 'f') ADVANCE(152);
      END_STATE();
    case 121:
      if (lookahead == 'f') ADVANCE(146);
      END_STATE();
    case 122:
      if (lookahead == 'g') ADVANCE(332);
      END_STATE();
    case 123:
      if (lookahead == 'g') ADVANCE(18);
      END_STATE();
    case 124:
      if (lookahead == 'g') ADVANCE(87);
      END_STATE();
    case 125:
      if (lookahead == 'g') ADVANCE(101);
      END_STATE();
    case 126:
      if (lookahead == 'g') ADVANCE(109);
      END_STATE();
    case 127:
      if (lookahead == 'g') ADVANCE(180);
      END_STATE();
    case 128:
      if (lookahead == 'g') ADVANCE(218);
      END_STATE();
    case 129:
      if (lookahead == 'h') ADVANCE(347);
      END_STATE();
    case 130:
      if (lookahead == 'h') ADVANCE(318);
      END_STATE();
    case 131:
      if (lookahead == 'h') ADVANCE(498);
      END_STATE();
    case 132:
      if (lookahead == 'h') ADVANCE(143);
      END_STATE();
    case 133:
      if (lookahead == 'h') ADVANCE(99);
      if (lookahead == 'n') ADVANCE(64);
      if (lookahead == 'r') ADVANCE(83);
      if (lookahead == 't') ADVANCE(361);
      END_STATE();
    case 134:
      if (lookahead == 'h') ADVANCE(145);
      END_STATE();
    case 135:
      if (lookahead == 'h') ADVANCE(108);
      END_STATE();
    case 136:
      if (lookahead == 'h') ADVANCE(103);
      END_STATE();
    case 137:
      if (lookahead == 'i') ADVANCE(118);
      END_STATE();
    case 138:
      if (lookahead == 'i') ADVANCE(162);
      if (lookahead == 'o') ADVANCE(210);
      END_STATE();
    case 139:
      if (lookahead == 'i') ADVANCE(58);
      if (lookahead == 'r') ADVANCE(200);
      END_STATE();
    case 140:
      if (lookahead == 'i') ADVANCE(186);
      END_STATE();
    case 141:
      if (lookahead == 'i') ADVANCE(120);
      END_STATE();
    case 142:
      if (lookahead == 'i') ADVANCE(127);
      END_STATE();
    case 143:
      if (lookahead == 'i') ADVANCE(173);
      END_STATE();
    case 144:
      if (lookahead == 'i') ADVANCE(158);
      END_STATE();
    case 145:
      if (lookahead == 'i') ADVANCE(178);
      END_STATE();
    case 146:
      if (lookahead == 'i') ADVANCE(163);
      END_STATE();
    case 147:
      if (lookahead == 'i') ADVANCE(183);
      END_STATE();
    case 148:
      if (lookahead == 'i') ADVANCE(237);
      END_STATE();
    case 149:
      if (lookahead == 'i') ADVANCE(179);
      END_STATE();
    case 150:
      if (lookahead == 'i') ADVANCE(232);
      END_STATE();
    case 151:
      if (lookahead == 'i') ADVANCE(245);
      END_STATE();
    case 152:
      if (lookahead == 'i') ADVANCE(110);
      END_STATE();
    case 153:
      if (lookahead == 'i') ADVANCE(242);
      END_STATE();
    case 154:
      if (lookahead == 'k') ADVANCE(496);
      END_STATE();
    case 155:
      if (lookahead == 'l') ADVANCE(504);
      END_STATE();
    case 156:
      if (lookahead == 'l') ADVANCE(140);
      END_STATE();
    case 157:
      if (lookahead == 'l') ADVANCE(258);
      END_STATE();
    case 158:
      if (lookahead == 'l') ADVANCE(161);
      END_STATE();
    case 159:
      if (lookahead == 'l') ADVANCE(191);
      END_STATE();
    case 160:
      if (lookahead == 'l') ADVANCE(116);
      if (lookahead == 't') ADVANCE(46);
      END_STATE();
    case 161:
      if (lookahead == 'l') ADVANCE(20);
      END_STATE();
    case 162:
      if (lookahead == 'l') ADVANCE(93);
      if (lookahead == 'x') ADVANCE(247);
      END_STATE();
    case 163:
      if (lookahead == 'l') ADVANCE(89);
      END_STATE();
    case 164:
      if (lookahead == 'm') ADVANCE(50);
      END_STATE();
    case 165:
      if (lookahead == 'm') ADVANCE(188);
      END_STATE();
    case 166:
      if (lookahead == 'm') ADVANCE(164);
      END_STATE();
    case 167:
      if (lookahead == 'm') ADVANCE(190);
      END_STATE();
    case 168:
      if (lookahead == 'm') ADVANCE(172);
      END_STATE();
    case 169:
      if (lookahead == 'm') ADVANCE(106);
      END_STATE();
    case 170:
      if (lookahead == 'm') ADVANCE(51);
      END_STATE();
    case 171:
      if (lookahead == 'm') ADVANCE(170);
      END_STATE();
    case 172:
      if (lookahead == 'm') ADVANCE(151);
      END_STATE();
    case 173:
      if (lookahead == 'n') ADVANCE(123);
      END_STATE();
    case 174:
      if (lookahead == 'n') ADVANCE(302);
      if (lookahead == 's') ADVANCE(344);
      END_STATE();
    case 175:
      if (lookahead == 'n') ADVANCE(320);
      END_STATE();
    case 176:
      if (lookahead == 'n') ADVANCE(187);
      END_STATE();
    case 177:
      if (lookahead == 'n') ADVANCE(60);
      END_STATE();
    case 178:
      if (lookahead == 'n') ADVANCE(66);
      END_STATE();
    case 179:
      if (lookahead == 'n') ADVANCE(122);
      END_STATE();
    case 180:
      if (lookahead == 'n') ADVANCE(199);
      END_STATE();
    case 181:
      if (lookahead == 'n') ADVANCE(69);
      END_STATE();
    case 182:
      if (lookahead == 'n') ADVANCE(239);
      END_STATE();
    case 183:
      if (lookahead == 'n') ADVANCE(149);
      END_STATE();
    case 184:
      if (lookahead == 'n') ADVANCE(77);
      END_STATE();
    case 185:
      if (lookahead == 'n') ADVANCE(85);
      END_STATE();
    case 186:
      if (lookahead == 'n') ADVANCE(104);
      END_STATE();
    case 187:
      if (lookahead == 'o') ADVANCE(238);
      END_STATE();
    case 188:
      if (lookahead == 'o') ADVANCE(63);
      END_STATE();
    case 189:
      if (lookahead == 'o') ADVANCE(214);
      END_STATE();
    case 190:
      if (lookahead == 'o') ADVANCE(249);
      END_STATE();
    case 191:
      if (lookahead == 'o') ADVANCE(255);
      END_STATE();
    case 192:
      if (lookahead == 'o') ADVANCE(246);
      END_STATE();
    case 193:
      if (lookahead == 'o') ADVANCE(330);
      END_STATE();
    case 194:
      if (lookahead == 'o') ADVANCE(305);
      END_STATE();
    case 195:
      if (lookahead == 'o') ADVANCE(331);
      END_STATE();
    case 196:
      if (lookahead == 'o') ADVANCE(166);
      if (lookahead == 'u') ADVANCE(219);
      END_STATE();
    case 197:
      if (lookahead == 'o') ADVANCE(206);
      END_STATE();
    case 198:
      if (lookahead == 'o') ADVANCE(10);
      END_STATE();
    case 199:
      if (lookahead == 'o') ADVANCE(216);
      END_STATE();
    case 200:
      if (lookahead == 'o') ADVANCE(128);
      END_STATE();
    case 201:
      if (lookahead == 'o') ADVANCE(215);
      END_STATE();
    case 202:
      if (lookahead == 'o') ADVANCE(250);
      END_STATE();
    case 203:
      if (lookahead == 'o') ADVANCE(168);
      END_STATE();
    case 204:
      if (lookahead == 'o') ADVANCE(171);
      END_STATE();
    case 205:
      if (lookahead == 'p') ADVANCE(345);
      END_STATE();
    case 206:
      if (lookahead == 'p') ADVANCE(501);
      END_STATE();
    case 207:
      if (lookahead == 'p') ADVANCE(500);
      END_STATE();
    case 208:
      if (lookahead == 'q') ADVANCE(248);
      if (lookahead == 't') ADVANCE(49);
      END_STATE();
    case 209:
      if (lookahead == 'r') ADVANCE(113);
      END_STATE();
    case 210:
      if (lookahead == 'r') ADVANCE(340);
      END_STATE();
    case 211:
      if (lookahead == 'r') ADVANCE(311);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(292);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead)) ADVANCE(316);
      END_STATE();
    case 212:
      if (lookahead == 'r') ADVANCE(259);
      END_STATE();
    case 213:
      if (lookahead == 'r') ADVANCE(124);
      END_STATE();
    case 214:
      if (lookahead == 'r') ADVANCE(17);
      END_STATE();
    case 215:
      if (lookahead == 'r') ADVANCE(67);
      END_STATE();
    case 216:
      if (lookahead == 'r') ADVANCE(91);
      END_STATE();
    case 217:
      if (lookahead == 'r') ADVANCE(98);
      END_STATE();
    case 218:
      if (lookahead == 'r') ADVANCE(97);
      END_STATE();
    case 219:
      if (lookahead == 'r') ADVANCE(217);
      END_STATE();
    case 220:
      if (lookahead == 'r') ADVANCE(126);
      END_STATE();
    case 221:
      if (lookahead == 's') ADVANCE(364);
      END_STATE();
    case 222:
      if (lookahead == 's') ADVANCE(303);
      END_STATE();
    case 223:
      if (lookahead == 's') ADVANCE(334);
      END_STATE();
    case 224:
      if (lookahead == 's') ADVANCE(131);
      END_STATE();
    case 225:
      if (lookahead == 's') ADVANCE(222);
      END_STATE();
    case 226:
      if (lookahead == 's') ADVANCE(230);
      END_STATE();
    case 227:
      if (lookahead == 's') ADVANCE(88);
      END_STATE();
    case 228:
      if (lookahead == 't') ADVANCE(136);
      END_STATE();
    case 229:
      if (lookahead == 't') ADVANCE(338);
      END_STATE();
    case 230:
      if (lookahead == 't') ADVANCE(322);
      END_STATE();
    case 231:
      if (lookahead == 't') ADVANCE(329);
      END_STATE();
    case 232:
      if (lookahead == 't') ADVANCE(497);
      END_STATE();
    case 233:
      if (lookahead == 't') ADVANCE(505);
      END_STATE();
    case 234:
      if (lookahead == 't') ADVANCE(354);
      END_STATE();
    case 235:
      if (lookahead == 't') ADVANCE(341);
      END_STATE();
    case 236:
      if (lookahead == 't') ADVANCE(132);
      END_STATE();
    case 237:
      if (lookahead == 't') ADVANCE(129);
      END_STATE();
    case 238:
      if (lookahead == 't') ADVANCE(11);
      END_STATE();
    case 239:
      if (lookahead == 't') ADVANCE(157);
      END_STATE();
    case 240:
      if (lookahead == 't') ADVANCE(84);
      END_STATE();
    case 241:
      if (lookahead == 't') ADVANCE(194);
      END_STATE();
    case 242:
      if (lookahead == 't') ADVANCE(19);
      END_STATE();
    case 243:
      if (lookahead == 't') ADVANCE(105);
      END_STATE();
    case 244:
      if (lookahead == 't') ADVANCE(112);
      END_STATE();
    case 245:
      if (lookahead == 't') ADVANCE(244);
      END_STATE();
    case 246:
      if (lookahead == 'u') ADVANCE(306);
      END_STATE();
    case 247:
      if (lookahead == 'u') ADVANCE(207);
      END_STATE();
    case 248:
      if (lookahead == 'u') ADVANCE(45);
      END_STATE();
    case 249:
      if (lookahead == 'v') ADVANCE(100);
      END_STATE();
    case 250:
      if (lookahead == 'v') ADVANCE(80);
      END_STATE();
    case 251:
      if (lookahead == 'v') ADVANCE(95);
      END_STATE();
    case 252:
      if (lookahead == 'v') ADVANCE(117);
      END_STATE();
    case 253:
      if (lookahead == 'v') ADVANCE(86);
      END_STATE();
    case 254:
      if (lookahead == 'w') ADVANCE(144);
      END_STATE();
    case 255:
      if (lookahead == 'w') ADVANCE(22);
      END_STATE();
    case 256:
      if (lookahead == 'w') ADVANCE(24);
      END_STATE();
    case 257:
      if (lookahead == 'x') ADVANCE(231);
      END_STATE();
    case 258:
      if (lookahead == 'y') ADVANCE(308);
      END_STATE();
    case 259:
      if (lookahead == 'y') ADVANCE(236);
      END_STATE();
    case 260:
      if (lookahead == 'y') ADVANCE(21);
      END_STATE();
    case 261:
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(293);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(494);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead) &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 262:
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(293);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead) &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 263:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(7);
      END_STATE();
    case 264:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(278);
      END_STATE();
    case 265:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(511);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(510);
      if (lookahead != 0) ADVANCE(511);
      END_STATE();
    case 266:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(297);
      if (lookahead == '\'') ADVANCE(319);
      if (lookahead == '(') ADVANCE(326);
      if (lookahead == ')') ADVANCE(328);
      if (lookahead == ',') ADVANCE(358);
      if (lookahead == '-') ADVANCE(35);
      if (lookahead == '.') ADVANCE(321);
      if (lookahead == ':') ADVANCE(298);
      if (lookahead == ';') ADVANCE(304);
      if (lookahead == '=') ADVANCE(299);
      if (lookahead == 'L') ADVANCE(39);
      if (lookahead == 'N') ADVANCE(81);
      if (lookahead == 'Y') ADVANCE(192);
      if (lookahead == 'a') ADVANCE(133);
      if (lookahead == 'b') ADVANCE(115);
      if (lookahead == 'c') ADVANCE(196);
      if (lookahead == 'd') ADVANCE(42);
      if (lookahead == 'e') ADVANCE(78);
      if (lookahead == 'f') ADVANCE(138);
      if (lookahead == 'h') ADVANCE(52);
      if (lookahead == 'i') ADVANCE(174);
      if (lookahead == 'l') ADVANCE(48);
      if (lookahead == 'm') ADVANCE(102);
      if (lookahead == 'n') ADVANCE(114);
      if (lookahead == 'o') ADVANCE(119);
      if (lookahead == 'p') ADVANCE(139);
      if (lookahead == 'r') ADVANCE(82);
      if (lookahead == 's') ADVANCE(208);
      if (lookahead == 't') ADVANCE(193);
      if (lookahead == 'u') ADVANCE(205);
      if (lookahead == 'w') ADVANCE(148);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(294);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(327);
      END_STATE();
    case 267:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(297);
      if (lookahead == '@') ADVANCE(483);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(291);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(481);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      if (lookahead != 0) ADVANCE(484);
      END_STATE();
    case 268:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(297);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(291);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      if (lookahead != 0) ADVANCE(484);
      END_STATE();
    case 269:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(297);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(280);
      if (lookahead != 0) ADVANCE(279);
      END_STATE();
    case 270:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(486);
      if (lookahead == 'C') ADVANCE(454);
      if (lookahead == 'H') ADVANCE(440);
      if (lookahead == 'O') ADVANCE(459);
      if (lookahead == 'U') ADVANCE(460);
      if (lookahead == 'Y') ADVANCE(464);
      if (lookahead == 'i') ADVANCE(463);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(291);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      if (lookahead != 0) ADVANCE(484);
      END_STATE();
    case 271:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(486);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(291);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      if (lookahead != 0) ADVANCE(484);
      END_STATE();
    case 272:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(485);
      if (lookahead == ':') ADVANCE(298);
      if (lookahead == '=') ADVANCE(299);
      if (lookahead == '@') ADVANCE(483);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(291);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(481);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      if (lookahead != 0) ADVANCE(484);
      END_STATE();
    case 273:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(485);
      if (lookahead == '@') ADVANCE(483);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(291);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(481);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      if (lookahead != 0) ADVANCE(484);
      END_STATE();
    case 274:
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(282);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(289);
      if (lookahead != 0) ADVANCE(290);
      END_STATE();
    case 275:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 276:
      ACCEPT_TOKEN(aux_sym_source_token1);
      END_STATE();
    case 277:
      ACCEPT_TOKEN(aux_sym_source_token1);
      if (lookahead == '\n') ADVANCE(276);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(aux_sym_source_token1);
      if (lookahead == '\n') ADVANCE(276);
      if (lookahead == '\r') ADVANCE(277);
      END_STATE();
    case 279:
      ACCEPT_TOKEN(aux_sym_subject_token1);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(aux_sym_subject_token1);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(294);
      END_STATE();
    case 281:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead == '\n') ADVANCE(28);
      if (lookahead == '\r') ADVANCE(4);
      if (lookahead == '-') ADVANCE(281);
      if (lookahead != 0) ADVANCE(290);
      END_STATE();
    case 282:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead == ' ') ADVANCE(285);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(290);
      END_STATE();
    case 283:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead == ' ') ADVANCE(288);
      if (lookahead == '-') ADVANCE(283);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(290);
      END_STATE();
    case 284:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead == ' ') ADVANCE(286);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(290);
      END_STATE();
    case 285:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead == '-') ADVANCE(283);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(290);
      END_STATE();
    case 286:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead == '-') ADVANCE(281);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(290);
      END_STATE();
    case 287:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead == '8') ADVANCE(284);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(290);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead == '>') ADVANCE(287);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(290);
      END_STATE();
    case 289:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(289);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead)) ADVANCE(290);
      END_STATE();
    case 290:
      ACCEPT_TOKEN(aux_sym_subject_token2);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(290);
      END_STATE();
    case 291:
      ACCEPT_TOKEN(aux_sym_message_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(291);
      END_STATE();
    case 292:
      ACCEPT_TOKEN(aux_sym_message_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(292);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead)) ADVANCE(316);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(aux_sym_message_token1);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(294);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(293);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead) &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(aux_sym_message_token1);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(294);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(aux_sym_message_token1);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(295);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(515);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(anon_sym_POUND);
      if (lookahead == ' ') ADVANCE(30);
      END_STATE();
    case 298:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 300:
      ACCEPT_TOKEN(anon_sym_interactive);
      END_STATE();
    case 301:
      ACCEPT_TOKEN(anon_sym_rebase);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(anon_sym_in);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(anon_sym_progress);
      END_STATE();
    case 304:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 305:
      ACCEPT_TOKEN(anon_sym_onto);
      END_STATE();
    case 306:
      ACCEPT_TOKEN(anon_sym_You);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(anon_sym_are);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(anon_sym_currently);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(aux_sym__rebase_summary_token1);
      if (lookahead == 'a') ADVANCE(315);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(316);
      END_STATE();
    case 310:
      ACCEPT_TOKEN(aux_sym__rebase_summary_token1);
      if (lookahead == 'b') ADVANCE(309);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(316);
      END_STATE();
    case 311:
      ACCEPT_TOKEN(aux_sym__rebase_summary_token1);
      if (lookahead == 'e') ADVANCE(310);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(316);
      END_STATE();
    case 312:
      ACCEPT_TOKEN(aux_sym__rebase_summary_token1);
      if (lookahead == 'g') ADVANCE(317);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(316);
      END_STATE();
    case 313:
      ACCEPT_TOKEN(aux_sym__rebase_summary_token1);
      if (lookahead == 'i') ADVANCE(314);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(316);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(aux_sym__rebase_summary_token1);
      if (lookahead == 'n') ADVANCE(312);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(316);
      END_STATE();
    case 315:
      ACCEPT_TOKEN(aux_sym__rebase_summary_token1);
      if (lookahead == 's') ADVANCE(313);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(316);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(aux_sym__rebase_summary_token1);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(316);
      END_STATE();
    case 317:
      ACCEPT_TOKEN(anon_sym_rebasing);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(316);
      END_STATE();
    case 318:
      ACCEPT_TOKEN(anon_sym_branch);
      END_STATE();
    case 319:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 320:
      ACCEPT_TOKEN(anon_sym_on);
      END_STATE();
    case 321:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 322:
      ACCEPT_TOKEN(anon_sym_Last);
      END_STATE();
    case 323:
      ACCEPT_TOKEN(aux_sym__rebase_header_token1);
      END_STATE();
    case 324:
      ACCEPT_TOKEN(aux_sym__rebase_header_token1);
      if (lookahead == 's') ADVANCE(323);
      END_STATE();
    case 325:
      ACCEPT_TOKEN(anon_sym_done);
      END_STATE();
    case 326:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 327:
      ACCEPT_TOKEN(aux_sym__rebase_header_token2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(327);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 329:
      ACCEPT_TOKEN(anon_sym_Next);
      END_STATE();
    case 330:
      ACCEPT_TOKEN(anon_sym_to);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(anon_sym_do);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(anon_sym_remaining);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(anon_sym_No);
      END_STATE();
    case 334:
      ACCEPT_TOKEN(anon_sym_commands);
      END_STATE();
    case 335:
      ACCEPT_TOKEN(anon_sym_Changes);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(anon_sym_be);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(anon_sym_committed);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(anon_sym_not);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(anon_sym_staged);
      END_STATE();
    case 340:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(anon_sym_commit);
      END_STATE();
    case 342:
      ACCEPT_TOKEN(anon_sym_On);
      END_STATE();
    case 343:
      ACCEPT_TOKEN(anon_sym_Your);
      END_STATE();
    case 344:
      ACCEPT_TOKEN(anon_sym_is);
      END_STATE();
    case 345:
      ACCEPT_TOKEN(anon_sym_up);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(anon_sym_date);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(anon_sym_with);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(anon_sym_SQUOTE_DOT);
      END_STATE();
    case 349:
      ACCEPT_TOKEN(anon_sym_ahead);
      END_STATE();
    case 350:
      ACCEPT_TOKEN(anon_sym_of);
      END_STATE();
    case 351:
      ACCEPT_TOKEN(anon_sym_behind);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(anon_sym_by);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(aux_sym__branch_declaration_token1);
      END_STATE();
    case 354:
      ACCEPT_TOKEN(aux_sym__branch_declaration_token1);
      if (lookahead == 's') ADVANCE(353);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(anon_sym_and);
      END_STATE();
    case 356:
      ACCEPT_TOKEN(anon_sym_have);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(anon_sym_diverged);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(anon_sym_HEAD);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(anon_sym_detached);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(anon_sym_at);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(anon_sym_Conflicts);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(anon_sym_Untracked);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(anon_sym_files);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(anon_sym_newfile);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(anon_sym_modified);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(anon_sym_renamed);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(anon_sym_deleted);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(sym_commit);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(370);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(437);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(371);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(372);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(373);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(374);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 377:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(375);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(376);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(377);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(378);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(379);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(380);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(381);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(382);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(383);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(384);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(385);
      END_STATE();
    case 388:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(386);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 389:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(387);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(388);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(389);
      END_STATE();
    case 392:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(390);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(391);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(392);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 395:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(393);
      END_STATE();
    case 396:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(394);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 397:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(395);
      END_STATE();
    case 398:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(396);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 399:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(397);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(398);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 401:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(399);
      END_STATE();
    case 402:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(400);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(401);
      END_STATE();
    case 404:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(402);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 405:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(403);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(404);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(405);
      END_STATE();
    case 408:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(406);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 409:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(407);
      END_STATE();
    case 410:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(408);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 411:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(409);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(410);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(411);
      END_STATE();
    case 414:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(412);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 415:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(413);
      END_STATE();
    case 416:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(414);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 417:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(415);
      END_STATE();
    case 418:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(416);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 419:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(417);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(418);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 421:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(419);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(420);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 423:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(421);
      END_STATE();
    case 424:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(422);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 425:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(423);
      END_STATE();
    case 426:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(424);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 427:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(425);
      END_STATE();
    case 428:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(426);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 429:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(427);
      END_STATE();
    case 430:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(428);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 431:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(429);
      END_STATE();
    case 432:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(430);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 433:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(431);
      END_STATE();
    case 434:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(432);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 435:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(433);
      END_STATE();
    case 436:
      ACCEPT_TOKEN(sym_commit);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(434);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 437:
      ACCEPT_TOKEN(sym_commit);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 438:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'A') ADVANCE(439);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 439:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'D') ADVANCE(359);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 440:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'E') ADVANCE(438);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 441:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'a') ADVANCE(444);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 442:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'a') ADVANCE(462);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 443:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'a') ADVANCE(446);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 444:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'c') ADVANCE(457);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 445:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'c') ADVANCE(473);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 446:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'c') ADVANCE(472);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 447:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'd') ADVANCE(363);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 448:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'e') ADVANCE(468);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 449:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'e') ADVANCE(447);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 450:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'e') ADVANCE(300);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 451:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'e') ADVANCE(467);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 452:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'f') ADVANCE(458);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 453:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'g') ADVANCE(448);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 454:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'h') ADVANCE(442);
      if (lookahead == 'o') ADVANCE(461);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 455:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'i') ADVANCE(475);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 456:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'i') ADVANCE(445);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 457:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'k') ADVANCE(449);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 458:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'l') ADVANCE(456);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 459:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'n') ADVANCE(342);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 460:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'n') ADVANCE(470);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 461:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'n') ADVANCE(452);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 462:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'n') ADVANCE(453);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 463:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'n') ADVANCE(471);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 464:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'o') ADVANCE(474);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 465:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'r') ADVANCE(343);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 466:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'r') ADVANCE(441);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 467:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'r') ADVANCE(443);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 468:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 's') ADVANCE(335);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 469:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 's') ADVANCE(362);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 470:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 't') ADVANCE(466);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 471:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 't') ADVANCE(451);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 472:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 't') ADVANCE(455);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 473:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 't') ADVANCE(469);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 474:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'u') ADVANCE(465);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 475:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == 'v') ADVANCE(450);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 476:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(435);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 477:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(476);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 478:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(477);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 479:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(478);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 480:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(479);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 481:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(480);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 482:
      ACCEPT_TOKEN(sym__non_punctuated_word);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(482);
      END_STATE();
    case 483:
      ACCEPT_TOKEN(sym__non_comment);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(513);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '#') ADVANCE(484);
      END_STATE();
    case 484:
      ACCEPT_TOKEN(sym__non_comment);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '#') ADVANCE(484);
      END_STATE();
    case 485:
      ACCEPT_TOKEN(sym__any_word);
      if (lookahead == ' ') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(512);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(488);
      END_STATE();
    case 486:
      ACCEPT_TOKEN(sym__any_word);
      if (lookahead == ' ') ADVANCE(30);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(488);
      END_STATE();
    case 487:
      ACCEPT_TOKEN(sym__any_word);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(512);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(488);
      END_STATE();
    case 488:
      ACCEPT_TOKEN(sym__any_word);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(488);
      END_STATE();
    case 489:
      ACCEPT_TOKEN(sym_branch);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(436);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 490:
      ACCEPT_TOKEN(sym_branch);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(489);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 491:
      ACCEPT_TOKEN(sym_branch);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(490);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 492:
      ACCEPT_TOKEN(sym_branch);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(491);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 493:
      ACCEPT_TOKEN(sym_branch);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(492);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 494:
      ACCEPT_TOKEN(sym_branch);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(493);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 495:
      ACCEPT_TOKEN(sym_branch);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '\'' &&
          lookahead != '.') ADVANCE(495);
      END_STATE();
    case 496:
      ACCEPT_TOKEN(anon_sym_pick);
      END_STATE();
    case 497:
      ACCEPT_TOKEN(anon_sym_edit);
      END_STATE();
    case 498:
      ACCEPT_TOKEN(anon_sym_squash);
      END_STATE();
    case 499:
      ACCEPT_TOKEN(anon_sym_merge);
      END_STATE();
    case 500:
      ACCEPT_TOKEN(anon_sym_fixup);
      END_STATE();
    case 501:
      ACCEPT_TOKEN(anon_sym_drop);
      END_STATE();
    case 502:
      ACCEPT_TOKEN(anon_sym_reword);
      END_STATE();
    case 503:
      ACCEPT_TOKEN(anon_sym_exec);
      END_STATE();
    case 504:
      ACCEPT_TOKEN(anon_sym_label);
      END_STATE();
    case 505:
      ACCEPT_TOKEN(anon_sym_reset);
      END_STATE();
    case 506:
      ACCEPT_TOKEN(aux_sym_path_token1);
      if (lookahead == '#') ADVANCE(511);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(506);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(508);
      END_STATE();
    case 507:
      ACCEPT_TOKEN(aux_sym_path_token1);
      if (lookahead == '#') ADVANCE(511);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(507);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(508);
      END_STATE();
    case 508:
      ACCEPT_TOKEN(aux_sym_path_token1);
      if (lookahead == '#') ADVANCE(511);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(508);
      END_STATE();
    case 509:
      ACCEPT_TOKEN(aux_sym_path_token1);
      if (lookahead == '>') ADVANCE(369);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(511);
      END_STATE();
    case 510:
      ACCEPT_TOKEN(aux_sym_path_token1);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(510);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ') ADVANCE(511);
      END_STATE();
    case 511:
      ACCEPT_TOKEN(aux_sym_path_token1);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(511);
      END_STATE();
    case 512:
      ACCEPT_TOKEN(sym_item);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(512);
      END_STATE();
    case 513:
      ACCEPT_TOKEN(sym_user);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(513);
      END_STATE();
    case 514:
      ACCEPT_TOKEN(aux_sym__rest_token1);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead != 0) ADVANCE(515);
      END_STATE();
    case 515:
      ACCEPT_TOKEN(aux_sym__rest_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(515);
      END_STATE();
    case 516:
      ACCEPT_TOKEN(aux_sym__rest_token1);
      if (eof) ADVANCE(275);
      if (lookahead == '\n') ADVANCE(517);
      if (lookahead == '\r') ADVANCE(514);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(295);
      if (lookahead != 0) ADVANCE(515);
      END_STATE();
    case 517:
      ACCEPT_TOKEN(sym__newline);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 269},
  [2] = {.lex_state = 270},
  [3] = {.lex_state = 266},
  [4] = {.lex_state = 266},
  [5] = {.lex_state = 272},
  [6] = {.lex_state = 267},
  [7] = {.lex_state = 266},
  [8] = {.lex_state = 266},
  [9] = {.lex_state = 273},
  [10] = {.lex_state = 266},
  [11] = {.lex_state = 273},
  [12] = {.lex_state = 273},
  [13] = {.lex_state = 266},
  [14] = {.lex_state = 273},
  [15] = {.lex_state = 268},
  [16] = {.lex_state = 273},
  [17] = {.lex_state = 267},
  [18] = {.lex_state = 26},
  [19] = {.lex_state = 270},
  [20] = {.lex_state = 270},
  [21] = {.lex_state = 266},
  [22] = {.lex_state = 270},
  [23] = {.lex_state = 271},
  [24] = {.lex_state = 271},
  [25] = {.lex_state = 265},
  [26] = {.lex_state = 270},
  [27] = {.lex_state = 25},
  [28] = {.lex_state = 270},
  [29] = {.lex_state = 270},
  [30] = {.lex_state = 266},
  [31] = {.lex_state = 266},
  [32] = {.lex_state = 266},
  [33] = {.lex_state = 266},
  [34] = {.lex_state = 274},
  [35] = {.lex_state = 266},
  [36] = {.lex_state = 270},
  [37] = {.lex_state = 270},
  [38] = {.lex_state = 266},
  [39] = {.lex_state = 274},
  [40] = {.lex_state = 274},
  [41] = {.lex_state = 2},
  [42] = {.lex_state = 270},
  [43] = {.lex_state = 270},
  [44] = {.lex_state = 266},
  [45] = {.lex_state = 516},
  [46] = {.lex_state = 516},
  [47] = {.lex_state = 516},
  [48] = {.lex_state = 270},
  [49] = {.lex_state = 516},
  [50] = {.lex_state = 516},
  [51] = {.lex_state = 516},
  [52] = {.lex_state = 266},
  [53] = {.lex_state = 516},
  [54] = {.lex_state = 266},
  [55] = {.lex_state = 266},
  [56] = {.lex_state = 516},
  [57] = {.lex_state = 516},
  [58] = {.lex_state = 266},
  [59] = {.lex_state = 270},
  [60] = {.lex_state = 266},
  [61] = {.lex_state = 266},
  [62] = {.lex_state = 2},
  [63] = {.lex_state = 516},
  [64] = {.lex_state = 516},
  [65] = {.lex_state = 270},
  [66] = {.lex_state = 270},
  [67] = {.lex_state = 265},
  [68] = {.lex_state = 211},
  [69] = {.lex_state = 516},
  [70] = {.lex_state = 265},
  [71] = {.lex_state = 270},
  [72] = {.lex_state = 270},
  [73] = {.lex_state = 265},
  [74] = {.lex_state = 270},
  [75] = {.lex_state = 211},
  [76] = {.lex_state = 270},
  [77] = {.lex_state = 270},
  [78] = {.lex_state = 266},
  [79] = {.lex_state = 211},
  [80] = {.lex_state = 211},
  [81] = {.lex_state = 270},
  [82] = {.lex_state = 211},
  [83] = {.lex_state = 3},
  [84] = {.lex_state = 270},
  [85] = {.lex_state = 270},
  [86] = {.lex_state = 3},
  [87] = {.lex_state = 270},
  [88] = {.lex_state = 270},
  [89] = {.lex_state = 211},
  [90] = {.lex_state = 270},
  [91] = {.lex_state = 211},
  [92] = {.lex_state = 270},
  [93] = {.lex_state = 270},
  [94] = {.lex_state = 265},
  [95] = {.lex_state = 270},
  [96] = {.lex_state = 265},
  [97] = {.lex_state = 270},
  [98] = {.lex_state = 27},
  [99] = {.lex_state = 266},
  [100] = {.lex_state = 27},
  [101] = {.lex_state = 266},
  [102] = {.lex_state = 261},
  [103] = {.lex_state = 27},
  [104] = {.lex_state = 3},
  [105] = {.lex_state = 27},
  [106] = {.lex_state = 27},
  [107] = {.lex_state = 266},
  [108] = {.lex_state = 27},
  [109] = {.lex_state = 27},
  [110] = {.lex_state = 27},
  [111] = {.lex_state = 266},
  [112] = {.lex_state = 266},
  [113] = {.lex_state = 27},
  [114] = {.lex_state = 266},
  [115] = {.lex_state = 266},
  [116] = {.lex_state = 270},
  [117] = {.lex_state = 266},
  [118] = {.lex_state = 262},
  [119] = {.lex_state = 266},
  [120] = {.lex_state = 266},
  [121] = {.lex_state = 270},
  [122] = {.lex_state = 266},
  [123] = {.lex_state = 266},
  [124] = {.lex_state = 266},
  [125] = {.lex_state = 27},
  [126] = {.lex_state = 262},
  [127] = {.lex_state = 266},
  [128] = {.lex_state = 266},
  [129] = {.lex_state = 266},
  [130] = {.lex_state = 270},
  [131] = {.lex_state = 270},
  [132] = {.lex_state = 27},
  [133] = {.lex_state = 266},
  [134] = {.lex_state = 266},
  [135] = {.lex_state = 266},
  [136] = {.lex_state = 266},
  [137] = {.lex_state = 266},
  [138] = {.lex_state = 27},
  [139] = {.lex_state = 270},
  [140] = {.lex_state = 262},
  [141] = {.lex_state = 266},
  [142] = {.lex_state = 266},
  [143] = {.lex_state = 266},
  [144] = {.lex_state = 266},
  [145] = {.lex_state = 266},
  [146] = {.lex_state = 266},
  [147] = {.lex_state = 266},
  [148] = {.lex_state = 266},
  [149] = {.lex_state = 266},
  [150] = {.lex_state = 27},
  [151] = {.lex_state = 266},
  [152] = {.lex_state = 266},
  [153] = {.lex_state = 270},
  [154] = {.lex_state = 266},
  [155] = {.lex_state = 266},
  [156] = {.lex_state = 266},
  [157] = {.lex_state = 270},
  [158] = {.lex_state = 270},
  [159] = {.lex_state = 270},
  [160] = {.lex_state = 270},
  [161] = {.lex_state = 270},
  [162] = {.lex_state = 266},
  [163] = {.lex_state = 266},
  [164] = {.lex_state = 266},
  [165] = {.lex_state = 262},
  [166] = {.lex_state = 266},
  [167] = {.lex_state = 27},
  [168] = {.lex_state = 270},
  [169] = {.lex_state = 266},
  [170] = {.lex_state = 266},
  [171] = {.lex_state = 266},
  [172] = {.lex_state = 266},
  [173] = {.lex_state = 27},
  [174] = {.lex_state = 266},
  [175] = {.lex_state = 266},
  [176] = {.lex_state = 266},
  [177] = {.lex_state = 266},
  [178] = {.lex_state = 266},
  [179] = {.lex_state = 270},
  [180] = {.lex_state = 266},
  [181] = {.lex_state = 272},
  [182] = {.lex_state = 266},
  [183] = {.lex_state = 266},
  [184] = {.lex_state = 266},
  [185] = {.lex_state = 266},
  [186] = {.lex_state = 270},
  [187] = {.lex_state = 266},
  [188] = {.lex_state = 62},
  [189] = {.lex_state = 266},
  [190] = {.lex_state = 266},
  [191] = {.lex_state = 270},
  [192] = {.lex_state = 266},
  [193] = {.lex_state = 266},
  [194] = {.lex_state = 266},
  [195] = {.lex_state = 266},
  [196] = {.lex_state = 266},
  [197] = {.lex_state = 266},
  [198] = {.lex_state = 270},
  [199] = {.lex_state = 266},
  [200] = {.lex_state = 262},
  [201] = {.lex_state = 266},
  [202] = {.lex_state = 266},
  [203] = {.lex_state = 262},
  [204] = {.lex_state = 266},
  [205] = {.lex_state = 262},
  [206] = {.lex_state = 266},
  [207] = {.lex_state = 266},
  [208] = {.lex_state = 27},
  [209] = {.lex_state = 266},
  [210] = {.lex_state = 262},
  [211] = {.lex_state = 266},
  [212] = {.lex_state = 266},
  [213] = {.lex_state = 27},
  [214] = {.lex_state = 266},
  [215] = {.lex_state = 262},
  [216] = {.lex_state = 272},
  [217] = {.lex_state = 266},
  [218] = {.lex_state = 27},
  [219] = {.lex_state = 266},
  [220] = {.lex_state = 266},
  [221] = {.lex_state = 272},
  [222] = {.lex_state = 266},
  [223] = {.lex_state = 27},
  [224] = {.lex_state = 266},
  [225] = {.lex_state = 266},
  [226] = {.lex_state = 272},
  [227] = {.lex_state = 266},
  [228] = {.lex_state = 270},
  [229] = {.lex_state = 266},
  [230] = {.lex_state = 266},
  [231] = {.lex_state = 272},
  [232] = {.lex_state = 266},
  [233] = {.lex_state = 270},
  [234] = {.lex_state = 266},
  [235] = {.lex_state = 266},
  [236] = {.lex_state = 27},
  [237] = {.lex_state = 266},
  [238] = {.lex_state = 270},
  [239] = {.lex_state = 266},
  [240] = {.lex_state = 270},
  [241] = {.lex_state = 270},
  [242] = {.lex_state = 270},
  [243] = {.lex_state = 266},
  [244] = {.lex_state = 266},
  [245] = {.lex_state = 266},
  [246] = {.lex_state = 266},
  [247] = {.lex_state = 266},
  [248] = {.lex_state = 266},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_source_token1] = ACTIONS(1),
    [aux_sym_subject_token1] = ACTIONS(1),
    [aux_sym_message_token1] = ACTIONS(3),
    [anon_sym_POUND] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [sym__non_punctuated_word] = ACTIONS(1),
    [sym__non_comment] = ACTIONS(1),
    [sym__any_word] = ACTIONS(1),
    [aux_sym_path_token1] = ACTIONS(1),
    [sym__newline] = ACTIONS(1),
  },
  [1] = {
    [sym_source] = STATE(131),
    [sym_subject] = STATE(22),
    [sym__body_line] = STATE(19),
    [sym__trailer] = STATE(29),
    [sym_comment] = STATE(22),
    [aux_sym_source_repeat1] = STATE(19),
    [aux_sym_source_repeat2] = STATE(29),
    [aux_sym_source_repeat3] = STATE(65),
    [ts_builtin_sym_end] = ACTIONS(5),
    [aux_sym_source_token1] = ACTIONS(7),
    [aux_sym_subject_token1] = ACTIONS(9),
    [aux_sym_message_token1] = ACTIONS(3),
    [anon_sym_POUND] = ACTIONS(11),
    [sym__newline] = ACTIONS(13),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 14,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(19), 1,
      anon_sym_interactive,
    ACTIONS(21), 1,
      anon_sym_Changes,
    ACTIONS(23), 1,
      anon_sym_On,
    ACTIONS(25), 1,
      anon_sym_Your,
    ACTIONS(27), 1,
      anon_sym_HEAD,
    ACTIONS(29), 1,
      anon_sym_Conflicts,
    ACTIONS(31), 1,
      anon_sym_Untracked,
    STATE(240), 1,
      sym_header,
    STATE(241), 1,
      sym__change_header,
    STATE(24), 2,
      sym__word,
      aux_sym__comment_body_repeat1,
    ACTIONS(15), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(33), 3,
      sym__non_punctuated_word,
      sym__non_comment,
      sym__any_word,
    STATE(84), 4,
      sym__comment_body,
      sym__rebase_summary,
      sym_summary,
      sym__branch_declaration,
  [51] = 7,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(35), 1,
      anon_sym_Last,
    ACTIONS(37), 1,
      anon_sym_Next,
    ACTIONS(39), 1,
      anon_sym_No,
    STATE(160), 1,
      sym_rebase_command,
    STATE(168), 1,
      sym__rebase_header,
    ACTIONS(41), 10,
      anon_sym_pick,
      anon_sym_edit,
      anon_sym_squash,
      anon_sym_merge,
      anon_sym_fixup,
      anon_sym_drop,
      anon_sym_reword,
      anon_sym_exec,
      anon_sym_label,
      anon_sym_reset,
  [82] = 7,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(35), 1,
      anon_sym_Last,
    ACTIONS(37), 1,
      anon_sym_Next,
    ACTIONS(39), 1,
      anon_sym_No,
    STATE(159), 1,
      sym__rebase_header,
    STATE(160), 1,
      sym_rebase_command,
    ACTIONS(41), 10,
      anon_sym_pick,
      anon_sym_edit,
      anon_sym_squash,
      anon_sym_merge,
      anon_sym_fixup,
      anon_sym_drop,
      anon_sym_reword,
      anon_sym_exec,
      anon_sym_label,
      anon_sym_reset,
  [113] = 6,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(45), 2,
      anon_sym_COLON,
      anon_sym_EQ,
    ACTIONS(43), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(47), 3,
      sym_commit,
      sym_item,
      sym_user,
    ACTIONS(49), 3,
      sym__non_punctuated_word,
      sym__non_comment,
      sym__any_word,
    STATE(11), 3,
      sym__text,
      sym__word,
      aux_sym_message_repeat1,
  [141] = 10,
    ACTIONS(11), 1,
      anon_sym_POUND,
    ACTIONS(57), 1,
      sym__any_word,
    STATE(66), 1,
      sym_comment,
    STATE(95), 1,
      sym_trailer,
    STATE(97), 1,
      sym_message,
    STATE(99), 1,
      sym__word,
    STATE(101), 1,
      sym_trailer_key,
    ACTIONS(55), 2,
      sym__non_punctuated_word,
      sym__non_comment,
    ACTIONS(51), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(53), 3,
      aux_sym_message_token1,
      sym_commit,
      sym_user,
  [177] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(59), 1,
      anon_sym_You,
    STATE(160), 1,
      sym_rebase_command,
    ACTIONS(41), 10,
      anon_sym_pick,
      anon_sym_edit,
      anon_sym_squash,
      anon_sym_merge,
      anon_sym_fixup,
      anon_sym_drop,
      anon_sym_reword,
      anon_sym_exec,
      anon_sym_label,
      anon_sym_reset,
  [199] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(61), 1,
      anon_sym_You,
    STATE(160), 1,
      sym_rebase_command,
    ACTIONS(41), 10,
      anon_sym_pick,
      anon_sym_edit,
      anon_sym_squash,
      anon_sym_merge,
      anon_sym_fixup,
      anon_sym_drop,
      anon_sym_reword,
      anon_sym_exec,
      anon_sym_label,
      anon_sym_reset,
  [221] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(43), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(47), 3,
      sym_commit,
      sym_item,
      sym_user,
    ACTIONS(49), 3,
      sym__non_punctuated_word,
      sym__non_comment,
      sym__any_word,
    STATE(11), 3,
      sym__text,
      sym__word,
      aux_sym_message_repeat1,
  [245] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(63), 1,
      anon_sym_You,
    STATE(160), 1,
      sym_rebase_command,
    ACTIONS(41), 10,
      anon_sym_pick,
      anon_sym_edit,
      anon_sym_squash,
      anon_sym_merge,
      anon_sym_fixup,
      anon_sym_drop,
      anon_sym_reword,
      anon_sym_exec,
      anon_sym_label,
      anon_sym_reset,
  [267] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(65), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(67), 3,
      sym_commit,
      sym_item,
      sym_user,
    ACTIONS(69), 3,
      sym__non_punctuated_word,
      sym__non_comment,
      sym__any_word,
    STATE(12), 3,
      sym__text,
      sym__word,
      aux_sym_message_repeat1,
  [291] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(71), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(73), 3,
      sym_commit,
      sym_item,
      sym_user,
    ACTIONS(76), 3,
      sym__non_punctuated_word,
      sym__non_comment,
      sym__any_word,
    STATE(12), 3,
      sym__text,
      sym__word,
      aux_sym_message_repeat1,
  [315] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    STATE(160), 1,
      sym_rebase_command,
    ACTIONS(41), 10,
      anon_sym_pick,
      anon_sym_edit,
      anon_sym_squash,
      anon_sym_merge,
      anon_sym_fixup,
      anon_sym_drop,
      anon_sym_reword,
      anon_sym_exec,
      anon_sym_label,
      anon_sym_reset,
  [334] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    STATE(14), 2,
      sym__word,
      aux_sym_trailer_value_repeat1,
    ACTIONS(79), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(81), 3,
      sym_commit,
      sym_item,
      sym_user,
    ACTIONS(84), 3,
      sym__non_punctuated_word,
      sym__non_comment,
      sym__any_word,
  [357] = 9,
    ACTIONS(11), 1,
      anon_sym_POUND,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(89), 1,
      sym__non_punctuated_word,
    STATE(71), 1,
      sym_comment,
    STATE(95), 1,
      sym_trailer,
    STATE(99), 1,
      sym__word,
    STATE(101), 1,
      sym_trailer_key,
    ACTIONS(57), 2,
      sym__non_comment,
      sym__any_word,
    ACTIONS(87), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [388] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    STATE(14), 2,
      sym__word,
      aux_sym_trailer_value_repeat1,
    ACTIONS(91), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(93), 3,
      sym_commit,
      sym_item,
      sym_user,
    ACTIONS(95), 3,
      sym__non_punctuated_word,
      sym__non_comment,
      sym__any_word,
  [411] = 5,
    ACTIONS(11), 1,
      anon_sym_POUND,
    ACTIONS(97), 2,
      sym__non_punctuated_word,
      sym__non_comment,
    STATE(97), 2,
      sym_message,
      sym_comment,
    ACTIONS(51), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(53), 3,
      aux_sym_message_token1,
      sym_commit,
      sym_user,
  [433] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    STATE(76), 1,
      sym_trailer_value,
    STATE(16), 2,
      sym__word,
      aux_sym_trailer_value_repeat1,
    ACTIONS(99), 3,
      sym_commit,
      sym_item,
      sym_user,
    ACTIONS(101), 3,
      sym__non_punctuated_word,
      sym__non_comment,
      sym__any_word,
  [454] = 7,
    ACTIONS(13), 1,
      sym__newline,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(103), 1,
      ts_builtin_sym_end,
    ACTIONS(105), 1,
      aux_sym_source_token1,
    STATE(42), 1,
      aux_sym_source_repeat3,
    STATE(28), 2,
      sym__trailer,
      aux_sym_source_repeat2,
    STATE(36), 2,
      sym__body_line,
      aux_sym_source_repeat1,
  [478] = 7,
    ACTIONS(13), 1,
      sym__newline,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(107), 1,
      ts_builtin_sym_end,
    ACTIONS(109), 1,
      aux_sym_source_token1,
    STATE(59), 1,
      aux_sym_source_repeat3,
    STATE(26), 2,
      sym__trailer,
      aux_sym_source_repeat2,
    STATE(36), 2,
      sym__body_line,
      aux_sym_source_repeat1,
  [502] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    STATE(161), 1,
      sym_change,
    ACTIONS(111), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
    ACTIONS(113), 4,
      anon_sym_newfile,
      anon_sym_modified,
      anon_sym_renamed,
      anon_sym_deleted,
  [520] = 7,
    ACTIONS(13), 1,
      sym__newline,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(103), 1,
      ts_builtin_sym_end,
    ACTIONS(105), 1,
      aux_sym_source_token1,
    STATE(42), 1,
      aux_sym_source_repeat3,
    STATE(20), 2,
      sym__body_line,
      aux_sym_source_repeat1,
    STATE(28), 2,
      sym__trailer,
      aux_sym_source_repeat2,
  [544] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(117), 1,
      sym__non_punctuated_word,
    ACTIONS(120), 2,
      sym__non_comment,
      sym__any_word,
    STATE(23), 2,
      sym__word,
      aux_sym__comment_body_repeat1,
    ACTIONS(115), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [564] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(125), 1,
      sym__non_punctuated_word,
    ACTIONS(127), 2,
      sym__non_comment,
      sym__any_word,
    STATE(23), 2,
      sym__word,
      aux_sym__comment_body_repeat1,
    ACTIONS(123), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [584] = 6,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(129), 1,
      aux_sym_source_token1,
    ACTIONS(131), 1,
      aux_sym_path_token1,
    STATE(94), 1,
      aux_sym_path_repeat1,
    STATE(158), 1,
      sym_path,
    ACTIONS(111), 2,
      ts_builtin_sym_end,
      sym__newline,
  [604] = 6,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(133), 1,
      ts_builtin_sym_end,
    ACTIONS(135), 1,
      aux_sym_source_token1,
    ACTIONS(137), 1,
      sym__newline,
    STATE(48), 1,
      aux_sym_source_repeat3,
    STATE(37), 2,
      sym__trailer,
      aux_sym_source_repeat2,
  [624] = 6,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(89), 1,
      sym__non_punctuated_word,
    STATE(95), 1,
      sym_trailer,
    STATE(99), 1,
      sym__word,
    STATE(101), 1,
      sym_trailer_key,
    ACTIONS(57), 2,
      sym__non_comment,
      sym__any_word,
  [644] = 6,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(107), 1,
      ts_builtin_sym_end,
    ACTIONS(109), 1,
      aux_sym_source_token1,
    ACTIONS(137), 1,
      sym__newline,
    STATE(59), 1,
      aux_sym_source_repeat3,
    STATE(37), 2,
      sym__trailer,
      aux_sym_source_repeat2,
  [664] = 6,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(103), 1,
      ts_builtin_sym_end,
    ACTIONS(105), 1,
      aux_sym_source_token1,
    ACTIONS(137), 1,
      sym__newline,
    STATE(42), 1,
      aux_sym_source_repeat3,
    STATE(37), 2,
      sym__trailer,
      aux_sym_source_repeat2,
  [684] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(141), 1,
      anon_sym_POUND,
    STATE(33), 1,
      aux_sym_summary_repeat1,
    ACTIONS(139), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [699] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(143), 1,
      anon_sym_POUND,
    STATE(32), 1,
      aux_sym_summary_repeat2,
    ACTIONS(139), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [714] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(147), 1,
      anon_sym_POUND,
    STATE(32), 1,
      aux_sym_summary_repeat2,
    ACTIONS(145), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [729] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(152), 1,
      anon_sym_POUND,
    STATE(33), 1,
      aux_sym_summary_repeat1,
    ACTIONS(150), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [744] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(157), 1,
      aux_sym_subject_token2,
    STATE(39), 1,
      aux_sym_subject_repeat1,
    ACTIONS(155), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [759] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    STATE(161), 1,
      sym_change,
    ACTIONS(113), 4,
      anon_sym_newfile,
      anon_sym_modified,
      anon_sym_renamed,
      anon_sym_deleted,
  [772] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(161), 1,
      sym__newline,
    ACTIONS(159), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
    STATE(36), 2,
      sym__body_line,
      aux_sym_source_repeat1,
  [787] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(166), 1,
      sym__newline,
    ACTIONS(164), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
    STATE(37), 2,
      sym__trailer,
      aux_sym_source_repeat2,
  [802] = 4,
    ACTIONS(11), 1,
      anon_sym_POUND,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    STATE(71), 1,
      sym_comment,
    ACTIONS(87), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [817] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(171), 1,
      aux_sym_subject_token2,
    STATE(39), 1,
      aux_sym_subject_repeat1,
    ACTIONS(169), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [832] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(176), 1,
      aux_sym_subject_token2,
    STATE(34), 1,
      aux_sym_subject_repeat1,
    ACTIONS(174), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [847] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(180), 1,
      aux_sym_path_token1,
    STATE(62), 1,
      aux_sym_path_repeat1,
    ACTIONS(178), 2,
      anon_sym_DASH_GT,
      sym__newline,
  [861] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(107), 1,
      ts_builtin_sym_end,
    ACTIONS(109), 1,
      aux_sym_source_token1,
    ACTIONS(182), 1,
      sym__newline,
    STATE(43), 1,
      aux_sym_source_repeat3,
  [877] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(186), 1,
      sym__newline,
    STATE(43), 1,
      aux_sym_source_repeat3,
    ACTIONS(184), 2,
      ts_builtin_sym_end,
      aux_sym_source_token1,
  [891] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(191), 1,
      anon_sym_POUND,
    ACTIONS(189), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [903] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(193), 1,
      ts_builtin_sym_end,
    ACTIONS(195), 1,
      aux_sym__rest_token1,
    ACTIONS(198), 1,
      sym__newline,
    STATE(45), 1,
      aux_sym__rest,
  [919] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(103), 1,
      ts_builtin_sym_end,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    STATE(63), 1,
      aux_sym__rest,
  [935] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(107), 1,
      ts_builtin_sym_end,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    STATE(56), 1,
      aux_sym__rest,
  [951] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(182), 1,
      sym__newline,
    ACTIONS(205), 1,
      ts_builtin_sym_end,
    ACTIONS(207), 1,
      aux_sym_source_token1,
    STATE(43), 1,
      aux_sym_source_repeat3,
  [967] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    ACTIONS(205), 1,
      ts_builtin_sym_end,
    STATE(51), 1,
      aux_sym__rest,
  [983] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    ACTIONS(209), 1,
      ts_builtin_sym_end,
    STATE(45), 1,
      aux_sym__rest,
  [999] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    ACTIONS(211), 1,
      ts_builtin_sym_end,
    STATE(45), 1,
      aux_sym__rest,
  [1015] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(215), 1,
      anon_sym_POUND,
    ACTIONS(213), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1027] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    ACTIONS(217), 1,
      ts_builtin_sym_end,
    STATE(64), 1,
      aux_sym__rest,
  [1043] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(221), 1,
      anon_sym_POUND,
    ACTIONS(219), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1055] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(35), 1,
      anon_sym_Last,
    ACTIONS(37), 1,
      anon_sym_Next,
    ACTIONS(39), 1,
      anon_sym_No,
    STATE(139), 1,
      sym__rebase_header,
  [1071] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    ACTIONS(223), 1,
      ts_builtin_sym_end,
    STATE(45), 1,
      aux_sym__rest,
  [1087] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(133), 1,
      ts_builtin_sym_end,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    STATE(50), 1,
      aux_sym__rest,
  [1103] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(227), 1,
      anon_sym_POUND,
    ACTIONS(225), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1115] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(133), 1,
      ts_builtin_sym_end,
    ACTIONS(135), 1,
      aux_sym_source_token1,
    ACTIONS(182), 1,
      sym__newline,
    STATE(43), 1,
      aux_sym_source_repeat3,
  [1131] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(231), 1,
      anon_sym_POUND,
    ACTIONS(229), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1143] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(235), 1,
      anon_sym_POUND,
    ACTIONS(233), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1155] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(239), 1,
      aux_sym_path_token1,
    STATE(62), 1,
      aux_sym_path_repeat1,
    ACTIONS(237), 2,
      anon_sym_DASH_GT,
      sym__newline,
  [1169] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    ACTIONS(242), 1,
      ts_builtin_sym_end,
    STATE(45), 1,
      aux_sym__rest,
  [1185] = 5,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(201), 1,
      aux_sym__rest_token1,
    ACTIONS(203), 1,
      sym__newline,
    ACTIONS(244), 1,
      ts_builtin_sym_end,
    STATE(45), 1,
      aux_sym__rest,
  [1201] = 5,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(103), 1,
      ts_builtin_sym_end,
    ACTIONS(105), 1,
      aux_sym_source_token1,
    ACTIONS(182), 1,
      sym__newline,
    STATE(43), 1,
      aux_sym_source_repeat3,
  [1217] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(246), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1226] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(248), 1,
      aux_sym_path_token1,
    STATE(41), 1,
      aux_sym_path_repeat1,
    STATE(112), 1,
      sym_path,
  [1239] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(250), 1,
      aux_sym__rebase_summary_token1,
    ACTIONS(252), 1,
      anon_sym_rebasing,
    STATE(79), 1,
      aux_sym__rebase_summary_repeat2,
  [1252] = 3,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(256), 1,
      aux_sym__rest_token1,
    ACTIONS(254), 2,
      ts_builtin_sym_end,
      sym__newline,
  [1263] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(237), 1,
      sym__newline,
    ACTIONS(258), 1,
      aux_sym_path_token1,
    STATE(70), 1,
      aux_sym_path_repeat1,
  [1276] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(184), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1285] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(261), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1294] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(131), 1,
      aux_sym_path_token1,
    STATE(94), 1,
      aux_sym_path_repeat1,
    STATE(158), 1,
      sym_path,
  [1307] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(263), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1316] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(265), 1,
      aux_sym__rebase_summary_token1,
    ACTIONS(268), 1,
      anon_sym_rebasing,
    STATE(75), 1,
      aux_sym__rebase_summary_repeat2,
  [1329] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(270), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1338] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(225), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1347] = 4,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(272), 1,
      anon_sym_up,
    ACTIONS(274), 1,
      anon_sym_ahead,
    ACTIONS(276), 1,
      anon_sym_behind,
  [1360] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(278), 1,
      aux_sym__rebase_summary_token1,
    ACTIONS(280), 1,
      anon_sym_rebasing,
    STATE(75), 1,
      aux_sym__rebase_summary_repeat2,
  [1373] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(280), 1,
      anon_sym_rebasing,
    ACTIONS(282), 1,
      aux_sym__rebase_summary_token1,
    STATE(82), 1,
      aux_sym__rebase_summary_repeat2,
  [1386] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(219), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1395] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(278), 1,
      aux_sym__rebase_summary_token1,
    ACTIONS(284), 1,
      anon_sym_rebasing,
    STATE(75), 1,
      aux_sym__rebase_summary_repeat2,
  [1408] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(286), 1,
      aux_sym__rebase_summary_token1,
    ACTIONS(289), 1,
      sym__newline,
    STATE(83), 1,
      aux_sym__rebase_summary_repeat2,
  [1421] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(291), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1430] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(293), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1439] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(295), 1,
      aux_sym__rebase_summary_token1,
    ACTIONS(297), 1,
      sym__newline,
    STATE(83), 1,
      aux_sym__rebase_summary_repeat2,
  [1452] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(299), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1461] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(301), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1470] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(284), 1,
      anon_sym_rebasing,
    ACTIONS(303), 1,
      aux_sym__rebase_summary_token1,
    STATE(91), 1,
      aux_sym__rebase_summary_repeat2,
  [1483] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(305), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1492] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(278), 1,
      aux_sym__rebase_summary_token1,
    ACTIONS(307), 1,
      anon_sym_rebasing,
    STATE(75), 1,
      aux_sym__rebase_summary_repeat2,
  [1505] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(213), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1514] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(309), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1523] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(178), 1,
      sym__newline,
    ACTIONS(311), 1,
      aux_sym_path_token1,
    STATE(70), 1,
      aux_sym_path_repeat1,
  [1536] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(313), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1545] = 4,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(131), 1,
      aux_sym_path_token1,
    STATE(94), 1,
      aux_sym_path_repeat1,
    STATE(130), 1,
      sym_path,
  [1558] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(246), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      sym__newline,
  [1567] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(315), 1,
      anon_sym_POUND,
    STATE(30), 1,
      aux_sym_summary_repeat1,
  [1577] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(317), 2,
      anon_sym_COLON,
      anon_sym_EQ,
  [1585] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(319), 1,
      anon_sym_POUND,
    STATE(105), 1,
      aux_sym__rebase_summary_repeat1,
  [1595] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(321), 2,
      anon_sym_COLON,
      anon_sym_EQ,
  [1603] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(323), 2,
      sym_commit,
      sym_branch,
  [1611] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(325), 1,
      anon_sym_POUND,
    STATE(100), 1,
      aux_sym__rebase_summary_repeat1,
  [1621] = 3,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(327), 1,
      aux_sym__rebase_summary_token1,
    STATE(86), 1,
      aux_sym__rebase_summary_repeat2,
  [1631] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(329), 1,
      anon_sym_POUND,
    STATE(105), 1,
      aux_sym__rebase_summary_repeat1,
  [1641] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(332), 1,
      anon_sym_POUND,
    STATE(108), 1,
      aux_sym__rebase_summary_repeat1,
  [1651] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(334), 1,
      anon_sym_is,
    ACTIONS(336), 1,
      anon_sym_and,
  [1661] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(338), 1,
      anon_sym_POUND,
    STATE(105), 1,
      aux_sym__rebase_summary_repeat1,
  [1671] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(340), 1,
      anon_sym_POUND,
    STATE(31), 1,
      aux_sym_summary_repeat2,
  [1681] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(342), 1,
      anon_sym_POUND,
    STATE(105), 1,
      aux_sym__rebase_summary_repeat1,
  [1691] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(344), 1,
      anon_sym_to,
    ACTIONS(346), 1,
      anon_sym_not,
  [1701] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(348), 1,
      anon_sym_DASH_GT,
    ACTIONS(350), 1,
      sym__newline,
  [1711] = 3,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(338), 1,
      anon_sym_POUND,
    STATE(110), 1,
      aux_sym__rebase_summary_repeat1,
  [1721] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(352), 1,
      anon_sym_diverged,
  [1728] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(354), 1,
      anon_sym_COLON,
  [1735] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(356), 1,
      sym__newline,
  [1742] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(358), 1,
      anon_sym_with,
  [1749] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(360), 1,
      sym_branch,
  [1756] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(362), 1,
      anon_sym_SQUOTE,
  [1763] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(364), 1,
      anon_sym_have,
  [1770] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(366), 1,
      sym__newline,
  [1777] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(368), 1,
      anon_sym_SQUOTE,
  [1784] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(370), 1,
      anon_sym_SQUOTE,
  [1791] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(372), 1,
      anon_sym_by,
  [1798] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(374), 1,
      anon_sym_POUND,
  [1805] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(376), 1,
      sym_branch,
  [1812] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(378), 1,
      anon_sym_by,
  [1819] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(380), 1,
      aux_sym__rebase_header_token2,
  [1826] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(382), 1,
      anon_sym_COMMA,
  [1833] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(384), 1,
      sym__newline,
  [1840] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(386), 1,
      ts_builtin_sym_end,
  [1847] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(388), 1,
      anon_sym_SQUOTE_DOT,
  [1854] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(390), 1,
      aux_sym__rebase_header_token2,
  [1861] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(392), 1,
      aux_sym__branch_declaration_token1,
  [1868] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(394), 1,
      anon_sym_SQUOTE,
  [1875] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(396), 1,
      aux_sym__rebase_header_token1,
  [1882] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(398), 1,
      aux_sym__rebase_header_token1,
  [1889] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(400), 1,
      anon_sym_commands,
  [1896] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(402), 1,
      sym__newline,
  [1903] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(404), 1,
      sym_branch,
  [1910] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(406), 1,
      aux_sym__branch_declaration_token1,
  [1917] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(408), 1,
      anon_sym_DOT,
  [1924] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(410), 1,
      anon_sym_done,
  [1931] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(412), 1,
      anon_sym_to,
  [1938] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(414), 1,
      anon_sym_remaining,
  [1945] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(416), 1,
      anon_sym_SQUOTE,
  [1952] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(418), 1,
      anon_sym_DOT,
  [1959] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(420), 1,
      anon_sym_date,
  [1966] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(422), 1,
      anon_sym_LPAREN,
  [1973] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(424), 1,
      anon_sym_do,
  [1980] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(426), 1,
      anon_sym_DOT,
  [1987] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(428), 1,
      anon_sym_COLON,
  [1994] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(430), 1,
      sym__newline,
  [2001] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(432), 1,
      anon_sym_onto,
  [2008] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(434), 1,
      aux_sym__rebase_header_token2,
  [2015] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(436), 1,
      anon_sym_LPAREN,
  [2022] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(438), 1,
      sym__newline,
  [2029] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(440), 1,
      sym__newline,
  [2036] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(442), 1,
      sym__newline,
  [2043] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(444), 1,
      sym__newline,
  [2050] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(446), 1,
      sym__newline,
  [2057] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(448), 1,
      anon_sym_COLON,
  [2064] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(450), 1,
      aux_sym__rebase_header_token1,
  [2071] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(452), 1,
      aux_sym__rebase_header_token2,
  [2078] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(454), 1,
      sym_branch,
  [2085] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(456), 1,
      anon_sym_SQUOTE,
  [2092] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(458), 1,
      anon_sym_POUND,
  [2099] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(460), 1,
      sym__newline,
  [2106] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(462), 1,
      anon_sym_of,
  [2113] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(464), 1,
      anon_sym_done,
  [2120] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(466), 1,
      anon_sym_remaining,
  [2127] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(468), 1,
      anon_sym_to,
  [2134] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(470), 1,
      anon_sym_commit,
  [2141] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(472), 1,
      anon_sym_COLON,
  [2148] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(474), 1,
      anon_sym_SEMI,
  [2155] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(476), 1,
      anon_sym_RPAREN,
  [2162] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(478), 1,
      aux_sym__rebase_header_token1,
  [2169] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(480), 1,
      anon_sym_are,
  [2176] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(482), 1,
      sym__newline,
  [2183] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(484), 1,
      anon_sym_SQUOTE,
  [2190] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(486), 1,
      sym_commit,
  [2197] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(488), 1,
      anon_sym_RPAREN,
  [2204] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(490), 1,
      anon_sym_currently,
  [2211] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(492), 1,
      anon_sym_are,
  [2218] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(494), 1,
      anon_sym_for,
  [2225] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(496), 1,
      sym__newline,
  [2232] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(498), 1,
      anon_sym_COLON,
  [2239] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(500), 1,
      anon_sym_committed,
  [2246] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(502), 1,
      anon_sym_currently,
  [2253] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(504), 1,
      anon_sym_are,
  [2260] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(506), 1,
      sym__newline,
  [2267] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(508), 1,
      anon_sym_branch,
  [2274] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(510), 1,
      anon_sym_progress,
  [2281] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(512), 1,
      anon_sym_COLON,
  [2288] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(514), 1,
      anon_sym_currently,
  [2295] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(516), 1,
      anon_sym_SQUOTE,
  [2302] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(518), 1,
      anon_sym_branch,
  [2309] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(520), 1,
      sym__newline,
  [2316] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(522), 1,
      anon_sym_at,
  [2323] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(524), 1,
      sym_branch,
  [2330] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(526), 1,
      anon_sym_SQUOTE,
  [2337] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(528), 1,
      anon_sym_branch,
  [2344] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(530), 1,
      sym_branch,
  [2351] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(532), 1,
      anon_sym_SQUOTE,
  [2358] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(534), 1,
      sym_branch,
  [2365] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(536), 1,
      anon_sym_SQUOTE,
  [2372] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(538), 1,
      anon_sym_branch,
  [2379] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(540), 1,
      anon_sym_on,
  [2386] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(542), 1,
      anon_sym_SQUOTE,
  [2393] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(544), 1,
      sym_branch,
  [2400] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(546), 1,
      anon_sym_SQUOTE,
  [2407] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(548), 1,
      anon_sym_SQUOTE,
  [2414] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(550), 1,
      anon_sym_on,
  [2421] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(552), 1,
      anon_sym_SQUOTE,
  [2428] = 2,
    ACTIONS(3), 1,
      aux_sym_message_token1,
    ACTIONS(554), 1,
      sym_branch,
  [2435] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(556), 1,
      sym_commit,
  [2442] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(558), 1,
      anon_sym_SQUOTE,
  [2449] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(560), 1,
      anon_sym_on,
  [2456] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(562), 1,
      anon_sym_SQUOTE,
  [2463] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(564), 1,
      anon_sym_SQUOTE,
  [2470] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(566), 1,
      sym_commit,
  [2477] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(568), 1,
      anon_sym_SQUOTE,
  [2484] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(570), 1,
      anon_sym_on,
  [2491] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(572), 1,
      anon_sym_DOT,
  [2498] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(574), 1,
      anon_sym_SQUOTE,
  [2505] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(576), 1,
      sym_commit,
  [2512] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(578), 1,
      anon_sym_SQUOTE,
  [2519] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(580), 1,
      sym__newline,
  [2526] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(582), 1,
      anon_sym_DOT,
  [2533] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(584), 1,
      anon_sym_SQUOTE,
  [2540] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(586), 1,
      sym_commit,
  [2547] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(588), 1,
      anon_sym_staged,
  [2554] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(590), 1,
      sym__newline,
  [2561] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(592), 1,
      anon_sym_DOT,
  [2568] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(594), 1,
      anon_sym_SQUOTE,
  [2575] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(596), 1,
      anon_sym_be,
  [2582] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(598), 1,
      anon_sym_in,
  [2589] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(600), 1,
      sym__newline,
  [2596] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(602), 1,
      anon_sym_DOT,
  [2603] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(604), 1,
      sym__newline,
  [2610] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(606), 1,
      sym__newline,
  [2617] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(608), 1,
      sym__newline,
  [2624] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(610), 1,
      anon_sym_files,
  [2631] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(612), 1,
      anon_sym_COLON,
  [2638] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(614), 1,
      anon_sym_detached,
  [2645] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(616), 1,
      anon_sym_branch,
  [2652] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(618), 1,
      anon_sym_branch,
  [2659] = 2,
    ACTIONS(17), 1,
      aux_sym_message_token1,
    ACTIONS(620), 1,
      anon_sym_rebase,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 51,
  [SMALL_STATE(4)] = 82,
  [SMALL_STATE(5)] = 113,
  [SMALL_STATE(6)] = 141,
  [SMALL_STATE(7)] = 177,
  [SMALL_STATE(8)] = 199,
  [SMALL_STATE(9)] = 221,
  [SMALL_STATE(10)] = 245,
  [SMALL_STATE(11)] = 267,
  [SMALL_STATE(12)] = 291,
  [SMALL_STATE(13)] = 315,
  [SMALL_STATE(14)] = 334,
  [SMALL_STATE(15)] = 357,
  [SMALL_STATE(16)] = 388,
  [SMALL_STATE(17)] = 411,
  [SMALL_STATE(18)] = 433,
  [SMALL_STATE(19)] = 454,
  [SMALL_STATE(20)] = 478,
  [SMALL_STATE(21)] = 502,
  [SMALL_STATE(22)] = 520,
  [SMALL_STATE(23)] = 544,
  [SMALL_STATE(24)] = 564,
  [SMALL_STATE(25)] = 584,
  [SMALL_STATE(26)] = 604,
  [SMALL_STATE(27)] = 624,
  [SMALL_STATE(28)] = 644,
  [SMALL_STATE(29)] = 664,
  [SMALL_STATE(30)] = 684,
  [SMALL_STATE(31)] = 699,
  [SMALL_STATE(32)] = 714,
  [SMALL_STATE(33)] = 729,
  [SMALL_STATE(34)] = 744,
  [SMALL_STATE(35)] = 759,
  [SMALL_STATE(36)] = 772,
  [SMALL_STATE(37)] = 787,
  [SMALL_STATE(38)] = 802,
  [SMALL_STATE(39)] = 817,
  [SMALL_STATE(40)] = 832,
  [SMALL_STATE(41)] = 847,
  [SMALL_STATE(42)] = 861,
  [SMALL_STATE(43)] = 877,
  [SMALL_STATE(44)] = 891,
  [SMALL_STATE(45)] = 903,
  [SMALL_STATE(46)] = 919,
  [SMALL_STATE(47)] = 935,
  [SMALL_STATE(48)] = 951,
  [SMALL_STATE(49)] = 967,
  [SMALL_STATE(50)] = 983,
  [SMALL_STATE(51)] = 999,
  [SMALL_STATE(52)] = 1015,
  [SMALL_STATE(53)] = 1027,
  [SMALL_STATE(54)] = 1043,
  [SMALL_STATE(55)] = 1055,
  [SMALL_STATE(56)] = 1071,
  [SMALL_STATE(57)] = 1087,
  [SMALL_STATE(58)] = 1103,
  [SMALL_STATE(59)] = 1115,
  [SMALL_STATE(60)] = 1131,
  [SMALL_STATE(61)] = 1143,
  [SMALL_STATE(62)] = 1155,
  [SMALL_STATE(63)] = 1169,
  [SMALL_STATE(64)] = 1185,
  [SMALL_STATE(65)] = 1201,
  [SMALL_STATE(66)] = 1217,
  [SMALL_STATE(67)] = 1226,
  [SMALL_STATE(68)] = 1239,
  [SMALL_STATE(69)] = 1252,
  [SMALL_STATE(70)] = 1263,
  [SMALL_STATE(71)] = 1276,
  [SMALL_STATE(72)] = 1285,
  [SMALL_STATE(73)] = 1294,
  [SMALL_STATE(74)] = 1307,
  [SMALL_STATE(75)] = 1316,
  [SMALL_STATE(76)] = 1329,
  [SMALL_STATE(77)] = 1338,
  [SMALL_STATE(78)] = 1347,
  [SMALL_STATE(79)] = 1360,
  [SMALL_STATE(80)] = 1373,
  [SMALL_STATE(81)] = 1386,
  [SMALL_STATE(82)] = 1395,
  [SMALL_STATE(83)] = 1408,
  [SMALL_STATE(84)] = 1421,
  [SMALL_STATE(85)] = 1430,
  [SMALL_STATE(86)] = 1439,
  [SMALL_STATE(87)] = 1452,
  [SMALL_STATE(88)] = 1461,
  [SMALL_STATE(89)] = 1470,
  [SMALL_STATE(90)] = 1483,
  [SMALL_STATE(91)] = 1492,
  [SMALL_STATE(92)] = 1505,
  [SMALL_STATE(93)] = 1514,
  [SMALL_STATE(94)] = 1523,
  [SMALL_STATE(95)] = 1536,
  [SMALL_STATE(96)] = 1545,
  [SMALL_STATE(97)] = 1558,
  [SMALL_STATE(98)] = 1567,
  [SMALL_STATE(99)] = 1577,
  [SMALL_STATE(100)] = 1585,
  [SMALL_STATE(101)] = 1595,
  [SMALL_STATE(102)] = 1603,
  [SMALL_STATE(103)] = 1611,
  [SMALL_STATE(104)] = 1621,
  [SMALL_STATE(105)] = 1631,
  [SMALL_STATE(106)] = 1641,
  [SMALL_STATE(107)] = 1651,
  [SMALL_STATE(108)] = 1661,
  [SMALL_STATE(109)] = 1671,
  [SMALL_STATE(110)] = 1681,
  [SMALL_STATE(111)] = 1691,
  [SMALL_STATE(112)] = 1701,
  [SMALL_STATE(113)] = 1711,
  [SMALL_STATE(114)] = 1721,
  [SMALL_STATE(115)] = 1728,
  [SMALL_STATE(116)] = 1735,
  [SMALL_STATE(117)] = 1742,
  [SMALL_STATE(118)] = 1749,
  [SMALL_STATE(119)] = 1756,
  [SMALL_STATE(120)] = 1763,
  [SMALL_STATE(121)] = 1770,
  [SMALL_STATE(122)] = 1777,
  [SMALL_STATE(123)] = 1784,
  [SMALL_STATE(124)] = 1791,
  [SMALL_STATE(125)] = 1798,
  [SMALL_STATE(126)] = 1805,
  [SMALL_STATE(127)] = 1812,
  [SMALL_STATE(128)] = 1819,
  [SMALL_STATE(129)] = 1826,
  [SMALL_STATE(130)] = 1833,
  [SMALL_STATE(131)] = 1840,
  [SMALL_STATE(132)] = 1847,
  [SMALL_STATE(133)] = 1854,
  [SMALL_STATE(134)] = 1861,
  [SMALL_STATE(135)] = 1868,
  [SMALL_STATE(136)] = 1875,
  [SMALL_STATE(137)] = 1882,
  [SMALL_STATE(138)] = 1889,
  [SMALL_STATE(139)] = 1896,
  [SMALL_STATE(140)] = 1903,
  [SMALL_STATE(141)] = 1910,
  [SMALL_STATE(142)] = 1917,
  [SMALL_STATE(143)] = 1924,
  [SMALL_STATE(144)] = 1931,
  [SMALL_STATE(145)] = 1938,
  [SMALL_STATE(146)] = 1945,
  [SMALL_STATE(147)] = 1952,
  [SMALL_STATE(148)] = 1959,
  [SMALL_STATE(149)] = 1966,
  [SMALL_STATE(150)] = 1973,
  [SMALL_STATE(151)] = 1980,
  [SMALL_STATE(152)] = 1987,
  [SMALL_STATE(153)] = 1994,
  [SMALL_STATE(154)] = 2001,
  [SMALL_STATE(155)] = 2008,
  [SMALL_STATE(156)] = 2015,
  [SMALL_STATE(157)] = 2022,
  [SMALL_STATE(158)] = 2029,
  [SMALL_STATE(159)] = 2036,
  [SMALL_STATE(160)] = 2043,
  [SMALL_STATE(161)] = 2050,
  [SMALL_STATE(162)] = 2057,
  [SMALL_STATE(163)] = 2064,
  [SMALL_STATE(164)] = 2071,
  [SMALL_STATE(165)] = 2078,
  [SMALL_STATE(166)] = 2085,
  [SMALL_STATE(167)] = 2092,
  [SMALL_STATE(168)] = 2099,
  [SMALL_STATE(169)] = 2106,
  [SMALL_STATE(170)] = 2113,
  [SMALL_STATE(171)] = 2120,
  [SMALL_STATE(172)] = 2127,
  [SMALL_STATE(173)] = 2134,
  [SMALL_STATE(174)] = 2141,
  [SMALL_STATE(175)] = 2148,
  [SMALL_STATE(176)] = 2155,
  [SMALL_STATE(177)] = 2162,
  [SMALL_STATE(178)] = 2169,
  [SMALL_STATE(179)] = 2176,
  [SMALL_STATE(180)] = 2183,
  [SMALL_STATE(181)] = 2190,
  [SMALL_STATE(182)] = 2197,
  [SMALL_STATE(183)] = 2204,
  [SMALL_STATE(184)] = 2211,
  [SMALL_STATE(185)] = 2218,
  [SMALL_STATE(186)] = 2225,
  [SMALL_STATE(187)] = 2232,
  [SMALL_STATE(188)] = 2239,
  [SMALL_STATE(189)] = 2246,
  [SMALL_STATE(190)] = 2253,
  [SMALL_STATE(191)] = 2260,
  [SMALL_STATE(192)] = 2267,
  [SMALL_STATE(193)] = 2274,
  [SMALL_STATE(194)] = 2281,
  [SMALL_STATE(195)] = 2288,
  [SMALL_STATE(196)] = 2295,
  [SMALL_STATE(197)] = 2302,
  [SMALL_STATE(198)] = 2309,
  [SMALL_STATE(199)] = 2316,
  [SMALL_STATE(200)] = 2323,
  [SMALL_STATE(201)] = 2330,
  [SMALL_STATE(202)] = 2337,
  [SMALL_STATE(203)] = 2344,
  [SMALL_STATE(204)] = 2351,
  [SMALL_STATE(205)] = 2358,
  [SMALL_STATE(206)] = 2365,
  [SMALL_STATE(207)] = 2372,
  [SMALL_STATE(208)] = 2379,
  [SMALL_STATE(209)] = 2386,
  [SMALL_STATE(210)] = 2393,
  [SMALL_STATE(211)] = 2400,
  [SMALL_STATE(212)] = 2407,
  [SMALL_STATE(213)] = 2414,
  [SMALL_STATE(214)] = 2421,
  [SMALL_STATE(215)] = 2428,
  [SMALL_STATE(216)] = 2435,
  [SMALL_STATE(217)] = 2442,
  [SMALL_STATE(218)] = 2449,
  [SMALL_STATE(219)] = 2456,
  [SMALL_STATE(220)] = 2463,
  [SMALL_STATE(221)] = 2470,
  [SMALL_STATE(222)] = 2477,
  [SMALL_STATE(223)] = 2484,
  [SMALL_STATE(224)] = 2491,
  [SMALL_STATE(225)] = 2498,
  [SMALL_STATE(226)] = 2505,
  [SMALL_STATE(227)] = 2512,
  [SMALL_STATE(228)] = 2519,
  [SMALL_STATE(229)] = 2526,
  [SMALL_STATE(230)] = 2533,
  [SMALL_STATE(231)] = 2540,
  [SMALL_STATE(232)] = 2547,
  [SMALL_STATE(233)] = 2554,
  [SMALL_STATE(234)] = 2561,
  [SMALL_STATE(235)] = 2568,
  [SMALL_STATE(236)] = 2575,
  [SMALL_STATE(237)] = 2582,
  [SMALL_STATE(238)] = 2589,
  [SMALL_STATE(239)] = 2596,
  [SMALL_STATE(240)] = 2603,
  [SMALL_STATE(241)] = 2610,
  [SMALL_STATE(242)] = 2617,
  [SMALL_STATE(243)] = 2624,
  [SMALL_STATE(244)] = 2631,
  [SMALL_STATE(245)] = 2638,
  [SMALL_STATE(246)] = 2645,
  [SMALL_STATE(247)] = 2652,
  [SMALL_STATE(248)] = 2659,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 1),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(248),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(247),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(246),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(245),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(243),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_message, 1),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__word, 1),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [49] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__body_line, 1),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [55] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [57] = {.entry = {.count = 1, .reusable = false}}, SHIFT(99),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_message, 2),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [69] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_message_repeat1, 2),
  [73] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_message_repeat1, 2), SHIFT_REPEAT(12),
  [76] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_message_repeat1, 2), SHIFT_REPEAT(12),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_trailer_value_repeat1, 2),
  [81] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_trailer_value_repeat1, 2), SHIFT_REPEAT(14),
  [84] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_trailer_value_repeat1, 2), SHIFT_REPEAT(14),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_repeat3, 1),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_trailer_value, 1),
  [93] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [95] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [97] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [101] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [103] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 1),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 2),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_summary, 4),
  [113] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__comment_body_repeat1, 2),
  [117] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__comment_body_repeat1, 2), SHIFT_REPEAT(23),
  [120] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__comment_body_repeat1, 2), SHIFT_REPEAT(23),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comment_body, 1),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [127] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [129] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_summary, 4),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 3),
  [135] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [137] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_summary, 3),
  [141] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [143] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_summary_repeat2, 2),
  [147] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_summary_repeat2, 2), SHIFT_REPEAT(73),
  [150] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_summary_repeat1, 2),
  [152] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_summary_repeat1, 2), SHIFT_REPEAT(35),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subject, 2),
  [157] = {.entry = {.count = 1, .reusable = false}}, SHIFT(39),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2),
  [161] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2), SHIFT_REPEAT(17),
  [164] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_repeat2, 2),
  [166] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat2, 2), SHIFT_REPEAT(27),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_subject_repeat1, 2),
  [171] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_repeat1, 2), SHIFT_REPEAT(39),
  [174] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subject, 1),
  [176] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [178] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_path, 1),
  [180] = {.entry = {.count = 1, .reusable = false}}, SHIFT(62),
  [182] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [184] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_repeat3, 2),
  [186] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat3, 2), SHIFT_REPEAT(38),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rebase_summary, 29),
  [191] = {.entry = {.count = 1, .reusable = false}}, SHIFT(92),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__rest, 2),
  [195] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__rest, 2), SHIFT_REPEAT(69),
  [198] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__rest, 2), SHIFT_REPEAT(69),
  [201] = {.entry = {.count = 1, .reusable = false}}, SHIFT(69),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 4),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 4, .production_id = 4),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 5, .production_id = 5),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rebase_summary, 30),
  [215] = {.entry = {.count = 1, .reusable = false}}, SHIFT(81),
  [217] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 5),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rebase_summary, 31),
  [221] = {.entry = {.count = 1, .reusable = false}}, SHIFT(77),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 3, .production_id = 2),
  [225] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rebase_summary, 32),
  [227] = {.entry = {.count = 1, .reusable = false}}, SHIFT(72),
  [229] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_summary_repeat1, 3),
  [231] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_summary_repeat1, 3),
  [233] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_summary_repeat2, 3),
  [235] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_summary_repeat2, 3),
  [237] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_path_repeat1, 2),
  [239] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_path_repeat1, 2), SHIFT_REPEAT(62),
  [242] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 2, .production_id = 1),
  [244] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 6, .production_id = 6),
  [246] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__body_line, 2),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [250] = {.entry = {.count = 1, .reusable = false}}, SHIFT(79),
  [252] = {.entry = {.count = 1, .reusable = false}}, SHIFT(192),
  [254] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__rest, 1),
  [256] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__rest, 1),
  [258] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_path_repeat1, 2), SHIFT_REPEAT(70),
  [261] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rebase_summary, 33),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__branch_declaration, 3),
  [265] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__rebase_summary_repeat2, 2), SHIFT_REPEAT(75),
  [268] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__rebase_summary_repeat2, 2),
  [270] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_trailer, 3, .production_id = 3),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [276] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [278] = {.entry = {.count = 1, .reusable = false}}, SHIFT(75),
  [280] = {.entry = {.count = 1, .reusable = false}}, SHIFT(197),
  [282] = {.entry = {.count = 1, .reusable = false}}, SHIFT(82),
  [284] = {.entry = {.count = 1, .reusable = false}}, SHIFT(202),
  [286] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__rebase_summary_repeat2, 2), SHIFT_REPEAT(83),
  [289] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__rebase_summary_repeat2, 2),
  [291] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 2),
  [293] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__branch_declaration, 9),
  [295] = {.entry = {.count = 1, .reusable = false}}, SHIFT(83),
  [297] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rebase_command, 2),
  [299] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__branch_declaration, 10),
  [301] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__branch_declaration, 4),
  [303] = {.entry = {.count = 1, .reusable = false}}, SHIFT(91),
  [305] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__branch_declaration, 11),
  [307] = {.entry = {.count = 1, .reusable = false}}, SHIFT(207),
  [309] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__branch_declaration, 12),
  [311] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [313] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__trailer, 2),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [317] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_trailer_key, 1),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [323] = {.entry = {.count = 1, .reusable = false}}, SHIFT(88),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [327] = {.entry = {.count = 1, .reusable = false}}, SHIFT(86),
  [329] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__rebase_summary_repeat1, 2), SHIFT_REPEAT(13),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [340] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [344] = {.entry = {.count = 1, .reusable = true}}, SHIFT(236),
  [346] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [348] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [350] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_change, 3, .production_id = 7),
  [352] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [354] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [356] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__change_header, 6),
  [358] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [360] = {.entry = {.count = 1, .reusable = false}}, SHIFT(123),
  [362] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [364] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [366] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [368] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [372] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [374] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [376] = {.entry = {.count = 1, .reusable = false}}, SHIFT(132),
  [378] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [380] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [384] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_change, 5, .production_id = 7),
  [386] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [392] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [396] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [398] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [400] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [402] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [404] = {.entry = {.count = 1, .reusable = false}}, SHIFT(119),
  [406] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [408] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [410] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [412] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [414] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [416] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [418] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [420] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [422] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [424] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [426] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [428] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [430] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__change_header, 5),
  [432] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [434] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [438] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rebase_header, 4),
  [440] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [442] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [444] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [446] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [448] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [450] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [452] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [454] = {.entry = {.count = 1, .reusable = false}}, SHIFT(135),
  [456] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [458] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__rebase_summary_repeat1, 3),
  [460] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [462] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [464] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [466] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [468] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [470] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [472] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [474] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [476] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [478] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [480] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [482] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_header, 3),
  [484] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [486] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [488] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [490] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [492] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [494] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [496] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rebase_header, 9),
  [498] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [500] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [502] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [504] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [506] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rebase_header, 10),
  [508] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [510] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [512] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [514] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [516] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [518] = {.entry = {.count = 1, .reusable = true}}, SHIFT(201),
  [520] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_header, 2),
  [522] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [524] = {.entry = {.count = 1, .reusable = false}}, SHIFT(204),
  [526] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [528] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [530] = {.entry = {.count = 1, .reusable = false}}, SHIFT(74),
  [532] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [534] = {.entry = {.count = 1, .reusable = false}}, SHIFT(209),
  [536] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [538] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [540] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [542] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [544] = {.entry = {.count = 1, .reusable = false}}, SHIFT(214),
  [546] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [548] = {.entry = {.count = 1, .reusable = true}}, SHIFT(216),
  [550] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [552] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [554] = {.entry = {.count = 1, .reusable = false}}, SHIFT(219),
  [556] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [558] = {.entry = {.count = 1, .reusable = true}}, SHIFT(221),
  [560] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [562] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [564] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [566] = {.entry = {.count = 1, .reusable = true}}, SHIFT(225),
  [568] = {.entry = {.count = 1, .reusable = true}}, SHIFT(226),
  [570] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [572] = {.entry = {.count = 1, .reusable = true}}, SHIFT(228),
  [574] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [576] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [578] = {.entry = {.count = 1, .reusable = true}}, SHIFT(231),
  [580] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [582] = {.entry = {.count = 1, .reusable = true}}, SHIFT(233),
  [584] = {.entry = {.count = 1, .reusable = true}}, SHIFT(234),
  [586] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [588] = {.entry = {.count = 1, .reusable = true}}, SHIFT(185),
  [590] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [592] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [594] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [596] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [598] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [600] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [602] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [604] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [606] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [608] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [610] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [612] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [614] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [616] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [618] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [620] = {.entry = {.count = 1, .reusable = true}}, SHIFT(237),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_git_commit(void) {
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
