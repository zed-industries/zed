#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
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
#define STATE_COUNT 381
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 135
#define ALIAS_COUNT 9
#define TOKEN_COUNT 72
#define EXTERNAL_TOKEN_COUNT 3
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 17

enum ts_symbol_identifiers {
  anon_sym_ATimport = 1,
  anon_sym_COMMA = 2,
  anon_sym_SEMI = 3,
  anon_sym_ATmedia = 4,
  anon_sym_ATcharset = 5,
  anon_sym_ATnamespace = 6,
  anon_sym_ATkeyframes = 7,
  aux_sym_keyframes_statement_token1 = 8,
  anon_sym_LBRACE = 9,
  anon_sym_RBRACE = 10,
  sym_from = 11,
  sym_to = 12,
  anon_sym_ATsupports = 13,
  sym_nesting_selector = 14,
  anon_sym_STAR = 15,
  anon_sym_DOT = 16,
  anon_sym_has = 17,
  anon_sym_not = 18,
  anon_sym_is = 19,
  anon_sym_where = 20,
  anon_sym_host = 21,
  anon_sym_host_DASHcontext = 22,
  anon_sym_nth_DASHchild = 23,
  anon_sym_nth_DASHlast_DASHchild = 24,
  anon_sym_COLON_COLON = 25,
  anon_sym_POUND = 26,
  anon_sym_LBRACK = 27,
  anon_sym_EQ = 28,
  anon_sym_TILDE_EQ = 29,
  anon_sym_CARET_EQ = 30,
  anon_sym_PIPE_EQ = 31,
  anon_sym_STAR_EQ = 32,
  anon_sym_DOLLAR_EQ = 33,
  anon_sym_RBRACK = 34,
  anon_sym_GT = 35,
  anon_sym_TILDE = 36,
  anon_sym_PLUS = 37,
  anon_sym_PIPE = 38,
  anon_sym_LPAREN = 39,
  anon_sym_RPAREN = 40,
  anon_sym_even = 41,
  anon_sym_odd = 42,
  anon_sym_of = 43,
  sym__nth_functional_notation = 44,
  anon_sym_COLON = 45,
  sym_important = 46,
  anon_sym_LPAREN2 = 47,
  anon_sym_and = 48,
  anon_sym_or = 49,
  anon_sym_only = 50,
  anon_sym_selector = 51,
  aux_sym_color_value_token1 = 52,
  anon_sym_SQUOTE = 53,
  aux_sym_string_value_token1 = 54,
  anon_sym_DQUOTE = 55,
  aux_sym_string_value_token2 = 56,
  sym_escape_sequence = 57,
  aux_sym_integer_value_token1 = 58,
  aux_sym_float_value_token1 = 59,
  sym_unit = 60,
  anon_sym_LBRACK2 = 61,
  anon_sym_DASH = 62,
  anon_sym_SLASH = 63,
  sym_identifier = 64,
  sym_at_keyword = 65,
  sym_js_comment = 66,
  sym_comment = 67,
  sym_plain_value = 68,
  sym__descendant_operator = 69,
  sym__pseudo_class_selector_colon = 70,
  sym___error_recovery = 71,
  sym_stylesheet = 72,
  sym_import_statement = 73,
  sym_media_statement = 74,
  sym_charset_statement = 75,
  sym_namespace_statement = 76,
  sym_keyframes_statement = 77,
  sym_keyframe_block_list = 78,
  sym_keyframe_block = 79,
  sym_supports_statement = 80,
  sym_postcss_statement = 81,
  sym_at_rule = 82,
  sym_rule_set = 83,
  sym_selectors = 84,
  sym_block = 85,
  sym__selector = 86,
  sym_universal_selector = 87,
  sym_class_selector = 88,
  sym_pseudo_class_selector = 89,
  sym__nth_child_pseudo_class_selector = 90,
  sym_pseudo_element_selector = 91,
  sym_id_selector = 92,
  sym_attribute_selector = 93,
  sym_child_selector = 94,
  sym_descendant_selector = 95,
  sym_sibling_selector = 96,
  sym_adjacent_sibling_selector = 97,
  sym_namespace_selector = 98,
  sym_pseudo_class_arguments = 99,
  sym_pseudo_class_with_selector_arguments = 100,
  sym_pseudo_class_nth_child_arguments = 101,
  sym_pseudo_element_arguments = 102,
  sym_declaration = 103,
  sym_last_declaration = 104,
  sym__query = 105,
  sym_feature_query = 106,
  sym_parenthesized_query = 107,
  sym_binary_query = 108,
  sym_unary_query = 109,
  sym_selector_query = 110,
  sym__value = 111,
  sym_parenthesized_value = 112,
  sym_color_value = 113,
  sym_string_value = 114,
  sym_integer_value = 115,
  sym_float_value = 116,
  sym_grid_value = 117,
  sym_call_expression = 118,
  sym_binary_expression = 119,
  sym_arguments = 120,
  sym_class_name = 121,
  aux_sym_stylesheet_repeat1 = 122,
  aux_sym_import_statement_repeat1 = 123,
  aux_sym_keyframe_block_list_repeat1 = 124,
  aux_sym_postcss_statement_repeat1 = 125,
  aux_sym_selectors_repeat1 = 126,
  aux_sym_block_repeat1 = 127,
  aux_sym_pseudo_class_arguments_repeat1 = 128,
  aux_sym_declaration_repeat1 = 129,
  aux_sym_string_value_repeat1 = 130,
  aux_sym_string_value_repeat2 = 131,
  aux_sym_grid_value_repeat1 = 132,
  aux_sym_arguments_repeat1 = 133,
  aux_sym_class_name_repeat1 = 134,
  alias_sym_attribute_name = 135,
  alias_sym_feature_name = 136,
  alias_sym_function_name = 137,
  alias_sym_id_name = 138,
  alias_sym_keyframes_name = 139,
  alias_sym_keyword_query = 140,
  alias_sym_namespace_name = 141,
  alias_sym_property_name = 142,
  alias_sym_tag_name = 143,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_ATimport] = "@import",
  [anon_sym_COMMA] = ",",
  [anon_sym_SEMI] = ";",
  [anon_sym_ATmedia] = "@media",
  [anon_sym_ATcharset] = "@charset",
  [anon_sym_ATnamespace] = "@namespace",
  [anon_sym_ATkeyframes] = "@keyframes",
  [aux_sym_keyframes_statement_token1] = "at_keyword",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [sym_from] = "from",
  [sym_to] = "to",
  [anon_sym_ATsupports] = "@supports",
  [sym_nesting_selector] = "nesting_selector",
  [anon_sym_STAR] = "*",
  [anon_sym_DOT] = ".",
  [anon_sym_has] = "class_name",
  [anon_sym_not] = "not",
  [anon_sym_is] = "class_name",
  [anon_sym_where] = "class_name",
  [anon_sym_host] = "class_name",
  [anon_sym_host_DASHcontext] = "class_name",
  [anon_sym_nth_DASHchild] = "class_name",
  [anon_sym_nth_DASHlast_DASHchild] = "class_name",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_POUND] = "#",
  [anon_sym_LBRACK] = "[",
  [anon_sym_EQ] = "=",
  [anon_sym_TILDE_EQ] = "~=",
  [anon_sym_CARET_EQ] = "^=",
  [anon_sym_PIPE_EQ] = "|=",
  [anon_sym_STAR_EQ] = "*=",
  [anon_sym_DOLLAR_EQ] = "$=",
  [anon_sym_RBRACK] = "]",
  [anon_sym_GT] = ">",
  [anon_sym_TILDE] = "~",
  [anon_sym_PLUS] = "+",
  [anon_sym_PIPE] = "|",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_even] = "plain_value",
  [anon_sym_odd] = "plain_value",
  [anon_sym_of] = "of",
  [sym__nth_functional_notation] = "plain_value",
  [anon_sym_COLON] = ":",
  [sym_important] = "important",
  [anon_sym_LPAREN2] = "(",
  [anon_sym_and] = "and",
  [anon_sym_or] = "or",
  [anon_sym_only] = "only",
  [anon_sym_selector] = "selector",
  [aux_sym_color_value_token1] = "color_value_token1",
  [anon_sym_SQUOTE] = "'",
  [aux_sym_string_value_token1] = "string_content",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_value_token2] = "string_content",
  [sym_escape_sequence] = "escape_sequence",
  [aux_sym_integer_value_token1] = "integer_value_token1",
  [aux_sym_float_value_token1] = "float_value_token1",
  [sym_unit] = "unit",
  [anon_sym_LBRACK2] = "[",
  [anon_sym_DASH] = "-",
  [anon_sym_SLASH] = "/",
  [sym_identifier] = "identifier",
  [sym_at_keyword] = "at_keyword",
  [sym_js_comment] = "js_comment",
  [sym_comment] = "comment",
  [sym_plain_value] = "plain_value",
  [sym__descendant_operator] = "_descendant_operator",
  [sym__pseudo_class_selector_colon] = ":",
  [sym___error_recovery] = "__error_recovery",
  [sym_stylesheet] = "stylesheet",
  [sym_import_statement] = "import_statement",
  [sym_media_statement] = "media_statement",
  [sym_charset_statement] = "charset_statement",
  [sym_namespace_statement] = "namespace_statement",
  [sym_keyframes_statement] = "keyframes_statement",
  [sym_keyframe_block_list] = "keyframe_block_list",
  [sym_keyframe_block] = "keyframe_block",
  [sym_supports_statement] = "supports_statement",
  [sym_postcss_statement] = "postcss_statement",
  [sym_at_rule] = "at_rule",
  [sym_rule_set] = "rule_set",
  [sym_selectors] = "selectors",
  [sym_block] = "block",
  [sym__selector] = "_selector",
  [sym_universal_selector] = "universal_selector",
  [sym_class_selector] = "class_selector",
  [sym_pseudo_class_selector] = "pseudo_class_selector",
  [sym__nth_child_pseudo_class_selector] = "_nth_child_pseudo_class_selector",
  [sym_pseudo_element_selector] = "pseudo_element_selector",
  [sym_id_selector] = "id_selector",
  [sym_attribute_selector] = "attribute_selector",
  [sym_child_selector] = "child_selector",
  [sym_descendant_selector] = "descendant_selector",
  [sym_sibling_selector] = "sibling_selector",
  [sym_adjacent_sibling_selector] = "adjacent_sibling_selector",
  [sym_namespace_selector] = "namespace_selector",
  [sym_pseudo_class_arguments] = "arguments",
  [sym_pseudo_class_with_selector_arguments] = "arguments",
  [sym_pseudo_class_nth_child_arguments] = "arguments",
  [sym_pseudo_element_arguments] = "arguments",
  [sym_declaration] = "declaration",
  [sym_last_declaration] = "declaration",
  [sym__query] = "_query",
  [sym_feature_query] = "feature_query",
  [sym_parenthesized_query] = "parenthesized_query",
  [sym_binary_query] = "binary_query",
  [sym_unary_query] = "unary_query",
  [sym_selector_query] = "selector_query",
  [sym__value] = "_value",
  [sym_parenthesized_value] = "parenthesized_value",
  [sym_color_value] = "color_value",
  [sym_string_value] = "string_value",
  [sym_integer_value] = "integer_value",
  [sym_float_value] = "float_value",
  [sym_grid_value] = "grid_value",
  [sym_call_expression] = "call_expression",
  [sym_binary_expression] = "binary_expression",
  [sym_arguments] = "arguments",
  [sym_class_name] = "class_name",
  [aux_sym_stylesheet_repeat1] = "stylesheet_repeat1",
  [aux_sym_import_statement_repeat1] = "import_statement_repeat1",
  [aux_sym_keyframe_block_list_repeat1] = "keyframe_block_list_repeat1",
  [aux_sym_postcss_statement_repeat1] = "postcss_statement_repeat1",
  [aux_sym_selectors_repeat1] = "selectors_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym_pseudo_class_arguments_repeat1] = "pseudo_class_arguments_repeat1",
  [aux_sym_declaration_repeat1] = "declaration_repeat1",
  [aux_sym_string_value_repeat1] = "string_value_repeat1",
  [aux_sym_string_value_repeat2] = "string_value_repeat2",
  [aux_sym_grid_value_repeat1] = "grid_value_repeat1",
  [aux_sym_arguments_repeat1] = "arguments_repeat1",
  [aux_sym_class_name_repeat1] = "class_name_repeat1",
  [alias_sym_attribute_name] = "attribute_name",
  [alias_sym_feature_name] = "feature_name",
  [alias_sym_function_name] = "function_name",
  [alias_sym_id_name] = "id_name",
  [alias_sym_keyframes_name] = "keyframes_name",
  [alias_sym_keyword_query] = "keyword_query",
  [alias_sym_namespace_name] = "namespace_name",
  [alias_sym_property_name] = "property_name",
  [alias_sym_tag_name] = "tag_name",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_ATimport] = anon_sym_ATimport,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_ATmedia] = anon_sym_ATmedia,
  [anon_sym_ATcharset] = anon_sym_ATcharset,
  [anon_sym_ATnamespace] = anon_sym_ATnamespace,
  [anon_sym_ATkeyframes] = anon_sym_ATkeyframes,
  [aux_sym_keyframes_statement_token1] = sym_at_keyword,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [sym_from] = sym_from,
  [sym_to] = sym_to,
  [anon_sym_ATsupports] = anon_sym_ATsupports,
  [sym_nesting_selector] = sym_nesting_selector,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_has] = sym_class_name,
  [anon_sym_not] = anon_sym_not,
  [anon_sym_is] = sym_class_name,
  [anon_sym_where] = sym_class_name,
  [anon_sym_host] = sym_class_name,
  [anon_sym_host_DASHcontext] = sym_class_name,
  [anon_sym_nth_DASHchild] = sym_class_name,
  [anon_sym_nth_DASHlast_DASHchild] = sym_class_name,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_POUND] = anon_sym_POUND,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_TILDE_EQ] = anon_sym_TILDE_EQ,
  [anon_sym_CARET_EQ] = anon_sym_CARET_EQ,
  [anon_sym_PIPE_EQ] = anon_sym_PIPE_EQ,
  [anon_sym_STAR_EQ] = anon_sym_STAR_EQ,
  [anon_sym_DOLLAR_EQ] = anon_sym_DOLLAR_EQ,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_even] = sym_plain_value,
  [anon_sym_odd] = sym_plain_value,
  [anon_sym_of] = anon_sym_of,
  [sym__nth_functional_notation] = sym_plain_value,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym_important] = sym_important,
  [anon_sym_LPAREN2] = anon_sym_LPAREN,
  [anon_sym_and] = anon_sym_and,
  [anon_sym_or] = anon_sym_or,
  [anon_sym_only] = anon_sym_only,
  [anon_sym_selector] = anon_sym_selector,
  [aux_sym_color_value_token1] = aux_sym_color_value_token1,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [aux_sym_string_value_token1] = aux_sym_string_value_token1,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_value_token2] = aux_sym_string_value_token1,
  [sym_escape_sequence] = sym_escape_sequence,
  [aux_sym_integer_value_token1] = aux_sym_integer_value_token1,
  [aux_sym_float_value_token1] = aux_sym_float_value_token1,
  [sym_unit] = sym_unit,
  [anon_sym_LBRACK2] = anon_sym_LBRACK,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [sym_identifier] = sym_identifier,
  [sym_at_keyword] = sym_at_keyword,
  [sym_js_comment] = sym_js_comment,
  [sym_comment] = sym_comment,
  [sym_plain_value] = sym_plain_value,
  [sym__descendant_operator] = sym__descendant_operator,
  [sym__pseudo_class_selector_colon] = anon_sym_COLON,
  [sym___error_recovery] = sym___error_recovery,
  [sym_stylesheet] = sym_stylesheet,
  [sym_import_statement] = sym_import_statement,
  [sym_media_statement] = sym_media_statement,
  [sym_charset_statement] = sym_charset_statement,
  [sym_namespace_statement] = sym_namespace_statement,
  [sym_keyframes_statement] = sym_keyframes_statement,
  [sym_keyframe_block_list] = sym_keyframe_block_list,
  [sym_keyframe_block] = sym_keyframe_block,
  [sym_supports_statement] = sym_supports_statement,
  [sym_postcss_statement] = sym_postcss_statement,
  [sym_at_rule] = sym_at_rule,
  [sym_rule_set] = sym_rule_set,
  [sym_selectors] = sym_selectors,
  [sym_block] = sym_block,
  [sym__selector] = sym__selector,
  [sym_universal_selector] = sym_universal_selector,
  [sym_class_selector] = sym_class_selector,
  [sym_pseudo_class_selector] = sym_pseudo_class_selector,
  [sym__nth_child_pseudo_class_selector] = sym__nth_child_pseudo_class_selector,
  [sym_pseudo_element_selector] = sym_pseudo_element_selector,
  [sym_id_selector] = sym_id_selector,
  [sym_attribute_selector] = sym_attribute_selector,
  [sym_child_selector] = sym_child_selector,
  [sym_descendant_selector] = sym_descendant_selector,
  [sym_sibling_selector] = sym_sibling_selector,
  [sym_adjacent_sibling_selector] = sym_adjacent_sibling_selector,
  [sym_namespace_selector] = sym_namespace_selector,
  [sym_pseudo_class_arguments] = sym_arguments,
  [sym_pseudo_class_with_selector_arguments] = sym_arguments,
  [sym_pseudo_class_nth_child_arguments] = sym_arguments,
  [sym_pseudo_element_arguments] = sym_arguments,
  [sym_declaration] = sym_declaration,
  [sym_last_declaration] = sym_declaration,
  [sym__query] = sym__query,
  [sym_feature_query] = sym_feature_query,
  [sym_parenthesized_query] = sym_parenthesized_query,
  [sym_binary_query] = sym_binary_query,
  [sym_unary_query] = sym_unary_query,
  [sym_selector_query] = sym_selector_query,
  [sym__value] = sym__value,
  [sym_parenthesized_value] = sym_parenthesized_value,
  [sym_color_value] = sym_color_value,
  [sym_string_value] = sym_string_value,
  [sym_integer_value] = sym_integer_value,
  [sym_float_value] = sym_float_value,
  [sym_grid_value] = sym_grid_value,
  [sym_call_expression] = sym_call_expression,
  [sym_binary_expression] = sym_binary_expression,
  [sym_arguments] = sym_arguments,
  [sym_class_name] = sym_class_name,
  [aux_sym_stylesheet_repeat1] = aux_sym_stylesheet_repeat1,
  [aux_sym_import_statement_repeat1] = aux_sym_import_statement_repeat1,
  [aux_sym_keyframe_block_list_repeat1] = aux_sym_keyframe_block_list_repeat1,
  [aux_sym_postcss_statement_repeat1] = aux_sym_postcss_statement_repeat1,
  [aux_sym_selectors_repeat1] = aux_sym_selectors_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym_pseudo_class_arguments_repeat1] = aux_sym_pseudo_class_arguments_repeat1,
  [aux_sym_declaration_repeat1] = aux_sym_declaration_repeat1,
  [aux_sym_string_value_repeat1] = aux_sym_string_value_repeat1,
  [aux_sym_string_value_repeat2] = aux_sym_string_value_repeat2,
  [aux_sym_grid_value_repeat1] = aux_sym_grid_value_repeat1,
  [aux_sym_arguments_repeat1] = aux_sym_arguments_repeat1,
  [aux_sym_class_name_repeat1] = aux_sym_class_name_repeat1,
  [alias_sym_attribute_name] = alias_sym_attribute_name,
  [alias_sym_feature_name] = alias_sym_feature_name,
  [alias_sym_function_name] = alias_sym_function_name,
  [alias_sym_id_name] = alias_sym_id_name,
  [alias_sym_keyframes_name] = alias_sym_keyframes_name,
  [alias_sym_keyword_query] = alias_sym_keyword_query,
  [alias_sym_namespace_name] = alias_sym_namespace_name,
  [alias_sym_property_name] = alias_sym_property_name,
  [alias_sym_tag_name] = alias_sym_tag_name,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_ATimport] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATmedia] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATcharset] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATnamespace] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATkeyframes] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_keyframes_statement_token1] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [sym_from] = {
    .visible = true,
    .named = true,
  },
  [sym_to] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_ATsupports] = {
    .visible = true,
    .named = false,
  },
  [sym_nesting_selector] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_has] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_not] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_is] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_where] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_host] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_host_DASHcontext] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_nth_DASHchild] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_nth_DASHlast_DASHchild] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COLON_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOLLAR_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
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
  [anon_sym_even] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_odd] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_of] = {
    .visible = true,
    .named = false,
  },
  [sym__nth_functional_notation] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [sym_important] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LPAREN2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_and] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_or] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_only] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_selector] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_color_value_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_value_token1] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_value_token2] = {
    .visible = true,
    .named = true,
  },
  [sym_escape_sequence] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_integer_value_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_float_value_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_unit] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_at_keyword] = {
    .visible = true,
    .named = true,
  },
  [sym_js_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_plain_value] = {
    .visible = true,
    .named = true,
  },
  [sym__descendant_operator] = {
    .visible = false,
    .named = true,
  },
  [sym__pseudo_class_selector_colon] = {
    .visible = true,
    .named = false,
  },
  [sym___error_recovery] = {
    .visible = false,
    .named = true,
  },
  [sym_stylesheet] = {
    .visible = true,
    .named = true,
  },
  [sym_import_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_media_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_charset_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_namespace_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_keyframes_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_keyframe_block_list] = {
    .visible = true,
    .named = true,
  },
  [sym_keyframe_block] = {
    .visible = true,
    .named = true,
  },
  [sym_supports_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_postcss_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_at_rule] = {
    .visible = true,
    .named = true,
  },
  [sym_rule_set] = {
    .visible = true,
    .named = true,
  },
  [sym_selectors] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym__selector] = {
    .visible = false,
    .named = true,
  },
  [sym_universal_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_class_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_pseudo_class_selector] = {
    .visible = true,
    .named = true,
  },
  [sym__nth_child_pseudo_class_selector] = {
    .visible = false,
    .named = true,
  },
  [sym_pseudo_element_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_id_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_child_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_descendant_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_sibling_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_adjacent_sibling_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_namespace_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_pseudo_class_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_pseudo_class_with_selector_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_pseudo_class_nth_child_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_pseudo_element_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_last_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym__query] = {
    .visible = false,
    .named = true,
  },
  [sym_feature_query] = {
    .visible = true,
    .named = true,
  },
  [sym_parenthesized_query] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_query] = {
    .visible = true,
    .named = true,
  },
  [sym_unary_query] = {
    .visible = true,
    .named = true,
  },
  [sym_selector_query] = {
    .visible = true,
    .named = true,
  },
  [sym__value] = {
    .visible = false,
    .named = true,
  },
  [sym_parenthesized_value] = {
    .visible = true,
    .named = true,
  },
  [sym_color_value] = {
    .visible = true,
    .named = true,
  },
  [sym_string_value] = {
    .visible = true,
    .named = true,
  },
  [sym_integer_value] = {
    .visible = true,
    .named = true,
  },
  [sym_float_value] = {
    .visible = true,
    .named = true,
  },
  [sym_grid_value] = {
    .visible = true,
    .named = true,
  },
  [sym_call_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_class_name] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_stylesheet_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_import_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_keyframe_block_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_postcss_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_selectors_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_pseudo_class_arguments_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_declaration_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_value_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_value_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_grid_value_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_arguments_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_class_name_repeat1] = {
    .visible = false,
    .named = false,
  },
  [alias_sym_attribute_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_feature_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_function_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_id_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_keyframes_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_keyword_query] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_namespace_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_property_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_tag_name] = {
    .visible = true,
    .named = true,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
  [1] = {
    [0] = alias_sym_tag_name,
  },
  [2] = {
    [1] = sym_class_name,
  },
  [3] = {
    [0] = sym_plain_value,
  },
  [4] = {
    [0] = alias_sym_keyword_query,
  },
  [5] = {
    [1] = alias_sym_tag_name,
  },
  [6] = {
    [1] = alias_sym_id_name,
  },
  [7] = {
    [0] = alias_sym_function_name,
  },
  [8] = {
    [1] = alias_sym_keyframes_name,
  },
  [9] = {
    [1] = alias_sym_attribute_name,
  },
  [10] = {
    [2] = sym_class_name,
  },
  [11] = {
    [2] = alias_sym_tag_name,
  },
  [12] = {
    [2] = alias_sym_id_name,
  },
  [13] = {
    [1] = alias_sym_namespace_name,
  },
  [14] = {
    [0] = alias_sym_property_name,
  },
  [15] = {
    [2] = alias_sym_attribute_name,
  },
  [16] = {
    [1] = alias_sym_feature_name,
  },
};

static const uint16_t ts_non_terminal_alias_map[] = {
  sym_namespace_selector, 2,
    sym_namespace_selector,
    alias_sym_attribute_name,
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 2,
  [4] = 4,
  [5] = 4,
  [6] = 4,
  [7] = 2,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 16,
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
  [35] = 34,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 37,
  [40] = 40,
  [41] = 41,
  [42] = 37,
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
  [67] = 50,
  [68] = 68,
  [69] = 52,
  [70] = 54,
  [71] = 56,
  [72] = 61,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 68,
  [79] = 45,
  [80] = 46,
  [81] = 48,
  [82] = 51,
  [83] = 53,
  [84] = 57,
  [85] = 58,
  [86] = 59,
  [87] = 63,
  [88] = 65,
  [89] = 73,
  [90] = 74,
  [91] = 44,
  [92] = 75,
  [93] = 93,
  [94] = 77,
  [95] = 76,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 97,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 105,
  [107] = 107,
  [108] = 108,
  [109] = 105,
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
  [135] = 36,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 41,
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
  [159] = 158,
  [160] = 160,
  [161] = 161,
  [162] = 162,
  [163] = 163,
  [164] = 158,
  [165] = 165,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 166,
  [170] = 165,
  [171] = 156,
  [172] = 168,
  [173] = 160,
  [174] = 156,
  [175] = 168,
  [176] = 176,
  [177] = 36,
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
  [188] = 41,
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
  [201] = 199,
  [202] = 198,
  [203] = 200,
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
  [214] = 210,
  [215] = 149,
  [216] = 216,
  [217] = 217,
  [218] = 179,
  [219] = 219,
  [220] = 220,
  [221] = 221,
  [222] = 217,
  [223] = 161,
  [224] = 224,
  [225] = 225,
  [226] = 226,
  [227] = 220,
  [228] = 228,
  [229] = 162,
  [230] = 186,
  [231] = 187,
  [232] = 184,
  [233] = 233,
  [234] = 36,
  [235] = 149,
  [236] = 41,
  [237] = 183,
  [238] = 178,
  [239] = 182,
  [240] = 185,
  [241] = 180,
  [242] = 181,
  [243] = 243,
  [244] = 176,
  [245] = 161,
  [246] = 162,
  [247] = 179,
  [248] = 185,
  [249] = 183,
  [250] = 41,
  [251] = 176,
  [252] = 182,
  [253] = 187,
  [254] = 184,
  [255] = 36,
  [256] = 186,
  [257] = 178,
  [258] = 180,
  [259] = 181,
  [260] = 260,
  [261] = 261,
  [262] = 262,
  [263] = 263,
  [264] = 261,
  [265] = 265,
  [266] = 266,
  [267] = 265,
  [268] = 265,
  [269] = 266,
  [270] = 260,
  [271] = 263,
  [272] = 272,
  [273] = 273,
  [274] = 274,
  [275] = 275,
  [276] = 276,
  [277] = 277,
  [278] = 274,
  [279] = 279,
  [280] = 280,
  [281] = 281,
  [282] = 282,
  [283] = 283,
  [284] = 282,
  [285] = 281,
  [286] = 286,
  [287] = 287,
  [288] = 288,
  [289] = 288,
  [290] = 290,
  [291] = 282,
  [292] = 286,
  [293] = 283,
  [294] = 294,
  [295] = 295,
  [296] = 296,
  [297] = 297,
  [298] = 298,
  [299] = 299,
  [300] = 300,
  [301] = 301,
  [302] = 302,
  [303] = 303,
  [304] = 304,
  [305] = 63,
  [306] = 306,
  [307] = 297,
  [308] = 299,
  [309] = 303,
  [310] = 298,
  [311] = 311,
  [312] = 304,
  [313] = 311,
  [314] = 311,
  [315] = 75,
  [316] = 316,
  [317] = 297,
  [318] = 303,
  [319] = 298,
  [320] = 311,
  [321] = 316,
  [322] = 303,
  [323] = 298,
  [324] = 311,
  [325] = 316,
  [326] = 316,
  [327] = 327,
  [328] = 298,
  [329] = 329,
  [330] = 51,
  [331] = 331,
  [332] = 332,
  [333] = 303,
  [334] = 316,
  [335] = 335,
  [336] = 336,
  [337] = 337,
  [338] = 338,
  [339] = 336,
  [340] = 340,
  [341] = 341,
  [342] = 341,
  [343] = 343,
  [344] = 344,
  [345] = 345,
  [346] = 161,
  [347] = 341,
  [348] = 348,
  [349] = 349,
  [350] = 350,
  [351] = 351,
  [352] = 161,
  [353] = 353,
  [354] = 354,
  [355] = 355,
  [356] = 354,
  [357] = 357,
  [358] = 353,
  [359] = 359,
  [360] = 360,
  [361] = 361,
  [362] = 362,
  [363] = 363,
  [364] = 364,
  [365] = 362,
  [366] = 361,
  [367] = 367,
  [368] = 368,
  [369] = 369,
  [370] = 367,
  [371] = 371,
  [372] = 372,
  [373] = 373,
  [374] = 360,
  [375] = 364,
  [376] = 362,
  [377] = 360,
  [378] = 364,
  [379] = 379,
  [380] = 372,
};

static TSCharacterRange sym_plain_value_character_set_1[] = {
  {0, 0x08}, {0x0e, 0x1f}, {'"', '\''}, {'+', '+'}, {'-', ':'}, {'<', 'Z'}, {'\\', '\\'}, {'^', 'z'},
  {'|', '|'}, {'~', 0x10ffff},
};

static TSCharacterRange sym_plain_value_character_set_2[] = {
  {0, 0x08}, {0x0e, 0x1f}, {'"', '\''}, {'*', '+'}, {'-', ':'}, {'<', 'Z'}, {'\\', '\\'}, {'^', 'z'},
  {'|', '|'}, {'~', 0x10ffff},
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(134);
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '$', 50,
        '&', 148,
        '\'', 241,
        '(', 187,
        ')', 188,
        '*', 149,
        '+', 184,
        ',', 136,
        '-', 290,
        '.', 151,
        '/', 292,
        ':', 196,
        ';', 137,
        '=', 173,
        '>', 180,
        '@', 56,
        'E', 38,
        '[', 172,
        '\\', 125,
        ']', 179,
        '^', 51,
        'a', 89,
        'e', 36,
        'f', 103,
        'h', 53,
        'i', 106,
        'n', 94,
        'o', 66,
        's', 70,
        't', 95,
        'w', 77,
        '{', 143,
        '|', 186,
        '}', 144,
        '~', 182,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(131);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('b' <= lookahead && lookahead <= 'd')) ADVANCE(127);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(262);
      END_STATE();
    case 1:
      if (lookahead == '\n') SKIP(1);
      if (lookahead == '"') ADVANCE(247);
      if (lookahead == '/') ADVANCE(248);
      if (lookahead == '\\') ADVANCE(125);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(251);
      if (lookahead != 0) ADVANCE(252);
      END_STATE();
    case 2:
      if (lookahead == '\n') SKIP(2);
      if (lookahead == '\'') ADVANCE(241);
      if (lookahead == '/') ADVANCE(242);
      if (lookahead == '\\') ADVANCE(125);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(245);
      if (lookahead != 0) ADVANCE(246);
      END_STATE();
    case 3:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '%', 286,
        '\'', 241,
        '(', 198,
        ')', 188,
        '*', 149,
        '+', 184,
        ',', 136,
        '-', 289,
        '.', 122,
        '/', 293,
        ';', 137,
        '[', 287,
        '_', 297,
        '}', 144,
        'E', 282,
        'e', 282,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(9);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(283);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 4:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '&', 148,
        '\'', 241,
        '(', 198,
        ')', 188,
        '*', 149,
        '+', 184,
        '-', 42,
        '.', 151,
        '/', 28,
        ':', 49,
        '>', 180,
        '[', 172,
        '_', 297,
        '|', 185,
        '~', 181,
        'E', 294,
        'e', 294,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(4);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 5:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '\'', 241,
        '(', 187,
        ')', 188,
        '*', 149,
        '+', 184,
        ',', 136,
        '-', 289,
        '.', 151,
        '/', 293,
        ':', 49,
        '>', 180,
        '[', 172,
        '_', 297,
        '|', 185,
        '~', 181,
        'E', 294,
        'e', 294,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(8);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 6:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '\'', 241,
        '(', 187,
        ')', 188,
        '*', 149,
        '+', 184,
        ',', 136,
        '-', 289,
        '.', 122,
        '/', 293,
        ';', 137,
        '[', 287,
        '_', 297,
        '}', 144,
        'E', 294,
        'e', 294,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(9);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 7:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '\'', 241,
        '(', 187,
        '*', 149,
        '+', 184,
        ',', 136,
        '-', 289,
        '.', 122,
        '/', 293,
        ';', 137,
        '[', 287,
        '_', 297,
        'a', 305,
        'o', 309,
        '{', 143,
        'E', 294,
        'e', 294,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(11);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 8:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '\'', 241,
        '(', 198,
        ')', 188,
        '*', 149,
        '+', 184,
        ',', 136,
        '-', 289,
        '.', 151,
        '/', 293,
        ':', 49,
        '>', 180,
        '[', 172,
        '_', 297,
        '|', 185,
        '~', 181,
        'E', 294,
        'e', 294,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(8);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 9:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '\'', 241,
        '(', 198,
        ')', 188,
        '*', 149,
        '+', 184,
        ',', 136,
        '-', 289,
        '.', 122,
        '/', 293,
        ';', 137,
        '[', 287,
        '_', 297,
        '}', 144,
        'E', 294,
        'e', 294,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(9);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 10:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '\'', 241,
        '(', 198,
        ')', 188,
        '+', 43,
        ',', 136,
        '-', 42,
        '.', 122,
        '/', 28,
        ';', 137,
        '[', 287,
        '_', 297,
        '}', 144,
        'E', 294,
        'e', 294,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(10);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 11:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '\'', 241,
        '(', 198,
        '*', 149,
        '+', 184,
        ',', 136,
        '-', 289,
        '.', 122,
        '/', 293,
        ';', 137,
        '[', 287,
        '_', 297,
        'a', 305,
        'o', 309,
        '{', 143,
        'E', 294,
        'e', 294,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(11);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 12:
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '\'', 241,
        '(', 198,
        '+', 43,
        '-', 42,
        '.', 122,
        '/', 28,
        ';', 137,
        '[', 287,
        '_', 297,
        'n', 306,
        'o', 304,
        's', 301,
        '{', 143,
        'E', 294,
        'e', 294,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 13:
      ADVANCE_MAP(
        '#', 171,
        '$', 50,
        '(', 187,
        ')', 188,
        '*', 52,
        '+', 183,
        ',', 136,
        '-', 128,
        '.', 150,
        '/', 27,
        ':', 49,
        '=', 173,
        '>', 180,
        '[', 172,
        '\\', 125,
        ']', 179,
        '^', 51,
        '{', 143,
        '|', 186,
        '~', 182,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(14);
      if (('A' <= lookahead && lookahead <= '_') ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 14:
      ADVANCE_MAP(
        '#', 171,
        '$', 50,
        ')', 188,
        '*', 52,
        '+', 183,
        ',', 136,
        '-', 128,
        '.', 150,
        '/', 27,
        ':', 49,
        '=', 173,
        '>', 180,
        '[', 172,
        '\\', 125,
        ']', 179,
        '^', 51,
        '{', 143,
        '|', 186,
        '~', 182,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(14);
      if (('A' <= lookahead && lookahead <= '_') ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 15:
      ADVANCE_MAP(
        '%', 286,
        '(', 198,
        '*', 149,
        '+', 183,
        '-', 291,
        '/', 292,
        ';', 137,
        'n', 275,
        'o', 274,
        's', 271,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(19);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 16:
      ADVANCE_MAP(
        '(', 187,
        ')', 188,
        '*', 149,
        '+', 183,
        ',', 136,
        '-', 288,
        '/', 292,
        ':', 195,
        ';', 137,
        ']', 179,
        'a', 88,
        'o', 74,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(21);
      END_STATE();
    case 17:
      ADVANCE_MAP(
        '(', 187,
        '*', 149,
        '+', 183,
        '-', 291,
        '/', 292,
        ';', 137,
        'n', 341,
        'o', 338,
        's', 326,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(19);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 18:
      ADVANCE_MAP(
        '(', 198,
        '*', 149,
        '+', 183,
        '-', 291,
        '/', 292,
        ';', 137,
        'n', 341,
        'o', 338,
        's', 326,
        '{', 143,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(18);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 19:
      ADVANCE_MAP(
        '(', 198,
        '*', 149,
        '+', 183,
        '-', 291,
        '/', 292,
        ';', 137,
        'n', 341,
        'o', 338,
        's', 326,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(19);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 20:
      ADVANCE_MAP(
        '(', 198,
        '-', 128,
        '/', 27,
        ';', 137,
        'n', 341,
        'o', 338,
        's', 326,
        '{', 143,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(20);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 21:
      ADVANCE_MAP(
        ')', 188,
        '*', 149,
        '+', 183,
        ',', 136,
        '-', 288,
        '/', 292,
        ':', 195,
        ';', 137,
        ']', 179,
        'a', 88,
        'o', 74,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(21);
      END_STATE();
    case 22:
      ADVANCE_MAP(
        ')', 188,
        '*', 149,
        '+', 183,
        ',', 136,
        '-', 288,
        '/', 292,
        ';', 137,
        ']', 179,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(22);
      END_STATE();
    case 23:
      ADVANCE_MAP(
        ')', 188,
        '*', 149,
        '+', 183,
        ',', 136,
        '-', 288,
        '/', 292,
        ';', 137,
        ']', 179,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(22);
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(286);
      END_STATE();
    case 24:
      ADVANCE_MAP(
        ')', 188,
        ',', 136,
        '/', 27,
        ':', 195,
        ';', 137,
        'a', 88,
        'f', 102,
        'o', 74,
        't', 95,
        '{', 143,
        '}', 144,
        '+', 124,
        '-', 124,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(24);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(264);
      END_STATE();
    case 25:
      if (lookahead == ')') ADVANCE(188);
      if (lookahead == '/') ADVANCE(27);
      if (lookahead == 'o') ADVANCE(285);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(26);
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(286);
      END_STATE();
    case 26:
      if (lookahead == ')') ADVANCE(188);
      if (lookahead == '/') ADVANCE(27);
      if (lookahead == 'o') ADVANCE(73);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(26);
      END_STATE();
    case 27:
      if (lookahead == '*') ADVANCE(30);
      if (lookahead == '/') ADVANCE(412);
      END_STATE();
    case 28:
      if (lookahead == '*') ADVANCE(30);
      if (lookahead == '/') ADVANCE(409);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(48);
      END_STATE();
    case 29:
      if (lookahead == '*') ADVANCE(29);
      if (lookahead == '/') ADVANCE(413);
      if (lookahead != 0) ADVANCE(30);
      END_STATE();
    case 30:
      if (lookahead == '*') ADVANCE(29);
      if (lookahead != 0) ADVANCE(30);
      END_STATE();
    case 31:
      if (lookahead == '+') ADVANCE(124);
      if (lookahead == '-') ADVANCE(91);
      if (lookahead == '/') ADVANCE(27);
      if (lookahead == 'e') ADVANCE(118);
      if (lookahead == 'n') ADVANCE(193);
      if (lookahead == 'o') ADVANCE(65);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(31);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(263);
      END_STATE();
    case 32:
      if (lookahead == '-') ADVANCE(57);
      END_STATE();
    case 33:
      if (lookahead == '-') ADVANCE(128);
      if (lookahead == '/') ADVANCE(27);
      if (lookahead == '\\') ADVANCE(125);
      if (lookahead == 'h') ADVANCE(315);
      if (lookahead == 'i') ADVANCE(346);
      if (lookahead == 'n') ADVANCE(340);
      if (lookahead == 'w') ADVANCE(330);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(33);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 34:
      if (lookahead == '-') ADVANCE(128);
      if (lookahead == '/') ADVANCE(27);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(35);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(127);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(359);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 35:
      if (lookahead == '-') ADVANCE(128);
      if (lookahead == '/') ADVANCE(27);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(35);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 36:
      if (lookahead == '-') ADVANCE(123);
      if (lookahead == 'v') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(268);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(126);
      END_STATE();
    case 37:
      if (lookahead == '-') ADVANCE(123);
      if (lookahead == 'v') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(267);
      END_STATE();
    case 38:
      if (lookahead == '-') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(268);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(126);
      END_STATE();
    case 39:
      if (lookahead == '-') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(267);
      END_STATE();
    case 40:
      if (lookahead == '-') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(229);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(238);
      END_STATE();
    case 41:
      if (lookahead == '-') ADVANCE(60);
      END_STATE();
    case 42:
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == '/') ADVANCE(129);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(297);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(294);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 43:
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      END_STATE();
    case 44:
      if (lookahead == '/') ADVANCE(27);
      if (lookahead == '{') ADVANCE(143);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(44);
      END_STATE();
    case 45:
      if (lookahead == '/') ADVANCE(27);
      if (lookahead == '{') ADVANCE(143);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(44);
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(286);
      END_STATE();
    case 46:
      if (lookahead == '/') ADVANCE(27);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(46);
      END_STATE();
    case 47:
      if (lookahead == '/') ADVANCE(27);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(46);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(127);
      END_STATE();
    case 48:
      if (lookahead == '/') ADVANCE(129);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(48);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(414);
      END_STATE();
    case 49:
      if (lookahead == ':') ADVANCE(170);
      END_STATE();
    case 50:
      if (lookahead == '=') ADVANCE(178);
      END_STATE();
    case 51:
      if (lookahead == '=') ADVANCE(175);
      END_STATE();
    case 52:
      if (lookahead == '=') ADVANCE(177);
      END_STATE();
    case 53:
      if (lookahead == 'a') ADVANCE(107);
      if (lookahead == 'o') ADVANCE(108);
      END_STATE();
    case 54:
      if (lookahead == 'a') ADVANCE(109);
      END_STATE();
    case 55:
      if (lookahead == 'a') ADVANCE(93);
      END_STATE();
    case 56:
      if (lookahead == 'c') ADVANCE(379);
      if (lookahead == 'i') ADVANCE(381);
      if (lookahead == 'k') ADVANCE(369);
      if (lookahead == 'm') ADVANCE(370);
      if (lookahead == 'n') ADVANCE(364);
      if (lookahead == 's') ADVANCE(404);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 57:
      if (lookahead == 'c') ADVANCE(76);
      if (lookahead == 'l') ADVANCE(54);
      END_STATE();
    case 58:
      if (lookahead == 'c') ADVANCE(117);
      END_STATE();
    case 59:
      if (lookahead == 'c') ADVANCE(97);
      END_STATE();
    case 60:
      if (lookahead == 'c') ADVANCE(78);
      END_STATE();
    case 61:
      if (lookahead == 'd') ADVANCE(199);
      END_STATE();
    case 62:
      if (lookahead == 'd') ADVANCE(190);
      END_STATE();
    case 63:
      if (lookahead == 'd') ADVANCE(166);
      END_STATE();
    case 64:
      if (lookahead == 'd') ADVANCE(168);
      END_STATE();
    case 65:
      if (lookahead == 'd') ADVANCE(62);
      END_STATE();
    case 66:
      if (lookahead == 'd') ADVANCE(62);
      if (lookahead == 'f') ADVANCE(191);
      if (lookahead == 'n') ADVANCE(82);
      if (lookahead == 'r') ADVANCE(201);
      END_STATE();
    case 67:
      if (lookahead == 'e') ADVANCE(58);
      END_STATE();
    case 68:
      if (lookahead == 'e') ADVANCE(160);
      END_STATE();
    case 69:
      if (lookahead == 'e') ADVANCE(119);
      END_STATE();
    case 70:
      if (lookahead == 'e') ADVANCE(84);
      END_STATE();
    case 71:
      if (lookahead == 'e') ADVANCE(90);
      END_STATE();
    case 72:
      if (lookahead == 'e') ADVANCE(105);
      END_STATE();
    case 73:
      if (lookahead == 'f') ADVANCE(191);
      END_STATE();
    case 74:
      if (lookahead == 'f') ADVANCE(191);
      if (lookahead == 'r') ADVANCE(201);
      END_STATE();
    case 75:
      if (lookahead == 'h') ADVANCE(32);
      END_STATE();
    case 76:
      if (lookahead == 'h') ADVANCE(80);
      END_STATE();
    case 77:
      if (lookahead == 'h') ADVANCE(72);
      END_STATE();
    case 78:
      if (lookahead == 'h') ADVANCE(81);
      END_STATE();
    case 79:
      if (lookahead == 'i') ADVANCE(86);
      END_STATE();
    case 80:
      if (lookahead == 'i') ADVANCE(83);
      END_STATE();
    case 81:
      if (lookahead == 'i') ADVANCE(85);
      END_STATE();
    case 82:
      if (lookahead == 'l') ADVANCE(120);
      END_STATE();
    case 83:
      if (lookahead == 'l') ADVANCE(63);
      END_STATE();
    case 84:
      if (lookahead == 'l') ADVANCE(67);
      END_STATE();
    case 85:
      if (lookahead == 'l') ADVANCE(64);
      END_STATE();
    case 86:
      if (lookahead == 'm') ADVANCE(100);
      END_STATE();
    case 87:
      if (lookahead == 'm') ADVANCE(145);
      END_STATE();
    case 88:
      if (lookahead == 'n') ADVANCE(61);
      END_STATE();
    case 89:
      if (lookahead == 'n') ADVANCE(61);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(126);
      END_STATE();
    case 90:
      if (lookahead == 'n') ADVANCE(189);
      END_STATE();
    case 91:
      if (lookahead == 'n') ADVANCE(193);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(263);
      END_STATE();
    case 92:
      if (lookahead == 'n') ADVANCE(116);
      END_STATE();
    case 93:
      if (lookahead == 'n') ADVANCE(112);
      END_STATE();
    case 94:
      if (lookahead == 'o') ADVANCE(110);
      if (lookahead == 't') ADVANCE(75);
      END_STATE();
    case 95:
      if (lookahead == 'o') ADVANCE(146);
      END_STATE();
    case 96:
      if (lookahead == 'o') ADVANCE(87);
      END_STATE();
    case 97:
      if (lookahead == 'o') ADVANCE(92);
      END_STATE();
    case 98:
      if (lookahead == 'o') ADVANCE(104);
      END_STATE();
    case 99:
      if (lookahead == 'o') ADVANCE(101);
      END_STATE();
    case 100:
      if (lookahead == 'p') ADVANCE(98);
      END_STATE();
    case 101:
      if (lookahead == 'r') ADVANCE(207);
      END_STATE();
    case 102:
      if (lookahead == 'r') ADVANCE(96);
      END_STATE();
    case 103:
      if (lookahead == 'r') ADVANCE(96);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(126);
      END_STATE();
    case 104:
      if (lookahead == 'r') ADVANCE(115);
      END_STATE();
    case 105:
      if (lookahead == 'r') ADVANCE(68);
      END_STATE();
    case 106:
      if (lookahead == 's') ADVANCE(158);
      END_STATE();
    case 107:
      if (lookahead == 's') ADVANCE(152);
      END_STATE();
    case 108:
      if (lookahead == 's') ADVANCE(111);
      END_STATE();
    case 109:
      if (lookahead == 's') ADVANCE(114);
      END_STATE();
    case 110:
      if (lookahead == 't') ADVANCE(154);
      END_STATE();
    case 111:
      if (lookahead == 't') ADVANCE(162);
      END_STATE();
    case 112:
      if (lookahead == 't') ADVANCE(197);
      END_STATE();
    case 113:
      if (lookahead == 't') ADVANCE(164);
      END_STATE();
    case 114:
      if (lookahead == 't') ADVANCE(41);
      END_STATE();
    case 115:
      if (lookahead == 't') ADVANCE(55);
      END_STATE();
    case 116:
      if (lookahead == 't') ADVANCE(69);
      END_STATE();
    case 117:
      if (lookahead == 't') ADVANCE(99);
      END_STATE();
    case 118:
      if (lookahead == 'v') ADVANCE(71);
      END_STATE();
    case 119:
      if (lookahead == 'x') ADVANCE(113);
      END_STATE();
    case 120:
      if (lookahead == 'y') ADVANCE(203);
      END_STATE();
    case 121:
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(121);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(194);
      END_STATE();
    case 122:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(266);
      END_STATE();
    case 123:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(267);
      END_STATE();
    case 124:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(264);
      END_STATE();
    case 125:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(259);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(253);
      END_STATE();
    case 126:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(238);
      END_STATE();
    case 127:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(126);
      END_STATE();
    case 128:
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 129:
      if ((!eof && set_contains(sym_plain_value_character_set_1, 10, lookahead))) ADVANCE(48);
      END_STATE();
    case 130:
      if ((!eof && set_contains(sym_plain_value_character_set_1, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 131:
      if (eof) ADVANCE(134);
      ADVANCE_MAP(
        '!', 79,
        '"', 247,
        '#', 171,
        '$', 50,
        '&', 148,
        '\'', 241,
        '(', 198,
        ')', 188,
        '*', 149,
        '+', 184,
        ',', 136,
        '-', 290,
        '.', 151,
        '/', 292,
        ':', 196,
        ';', 137,
        '=', 173,
        '>', 180,
        '@', 56,
        'E', 39,
        '[', 172,
        '\\', 125,
        ']', 179,
        '^', 51,
        'a', 88,
        'e', 37,
        'f', 102,
        'h', 53,
        'i', 106,
        'n', 94,
        'o', 66,
        's', 70,
        't', 95,
        'w', 77,
        '{', 143,
        '|', 186,
        '}', 144,
        '~', 182,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(131);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      END_STATE();
    case 132:
      if (eof) ADVANCE(134);
      ADVANCE_MAP(
        '"', 247,
        '#', 171,
        '&', 148,
        '\'', 241,
        '(', 187,
        ')', 188,
        '*', 149,
        '+', 183,
        ',', 136,
        '-', 128,
        '.', 150,
        '/', 27,
        ':', 196,
        ';', 137,
        '>', 180,
        '@', 56,
        '[', 172,
        '\\', 125,
        ']', 179,
        '{', 143,
        '|', 185,
        '}', 144,
        '~', 181,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(133);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 133:
      if (eof) ADVANCE(134);
      ADVANCE_MAP(
        '"', 247,
        '#', 171,
        '&', 148,
        '\'', 241,
        ')', 188,
        '*', 149,
        '+', 183,
        ',', 136,
        '-', 128,
        '.', 150,
        '/', 27,
        ':', 196,
        ';', 137,
        '>', 180,
        '@', 56,
        '[', 172,
        '\\', 125,
        ']', 179,
        '{', 143,
        '|', 185,
        '}', 144,
        '~', 181,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(133);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(anon_sym_ATimport);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 136:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(anon_sym_ATmedia);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 139:
      ACCEPT_TOKEN(anon_sym_ATcharset);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 140:
      ACCEPT_TOKEN(anon_sym_ATnamespace);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(anon_sym_ATkeyframes);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 142:
      ACCEPT_TOKEN(aux_sym_keyframes_statement_token1);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 143:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(sym_from);
      END_STATE();
    case 146:
      ACCEPT_TOKEN(sym_to);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(anon_sym_ATsupports);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(sym_nesting_selector);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(266);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(anon_sym_has);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_has);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 154:
      ACCEPT_TOKEN(anon_sym_not);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(anon_sym_not);
      if (lookahead == '%') ADVANCE(286);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(anon_sym_not);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(anon_sym_not);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 158:
      ACCEPT_TOKEN(anon_sym_is);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(anon_sym_is);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(anon_sym_where);
      END_STATE();
    case 161:
      ACCEPT_TOKEN(anon_sym_where);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 162:
      ACCEPT_TOKEN(anon_sym_host);
      if (lookahead == '-') ADVANCE(59);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(anon_sym_host);
      if (lookahead == '-') ADVANCE(318);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(anon_sym_host_DASHcontext);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(anon_sym_host_DASHcontext);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(anon_sym_nth_DASHchild);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(anon_sym_nth_DASHchild);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(anon_sym_nth_DASHlast_DASHchild);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(anon_sym_nth_DASHlast_DASHchild);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(anon_sym_TILDE_EQ);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(anon_sym_CARET_EQ);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(anon_sym_PIPE_EQ);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(anon_sym_STAR_EQ);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(anon_sym_DOLLAR_EQ);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(anon_sym_TILDE);
      if (lookahead == '=') ADVANCE(174);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(anon_sym_PLUS);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 186:
      ACCEPT_TOKEN(anon_sym_PIPE);
      if (lookahead == '=') ADVANCE(176);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 189:
      ACCEPT_TOKEN(anon_sym_even);
      END_STATE();
    case 190:
      ACCEPT_TOKEN(anon_sym_odd);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(anon_sym_of);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(anon_sym_of);
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(286);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(sym__nth_functional_notation);
      if (lookahead == '+') ADVANCE(121);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(193);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(sym__nth_functional_notation);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(194);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(170);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(sym_important);
      END_STATE();
    case 198:
      ACCEPT_TOKEN(anon_sym_LPAREN2);
      END_STATE();
    case 199:
      ACCEPT_TOKEN(anon_sym_and);
      END_STATE();
    case 200:
      ACCEPT_TOKEN(anon_sym_and);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(anon_sym_or);
      END_STATE();
    case 202:
      ACCEPT_TOKEN(anon_sym_or);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 203:
      ACCEPT_TOKEN(anon_sym_only);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(anon_sym_only);
      if (lookahead == '%') ADVANCE(286);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(anon_sym_only);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 206:
      ACCEPT_TOKEN(anon_sym_only);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(anon_sym_selector);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(anon_sym_selector);
      if (lookahead == '%') ADVANCE(286);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 209:
      ACCEPT_TOKEN(anon_sym_selector);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(anon_sym_selector);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      END_STATE();
    case 212:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(267);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(224);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(211);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(225);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(230);
      END_STATE();
    case 215:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(232);
      END_STATE();
    case 216:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(227);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(234);
      END_STATE();
    case 217:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(123);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(228);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(236);
      END_STATE();
    case 218:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(212);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(211);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(218);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(214);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(232);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(221);
      END_STATE();
    case 221:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(213);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(230);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(219);
      END_STATE();
    case 222:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(215);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(234);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(220);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(216);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(236);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(222);
      END_STATE();
    case 224:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(267);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(224);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(211);
      END_STATE();
    case 226:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(225);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(230);
      END_STATE();
    case 227:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(232);
      END_STATE();
    case 228:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(227);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(234);
      END_STATE();
    case 229:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(228);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(236);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(211);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(240);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(230);
      END_STATE();
    case 233:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(231);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 234:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(232);
      END_STATE();
    case 235:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(233);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 236:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(234);
      END_STATE();
    case 237:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(235);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(236);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(237);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 240:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 241:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(aux_sym_string_value_token1);
      if (lookahead == '*') ADVANCE(244);
      if (lookahead == '/') ADVANCE(246);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(246);
      END_STATE();
    case 243:
      ACCEPT_TOKEN(aux_sym_string_value_token1);
      if (lookahead == '*') ADVANCE(243);
      if (lookahead == '/') ADVANCE(246);
      if (lookahead == '\n' ||
          lookahead == '\'' ||
          lookahead == '\\') ADVANCE(30);
      if (lookahead != 0) ADVANCE(244);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(aux_sym_string_value_token1);
      if (lookahead == '*') ADVANCE(243);
      if (lookahead == '\n' ||
          lookahead == '\'' ||
          lookahead == '\\') ADVANCE(30);
      if (lookahead != 0) ADVANCE(244);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(aux_sym_string_value_token1);
      if (lookahead == '/') ADVANCE(242);
      if (lookahead == '\t' ||
          (0x0b <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(245);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(246);
      END_STATE();
    case 246:
      ACCEPT_TOKEN(aux_sym_string_value_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(246);
      END_STATE();
    case 247:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(aux_sym_string_value_token2);
      if (lookahead == '*') ADVANCE(250);
      if (lookahead == '/') ADVANCE(252);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(252);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(aux_sym_string_value_token2);
      if (lookahead == '*') ADVANCE(249);
      if (lookahead == '/') ADVANCE(252);
      if (lookahead == '\n' ||
          lookahead == '"' ||
          lookahead == '\\') ADVANCE(30);
      if (lookahead != 0) ADVANCE(250);
      END_STATE();
    case 250:
      ACCEPT_TOKEN(aux_sym_string_value_token2);
      if (lookahead == '*') ADVANCE(249);
      if (lookahead == '\n' ||
          lookahead == '"' ||
          lookahead == '\\') ADVANCE(30);
      if (lookahead != 0) ADVANCE(250);
      END_STATE();
    case 251:
      ACCEPT_TOKEN(aux_sym_string_value_token2);
      if (lookahead == '/') ADVANCE(248);
      if (lookahead == '\t' ||
          (0x0b <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(251);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(252);
      END_STATE();
    case 252:
      ACCEPT_TOKEN(aux_sym_string_value_token2);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(252);
      END_STATE();
    case 253:
      ACCEPT_TOKEN(sym_escape_sequence);
      END_STATE();
    case 254:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(253);
      END_STATE();
    case 255:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(253);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(254);
      END_STATE();
    case 256:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(253);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(255);
      END_STATE();
    case 257:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(253);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(256);
      END_STATE();
    case 258:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(253);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(257);
      END_STATE();
    case 259:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(253);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(258);
      END_STATE();
    case 260:
      ACCEPT_TOKEN(aux_sym_integer_value_token1);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(aux_sym_integer_value_token1);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(217);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(238);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(223);
      END_STATE();
    case 262:
      ACCEPT_TOKEN(aux_sym_integer_value_token1);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(40);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(126);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(261);
      END_STATE();
    case 263:
      ACCEPT_TOKEN(aux_sym_integer_value_token1);
      if (lookahead == 'n') ADVANCE(193);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(263);
      END_STATE();
    case 264:
      ACCEPT_TOKEN(aux_sym_integer_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(264);
      END_STATE();
    case 265:
      ACCEPT_TOKEN(aux_sym_float_value_token1);
      if (lookahead == '/') ADVANCE(130);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(265);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 266:
      ACCEPT_TOKEN(aux_sym_float_value_token1);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(266);
      END_STATE();
    case 267:
      ACCEPT_TOKEN(aux_sym_float_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(267);
      END_STATE();
    case 268:
      ACCEPT_TOKEN(aux_sym_float_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(229);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(238);
      END_STATE();
    case 269:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'c') ADVANCE(279);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 270:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'e') ADVANCE(269);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 271:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'e') ADVANCE(273);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 272:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'l') ADVANCE(280);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 273:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'l') ADVANCE(270);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 274:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'n') ADVANCE(272);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 275:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'o') ADVANCE(278);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 276:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'o') ADVANCE(277);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 277:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'r') ADVANCE(208);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 't') ADVANCE(155);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 279:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 't') ADVANCE(276);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (lookahead == 'y') ADVANCE(204);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 281:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(286);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(281);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 282:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(284);
      if (lookahead == '-') ADVANCE(313);
      if (lookahead == '/') ADVANCE(130);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(265);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(283);
      if (lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 283:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(284);
      if (lookahead == '/') ADVANCE(130);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(283);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 284:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(284);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 285:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == 'f') ADVANCE(192);
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(286);
      END_STATE();
    case 286:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(286);
      END_STATE();
    case 287:
      ACCEPT_TOKEN(anon_sym_LBRACK2);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 289:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == '/') ADVANCE(129);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(297);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(294);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if ((0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 290:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '.') ADVANCE(122);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(260);
      END_STATE();
    case 291:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 292:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(30);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(30);
      if (lookahead == '/') ADVANCE(409);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(48);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '-') ADVANCE(313);
      if (lookahead == '/') ADVANCE(130);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(265);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '-') ADVANCE(319);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '-') ADVANCE(320);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(129);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(297);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(314);
      if (('0' <= lookahead && lookahead <= '9') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 298:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'c') ADVANCE(311);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'd') ADVANCE(200);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 300:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'e') ADVANCE(298);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 301:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'e') ADVANCE(303);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'l') ADVANCE(312);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'l') ADVANCE(300);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 304:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'n') ADVANCE(302);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 305:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'n') ADVANCE(299);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 306:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'o') ADVANCE(310);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'o') ADVANCE(308);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'r') ADVANCE(209);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'r') ADVANCE(202);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 310:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 't') ADVANCE(156);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 311:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 't') ADVANCE(307);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 312:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == 'y') ADVANCE(205);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 313:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(265);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(130);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(314);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 315:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(347);
      if (lookahead == 'o') ADVANCE(348);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(349);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 317:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(353);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 318:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(343);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 319:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(329);
      if (lookahead == 'l') ADVANCE(316);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 320:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(331);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 321:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'd') ADVANCE(167);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 322:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'd') ADVANCE(169);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 323:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(317);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 324:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(161);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 325:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(356);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 326:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(336);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 327:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(345);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'h') ADVANCE(295);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 329:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'h') ADVANCE(332);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 330:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'h') ADVANCE(327);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'h') ADVANCE(333);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(335);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(337);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 334:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(357);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 335:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(321);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(323);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(322);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(334);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(355);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 340:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(350);
      if (lookahead == 't') ADVANCE(328);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(350);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 342:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(344);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 343:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(339);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 344:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(210);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 345:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(324);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(159);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(153);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(351);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 349:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(354);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 350:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(157);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 351:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(163);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(165);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(342);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 354:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(296);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(325);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 356:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'x') ADVANCE(352);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'y') ADVANCE(206);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(239);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(358);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa0 <= lookahead && lookahead <= 0xff)) ADVANCE(360);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(391);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(138);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(367);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(382);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(383);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(384);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'c') ADVANCE(371);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'd') ADVANCE(380);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(405);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(368);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(140);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(399);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(402);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(397);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(398);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(406);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 377:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'f') ADVANCE(394);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'f') ADVANCE(395);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'h') ADVANCE(361);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'i') ADVANCE(362);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'm') ADVANCE(387);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'm') ADVANCE(372);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'm') ADVANCE(374);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'm') ADVANCE(375);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'o') ADVANCE(392);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'o') ADVANCE(393);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'p') ADVANCE(385);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 388:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'p') ADVANCE(363);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 389:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'p') ADVANCE(386);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'p') ADVANCE(389);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'r') ADVANCE(400);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 392:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'r') ADVANCE(401);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'r') ADVANCE(403);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'r') ADVANCE(365);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 395:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'r') ADVANCE(366);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 396:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 's') ADVANCE(147);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 397:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 's') ADVANCE(141);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 398:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 's') ADVANCE(142);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 399:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 's') ADVANCE(388);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 's') ADVANCE(373);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 401:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 't') ADVANCE(135);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 402:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 't') ADVANCE(139);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 't') ADVANCE(396);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 404:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'u') ADVANCE(390);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 405:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'y') ADVANCE(377);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == 'y') ADVANCE(378);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(376);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(407);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(408);
      END_STATE();
    case 408:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(408);
      END_STATE();
    case 409:
      ACCEPT_TOKEN(sym_js_comment);
      if (lookahead == '/') ADVANCE(410);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(409);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(415);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(412);
      END_STATE();
    case 410:
      ACCEPT_TOKEN(sym_js_comment);
      if (lookahead == '\t' ||
          (0x0b <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == '!' ||
          ('(' <= lookahead && lookahead <= '*') ||
          lookahead == ',' ||
          lookahead == ';' ||
          lookahead == '[' ||
          lookahead == ']' ||
          lookahead == '{' ||
          lookahead == '}') ADVANCE(412);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead)) ADVANCE(409);
      END_STATE();
    case 411:
      ACCEPT_TOKEN(sym_js_comment);
      if (lookahead == '\t' ||
          (0x0b <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == '!' ||
          ('(' <= lookahead && lookahead <= '*') ||
          lookahead == ',' ||
          lookahead == ';' ||
          lookahead == '[' ||
          lookahead == ']' ||
          lookahead == '{' ||
          lookahead == '}') ADVANCE(412);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead)) ADVANCE(415);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(sym_js_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(412);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 414:
      ACCEPT_TOKEN(sym_plain_value);
      if (lookahead == '/') ADVANCE(130);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(414);
      END_STATE();
    case 415:
      ACCEPT_TOKEN(sym_plain_value);
      if (lookahead == '/') ADVANCE(411);
      if ((!eof && set_contains(sym_plain_value_character_set_2, 10, lookahead))) ADVANCE(415);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 132, .external_lex_state = 2},
  [2] = {.lex_state = 132, .external_lex_state = 2},
  [3] = {.lex_state = 132, .external_lex_state = 2},
  [4] = {.lex_state = 132, .external_lex_state = 2},
  [5] = {.lex_state = 132, .external_lex_state = 2},
  [6] = {.lex_state = 132, .external_lex_state = 2},
  [7] = {.lex_state = 132, .external_lex_state = 2},
  [8] = {.lex_state = 132, .external_lex_state = 2},
  [9] = {.lex_state = 132, .external_lex_state = 2},
  [10] = {.lex_state = 132, .external_lex_state = 2},
  [11] = {.lex_state = 4, .external_lex_state = 2},
  [12] = {.lex_state = 4, .external_lex_state = 2},
  [13] = {.lex_state = 4, .external_lex_state = 2},
  [14] = {.lex_state = 12},
  [15] = {.lex_state = 132, .external_lex_state = 2},
  [16] = {.lex_state = 12},
  [17] = {.lex_state = 12},
  [18] = {.lex_state = 9},
  [19] = {.lex_state = 132, .external_lex_state = 2},
  [20] = {.lex_state = 132, .external_lex_state = 2},
  [21] = {.lex_state = 132, .external_lex_state = 2},
  [22] = {.lex_state = 132, .external_lex_state = 2},
  [23] = {.lex_state = 132, .external_lex_state = 2},
  [24] = {.lex_state = 132, .external_lex_state = 2},
  [25] = {.lex_state = 132, .external_lex_state = 2},
  [26] = {.lex_state = 132, .external_lex_state = 2},
  [27] = {.lex_state = 132, .external_lex_state = 2},
  [28] = {.lex_state = 132, .external_lex_state = 2},
  [29] = {.lex_state = 132, .external_lex_state = 2},
  [30] = {.lex_state = 132, .external_lex_state = 2},
  [31] = {.lex_state = 132, .external_lex_state = 2},
  [32] = {.lex_state = 132, .external_lex_state = 2},
  [33] = {.lex_state = 5, .external_lex_state = 3},
  [34] = {.lex_state = 9},
  [35] = {.lex_state = 9},
  [36] = {.lex_state = 8, .external_lex_state = 3},
  [37] = {.lex_state = 10},
  [38] = {.lex_state = 8, .external_lex_state = 3},
  [39] = {.lex_state = 10},
  [40] = {.lex_state = 13, .external_lex_state = 3},
  [41] = {.lex_state = 8, .external_lex_state = 3},
  [42] = {.lex_state = 10},
  [43] = {.lex_state = 13, .external_lex_state = 3},
  [44] = {.lex_state = 132, .external_lex_state = 2},
  [45] = {.lex_state = 132, .external_lex_state = 2},
  [46] = {.lex_state = 132, .external_lex_state = 2},
  [47] = {.lex_state = 10},
  [48] = {.lex_state = 132, .external_lex_state = 2},
  [49] = {.lex_state = 132, .external_lex_state = 2},
  [50] = {.lex_state = 132, .external_lex_state = 2},
  [51] = {.lex_state = 132, .external_lex_state = 2},
  [52] = {.lex_state = 132, .external_lex_state = 2},
  [53] = {.lex_state = 132, .external_lex_state = 2},
  [54] = {.lex_state = 132, .external_lex_state = 2},
  [55] = {.lex_state = 10},
  [56] = {.lex_state = 132, .external_lex_state = 2},
  [57] = {.lex_state = 132, .external_lex_state = 2},
  [58] = {.lex_state = 132, .external_lex_state = 2},
  [59] = {.lex_state = 132, .external_lex_state = 2},
  [60] = {.lex_state = 10},
  [61] = {.lex_state = 132, .external_lex_state = 2},
  [62] = {.lex_state = 132, .external_lex_state = 2},
  [63] = {.lex_state = 132, .external_lex_state = 2},
  [64] = {.lex_state = 10},
  [65] = {.lex_state = 132, .external_lex_state = 2},
  [66] = {.lex_state = 10},
  [67] = {.lex_state = 132, .external_lex_state = 2},
  [68] = {.lex_state = 132, .external_lex_state = 2},
  [69] = {.lex_state = 132, .external_lex_state = 2},
  [70] = {.lex_state = 132, .external_lex_state = 2},
  [71] = {.lex_state = 132, .external_lex_state = 2},
  [72] = {.lex_state = 132, .external_lex_state = 2},
  [73] = {.lex_state = 132, .external_lex_state = 2},
  [74] = {.lex_state = 132, .external_lex_state = 2},
  [75] = {.lex_state = 132, .external_lex_state = 2},
  [76] = {.lex_state = 132, .external_lex_state = 2},
  [77] = {.lex_state = 132, .external_lex_state = 2},
  [78] = {.lex_state = 132, .external_lex_state = 2},
  [79] = {.lex_state = 132, .external_lex_state = 2},
  [80] = {.lex_state = 132, .external_lex_state = 2},
  [81] = {.lex_state = 132, .external_lex_state = 2},
  [82] = {.lex_state = 132, .external_lex_state = 2},
  [83] = {.lex_state = 132, .external_lex_state = 2},
  [84] = {.lex_state = 132, .external_lex_state = 2},
  [85] = {.lex_state = 132, .external_lex_state = 2},
  [86] = {.lex_state = 132, .external_lex_state = 2},
  [87] = {.lex_state = 132, .external_lex_state = 2},
  [88] = {.lex_state = 132, .external_lex_state = 2},
  [89] = {.lex_state = 132, .external_lex_state = 2},
  [90] = {.lex_state = 132, .external_lex_state = 2},
  [91] = {.lex_state = 132, .external_lex_state = 2},
  [92] = {.lex_state = 132, .external_lex_state = 2},
  [93] = {.lex_state = 10},
  [94] = {.lex_state = 132, .external_lex_state = 2},
  [95] = {.lex_state = 132, .external_lex_state = 2},
  [96] = {.lex_state = 13, .external_lex_state = 3},
  [97] = {.lex_state = 10},
  [98] = {.lex_state = 13, .external_lex_state = 3},
  [99] = {.lex_state = 13, .external_lex_state = 3},
  [100] = {.lex_state = 13, .external_lex_state = 3},
  [101] = {.lex_state = 10},
  [102] = {.lex_state = 13, .external_lex_state = 3},
  [103] = {.lex_state = 13, .external_lex_state = 3},
  [104] = {.lex_state = 10},
  [105] = {.lex_state = 10},
  [106] = {.lex_state = 10},
  [107] = {.lex_state = 7},
  [108] = {.lex_state = 10},
  [109] = {.lex_state = 10},
  [110] = {.lex_state = 10},
  [111] = {.lex_state = 13, .external_lex_state = 3},
  [112] = {.lex_state = 13, .external_lex_state = 3},
  [113] = {.lex_state = 13, .external_lex_state = 3},
  [114] = {.lex_state = 13, .external_lex_state = 3},
  [115] = {.lex_state = 13, .external_lex_state = 3},
  [116] = {.lex_state = 13, .external_lex_state = 3},
  [117] = {.lex_state = 13, .external_lex_state = 3},
  [118] = {.lex_state = 13, .external_lex_state = 3},
  [119] = {.lex_state = 13, .external_lex_state = 3},
  [120] = {.lex_state = 10},
  [121] = {.lex_state = 13, .external_lex_state = 3},
  [122] = {.lex_state = 13, .external_lex_state = 3},
  [123] = {.lex_state = 13, .external_lex_state = 3},
  [124] = {.lex_state = 13, .external_lex_state = 3},
  [125] = {.lex_state = 13, .external_lex_state = 3},
  [126] = {.lex_state = 13, .external_lex_state = 3},
  [127] = {.lex_state = 13, .external_lex_state = 3},
  [128] = {.lex_state = 13, .external_lex_state = 3},
  [129] = {.lex_state = 10},
  [130] = {.lex_state = 13, .external_lex_state = 3},
  [131] = {.lex_state = 13, .external_lex_state = 3},
  [132] = {.lex_state = 13, .external_lex_state = 3},
  [133] = {.lex_state = 13, .external_lex_state = 3},
  [134] = {.lex_state = 13, .external_lex_state = 3},
  [135] = {.lex_state = 13, .external_lex_state = 3},
  [136] = {.lex_state = 13, .external_lex_state = 3},
  [137] = {.lex_state = 13, .external_lex_state = 3},
  [138] = {.lex_state = 13, .external_lex_state = 3},
  [139] = {.lex_state = 13, .external_lex_state = 3},
  [140] = {.lex_state = 13, .external_lex_state = 3},
  [141] = {.lex_state = 13, .external_lex_state = 3},
  [142] = {.lex_state = 13, .external_lex_state = 3},
  [143] = {.lex_state = 13, .external_lex_state = 3},
  [144] = {.lex_state = 13, .external_lex_state = 3},
  [145] = {.lex_state = 13, .external_lex_state = 3},
  [146] = {.lex_state = 13, .external_lex_state = 3},
  [147] = {.lex_state = 13, .external_lex_state = 3},
  [148] = {.lex_state = 13, .external_lex_state = 3},
  [149] = {.lex_state = 6},
  [150] = {.lex_state = 13, .external_lex_state = 3},
  [151] = {.lex_state = 13, .external_lex_state = 3},
  [152] = {.lex_state = 13, .external_lex_state = 3},
  [153] = {.lex_state = 13, .external_lex_state = 3},
  [154] = {.lex_state = 10},
  [155] = {.lex_state = 10},
  [156] = {.lex_state = 10},
  [157] = {.lex_state = 10},
  [158] = {.lex_state = 10},
  [159] = {.lex_state = 10},
  [160] = {.lex_state = 10},
  [161] = {.lex_state = 3},
  [162] = {.lex_state = 3},
  [163] = {.lex_state = 10},
  [164] = {.lex_state = 10},
  [165] = {.lex_state = 10},
  [166] = {.lex_state = 10},
  [167] = {.lex_state = 10},
  [168] = {.lex_state = 10},
  [169] = {.lex_state = 10},
  [170] = {.lex_state = 10},
  [171] = {.lex_state = 10},
  [172] = {.lex_state = 10},
  [173] = {.lex_state = 10},
  [174] = {.lex_state = 10},
  [175] = {.lex_state = 10},
  [176] = {.lex_state = 9},
  [177] = {.lex_state = 9},
  [178] = {.lex_state = 9},
  [179] = {.lex_state = 9},
  [180] = {.lex_state = 9},
  [181] = {.lex_state = 9},
  [182] = {.lex_state = 9},
  [183] = {.lex_state = 9},
  [184] = {.lex_state = 9},
  [185] = {.lex_state = 9},
  [186] = {.lex_state = 9},
  [187] = {.lex_state = 9},
  [188] = {.lex_state = 9},
  [189] = {.lex_state = 9},
  [190] = {.lex_state = 13, .external_lex_state = 3},
  [191] = {.lex_state = 13, .external_lex_state = 3},
  [192] = {.lex_state = 9},
  [193] = {.lex_state = 13, .external_lex_state = 3},
  [194] = {.lex_state = 9},
  [195] = {.lex_state = 9},
  [196] = {.lex_state = 9},
  [197] = {.lex_state = 13, .external_lex_state = 3},
  [198] = {.lex_state = 9},
  [199] = {.lex_state = 9},
  [200] = {.lex_state = 18},
  [201] = {.lex_state = 9},
  [202] = {.lex_state = 9},
  [203] = {.lex_state = 18},
  [204] = {.lex_state = 20},
  [205] = {.lex_state = 132, .external_lex_state = 3},
  [206] = {.lex_state = 132, .external_lex_state = 3},
  [207] = {.lex_state = 132, .external_lex_state = 3},
  [208] = {.lex_state = 132, .external_lex_state = 3},
  [209] = {.lex_state = 132, .external_lex_state = 3},
  [210] = {.lex_state = 132, .external_lex_state = 3},
  [211] = {.lex_state = 33},
  [212] = {.lex_state = 33},
  [213] = {.lex_state = 132, .external_lex_state = 3},
  [214] = {.lex_state = 132, .external_lex_state = 3},
  [215] = {.lex_state = 17},
  [216] = {.lex_state = 132, .external_lex_state = 3},
  [217] = {.lex_state = 20},
  [218] = {.lex_state = 18},
  [219] = {.lex_state = 132, .external_lex_state = 3},
  [220] = {.lex_state = 20},
  [221] = {.lex_state = 20},
  [222] = {.lex_state = 20},
  [223] = {.lex_state = 15},
  [224] = {.lex_state = 20},
  [225] = {.lex_state = 20},
  [226] = {.lex_state = 132, .external_lex_state = 3},
  [227] = {.lex_state = 20},
  [228] = {.lex_state = 20},
  [229] = {.lex_state = 15},
  [230] = {.lex_state = 18},
  [231] = {.lex_state = 18},
  [232] = {.lex_state = 18},
  [233] = {.lex_state = 132, .external_lex_state = 3},
  [234] = {.lex_state = 18},
  [235] = {.lex_state = 16},
  [236] = {.lex_state = 18},
  [237] = {.lex_state = 18},
  [238] = {.lex_state = 18},
  [239] = {.lex_state = 18},
  [240] = {.lex_state = 18},
  [241] = {.lex_state = 18},
  [242] = {.lex_state = 18},
  [243] = {.lex_state = 16},
  [244] = {.lex_state = 18},
  [245] = {.lex_state = 23},
  [246] = {.lex_state = 23},
  [247] = {.lex_state = 16},
  [248] = {.lex_state = 16},
  [249] = {.lex_state = 16},
  [250] = {.lex_state = 16},
  [251] = {.lex_state = 16},
  [252] = {.lex_state = 16},
  [253] = {.lex_state = 16},
  [254] = {.lex_state = 16},
  [255] = {.lex_state = 16},
  [256] = {.lex_state = 16},
  [257] = {.lex_state = 16},
  [258] = {.lex_state = 16},
  [259] = {.lex_state = 16},
  [260] = {.lex_state = 132},
  [261] = {.lex_state = 24},
  [262] = {.lex_state = 24},
  [263] = {.lex_state = 24},
  [264] = {.lex_state = 24},
  [265] = {.lex_state = 16},
  [266] = {.lex_state = 24},
  [267] = {.lex_state = 16},
  [268] = {.lex_state = 16},
  [269] = {.lex_state = 24},
  [270] = {.lex_state = 132},
  [271] = {.lex_state = 24},
  [272] = {.lex_state = 24},
  [273] = {.lex_state = 24},
  [274] = {.lex_state = 24},
  [275] = {.lex_state = 24},
  [276] = {.lex_state = 16},
  [277] = {.lex_state = 24},
  [278] = {.lex_state = 24},
  [279] = {.lex_state = 24},
  [280] = {.lex_state = 24},
  [281] = {.lex_state = 16},
  [282] = {.lex_state = 16},
  [283] = {.lex_state = 132},
  [284] = {.lex_state = 16},
  [285] = {.lex_state = 16},
  [286] = {.lex_state = 24},
  [287] = {.lex_state = 24},
  [288] = {.lex_state = 132},
  [289] = {.lex_state = 132},
  [290] = {.lex_state = 31},
  [291] = {.lex_state = 16},
  [292] = {.lex_state = 24},
  [293] = {.lex_state = 132},
  [294] = {.lex_state = 16},
  [295] = {.lex_state = 16},
  [296] = {.lex_state = 24},
  [297] = {.lex_state = 132},
  [298] = {.lex_state = 1},
  [299] = {.lex_state = 132},
  [300] = {.lex_state = 132},
  [301] = {.lex_state = 132},
  [302] = {.lex_state = 132},
  [303] = {.lex_state = 2},
  [304] = {.lex_state = 24},
  [305] = {.lex_state = 24},
  [306] = {.lex_state = 132},
  [307] = {.lex_state = 132},
  [308] = {.lex_state = 132},
  [309] = {.lex_state = 2},
  [310] = {.lex_state = 1},
  [311] = {.lex_state = 2},
  [312] = {.lex_state = 24},
  [313] = {.lex_state = 2},
  [314] = {.lex_state = 2},
  [315] = {.lex_state = 24},
  [316] = {.lex_state = 1},
  [317] = {.lex_state = 132},
  [318] = {.lex_state = 2},
  [319] = {.lex_state = 1},
  [320] = {.lex_state = 2},
  [321] = {.lex_state = 1},
  [322] = {.lex_state = 2},
  [323] = {.lex_state = 1},
  [324] = {.lex_state = 2},
  [325] = {.lex_state = 1},
  [326] = {.lex_state = 1},
  [327] = {.lex_state = 132},
  [328] = {.lex_state = 1},
  [329] = {.lex_state = 24},
  [330] = {.lex_state = 24},
  [331] = {.lex_state = 2},
  [332] = {.lex_state = 1},
  [333] = {.lex_state = 2},
  [334] = {.lex_state = 1},
  [335] = {.lex_state = 132},
  [336] = {.lex_state = 132},
  [337] = {.lex_state = 132},
  [338] = {.lex_state = 132},
  [339] = {.lex_state = 132},
  [340] = {.lex_state = 132},
  [341] = {.lex_state = 132},
  [342] = {.lex_state = 132},
  [343] = {.lex_state = 132},
  [344] = {.lex_state = 132},
  [345] = {.lex_state = 24},
  [346] = {.lex_state = 25},
  [347] = {.lex_state = 132},
  [348] = {.lex_state = 24},
  [349] = {.lex_state = 132},
  [350] = {.lex_state = 132},
  [351] = {.lex_state = 132},
  [352] = {.lex_state = 45},
  [353] = {.lex_state = 132},
  [354] = {.lex_state = 132},
  [355] = {.lex_state = 132},
  [356] = {.lex_state = 132},
  [357] = {.lex_state = 34},
  [358] = {.lex_state = 132},
  [359] = {.lex_state = 132},
  [360] = {.lex_state = 132},
  [361] = {.lex_state = 132},
  [362] = {.lex_state = 132},
  [363] = {.lex_state = 132},
  [364] = {.lex_state = 47},
  [365] = {.lex_state = 132},
  [366] = {.lex_state = 132},
  [367] = {.lex_state = 132},
  [368] = {.lex_state = 132},
  [369] = {.lex_state = 132},
  [370] = {.lex_state = 132},
  [371] = {.lex_state = 132},
  [372] = {.lex_state = 132},
  [373] = {.lex_state = 132},
  [374] = {.lex_state = 132},
  [375] = {.lex_state = 47},
  [376] = {.lex_state = 132},
  [377] = {.lex_state = 132},
  [378] = {.lex_state = 47},
  [379] = {.lex_state = 20},
  [380] = {.lex_state = 132},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_ATimport] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_ATmedia] = ACTIONS(1),
    [anon_sym_ATcharset] = ACTIONS(1),
    [anon_sym_ATnamespace] = ACTIONS(1),
    [anon_sym_ATkeyframes] = ACTIONS(1),
    [aux_sym_keyframes_statement_token1] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [sym_from] = ACTIONS(1),
    [sym_to] = ACTIONS(1),
    [anon_sym_ATsupports] = ACTIONS(1),
    [sym_nesting_selector] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_has] = ACTIONS(1),
    [anon_sym_not] = ACTIONS(1),
    [anon_sym_is] = ACTIONS(1),
    [anon_sym_where] = ACTIONS(1),
    [anon_sym_host] = ACTIONS(1),
    [anon_sym_host_DASHcontext] = ACTIONS(1),
    [anon_sym_nth_DASHchild] = ACTIONS(1),
    [anon_sym_nth_DASHlast_DASHchild] = ACTIONS(1),
    [anon_sym_COLON_COLON] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_TILDE_EQ] = ACTIONS(1),
    [anon_sym_CARET_EQ] = ACTIONS(1),
    [anon_sym_PIPE_EQ] = ACTIONS(1),
    [anon_sym_DOLLAR_EQ] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_even] = ACTIONS(1),
    [anon_sym_odd] = ACTIONS(1),
    [anon_sym_of] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [sym_important] = ACTIONS(1),
    [anon_sym_LPAREN2] = ACTIONS(1),
    [anon_sym_and] = ACTIONS(1),
    [anon_sym_or] = ACTIONS(1),
    [anon_sym_only] = ACTIONS(1),
    [anon_sym_selector] = ACTIONS(1),
    [aux_sym_color_value_token1] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [sym_escape_sequence] = ACTIONS(1),
    [aux_sym_integer_value_token1] = ACTIONS(1),
    [aux_sym_float_value_token1] = ACTIONS(1),
    [anon_sym_LBRACK2] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [sym_at_keyword] = ACTIONS(1),
    [sym_js_comment] = ACTIONS(3),
    [sym_comment] = ACTIONS(5),
    [sym__descendant_operator] = ACTIONS(1),
    [sym__pseudo_class_selector_colon] = ACTIONS(1),
    [sym___error_recovery] = ACTIONS(1),
  },
  [1] = {
    [sym_stylesheet] = STATE(363),
    [sym_import_statement] = STATE(10),
    [sym_media_statement] = STATE(10),
    [sym_charset_statement] = STATE(10),
    [sym_namespace_statement] = STATE(10),
    [sym_keyframes_statement] = STATE(10),
    [sym_supports_statement] = STATE(10),
    [sym_at_rule] = STATE(10),
    [sym_rule_set] = STATE(10),
    [sym_selectors] = STATE(354),
    [sym__selector] = STATE(207),
    [sym_universal_selector] = STATE(207),
    [sym_class_selector] = STATE(207),
    [sym_pseudo_class_selector] = STATE(207),
    [sym_pseudo_element_selector] = STATE(207),
    [sym_id_selector] = STATE(207),
    [sym_attribute_selector] = STATE(207),
    [sym_child_selector] = STATE(207),
    [sym_descendant_selector] = STATE(207),
    [sym_sibling_selector] = STATE(207),
    [sym_adjacent_sibling_selector] = STATE(207),
    [sym_namespace_selector] = STATE(207),
    [sym_declaration] = STATE(10),
    [sym_string_value] = STATE(207),
    [aux_sym_stylesheet_repeat1] = STATE(10),
    [ts_builtin_sym_end] = ACTIONS(7),
    [anon_sym_ATimport] = ACTIONS(9),
    [anon_sym_ATmedia] = ACTIONS(11),
    [anon_sym_ATcharset] = ACTIONS(13),
    [anon_sym_ATnamespace] = ACTIONS(15),
    [anon_sym_ATkeyframes] = ACTIONS(17),
    [aux_sym_keyframes_statement_token1] = ACTIONS(17),
    [anon_sym_ATsupports] = ACTIONS(19),
    [sym_nesting_selector] = ACTIONS(21),
    [anon_sym_STAR] = ACTIONS(23),
    [anon_sym_DOT] = ACTIONS(25),
    [anon_sym_COLON_COLON] = ACTIONS(27),
    [anon_sym_POUND] = ACTIONS(29),
    [anon_sym_LBRACK] = ACTIONS(31),
    [anon_sym_GT] = ACTIONS(33),
    [anon_sym_TILDE] = ACTIONS(35),
    [anon_sym_PLUS] = ACTIONS(37),
    [anon_sym_PIPE] = ACTIONS(39),
    [anon_sym_SQUOTE] = ACTIONS(41),
    [anon_sym_DQUOTE] = ACTIONS(43),
    [sym_identifier] = ACTIONS(45),
    [sym_at_keyword] = ACTIONS(47),
    [sym_js_comment] = ACTIONS(5),
    [sym_comment] = ACTIONS(5),
    [sym__pseudo_class_selector_colon] = ACTIONS(49),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 27,
    ACTIONS(21), 1,
      sym_nesting_selector,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(51), 1,
      anon_sym_ATimport,
    ACTIONS(53), 1,
      anon_sym_ATmedia,
    ACTIONS(55), 1,
      anon_sym_ATcharset,
    ACTIONS(57), 1,
      anon_sym_ATnamespace,
    ACTIONS(61), 1,
      anon_sym_RBRACE,
    ACTIONS(63), 1,
      anon_sym_ATsupports,
    ACTIONS(65), 1,
      sym_identifier,
    ACTIONS(67), 1,
      sym_at_keyword,
    STATE(356), 1,
      sym_selectors,
    STATE(365), 1,
      sym_last_declaration,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(59), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(8), 11,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_postcss_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(207), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [106] = 27,
    ACTIONS(21), 1,
      sym_nesting_selector,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(51), 1,
      anon_sym_ATimport,
    ACTIONS(53), 1,
      anon_sym_ATmedia,
    ACTIONS(55), 1,
      anon_sym_ATcharset,
    ACTIONS(57), 1,
      anon_sym_ATnamespace,
    ACTIONS(63), 1,
      anon_sym_ATsupports,
    ACTIONS(65), 1,
      sym_identifier,
    ACTIONS(67), 1,
      sym_at_keyword,
    ACTIONS(69), 1,
      anon_sym_RBRACE,
    STATE(356), 1,
      sym_selectors,
    STATE(376), 1,
      sym_last_declaration,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(59), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(8), 11,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_postcss_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(207), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [212] = 27,
    ACTIONS(21), 1,
      sym_nesting_selector,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(51), 1,
      anon_sym_ATimport,
    ACTIONS(53), 1,
      anon_sym_ATmedia,
    ACTIONS(55), 1,
      anon_sym_ATcharset,
    ACTIONS(57), 1,
      anon_sym_ATnamespace,
    ACTIONS(63), 1,
      anon_sym_ATsupports,
    ACTIONS(65), 1,
      sym_identifier,
    ACTIONS(67), 1,
      sym_at_keyword,
    ACTIONS(71), 1,
      anon_sym_RBRACE,
    STATE(356), 1,
      sym_selectors,
    STATE(377), 1,
      sym_last_declaration,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(59), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(3), 11,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_postcss_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(207), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [318] = 27,
    ACTIONS(21), 1,
      sym_nesting_selector,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(51), 1,
      anon_sym_ATimport,
    ACTIONS(53), 1,
      anon_sym_ATmedia,
    ACTIONS(55), 1,
      anon_sym_ATcharset,
    ACTIONS(57), 1,
      anon_sym_ATnamespace,
    ACTIONS(63), 1,
      anon_sym_ATsupports,
    ACTIONS(65), 1,
      sym_identifier,
    ACTIONS(67), 1,
      sym_at_keyword,
    ACTIONS(73), 1,
      anon_sym_RBRACE,
    STATE(356), 1,
      sym_selectors,
    STATE(374), 1,
      sym_last_declaration,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(59), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(7), 11,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_postcss_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(207), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [424] = 27,
    ACTIONS(21), 1,
      sym_nesting_selector,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(51), 1,
      anon_sym_ATimport,
    ACTIONS(53), 1,
      anon_sym_ATmedia,
    ACTIONS(55), 1,
      anon_sym_ATcharset,
    ACTIONS(57), 1,
      anon_sym_ATnamespace,
    ACTIONS(63), 1,
      anon_sym_ATsupports,
    ACTIONS(65), 1,
      sym_identifier,
    ACTIONS(67), 1,
      sym_at_keyword,
    ACTIONS(75), 1,
      anon_sym_RBRACE,
    STATE(356), 1,
      sym_selectors,
    STATE(360), 1,
      sym_last_declaration,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(59), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(2), 11,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_postcss_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(207), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [530] = 27,
    ACTIONS(21), 1,
      sym_nesting_selector,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(51), 1,
      anon_sym_ATimport,
    ACTIONS(53), 1,
      anon_sym_ATmedia,
    ACTIONS(55), 1,
      anon_sym_ATcharset,
    ACTIONS(57), 1,
      anon_sym_ATnamespace,
    ACTIONS(63), 1,
      anon_sym_ATsupports,
    ACTIONS(65), 1,
      sym_identifier,
    ACTIONS(67), 1,
      sym_at_keyword,
    ACTIONS(77), 1,
      anon_sym_RBRACE,
    STATE(356), 1,
      sym_selectors,
    STATE(362), 1,
      sym_last_declaration,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(59), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(8), 11,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_postcss_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(207), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [636] = 26,
    ACTIONS(79), 1,
      anon_sym_ATimport,
    ACTIONS(82), 1,
      anon_sym_ATmedia,
    ACTIONS(85), 1,
      anon_sym_ATcharset,
    ACTIONS(88), 1,
      anon_sym_ATnamespace,
    ACTIONS(94), 1,
      anon_sym_RBRACE,
    ACTIONS(96), 1,
      anon_sym_ATsupports,
    ACTIONS(99), 1,
      sym_nesting_selector,
    ACTIONS(102), 1,
      anon_sym_STAR,
    ACTIONS(105), 1,
      anon_sym_DOT,
    ACTIONS(108), 1,
      anon_sym_COLON_COLON,
    ACTIONS(111), 1,
      anon_sym_POUND,
    ACTIONS(114), 1,
      anon_sym_LBRACK,
    ACTIONS(117), 1,
      anon_sym_GT,
    ACTIONS(120), 1,
      anon_sym_TILDE,
    ACTIONS(123), 1,
      anon_sym_PLUS,
    ACTIONS(126), 1,
      anon_sym_PIPE,
    ACTIONS(129), 1,
      anon_sym_SQUOTE,
    ACTIONS(132), 1,
      anon_sym_DQUOTE,
    ACTIONS(135), 1,
      sym_identifier,
    ACTIONS(138), 1,
      sym_at_keyword,
    ACTIONS(141), 1,
      sym__pseudo_class_selector_colon,
    STATE(356), 1,
      sym_selectors,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(91), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(8), 11,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_postcss_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(207), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [739] = 26,
    ACTIONS(144), 1,
      ts_builtin_sym_end,
    ACTIONS(146), 1,
      anon_sym_ATimport,
    ACTIONS(149), 1,
      anon_sym_ATmedia,
    ACTIONS(152), 1,
      anon_sym_ATcharset,
    ACTIONS(155), 1,
      anon_sym_ATnamespace,
    ACTIONS(161), 1,
      anon_sym_ATsupports,
    ACTIONS(164), 1,
      sym_nesting_selector,
    ACTIONS(167), 1,
      anon_sym_STAR,
    ACTIONS(170), 1,
      anon_sym_DOT,
    ACTIONS(173), 1,
      anon_sym_COLON_COLON,
    ACTIONS(176), 1,
      anon_sym_POUND,
    ACTIONS(179), 1,
      anon_sym_LBRACK,
    ACTIONS(182), 1,
      anon_sym_GT,
    ACTIONS(185), 1,
      anon_sym_TILDE,
    ACTIONS(188), 1,
      anon_sym_PLUS,
    ACTIONS(191), 1,
      anon_sym_PIPE,
    ACTIONS(194), 1,
      anon_sym_SQUOTE,
    ACTIONS(197), 1,
      anon_sym_DQUOTE,
    ACTIONS(200), 1,
      sym_identifier,
    ACTIONS(203), 1,
      sym_at_keyword,
    ACTIONS(206), 1,
      sym__pseudo_class_selector_colon,
    STATE(354), 1,
      sym_selectors,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(158), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(9), 10,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_stylesheet_repeat1,
    STATE(207), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [841] = 26,
    ACTIONS(9), 1,
      anon_sym_ATimport,
    ACTIONS(11), 1,
      anon_sym_ATmedia,
    ACTIONS(13), 1,
      anon_sym_ATcharset,
    ACTIONS(15), 1,
      anon_sym_ATnamespace,
    ACTIONS(19), 1,
      anon_sym_ATsupports,
    ACTIONS(21), 1,
      sym_nesting_selector,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(47), 1,
      sym_at_keyword,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(209), 1,
      ts_builtin_sym_end,
    STATE(354), 1,
      sym_selectors,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(17), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(9), 10,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_stylesheet_repeat1,
    STATE(207), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [943] = 27,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(211), 1,
      sym_nesting_selector,
    ACTIONS(213), 1,
      anon_sym_DOT,
    ACTIONS(215), 1,
      anon_sym_POUND,
    ACTIONS(217), 1,
      anon_sym_PLUS,
    ACTIONS(219), 1,
      anon_sym_RPAREN,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(225), 1,
      anon_sym_SQUOTE,
    ACTIONS(227), 1,
      anon_sym_DQUOTE,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(233), 1,
      anon_sym_LBRACK2,
    ACTIONS(235), 1,
      sym_identifier,
    ACTIONS(237), 1,
      sym_plain_value,
    STATE(38), 1,
      sym_string_value,
    STATE(93), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 8,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
    STATE(208), 12,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
  [1043] = 27,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(213), 1,
      anon_sym_DOT,
    ACTIONS(215), 1,
      anon_sym_POUND,
    ACTIONS(217), 1,
      anon_sym_PLUS,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(225), 1,
      anon_sym_SQUOTE,
    ACTIONS(227), 1,
      anon_sym_DQUOTE,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(233), 1,
      anon_sym_LBRACK2,
    ACTIONS(235), 1,
      sym_identifier,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(239), 1,
      sym_nesting_selector,
    ACTIONS(241), 1,
      anon_sym_RPAREN,
    STATE(38), 1,
      sym_string_value,
    STATE(47), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 8,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
    STATE(205), 12,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
  [1143] = 26,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(213), 1,
      anon_sym_DOT,
    ACTIONS(215), 1,
      anon_sym_POUND,
    ACTIONS(217), 1,
      anon_sym_PLUS,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(225), 1,
      anon_sym_SQUOTE,
    ACTIONS(227), 1,
      anon_sym_DQUOTE,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(233), 1,
      anon_sym_LBRACK2,
    ACTIONS(235), 1,
      sym_identifier,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(243), 1,
      sym_nesting_selector,
    STATE(38), 1,
      sym_string_value,
    STATE(104), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 8,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
    STATE(216), 12,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
  [1240] = 20,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(245), 1,
      anon_sym_SEMI,
    ACTIONS(247), 1,
      anon_sym_LBRACE,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(253), 1,
      anon_sym_LPAREN2,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(263), 1,
      sym_identifier,
    STATE(67), 1,
      sym_block,
    STATE(108), 1,
      aux_sym_postcss_statement_repeat1,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(264), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [1315] = 17,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(265), 1,
      sym_nesting_selector,
    ACTIONS(267), 1,
      anon_sym_RPAREN,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(206), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [1380] = 16,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(273), 1,
      sym_important,
    ACTIONS(275), 1,
      anon_sym_LPAREN2,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(287), 1,
      sym_identifier,
    ACTIONS(289), 1,
      sym_plain_value,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(345), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
    STATE(291), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [1443] = 16,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(275), 1,
      anon_sym_LPAREN2,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(287), 1,
      sym_identifier,
    ACTIONS(291), 1,
      sym_important,
    ACTIONS(293), 1,
      sym_plain_value,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(345), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
    STATE(284), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [1506] = 19,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(295), 1,
      anon_sym_COMMA,
    ACTIONS(297), 1,
      anon_sym_SEMI,
    ACTIONS(299), 1,
      anon_sym_RBRACE,
    ACTIONS(301), 1,
      anon_sym_STAR,
    ACTIONS(305), 1,
      sym_important,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(309), 1,
      sym_plain_value,
    STATE(66), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(303), 3,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_SLASH,
    STATE(195), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [1574] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(311), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(136), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [1636] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(313), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(213), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [1698] = 17,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(315), 1,
      sym_nesting_selector,
    ACTIONS(317), 1,
      sym_identifier,
    STATE(191), 1,
      sym_namespace_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(233), 12,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_string_value,
  [1762] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(319), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(146), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [1824] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(321), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(144), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [1886] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(323), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(145), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [1948] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(325), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(219), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [2010] = 17,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(315), 1,
      sym_nesting_selector,
    ACTIONS(327), 1,
      sym_identifier,
    STATE(197), 1,
      sym_namespace_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(233), 12,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_string_value,
  [2074] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(329), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(147), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [2136] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(331), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(139), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [2198] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(333), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(138), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [2260] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(335), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(137), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [2322] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(337), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(226), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [2384] = 16,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_DOT,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_GT,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_PLUS,
    ACTIONS(39), 1,
      anon_sym_PIPE,
    ACTIONS(41), 1,
      anon_sym_SQUOTE,
    ACTIONS(43), 1,
      anon_sym_DQUOTE,
    ACTIONS(49), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(269), 1,
      sym_identifier,
    ACTIONS(339), 1,
      sym_nesting_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(141), 13,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
      sym_namespace_selector,
      sym_string_value,
  [2446] = 8,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(347), 1,
      anon_sym_LPAREN,
    STATE(181), 1,
      sym_arguments,
    ACTIONS(345), 2,
      anon_sym_DOT,
      anon_sym_PLUS,
    ACTIONS(343), 4,
      anon_sym_STAR,
      sym_important,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
    ACTIONS(349), 8,
      anon_sym_LPAREN2,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_LBRACK2,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(341), 10,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PIPE,
      anon_sym_RPAREN,
  [2491] = 18,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(295), 1,
      anon_sym_COMMA,
    ACTIONS(297), 1,
      anon_sym_SEMI,
    ACTIONS(301), 1,
      anon_sym_STAR,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(309), 1,
      sym_plain_value,
    ACTIONS(351), 1,
      sym_important,
    STATE(101), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(303), 3,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_SLASH,
    STATE(195), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [2556] = 18,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(295), 1,
      anon_sym_COMMA,
    ACTIONS(301), 1,
      anon_sym_STAR,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(309), 1,
      sym_plain_value,
    ACTIONS(353), 1,
      anon_sym_SEMI,
    ACTIONS(355), 1,
      sym_important,
    STATE(97), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(303), 3,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_SLASH,
    STATE(195), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [2621] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(359), 9,
      anon_sym_DOT,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_LBRACK2,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(357), 15,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_STAR,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
  [2656] = 17,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(363), 1,
      anon_sym_RPAREN,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(317), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(361), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [2717] = 6,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(369), 2,
      anon_sym_DOT,
      anon_sym_PLUS,
    ACTIONS(367), 5,
      anon_sym_STAR,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
    ACTIONS(371), 7,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_LBRACK2,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(365), 10,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PIPE,
      anon_sym_RPAREN,
  [2756] = 17,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(373), 1,
      anon_sym_RPAREN,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(297), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(361), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [2817] = 5,
    STATE(40), 1,
      aux_sym_class_name_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(377), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(379), 2,
      sym_escape_sequence,
      sym_identifier,
    ACTIONS(375), 19,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
  [2854] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(384), 9,
      anon_sym_DOT,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_LBRACK2,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(382), 15,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_STAR,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PIPE,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
  [2889] = 17,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(386), 1,
      anon_sym_RPAREN,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(307), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(361), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [2950] = 5,
    STATE(40), 1,
      aux_sym_class_name_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(390), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(392), 2,
      sym_escape_sequence,
      sym_identifier,
    ACTIONS(388), 19,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
  [2987] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(394), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(396), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3019] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(400), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(398), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3051] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(404), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(402), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3083] = 17,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(406), 1,
      anon_sym_COMMA,
    ACTIONS(408), 1,
      anon_sym_RPAREN,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(340), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [3143] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(412), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(410), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3175] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(414), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(416), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3207] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(414), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(416), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3239] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(420), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(418), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3271] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(424), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(422), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3303] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(428), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(426), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3335] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(432), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(430), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3367] = 15,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(436), 1,
      anon_sym_POUND,
    ACTIONS(439), 1,
      sym_important,
    ACTIONS(442), 1,
      anon_sym_LPAREN2,
    ACTIONS(445), 1,
      anon_sym_SQUOTE,
    ACTIONS(448), 1,
      anon_sym_DQUOTE,
    ACTIONS(451), 1,
      aux_sym_integer_value_token1,
    ACTIONS(454), 1,
      aux_sym_float_value_token1,
    ACTIONS(457), 1,
      anon_sym_LBRACK2,
    ACTIONS(460), 1,
      sym_identifier,
    ACTIONS(463), 1,
      sym_plain_value,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    ACTIONS(434), 3,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RPAREN,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [3423] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(468), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(466), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3455] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(472), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(470), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3487] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(476), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(474), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3519] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(480), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(478), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3551] = 16,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(482), 1,
      anon_sym_COMMA,
    ACTIONS(487), 1,
      anon_sym_POUND,
    ACTIONS(490), 1,
      sym_important,
    ACTIONS(493), 1,
      anon_sym_LPAREN2,
    ACTIONS(496), 1,
      anon_sym_SQUOTE,
    ACTIONS(499), 1,
      anon_sym_DQUOTE,
    ACTIONS(502), 1,
      aux_sym_integer_value_token1,
    ACTIONS(505), 1,
      aux_sym_float_value_token1,
    ACTIONS(508), 1,
      anon_sym_LBRACK2,
    ACTIONS(511), 1,
      sym_identifier,
    ACTIONS(514), 1,
      sym_plain_value,
    STATE(60), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(485), 2,
      anon_sym_SEMI,
      anon_sym_RBRACE,
    STATE(195), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [3609] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(519), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(517), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3641] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(521), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(523), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3673] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(527), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(525), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3705] = 15,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    ACTIONS(529), 3,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RPAREN,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [3761] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(533), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(531), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3793] = 17,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(295), 1,
      anon_sym_COMMA,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(309), 1,
      sym_plain_value,
    ACTIONS(535), 1,
      anon_sym_SEMI,
    ACTIONS(537), 1,
      anon_sym_RBRACE,
    ACTIONS(539), 1,
      sym_important,
    STATE(60), 1,
      aux_sym_declaration_repeat1,
    STATE(195), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [3853] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(414), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(416), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3885] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(543), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(541), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3917] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(424), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(422), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3949] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(432), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(430), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [3981] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(468), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(466), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4013] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(519), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(517), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4045] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(545), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(547), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4077] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(549), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(551), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4109] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(553), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(555), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4141] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(557), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(559), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4173] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(561), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(563), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4205] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(543), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(541), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4237] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(400), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(398), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4269] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(404), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(402), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4301] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(412), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(410), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4333] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(420), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(418), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4365] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(428), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(426), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4397] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(472), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(470), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4429] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(476), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(474), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4461] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(480), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(478), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4493] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(527), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(525), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4525] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(533), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(531), 15,
      sym__pseudo_class_selector_colon,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4557] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(545), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(547), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4589] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(549), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(551), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4621] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(394), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(396), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4653] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(553), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(555), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4685] = 17,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(406), 1,
      anon_sym_COMMA,
    ACTIONS(565), 1,
      anon_sym_RPAREN,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(335), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [4745] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(561), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(563), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4777] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(557), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
    ACTIONS(559), 15,
      sym__pseudo_class_selector_colon,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      sym_identifier,
  [4809] = 5,
    ACTIONS(571), 1,
      anon_sym_LPAREN,
    STATE(113), 1,
      sym_pseudo_class_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(569), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(567), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [4844] = 16,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(295), 1,
      anon_sym_COMMA,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(309), 1,
      sym_plain_value,
    ACTIONS(573), 1,
      anon_sym_SEMI,
    ACTIONS(575), 1,
      sym_important,
    STATE(60), 1,
      aux_sym_declaration_repeat1,
    STATE(195), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [4901] = 5,
    ACTIONS(571), 1,
      anon_sym_LPAREN,
    STATE(151), 1,
      sym_pseudo_class_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(579), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(577), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [4936] = 5,
    ACTIONS(585), 1,
      anon_sym_LPAREN,
    STATE(112), 1,
      sym_pseudo_class_with_selector_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(583), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(581), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [4971] = 5,
    ACTIONS(585), 1,
      anon_sym_LPAREN,
    STATE(118), 1,
      sym_pseudo_class_with_selector_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(589), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(587), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5006] = 16,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(295), 1,
      anon_sym_COMMA,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(309), 1,
      sym_plain_value,
    ACTIONS(535), 1,
      anon_sym_SEMI,
    ACTIONS(591), 1,
      sym_important,
    STATE(60), 1,
      aux_sym_declaration_repeat1,
    STATE(195), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [5063] = 5,
    ACTIONS(597), 1,
      anon_sym_LPAREN,
    STATE(114), 1,
      sym_pseudo_element_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(595), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(593), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5098] = 5,
    ACTIONS(597), 1,
      anon_sym_LPAREN,
    STATE(153), 1,
      sym_pseudo_element_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(601), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(599), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5133] = 15,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    ACTIONS(603), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [5188] = 15,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(605), 1,
      anon_sym_RPAREN,
    STATE(42), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [5242] = 15,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(607), 1,
      anon_sym_RPAREN,
    STATE(37), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [5296] = 8,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(347), 1,
      anon_sym_LPAREN,
    STATE(181), 1,
      sym_arguments,
    ACTIONS(611), 2,
      anon_sym_and,
      anon_sym_or,
    ACTIONS(609), 3,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
    ACTIONS(343), 6,
      anon_sym_STAR,
      anon_sym_POUND,
      sym_important,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
    ACTIONS(349), 8,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
  [5336] = 15,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(613), 1,
      anon_sym_SEMI,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [5390] = 15,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(615), 1,
      anon_sym_RPAREN,
    STATE(39), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [5444] = 15,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(617), 1,
      anon_sym_RPAREN,
    STATE(55), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [5498] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(345), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(341), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5527] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(621), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(619), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5556] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(625), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(623), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5585] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(629), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(627), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5614] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(633), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(631), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5643] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(637), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(635), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5672] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(641), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(639), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5701] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(645), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(643), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5730] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(649), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(647), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5759] = 14,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    STATE(64), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [5810] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(653), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(651), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5839] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(657), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(655), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5868] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(661), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(659), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5897] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(665), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(663), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5926] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(669), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(667), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5955] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(673), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(671), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5984] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(677), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(675), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6013] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(681), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(679), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6042] = 14,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(221), 1,
      sym_important,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(237), 1,
      sym_plain_value,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    STATE(110), 1,
      aux_sym_postcss_statement_repeat1,
    STATE(196), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [6093] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(685), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(683), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6122] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(689), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(687), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6151] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(693), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(691), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6180] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(697), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(695), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6209] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(701), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(699), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6238] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(359), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(357), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6267] = 4,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(707), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(703), 17,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6298] = 4,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(711), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(709), 17,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6329] = 4,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(715), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(713), 17,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6360] = 4,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(719), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(717), 17,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6391] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(384), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(382), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6420] = 4,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(723), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(721), 17,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6451] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(727), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(725), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6480] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(731), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(729), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6509] = 4,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(735), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(733), 17,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6540] = 4,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(739), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(737), 17,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6571] = 4,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(743), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(741), 17,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6602] = 4,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(747), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(745), 17,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6633] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(751), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(749), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6662] = 6,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(347), 1,
      anon_sym_LPAREN,
    STATE(181), 1,
      sym_arguments,
    ACTIONS(349), 8,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(343), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [6697] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(755), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(753), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6726] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(569), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(567), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6755] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(579), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(577), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6784] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(759), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(757), 18,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
      anon_sym_RBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6813] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(761), 1,
      sym_important,
    ACTIONS(763), 1,
      sym_plain_value,
    STATE(189), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [6861] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(765), 1,
      sym_important,
    ACTIONS(767), 1,
      sym_plain_value,
    STATE(18), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [6909] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(769), 1,
      sym_important,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(775), 1,
      sym_plain_value,
    STATE(282), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [6957] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(777), 1,
      sym_important,
    ACTIONS(779), 1,
      sym_plain_value,
    STATE(294), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7005] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(781), 1,
      anon_sym_POUND,
    ACTIONS(783), 1,
      sym_important,
    ACTIONS(785), 1,
      anon_sym_LPAREN2,
    ACTIONS(787), 1,
      anon_sym_SQUOTE,
    ACTIONS(789), 1,
      anon_sym_DQUOTE,
    ACTIONS(791), 1,
      aux_sym_integer_value_token1,
    ACTIONS(793), 1,
      aux_sym_float_value_token1,
    ACTIONS(795), 1,
      anon_sym_LBRACK2,
    ACTIONS(797), 1,
      sym_identifier,
    ACTIONS(799), 1,
      sym_plain_value,
    STATE(232), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7053] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(801), 1,
      sym_important,
    ACTIONS(803), 1,
      sym_plain_value,
    STATE(254), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7101] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(805), 1,
      sym_important,
    ACTIONS(807), 1,
      sym_plain_value,
    STATE(35), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7149] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(813), 1,
      sym_unit,
    ACTIONS(811), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(809), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [7181] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(819), 1,
      sym_unit,
    ACTIONS(817), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(815), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [7213] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(821), 1,
      sym_important,
    ACTIONS(823), 1,
      sym_plain_value,
    STATE(295), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7261] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(825), 1,
      sym_important,
    ACTIONS(827), 1,
      sym_plain_value,
    STATE(184), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7309] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(829), 1,
      sym_important,
    ACTIONS(831), 1,
      sym_plain_value,
    STATE(285), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7357] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(781), 1,
      anon_sym_POUND,
    ACTIONS(785), 1,
      anon_sym_LPAREN2,
    ACTIONS(787), 1,
      anon_sym_SQUOTE,
    ACTIONS(789), 1,
      anon_sym_DQUOTE,
    ACTIONS(791), 1,
      aux_sym_integer_value_token1,
    ACTIONS(793), 1,
      aux_sym_float_value_token1,
    ACTIONS(795), 1,
      anon_sym_LBRACK2,
    ACTIONS(797), 1,
      sym_identifier,
    ACTIONS(833), 1,
      sym_important,
    ACTIONS(835), 1,
      sym_plain_value,
    STATE(203), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7405] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(837), 1,
      sym_important,
    ACTIONS(839), 1,
      sym_plain_value,
    STATE(276), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7453] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(841), 1,
      sym_important,
    ACTIONS(843), 1,
      sym_plain_value,
    STATE(267), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7501] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(781), 1,
      anon_sym_POUND,
    ACTIONS(785), 1,
      anon_sym_LPAREN2,
    ACTIONS(787), 1,
      anon_sym_SQUOTE,
    ACTIONS(789), 1,
      anon_sym_DQUOTE,
    ACTIONS(791), 1,
      aux_sym_integer_value_token1,
    ACTIONS(793), 1,
      aux_sym_float_value_token1,
    ACTIONS(795), 1,
      anon_sym_LBRACK2,
    ACTIONS(797), 1,
      sym_identifier,
    ACTIONS(845), 1,
      sym_important,
    ACTIONS(847), 1,
      sym_plain_value,
    STATE(200), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7549] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(849), 1,
      sym_important,
    ACTIONS(851), 1,
      sym_plain_value,
    STATE(281), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7597] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(291), 1,
      sym_important,
    ACTIONS(293), 1,
      sym_plain_value,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    STATE(284), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7645] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(853), 1,
      sym_important,
    ACTIONS(855), 1,
      sym_plain_value,
    STATE(265), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7693] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_LPAREN2,
    ACTIONS(229), 1,
      aux_sym_integer_value_token1,
    ACTIONS(231), 1,
      aux_sym_float_value_token1,
    ACTIONS(251), 1,
      anon_sym_POUND,
    ACTIONS(257), 1,
      anon_sym_SQUOTE,
    ACTIONS(259), 1,
      anon_sym_DQUOTE,
    ACTIONS(261), 1,
      anon_sym_LBRACK2,
    ACTIONS(307), 1,
      sym_identifier,
    ACTIONS(857), 1,
      sym_important,
    ACTIONS(859), 1,
      sym_plain_value,
    STATE(34), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7741] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(273), 1,
      sym_important,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(289), 1,
      sym_plain_value,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    STATE(291), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7789] = 13,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(271), 1,
      anon_sym_POUND,
    ACTIONS(277), 1,
      anon_sym_SQUOTE,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    ACTIONS(281), 1,
      aux_sym_integer_value_token1,
    ACTIONS(283), 1,
      aux_sym_float_value_token1,
    ACTIONS(285), 1,
      anon_sym_LBRACK2,
    ACTIONS(771), 1,
      anon_sym_LPAREN2,
    ACTIONS(773), 1,
      sym_identifier,
    ACTIONS(861), 1,
      sym_important,
    ACTIONS(863), 1,
      sym_plain_value,
    STATE(268), 9,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_string_value,
      sym_integer_value,
      sym_float_value,
      sym_grid_value,
      sym_call_expression,
      sym_binary_expression,
  [7837] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(867), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(865), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [7866] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(359), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(357), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [7895] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(871), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(869), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [7924] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(875), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(873), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [7953] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(879), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(877), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [7982] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(883), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(881), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8011] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(887), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(885), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8040] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(891), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(889), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8069] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(895), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(893), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8098] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(899), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(897), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8127] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(903), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(901), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8156] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(907), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(905), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8185] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(384), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(382), 11,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8214] = 6,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(301), 1,
      anon_sym_STAR,
    ACTIONS(303), 3,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_SLASH,
    ACTIONS(909), 4,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_identifier,
      sym_plain_value,
    ACTIONS(485), 9,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8246] = 5,
    ACTIONS(913), 1,
      anon_sym_RBRACK,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(345), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(911), 6,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
    ACTIONS(341), 8,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
  [8276] = 5,
    ACTIONS(913), 1,
      anon_sym_RBRACK,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(369), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(911), 6,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
    ACTIONS(365), 8,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
  [8306] = 6,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(535), 1,
      anon_sym_SEMI,
    ACTIONS(537), 1,
      anon_sym_RBRACE,
    ACTIONS(371), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(367), 8,
      anon_sym_COMMA,
      anon_sym_STAR,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8338] = 5,
    ACTIONS(917), 1,
      anon_sym_RBRACK,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(345), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(915), 6,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
    ACTIONS(341), 8,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
  [8368] = 6,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(919), 1,
      anon_sym_SEMI,
    ACTIONS(921), 1,
      anon_sym_RBRACE,
    ACTIONS(371), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(367), 8,
      anon_sym_COMMA,
      anon_sym_STAR,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8400] = 6,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(301), 1,
      anon_sym_STAR,
    ACTIONS(303), 3,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_SLASH,
    ACTIONS(925), 4,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_identifier,
      sym_plain_value,
    ACTIONS(923), 9,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8432] = 6,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(301), 1,
      anon_sym_STAR,
    ACTIONS(303), 3,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_SLASH,
    ACTIONS(929), 4,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_identifier,
      sym_plain_value,
    ACTIONS(927), 9,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8464] = 5,
    ACTIONS(917), 1,
      anon_sym_RBRACK,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(369), 2,
      anon_sym_TILDE,
      anon_sym_PIPE,
    ACTIONS(915), 6,
      anon_sym_EQ,
      anon_sym_TILDE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_STAR_EQ,
      anon_sym_DOLLAR_EQ,
    ACTIONS(365), 8,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_PLUS,
  [8494] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(931), 1,
      anon_sym_SEMI,
    ACTIONS(371), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(367), 8,
      anon_sym_COMMA,
      anon_sym_STAR,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8523] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(573), 1,
      anon_sym_SEMI,
    ACTIONS(371), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(367), 8,
      anon_sym_COMMA,
      anon_sym_STAR,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8552] = 10,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(933), 1,
      anon_sym_SEMI,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    ACTIONS(935), 2,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(939), 2,
      anon_sym_DASH,
      anon_sym_SLASH,
    STATE(286), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [8591] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(535), 1,
      anon_sym_SEMI,
    ACTIONS(371), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(367), 8,
      anon_sym_COMMA,
      anon_sym_STAR,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8620] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(919), 1,
      anon_sym_SEMI,
    ACTIONS(371), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
      sym_plain_value,
    ACTIONS(367), 8,
      anon_sym_COMMA,
      anon_sym_STAR,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      anon_sym_SQUOTE,
      anon_sym_DQUOTE,
      anon_sym_LBRACK2,
  [8649] = 10,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(943), 1,
      anon_sym_SEMI,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    ACTIONS(935), 2,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(939), 2,
      anon_sym_DASH,
      anon_sym_SLASH,
    STATE(292), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [8688] = 9,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(945), 1,
      anon_sym_SEMI,
    ACTIONS(947), 1,
      anon_sym_LBRACE,
    STATE(50), 1,
      sym_block,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(261), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [8723] = 14,
    ACTIONS(406), 1,
      anon_sym_COMMA,
    ACTIONS(408), 1,
      anon_sym_RPAREN,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(949), 1,
      anon_sym_COLON_COLON,
    ACTIONS(951), 1,
      anon_sym_POUND,
    ACTIONS(953), 1,
      anon_sym_LBRACK,
    ACTIONS(955), 1,
      anon_sym_GT,
    ACTIONS(957), 1,
      anon_sym_TILDE,
    ACTIONS(959), 1,
      anon_sym_PLUS,
    ACTIONS(961), 1,
      anon_sym_PIPE,
    ACTIONS(963), 1,
      sym__descendant_operator,
    ACTIONS(965), 1,
      sym__pseudo_class_selector_colon,
    STATE(340), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [8767] = 14,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(949), 1,
      anon_sym_COLON_COLON,
    ACTIONS(951), 1,
      anon_sym_POUND,
    ACTIONS(953), 1,
      anon_sym_LBRACK,
    ACTIONS(955), 1,
      anon_sym_GT,
    ACTIONS(957), 1,
      anon_sym_TILDE,
    ACTIONS(959), 1,
      anon_sym_PLUS,
    ACTIONS(961), 1,
      anon_sym_PIPE,
    ACTIONS(963), 1,
      sym__descendant_operator,
    ACTIONS(965), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(967), 1,
      anon_sym_COMMA,
    ACTIONS(969), 1,
      anon_sym_RPAREN,
    STATE(343), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [8811] = 14,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(949), 1,
      anon_sym_COLON_COLON,
    ACTIONS(951), 1,
      anon_sym_POUND,
    ACTIONS(953), 1,
      anon_sym_LBRACK,
    ACTIONS(955), 1,
      anon_sym_GT,
    ACTIONS(957), 1,
      anon_sym_TILDE,
    ACTIONS(959), 1,
      anon_sym_PLUS,
    ACTIONS(961), 1,
      anon_sym_PIPE,
    ACTIONS(963), 1,
      sym__descendant_operator,
    ACTIONS(965), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(967), 1,
      anon_sym_COMMA,
    ACTIONS(971), 1,
      anon_sym_LBRACE,
    STATE(344), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [8855] = 14,
    ACTIONS(406), 1,
      anon_sym_COMMA,
    ACTIONS(565), 1,
      anon_sym_RPAREN,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(949), 1,
      anon_sym_COLON_COLON,
    ACTIONS(951), 1,
      anon_sym_POUND,
    ACTIONS(953), 1,
      anon_sym_LBRACK,
    ACTIONS(955), 1,
      anon_sym_GT,
    ACTIONS(957), 1,
      anon_sym_TILDE,
    ACTIONS(959), 1,
      anon_sym_PLUS,
    ACTIONS(961), 1,
      anon_sym_PIPE,
    ACTIONS(963), 1,
      sym__descendant_operator,
    ACTIONS(965), 1,
      sym__pseudo_class_selector_colon,
    STATE(335), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [8899] = 3,
    ACTIONS(973), 1,
      anon_sym_COLON,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(341), 12,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
  [8921] = 3,
    ACTIONS(975), 1,
      anon_sym_COLON,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(341), 12,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
  [8943] = 9,
    ACTIONS(979), 1,
      anon_sym_host,
    ACTIONS(983), 1,
      sym_escape_sequence,
    ACTIONS(985), 1,
      sym_identifier,
    STATE(43), 1,
      aux_sym_class_name_repeat1,
    STATE(98), 1,
      sym_class_name,
    STATE(152), 1,
      sym__nth_child_pseudo_class_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(981), 2,
      anon_sym_nth_DASHchild,
      anon_sym_nth_DASHlast_DASHchild,
    ACTIONS(977), 5,
      anon_sym_has,
      anon_sym_not,
      anon_sym_is,
      anon_sym_where,
      anon_sym_host_DASHcontext,
  [8977] = 9,
    ACTIONS(983), 1,
      sym_escape_sequence,
    ACTIONS(985), 1,
      sym_identifier,
    ACTIONS(989), 1,
      anon_sym_host,
    STATE(43), 1,
      aux_sym_class_name_repeat1,
    STATE(96), 1,
      sym_class_name,
    STATE(151), 1,
      sym__nth_child_pseudo_class_selector,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(981), 2,
      anon_sym_nth_DASHchild,
      anon_sym_nth_DASHlast_DASHchild,
    ACTIONS(987), 5,
      anon_sym_has,
      anon_sym_not,
      anon_sym_is,
      anon_sym_where,
      anon_sym_host_DASHcontext,
  [9011] = 12,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(949), 1,
      anon_sym_COLON_COLON,
    ACTIONS(951), 1,
      anon_sym_POUND,
    ACTIONS(953), 1,
      anon_sym_LBRACK,
    ACTIONS(955), 1,
      anon_sym_GT,
    ACTIONS(957), 1,
      anon_sym_TILDE,
    ACTIONS(959), 1,
      anon_sym_PLUS,
    ACTIONS(961), 1,
      anon_sym_PIPE,
    ACTIONS(963), 1,
      sym__descendant_operator,
    ACTIONS(965), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(991), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
  [9051] = 3,
    ACTIONS(993), 1,
      anon_sym_COLON,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(341), 12,
      sym__descendant_operator,
      sym__pseudo_class_selector_colon,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_PIPE,
  [9073] = 6,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(995), 1,
      anon_sym_LPAREN,
    STATE(242), 1,
      sym_arguments,
    ACTIONS(343), 3,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
    ACTIONS(349), 7,
      anon_sym_not,
      anon_sym_LPAREN2,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9100] = 12,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(949), 1,
      anon_sym_COLON_COLON,
    ACTIONS(951), 1,
      anon_sym_POUND,
    ACTIONS(953), 1,
      anon_sym_LBRACK,
    ACTIONS(955), 1,
      anon_sym_GT,
    ACTIONS(957), 1,
      anon_sym_TILDE,
    ACTIONS(959), 1,
      anon_sym_PLUS,
    ACTIONS(961), 1,
      anon_sym_PIPE,
    ACTIONS(963), 1,
      sym__descendant_operator,
    ACTIONS(965), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(603), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [9139] = 6,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(312), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [9165] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(873), 5,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(875), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9187] = 12,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(949), 1,
      anon_sym_COLON_COLON,
    ACTIONS(951), 1,
      anon_sym_POUND,
    ACTIONS(953), 1,
      anon_sym_LBRACK,
    ACTIONS(955), 1,
      anon_sym_GT,
    ACTIONS(957), 1,
      anon_sym_TILDE,
    ACTIONS(959), 1,
      anon_sym_PLUS,
    ACTIONS(961), 1,
      anon_sym_PIPE,
    ACTIONS(963), 1,
      sym__descendant_operator,
    ACTIONS(965), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(997), 1,
      anon_sym_RPAREN,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [9225] = 6,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(278), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [9251] = 6,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(287), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [9277] = 6,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(304), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [9303] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(999), 1,
      sym_unit,
    ACTIONS(809), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(811), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9327] = 6,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(1001), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(345), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [9353] = 6,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(280), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [9379] = 12,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(949), 1,
      anon_sym_COLON_COLON,
    ACTIONS(951), 1,
      anon_sym_POUND,
    ACTIONS(953), 1,
      anon_sym_LBRACK,
    ACTIONS(955), 1,
      anon_sym_GT,
    ACTIONS(957), 1,
      anon_sym_TILDE,
    ACTIONS(959), 1,
      anon_sym_PLUS,
    ACTIONS(961), 1,
      anon_sym_PIPE,
    ACTIONS(963), 1,
      sym__descendant_operator,
    ACTIONS(965), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(1003), 1,
      anon_sym_RPAREN,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [9417] = 6,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(274), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [9443] = 6,
    ACTIONS(255), 1,
      anon_sym_selector,
    ACTIONS(937), 1,
      anon_sym_LPAREN2,
    ACTIONS(941), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(249), 2,
      anon_sym_not,
      anon_sym_only,
    STATE(275), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [9469] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1005), 1,
      sym_unit,
    ACTIONS(815), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(817), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9493] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(901), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(903), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9514] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(905), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(907), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9535] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(893), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(895), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9556] = 11,
    ACTIONS(705), 1,
      anon_sym_DOT,
    ACTIONS(949), 1,
      anon_sym_COLON_COLON,
    ACTIONS(951), 1,
      anon_sym_POUND,
    ACTIONS(953), 1,
      anon_sym_LBRACK,
    ACTIONS(955), 1,
      anon_sym_GT,
    ACTIONS(957), 1,
      anon_sym_TILDE,
    ACTIONS(959), 1,
      anon_sym_PLUS,
    ACTIONS(961), 1,
      anon_sym_PIPE,
    ACTIONS(963), 1,
      sym__descendant_operator,
    ACTIONS(965), 1,
      sym__pseudo_class_selector_colon,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [9591] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(357), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(359), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9612] = 6,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(349), 1,
      anon_sym_SLASH,
    ACTIONS(1007), 1,
      anon_sym_LPAREN,
    STATE(259), 1,
      sym_arguments,
    ACTIONS(343), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [9637] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(382), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(384), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9658] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(889), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(891), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9679] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(869), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(871), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9700] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(885), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(887), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9721] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(897), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(899), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9742] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(877), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(879), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9763] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(881), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(883), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9784] = 8,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(349), 1,
      anon_sym_SLASH,
    ACTIONS(1007), 1,
      anon_sym_LPAREN,
    ACTIONS(1009), 1,
      anon_sym_COLON,
    STATE(259), 1,
      sym_arguments,
    ACTIONS(343), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
    ACTIONS(609), 3,
      anon_sym_RPAREN,
      anon_sym_and,
      anon_sym_or,
  [9813] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(865), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
    ACTIONS(867), 6,
      anon_sym_not,
      anon_sym_only,
      anon_sym_selector,
      anon_sym_DASH,
      anon_sym_SLASH,
      sym_identifier,
  [9834] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(811), 1,
      anon_sym_SLASH,
    ACTIONS(1011), 1,
      sym_unit,
    ACTIONS(809), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [9856] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(817), 1,
      anon_sym_SLASH,
    ACTIONS(1013), 1,
      sym_unit,
    ACTIONS(815), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [9878] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(875), 1,
      anon_sym_SLASH,
    ACTIONS(873), 8,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_of,
      anon_sym_DASH,
  [9898] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(899), 1,
      anon_sym_SLASH,
    ACTIONS(897), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [9917] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(891), 1,
      anon_sym_SLASH,
    ACTIONS(889), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [9936] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(384), 1,
      anon_sym_SLASH,
    ACTIONS(382), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [9955] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(867), 1,
      anon_sym_SLASH,
    ACTIONS(865), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [9974] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(887), 1,
      anon_sym_SLASH,
    ACTIONS(885), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [9993] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(907), 1,
      anon_sym_SLASH,
    ACTIONS(905), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [10012] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(895), 1,
      anon_sym_SLASH,
    ACTIONS(893), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [10031] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(359), 1,
      anon_sym_SLASH,
    ACTIONS(357), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [10050] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(903), 1,
      anon_sym_SLASH,
    ACTIONS(901), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [10069] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(871), 1,
      anon_sym_SLASH,
    ACTIONS(869), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [10088] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(879), 1,
      anon_sym_SLASH,
    ACTIONS(877), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [10107] = 4,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(883), 1,
      anon_sym_SLASH,
    ACTIONS(881), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [10126] = 7,
    ACTIONS(787), 1,
      anon_sym_SQUOTE,
    ACTIONS(789), 1,
      anon_sym_DQUOTE,
    ACTIONS(995), 1,
      anon_sym_LPAREN,
    ACTIONS(1015), 1,
      sym_identifier,
    STATE(242), 1,
      sym_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(367), 2,
      sym_string_value,
      sym_call_expression,
  [10150] = 7,
    ACTIONS(947), 1,
      anon_sym_LBRACE,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    ACTIONS(1019), 1,
      anon_sym_SEMI,
    STATE(95), 1,
      sym_block,
    STATE(289), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
  [10174] = 6,
    ACTIONS(1023), 1,
      anon_sym_RBRACE,
    ACTIONS(1028), 1,
      aux_sym_integer_value_token1,
    STATE(351), 1,
      sym_integer_value,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1025), 2,
      sym_from,
      sym_to,
    STATE(262), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [10196] = 6,
    ACTIONS(1031), 1,
      anon_sym_RBRACE,
    ACTIONS(1035), 1,
      aux_sym_integer_value_token1,
    STATE(351), 1,
      sym_integer_value,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1033), 2,
      sym_from,
      sym_to,
    STATE(262), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [10218] = 7,
    ACTIONS(247), 1,
      anon_sym_LBRACE,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    ACTIONS(1037), 1,
      anon_sym_SEMI,
    STATE(76), 1,
      sym_block,
    STATE(288), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
  [10242] = 7,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1039), 1,
      anon_sym_COMMA,
    ACTIONS(1043), 1,
      anon_sym_RBRACK,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    STATE(347), 1,
      aux_sym_grid_value_repeat1,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10266] = 6,
    ACTIONS(1035), 1,
      aux_sym_integer_value_token1,
    ACTIONS(1047), 1,
      anon_sym_RBRACE,
    STATE(351), 1,
      sym_integer_value,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1033), 2,
      sym_from,
      sym_to,
    STATE(263), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [10288] = 7,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1039), 1,
      anon_sym_COMMA,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1049), 1,
      anon_sym_RBRACK,
    STATE(342), 1,
      aux_sym_grid_value_repeat1,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10312] = 7,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1039), 1,
      anon_sym_COMMA,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1051), 1,
      anon_sym_RBRACK,
    STATE(341), 1,
      aux_sym_grid_value_repeat1,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10336] = 6,
    ACTIONS(1035), 1,
      aux_sym_integer_value_token1,
    ACTIONS(1053), 1,
      anon_sym_RBRACE,
    STATE(351), 1,
      sym_integer_value,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1033), 2,
      sym_from,
      sym_to,
    STATE(271), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [10358] = 7,
    ACTIONS(787), 1,
      anon_sym_SQUOTE,
    ACTIONS(789), 1,
      anon_sym_DQUOTE,
    ACTIONS(995), 1,
      anon_sym_LPAREN,
    ACTIONS(1015), 1,
      sym_identifier,
    STATE(242), 1,
      sym_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(370), 2,
      sym_string_value,
      sym_call_expression,
  [10382] = 6,
    ACTIONS(1035), 1,
      aux_sym_integer_value_token1,
    ACTIONS(1055), 1,
      anon_sym_RBRACE,
    STATE(351), 1,
      sym_integer_value,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1033), 2,
      sym_from,
      sym_to,
    STATE(262), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [10404] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1057), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      anon_sym_and,
      anon_sym_or,
  [10417] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1059), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      anon_sym_and,
      anon_sym_or,
  [10430] = 6,
    ACTIONS(947), 1,
      anon_sym_LBRACE,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    STATE(54), 1,
      sym_block,
    STATE(308), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
  [10451] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1061), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      anon_sym_and,
      anon_sym_or,
  [10464] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1063), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10483] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(609), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      anon_sym_and,
      anon_sym_or,
  [10496] = 6,
    ACTIONS(247), 1,
      anon_sym_LBRACE,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    STATE(70), 1,
      sym_block,
    STATE(299), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
  [10517] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1065), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      anon_sym_and,
      anon_sym_or,
  [10530] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1067), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      anon_sym_and,
      anon_sym_or,
  [10543] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1069), 1,
      anon_sym_SEMI,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10561] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1071), 1,
      anon_sym_RPAREN,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10579] = 5,
    ACTIONS(787), 1,
      anon_sym_SQUOTE,
    ACTIONS(789), 1,
      anon_sym_DQUOTE,
    ACTIONS(1073), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(366), 2,
      sym_string_value,
      sym_call_expression,
  [10597] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1075), 1,
      anon_sym_RPAREN,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10615] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1077), 1,
      anon_sym_SEMI,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10633] = 5,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    ACTIONS(1079), 1,
      anon_sym_SEMI,
    STATE(336), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
  [10651] = 3,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
    ACTIONS(1081), 3,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
  [10665] = 6,
    ACTIONS(247), 1,
      anon_sym_LBRACE,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    ACTIONS(1083), 1,
      anon_sym_SEMI,
    STATE(83), 1,
      sym_block,
    STATE(300), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [10685] = 6,
    ACTIONS(947), 1,
      anon_sym_LBRACE,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    ACTIONS(1085), 1,
      anon_sym_SEMI,
    STATE(53), 1,
      sym_block,
    STATE(300), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [10705] = 4,
    ACTIONS(1089), 1,
      aux_sym_integer_value_token1,
    STATE(348), 1,
      sym_integer_value,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1087), 3,
      anon_sym_even,
      anon_sym_odd,
      sym__nth_functional_notation,
  [10721] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1091), 1,
      anon_sym_RPAREN,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10739] = 5,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    ACTIONS(1093), 1,
      anon_sym_SEMI,
    STATE(339), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
  [10757] = 5,
    ACTIONS(787), 1,
      anon_sym_SQUOTE,
    ACTIONS(789), 1,
      anon_sym_DQUOTE,
    ACTIONS(1095), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    STATE(361), 2,
      sym_string_value,
      sym_call_expression,
  [10775] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1097), 1,
      anon_sym_RBRACK,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10793] = 5,
    ACTIONS(3), 1,
      sym_js_comment,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(1045), 1,
      anon_sym_SLASH,
    ACTIONS(1099), 1,
      anon_sym_RBRACK,
    ACTIONS(1041), 3,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_DASH,
  [10811] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1101), 4,
      anon_sym_RBRACE,
      sym_from,
      sym_to,
      aux_sym_integer_value_token1,
  [10822] = 4,
    ACTIONS(1103), 1,
      anon_sym_RPAREN,
    STATE(301), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(361), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
  [10837] = 4,
    ACTIONS(1105), 1,
      anon_sym_DQUOTE,
    STATE(326), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1107), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [10852] = 5,
    ACTIONS(247), 1,
      anon_sym_LBRACE,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    STATE(78), 1,
      sym_block,
    STATE(300), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [10869] = 4,
    ACTIONS(1109), 1,
      anon_sym_COMMA,
    STATE(300), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1081), 2,
      anon_sym_SEMI,
      anon_sym_LBRACE,
  [10884] = 4,
    ACTIONS(529), 1,
      anon_sym_RPAREN,
    STATE(301), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1112), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
  [10899] = 4,
    STATE(43), 1,
      aux_sym_class_name_repeat1,
    STATE(142), 1,
      sym_class_name,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(983), 2,
      sym_escape_sequence,
      sym_identifier,
  [10914] = 4,
    ACTIONS(1115), 1,
      anon_sym_SQUOTE,
    STATE(313), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1117), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [10929] = 4,
    ACTIONS(247), 1,
      anon_sym_LBRACE,
    STATE(74), 1,
      sym_block,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
  [10944] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(525), 4,
      anon_sym_RBRACE,
      sym_from,
      sym_to,
      aux_sym_integer_value_token1,
  [10955] = 4,
    ACTIONS(1119), 1,
      anon_sym_COMMA,
    STATE(306), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(991), 2,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
  [10970] = 4,
    ACTIONS(1122), 1,
      anon_sym_RPAREN,
    STATE(301), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(361), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
  [10985] = 5,
    ACTIONS(947), 1,
      anon_sym_LBRACE,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    STATE(68), 1,
      sym_block,
    STATE(300), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11002] = 4,
    ACTIONS(1124), 1,
      anon_sym_SQUOTE,
    STATE(311), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1126), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11017] = 4,
    ACTIONS(1124), 1,
      anon_sym_DQUOTE,
    STATE(334), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1128), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11032] = 4,
    ACTIONS(1130), 1,
      anon_sym_SQUOTE,
    STATE(331), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1132), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11047] = 4,
    ACTIONS(947), 1,
      anon_sym_LBRACE,
    STATE(90), 1,
      sym_block,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
  [11062] = 4,
    ACTIONS(1134), 1,
      anon_sym_SQUOTE,
    STATE(331), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1132), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11077] = 4,
    ACTIONS(1136), 1,
      anon_sym_SQUOTE,
    STATE(331), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1132), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11092] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(555), 4,
      anon_sym_RBRACE,
      sym_from,
      sym_to,
      aux_sym_integer_value_token1,
  [11103] = 4,
    ACTIONS(1134), 1,
      anon_sym_DQUOTE,
    STATE(332), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1138), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11118] = 4,
    ACTIONS(1140), 1,
      anon_sym_RPAREN,
    STATE(301), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(361), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
  [11133] = 4,
    ACTIONS(1142), 1,
      anon_sym_SQUOTE,
    STATE(320), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1144), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11148] = 4,
    ACTIONS(1142), 1,
      anon_sym_DQUOTE,
    STATE(321), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1146), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11163] = 4,
    ACTIONS(1148), 1,
      anon_sym_SQUOTE,
    STATE(331), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1132), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11178] = 4,
    ACTIONS(1148), 1,
      anon_sym_DQUOTE,
    STATE(332), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1138), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11193] = 4,
    ACTIONS(1150), 1,
      anon_sym_SQUOTE,
    STATE(324), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1152), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11208] = 4,
    ACTIONS(1150), 1,
      anon_sym_DQUOTE,
    STATE(325), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1154), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11223] = 4,
    ACTIONS(1156), 1,
      anon_sym_SQUOTE,
    STATE(331), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1132), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11238] = 4,
    ACTIONS(1156), 1,
      anon_sym_DQUOTE,
    STATE(332), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1138), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11253] = 4,
    ACTIONS(1136), 1,
      anon_sym_DQUOTE,
    STATE(332), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1138), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11268] = 4,
    STATE(43), 1,
      aux_sym_class_name_repeat1,
    STATE(124), 1,
      sym_class_name,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(983), 2,
      sym_escape_sequence,
      sym_identifier,
  [11283] = 4,
    ACTIONS(1115), 1,
      anon_sym_DQUOTE,
    STATE(316), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1158), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11298] = 3,
    ACTIONS(1009), 1,
      anon_sym_COLON,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(609), 3,
      anon_sym_RPAREN,
      anon_sym_and,
      anon_sym_or,
  [11311] = 2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(418), 4,
      anon_sym_RBRACE,
      sym_from,
      sym_to,
      aux_sym_integer_value_token1,
  [11322] = 4,
    ACTIONS(1160), 1,
      anon_sym_SQUOTE,
    STATE(331), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1162), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11337] = 4,
    ACTIONS(1165), 1,
      anon_sym_DQUOTE,
    STATE(332), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1167), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11352] = 4,
    ACTIONS(1105), 1,
      anon_sym_SQUOTE,
    STATE(314), 1,
      aux_sym_string_value_repeat1,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1170), 2,
      aux_sym_string_value_token1,
      sym_escape_sequence,
  [11367] = 4,
    ACTIONS(1130), 1,
      anon_sym_DQUOTE,
    STATE(332), 1,
      aux_sym_string_value_repeat2,
    ACTIONS(3), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1138), 2,
      aux_sym_string_value_token2,
      sym_escape_sequence,
  [11382] = 4,
    ACTIONS(406), 1,
      anon_sym_COMMA,
    ACTIONS(1172), 1,
      anon_sym_RPAREN,
    STATE(337), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11396] = 4,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    ACTIONS(1174), 1,
      anon_sym_SEMI,
    STATE(300), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11410] = 4,
    ACTIONS(603), 1,
      anon_sym_RPAREN,
    ACTIONS(1176), 1,
      anon_sym_COMMA,
    STATE(337), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11424] = 4,
    ACTIONS(1063), 1,
      anon_sym_RBRACK,
    ACTIONS(1179), 1,
      anon_sym_COMMA,
    STATE(338), 1,
      aux_sym_grid_value_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11438] = 4,
    ACTIONS(1017), 1,
      anon_sym_COMMA,
    ACTIONS(1182), 1,
      anon_sym_SEMI,
    STATE(300), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11452] = 4,
    ACTIONS(406), 1,
      anon_sym_COMMA,
    ACTIONS(1184), 1,
      anon_sym_RPAREN,
    STATE(337), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11466] = 4,
    ACTIONS(1039), 1,
      anon_sym_COMMA,
    ACTIONS(1186), 1,
      anon_sym_RBRACK,
    STATE(338), 1,
      aux_sym_grid_value_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11480] = 4,
    ACTIONS(1039), 1,
      anon_sym_COMMA,
    ACTIONS(1188), 1,
      anon_sym_RBRACK,
    STATE(338), 1,
      aux_sym_grid_value_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11494] = 4,
    ACTIONS(967), 1,
      anon_sym_COMMA,
    ACTIONS(1190), 1,
      anon_sym_RPAREN,
    STATE(306), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11508] = 4,
    ACTIONS(967), 1,
      anon_sym_COMMA,
    ACTIONS(1192), 1,
      anon_sym_LBRACE,
    STATE(306), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11522] = 3,
    ACTIONS(1194), 1,
      anon_sym_RPAREN,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
    ACTIONS(1021), 2,
      anon_sym_and,
      anon_sym_or,
  [11534] = 4,
    ACTIONS(809), 1,
      anon_sym_RPAREN,
    ACTIONS(811), 1,
      anon_sym_of,
    ACTIONS(1196), 1,
      sym_unit,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11548] = 4,
    ACTIONS(1039), 1,
      anon_sym_COMMA,
    ACTIONS(1198), 1,
      anon_sym_RBRACK,
    STATE(338), 1,
      aux_sym_grid_value_repeat1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11562] = 3,
    ACTIONS(1200), 1,
      anon_sym_RPAREN,
    ACTIONS(1202), 1,
      anon_sym_of,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11573] = 3,
    ACTIONS(585), 1,
      anon_sym_LPAREN,
    STATE(112), 1,
      sym_pseudo_class_with_selector_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11584] = 3,
    ACTIONS(995), 1,
      anon_sym_LPAREN,
    STATE(242), 1,
      sym_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11595] = 3,
    ACTIONS(1204), 1,
      anon_sym_LBRACE,
    STATE(296), 1,
      sym_block,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11606] = 3,
    ACTIONS(809), 1,
      anon_sym_LBRACE,
    ACTIONS(1206), 1,
      sym_unit,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11617] = 3,
    ACTIONS(1208), 1,
      anon_sym_LBRACE,
    STATE(73), 1,
      sym_keyframe_block_list,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11628] = 3,
    ACTIONS(947), 1,
      anon_sym_LBRACE,
    STATE(91), 1,
      sym_block,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11639] = 3,
    ACTIONS(1210), 1,
      anon_sym_LPAREN,
    STATE(121), 1,
      sym_pseudo_class_nth_child_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11650] = 3,
    ACTIONS(247), 1,
      anon_sym_LBRACE,
    STATE(44), 1,
      sym_block,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11661] = 3,
    ACTIONS(1212), 1,
      aux_sym_color_value_token1,
    ACTIONS(1214), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11672] = 3,
    ACTIONS(1216), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_keyframe_block_list,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11683] = 3,
    ACTIONS(585), 1,
      anon_sym_LPAREN,
    STATE(118), 1,
      sym_pseudo_class_with_selector_arguments,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11694] = 2,
    ACTIONS(61), 1,
      anon_sym_RBRACE,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11702] = 2,
    ACTIONS(1218), 1,
      anon_sym_SEMI,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11710] = 2,
    ACTIONS(1220), 1,
      anon_sym_RBRACE,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11718] = 2,
    ACTIONS(1222), 1,
      ts_builtin_sym_end,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11726] = 2,
    ACTIONS(1224), 1,
      aux_sym_color_value_token1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11734] = 2,
    ACTIONS(1226), 1,
      anon_sym_RBRACE,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11742] = 2,
    ACTIONS(1228), 1,
      anon_sym_SEMI,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11750] = 2,
    ACTIONS(1230), 1,
      anon_sym_SEMI,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11758] = 2,
    ACTIONS(1232), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11766] = 2,
    ACTIONS(1234), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11774] = 2,
    ACTIONS(1236), 1,
      anon_sym_SEMI,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11782] = 2,
    ACTIONS(1238), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11790] = 2,
    ACTIONS(1240), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11798] = 2,
    ACTIONS(1242), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11806] = 2,
    ACTIONS(77), 1,
      anon_sym_RBRACE,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11814] = 2,
    ACTIONS(1244), 1,
      aux_sym_color_value_token1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11822] = 2,
    ACTIONS(1246), 1,
      anon_sym_RBRACE,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11830] = 2,
    ACTIONS(69), 1,
      anon_sym_RBRACE,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11838] = 2,
    ACTIONS(1248), 1,
      aux_sym_color_value_token1,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11846] = 2,
    ACTIONS(1250), 1,
      anon_sym_LPAREN2,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
  [11854] = 2,
    ACTIONS(1252), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_js_comment,
      sym_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 106,
  [SMALL_STATE(4)] = 212,
  [SMALL_STATE(5)] = 318,
  [SMALL_STATE(6)] = 424,
  [SMALL_STATE(7)] = 530,
  [SMALL_STATE(8)] = 636,
  [SMALL_STATE(9)] = 739,
  [SMALL_STATE(10)] = 841,
  [SMALL_STATE(11)] = 943,
  [SMALL_STATE(12)] = 1043,
  [SMALL_STATE(13)] = 1143,
  [SMALL_STATE(14)] = 1240,
  [SMALL_STATE(15)] = 1315,
  [SMALL_STATE(16)] = 1380,
  [SMALL_STATE(17)] = 1443,
  [SMALL_STATE(18)] = 1506,
  [SMALL_STATE(19)] = 1574,
  [SMALL_STATE(20)] = 1636,
  [SMALL_STATE(21)] = 1698,
  [SMALL_STATE(22)] = 1762,
  [SMALL_STATE(23)] = 1824,
  [SMALL_STATE(24)] = 1886,
  [SMALL_STATE(25)] = 1948,
  [SMALL_STATE(26)] = 2010,
  [SMALL_STATE(27)] = 2074,
  [SMALL_STATE(28)] = 2136,
  [SMALL_STATE(29)] = 2198,
  [SMALL_STATE(30)] = 2260,
  [SMALL_STATE(31)] = 2322,
  [SMALL_STATE(32)] = 2384,
  [SMALL_STATE(33)] = 2446,
  [SMALL_STATE(34)] = 2491,
  [SMALL_STATE(35)] = 2556,
  [SMALL_STATE(36)] = 2621,
  [SMALL_STATE(37)] = 2656,
  [SMALL_STATE(38)] = 2717,
  [SMALL_STATE(39)] = 2756,
  [SMALL_STATE(40)] = 2817,
  [SMALL_STATE(41)] = 2854,
  [SMALL_STATE(42)] = 2889,
  [SMALL_STATE(43)] = 2950,
  [SMALL_STATE(44)] = 2987,
  [SMALL_STATE(45)] = 3019,
  [SMALL_STATE(46)] = 3051,
  [SMALL_STATE(47)] = 3083,
  [SMALL_STATE(48)] = 3143,
  [SMALL_STATE(49)] = 3175,
  [SMALL_STATE(50)] = 3207,
  [SMALL_STATE(51)] = 3239,
  [SMALL_STATE(52)] = 3271,
  [SMALL_STATE(53)] = 3303,
  [SMALL_STATE(54)] = 3335,
  [SMALL_STATE(55)] = 3367,
  [SMALL_STATE(56)] = 3423,
  [SMALL_STATE(57)] = 3455,
  [SMALL_STATE(58)] = 3487,
  [SMALL_STATE(59)] = 3519,
  [SMALL_STATE(60)] = 3551,
  [SMALL_STATE(61)] = 3609,
  [SMALL_STATE(62)] = 3641,
  [SMALL_STATE(63)] = 3673,
  [SMALL_STATE(64)] = 3705,
  [SMALL_STATE(65)] = 3761,
  [SMALL_STATE(66)] = 3793,
  [SMALL_STATE(67)] = 3853,
  [SMALL_STATE(68)] = 3885,
  [SMALL_STATE(69)] = 3917,
  [SMALL_STATE(70)] = 3949,
  [SMALL_STATE(71)] = 3981,
  [SMALL_STATE(72)] = 4013,
  [SMALL_STATE(73)] = 4045,
  [SMALL_STATE(74)] = 4077,
  [SMALL_STATE(75)] = 4109,
  [SMALL_STATE(76)] = 4141,
  [SMALL_STATE(77)] = 4173,
  [SMALL_STATE(78)] = 4205,
  [SMALL_STATE(79)] = 4237,
  [SMALL_STATE(80)] = 4269,
  [SMALL_STATE(81)] = 4301,
  [SMALL_STATE(82)] = 4333,
  [SMALL_STATE(83)] = 4365,
  [SMALL_STATE(84)] = 4397,
  [SMALL_STATE(85)] = 4429,
  [SMALL_STATE(86)] = 4461,
  [SMALL_STATE(87)] = 4493,
  [SMALL_STATE(88)] = 4525,
  [SMALL_STATE(89)] = 4557,
  [SMALL_STATE(90)] = 4589,
  [SMALL_STATE(91)] = 4621,
  [SMALL_STATE(92)] = 4653,
  [SMALL_STATE(93)] = 4685,
  [SMALL_STATE(94)] = 4745,
  [SMALL_STATE(95)] = 4777,
  [SMALL_STATE(96)] = 4809,
  [SMALL_STATE(97)] = 4844,
  [SMALL_STATE(98)] = 4901,
  [SMALL_STATE(99)] = 4936,
  [SMALL_STATE(100)] = 4971,
  [SMALL_STATE(101)] = 5006,
  [SMALL_STATE(102)] = 5063,
  [SMALL_STATE(103)] = 5098,
  [SMALL_STATE(104)] = 5133,
  [SMALL_STATE(105)] = 5188,
  [SMALL_STATE(106)] = 5242,
  [SMALL_STATE(107)] = 5296,
  [SMALL_STATE(108)] = 5336,
  [SMALL_STATE(109)] = 5390,
  [SMALL_STATE(110)] = 5444,
  [SMALL_STATE(111)] = 5498,
  [SMALL_STATE(112)] = 5527,
  [SMALL_STATE(113)] = 5556,
  [SMALL_STATE(114)] = 5585,
  [SMALL_STATE(115)] = 5614,
  [SMALL_STATE(116)] = 5643,
  [SMALL_STATE(117)] = 5672,
  [SMALL_STATE(118)] = 5701,
  [SMALL_STATE(119)] = 5730,
  [SMALL_STATE(120)] = 5759,
  [SMALL_STATE(121)] = 5810,
  [SMALL_STATE(122)] = 5839,
  [SMALL_STATE(123)] = 5868,
  [SMALL_STATE(124)] = 5897,
  [SMALL_STATE(125)] = 5926,
  [SMALL_STATE(126)] = 5955,
  [SMALL_STATE(127)] = 5984,
  [SMALL_STATE(128)] = 6013,
  [SMALL_STATE(129)] = 6042,
  [SMALL_STATE(130)] = 6093,
  [SMALL_STATE(131)] = 6122,
  [SMALL_STATE(132)] = 6151,
  [SMALL_STATE(133)] = 6180,
  [SMALL_STATE(134)] = 6209,
  [SMALL_STATE(135)] = 6238,
  [SMALL_STATE(136)] = 6267,
  [SMALL_STATE(137)] = 6298,
  [SMALL_STATE(138)] = 6329,
  [SMALL_STATE(139)] = 6360,
  [SMALL_STATE(140)] = 6391,
  [SMALL_STATE(141)] = 6420,
  [SMALL_STATE(142)] = 6451,
  [SMALL_STATE(143)] = 6480,
  [SMALL_STATE(144)] = 6509,
  [SMALL_STATE(145)] = 6540,
  [SMALL_STATE(146)] = 6571,
  [SMALL_STATE(147)] = 6602,
  [SMALL_STATE(148)] = 6633,
  [SMALL_STATE(149)] = 6662,
  [SMALL_STATE(150)] = 6697,
  [SMALL_STATE(151)] = 6726,
  [SMALL_STATE(152)] = 6755,
  [SMALL_STATE(153)] = 6784,
  [SMALL_STATE(154)] = 6813,
  [SMALL_STATE(155)] = 6861,
  [SMALL_STATE(156)] = 6909,
  [SMALL_STATE(157)] = 6957,
  [SMALL_STATE(158)] = 7005,
  [SMALL_STATE(159)] = 7053,
  [SMALL_STATE(160)] = 7101,
  [SMALL_STATE(161)] = 7149,
  [SMALL_STATE(162)] = 7181,
  [SMALL_STATE(163)] = 7213,
  [SMALL_STATE(164)] = 7261,
  [SMALL_STATE(165)] = 7309,
  [SMALL_STATE(166)] = 7357,
  [SMALL_STATE(167)] = 7405,
  [SMALL_STATE(168)] = 7453,
  [SMALL_STATE(169)] = 7501,
  [SMALL_STATE(170)] = 7549,
  [SMALL_STATE(171)] = 7597,
  [SMALL_STATE(172)] = 7645,
  [SMALL_STATE(173)] = 7693,
  [SMALL_STATE(174)] = 7741,
  [SMALL_STATE(175)] = 7789,
  [SMALL_STATE(176)] = 7837,
  [SMALL_STATE(177)] = 7866,
  [SMALL_STATE(178)] = 7895,
  [SMALL_STATE(179)] = 7924,
  [SMALL_STATE(180)] = 7953,
  [SMALL_STATE(181)] = 7982,
  [SMALL_STATE(182)] = 8011,
  [SMALL_STATE(183)] = 8040,
  [SMALL_STATE(184)] = 8069,
  [SMALL_STATE(185)] = 8098,
  [SMALL_STATE(186)] = 8127,
  [SMALL_STATE(187)] = 8156,
  [SMALL_STATE(188)] = 8185,
  [SMALL_STATE(189)] = 8214,
  [SMALL_STATE(190)] = 8246,
  [SMALL_STATE(191)] = 8276,
  [SMALL_STATE(192)] = 8306,
  [SMALL_STATE(193)] = 8338,
  [SMALL_STATE(194)] = 8368,
  [SMALL_STATE(195)] = 8400,
  [SMALL_STATE(196)] = 8432,
  [SMALL_STATE(197)] = 8464,
  [SMALL_STATE(198)] = 8494,
  [SMALL_STATE(199)] = 8523,
  [SMALL_STATE(200)] = 8552,
  [SMALL_STATE(201)] = 8591,
  [SMALL_STATE(202)] = 8620,
  [SMALL_STATE(203)] = 8649,
  [SMALL_STATE(204)] = 8688,
  [SMALL_STATE(205)] = 8723,
  [SMALL_STATE(206)] = 8767,
  [SMALL_STATE(207)] = 8811,
  [SMALL_STATE(208)] = 8855,
  [SMALL_STATE(209)] = 8899,
  [SMALL_STATE(210)] = 8921,
  [SMALL_STATE(211)] = 8943,
  [SMALL_STATE(212)] = 8977,
  [SMALL_STATE(213)] = 9011,
  [SMALL_STATE(214)] = 9051,
  [SMALL_STATE(215)] = 9073,
  [SMALL_STATE(216)] = 9100,
  [SMALL_STATE(217)] = 9139,
  [SMALL_STATE(218)] = 9165,
  [SMALL_STATE(219)] = 9187,
  [SMALL_STATE(220)] = 9225,
  [SMALL_STATE(221)] = 9251,
  [SMALL_STATE(222)] = 9277,
  [SMALL_STATE(223)] = 9303,
  [SMALL_STATE(224)] = 9327,
  [SMALL_STATE(225)] = 9353,
  [SMALL_STATE(226)] = 9379,
  [SMALL_STATE(227)] = 9417,
  [SMALL_STATE(228)] = 9443,
  [SMALL_STATE(229)] = 9469,
  [SMALL_STATE(230)] = 9493,
  [SMALL_STATE(231)] = 9514,
  [SMALL_STATE(232)] = 9535,
  [SMALL_STATE(233)] = 9556,
  [SMALL_STATE(234)] = 9591,
  [SMALL_STATE(235)] = 9612,
  [SMALL_STATE(236)] = 9637,
  [SMALL_STATE(237)] = 9658,
  [SMALL_STATE(238)] = 9679,
  [SMALL_STATE(239)] = 9700,
  [SMALL_STATE(240)] = 9721,
  [SMALL_STATE(241)] = 9742,
  [SMALL_STATE(242)] = 9763,
  [SMALL_STATE(243)] = 9784,
  [SMALL_STATE(244)] = 9813,
  [SMALL_STATE(245)] = 9834,
  [SMALL_STATE(246)] = 9856,
  [SMALL_STATE(247)] = 9878,
  [SMALL_STATE(248)] = 9898,
  [SMALL_STATE(249)] = 9917,
  [SMALL_STATE(250)] = 9936,
  [SMALL_STATE(251)] = 9955,
  [SMALL_STATE(252)] = 9974,
  [SMALL_STATE(253)] = 9993,
  [SMALL_STATE(254)] = 10012,
  [SMALL_STATE(255)] = 10031,
  [SMALL_STATE(256)] = 10050,
  [SMALL_STATE(257)] = 10069,
  [SMALL_STATE(258)] = 10088,
  [SMALL_STATE(259)] = 10107,
  [SMALL_STATE(260)] = 10126,
  [SMALL_STATE(261)] = 10150,
  [SMALL_STATE(262)] = 10174,
  [SMALL_STATE(263)] = 10196,
  [SMALL_STATE(264)] = 10218,
  [SMALL_STATE(265)] = 10242,
  [SMALL_STATE(266)] = 10266,
  [SMALL_STATE(267)] = 10288,
  [SMALL_STATE(268)] = 10312,
  [SMALL_STATE(269)] = 10336,
  [SMALL_STATE(270)] = 10358,
  [SMALL_STATE(271)] = 10382,
  [SMALL_STATE(272)] = 10404,
  [SMALL_STATE(273)] = 10417,
  [SMALL_STATE(274)] = 10430,
  [SMALL_STATE(275)] = 10451,
  [SMALL_STATE(276)] = 10464,
  [SMALL_STATE(277)] = 10483,
  [SMALL_STATE(278)] = 10496,
  [SMALL_STATE(279)] = 10517,
  [SMALL_STATE(280)] = 10530,
  [SMALL_STATE(281)] = 10543,
  [SMALL_STATE(282)] = 10561,
  [SMALL_STATE(283)] = 10579,
  [SMALL_STATE(284)] = 10597,
  [SMALL_STATE(285)] = 10615,
  [SMALL_STATE(286)] = 10633,
  [SMALL_STATE(287)] = 10651,
  [SMALL_STATE(288)] = 10665,
  [SMALL_STATE(289)] = 10685,
  [SMALL_STATE(290)] = 10705,
  [SMALL_STATE(291)] = 10721,
  [SMALL_STATE(292)] = 10739,
  [SMALL_STATE(293)] = 10757,
  [SMALL_STATE(294)] = 10775,
  [SMALL_STATE(295)] = 10793,
  [SMALL_STATE(296)] = 10811,
  [SMALL_STATE(297)] = 10822,
  [SMALL_STATE(298)] = 10837,
  [SMALL_STATE(299)] = 10852,
  [SMALL_STATE(300)] = 10869,
  [SMALL_STATE(301)] = 10884,
  [SMALL_STATE(302)] = 10899,
  [SMALL_STATE(303)] = 10914,
  [SMALL_STATE(304)] = 10929,
  [SMALL_STATE(305)] = 10944,
  [SMALL_STATE(306)] = 10955,
  [SMALL_STATE(307)] = 10970,
  [SMALL_STATE(308)] = 10985,
  [SMALL_STATE(309)] = 11002,
  [SMALL_STATE(310)] = 11017,
  [SMALL_STATE(311)] = 11032,
  [SMALL_STATE(312)] = 11047,
  [SMALL_STATE(313)] = 11062,
  [SMALL_STATE(314)] = 11077,
  [SMALL_STATE(315)] = 11092,
  [SMALL_STATE(316)] = 11103,
  [SMALL_STATE(317)] = 11118,
  [SMALL_STATE(318)] = 11133,
  [SMALL_STATE(319)] = 11148,
  [SMALL_STATE(320)] = 11163,
  [SMALL_STATE(321)] = 11178,
  [SMALL_STATE(322)] = 11193,
  [SMALL_STATE(323)] = 11208,
  [SMALL_STATE(324)] = 11223,
  [SMALL_STATE(325)] = 11238,
  [SMALL_STATE(326)] = 11253,
  [SMALL_STATE(327)] = 11268,
  [SMALL_STATE(328)] = 11283,
  [SMALL_STATE(329)] = 11298,
  [SMALL_STATE(330)] = 11311,
  [SMALL_STATE(331)] = 11322,
  [SMALL_STATE(332)] = 11337,
  [SMALL_STATE(333)] = 11352,
  [SMALL_STATE(334)] = 11367,
  [SMALL_STATE(335)] = 11382,
  [SMALL_STATE(336)] = 11396,
  [SMALL_STATE(337)] = 11410,
  [SMALL_STATE(338)] = 11424,
  [SMALL_STATE(339)] = 11438,
  [SMALL_STATE(340)] = 11452,
  [SMALL_STATE(341)] = 11466,
  [SMALL_STATE(342)] = 11480,
  [SMALL_STATE(343)] = 11494,
  [SMALL_STATE(344)] = 11508,
  [SMALL_STATE(345)] = 11522,
  [SMALL_STATE(346)] = 11534,
  [SMALL_STATE(347)] = 11548,
  [SMALL_STATE(348)] = 11562,
  [SMALL_STATE(349)] = 11573,
  [SMALL_STATE(350)] = 11584,
  [SMALL_STATE(351)] = 11595,
  [SMALL_STATE(352)] = 11606,
  [SMALL_STATE(353)] = 11617,
  [SMALL_STATE(354)] = 11628,
  [SMALL_STATE(355)] = 11639,
  [SMALL_STATE(356)] = 11650,
  [SMALL_STATE(357)] = 11661,
  [SMALL_STATE(358)] = 11672,
  [SMALL_STATE(359)] = 11683,
  [SMALL_STATE(360)] = 11694,
  [SMALL_STATE(361)] = 11702,
  [SMALL_STATE(362)] = 11710,
  [SMALL_STATE(363)] = 11718,
  [SMALL_STATE(364)] = 11726,
  [SMALL_STATE(365)] = 11734,
  [SMALL_STATE(366)] = 11742,
  [SMALL_STATE(367)] = 11750,
  [SMALL_STATE(368)] = 11758,
  [SMALL_STATE(369)] = 11766,
  [SMALL_STATE(370)] = 11774,
  [SMALL_STATE(371)] = 11782,
  [SMALL_STATE(372)] = 11790,
  [SMALL_STATE(373)] = 11798,
  [SMALL_STATE(374)] = 11806,
  [SMALL_STATE(375)] = 11814,
  [SMALL_STATE(376)] = 11822,
  [SMALL_STATE(377)] = 11830,
  [SMALL_STATE(378)] = 11838,
  [SMALL_STATE(379)] = 11846,
  [SMALL_STATE(380)] = 11854,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_stylesheet, 0, 0, 0),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(166),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(227),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(165),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(283),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(380),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(217),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(327),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(369),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(373),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(303),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(328),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [47] = {.entry = {.count = 1, .reusable = false}}, SHIFT(204),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [51] = {.entry = {.count = 1, .reusable = false}}, SHIFT(169),
  [53] = {.entry = {.count = 1, .reusable = false}}, SHIFT(220),
  [55] = {.entry = {.count = 1, .reusable = false}}, SHIFT(170),
  [57] = {.entry = {.count = 1, .reusable = false}}, SHIFT(293),
  [59] = {.entry = {.count = 1, .reusable = false}}, SHIFT(372),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(330),
  [63] = {.entry = {.count = 1, .reusable = false}}, SHIFT(222),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(315),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [79] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(169),
  [82] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(220),
  [85] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(170),
  [88] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(293),
  [91] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(372),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0),
  [96] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(222),
  [99] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(207),
  [102] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(125),
  [105] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(327),
  [108] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(369),
  [111] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(373),
  [114] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(26),
  [117] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(19),
  [120] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(30),
  [123] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(29),
  [126] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(28),
  [129] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(303),
  [132] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(328),
  [135] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(214),
  [138] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(14),
  [141] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, 0, 0), SHIFT_REPEAT(211),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0),
  [146] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [149] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(227),
  [152] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(165),
  [155] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(283),
  [158] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(380),
  [161] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(217),
  [164] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(207),
  [167] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(125),
  [170] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(327),
  [173] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(369),
  [176] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(373),
  [179] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(26),
  [182] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(19),
  [185] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(30),
  [188] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(29),
  [191] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(28),
  [194] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(303),
  [197] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(328),
  [200] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(210),
  [203] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(204),
  [206] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, 0, 0), SHIFT_REPEAT(211),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_stylesheet, 1, 0, 0),
  [211] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [213] = {.entry = {.count = 1, .reusable = false}}, SHIFT(327),
  [215] = {.entry = {.count = 1, .reusable = true}}, SHIFT(357),
  [217] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [221] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [225] = {.entry = {.count = 1, .reusable = true}}, SHIFT(322),
  [227] = {.entry = {.count = 1, .reusable = true}}, SHIFT(323),
  [229] = {.entry = {.count = 1, .reusable = false}}, SHIFT(161),
  [231] = {.entry = {.count = 1, .reusable = false}}, SHIFT(162),
  [233] = {.entry = {.count = 1, .reusable = false}}, SHIFT(175),
  [235] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [237] = {.entry = {.count = 1, .reusable = false}}, SHIFT(196),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(216),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [249] = {.entry = {.count = 1, .reusable = false}}, SHIFT(225),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(375),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [255] = {.entry = {.count = 1, .reusable = false}}, SHIFT(379),
  [257] = {.entry = {.count = 1, .reusable = true}}, SHIFT(318),
  [259] = {.entry = {.count = 1, .reusable = true}}, SHIFT(319),
  [261] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [263] = {.entry = {.count = 1, .reusable = false}}, SHIFT(107),
  [265] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [267] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [269] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [271] = {.entry = {.count = 1, .reusable = true}}, SHIFT(378),
  [273] = {.entry = {.count = 1, .reusable = true}}, SHIFT(291),
  [275] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [277] = {.entry = {.count = 1, .reusable = true}}, SHIFT(309),
  [279] = {.entry = {.count = 1, .reusable = true}}, SHIFT(310),
  [281] = {.entry = {.count = 1, .reusable = false}}, SHIFT(245),
  [283] = {.entry = {.count = 1, .reusable = false}}, SHIFT(246),
  [285] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [287] = {.entry = {.count = 1, .reusable = false}}, SHIFT(243),
  [289] = {.entry = {.count = 1, .reusable = false}}, SHIFT(291),
  [291] = {.entry = {.count = 1, .reusable = true}}, SHIFT(284),
  [293] = {.entry = {.count = 1, .reusable = false}}, SHIFT(284),
  [295] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [297] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [299] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_last_declaration, 3, 0, 14),
  [301] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [303] = {.entry = {.count = 1, .reusable = false}}, SHIFT(164),
  [305] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [307] = {.entry = {.count = 1, .reusable = false}}, SHIFT(149),
  [309] = {.entry = {.count = 1, .reusable = false}}, SHIFT(195),
  [311] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [313] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(233),
  [317] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [323] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [327] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [329] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [331] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [337] = {.entry = {.count = 1, .reusable = true}}, SHIFT(226),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [341] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__selector, 1, 0, 1),
  [343] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 3),
  [345] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__selector, 1, 0, 1),
  [347] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [349] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, 0, 3),
  [351] = {.entry = {.count = 1, .reusable = true}}, SHIFT(201),
  [353] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [357] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_value, 3, 0, 0),
  [359] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_value, 3, 0, 0),
  [361] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [363] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [365] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__selector, 1, 0, 0),
  [367] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, 0, 0),
  [369] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__selector, 1, 0, 0),
  [371] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, 0, 0),
  [373] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [375] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_class_name_repeat1, 2, 0, 0),
  [377] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_class_name_repeat1, 2, 0, 0),
  [379] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_class_name_repeat1, 2, 0, 0), SHIFT_REPEAT(40),
  [382] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_value, 2, 0, 0),
  [384] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_value, 2, 0, 0),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(256),
  [388] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_class_name, 1, 0, 0),
  [390] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_class_name, 1, 0, 0),
  [392] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [394] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_rule_set, 2, 0, 0),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule_set, 2, 0, 0),
  [398] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_statement, 4, 0, 13),
  [400] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_statement, 4, 0, 13),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyframe_block_list, 2, 0, 0),
  [404] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_keyframe_block_list, 2, 0, 0),
  [406] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [408] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [410] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, 0, 14),
  [412] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 4, 0, 14),
  [414] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_at_rule, 2, 0, 0),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_rule, 2, 0, 0),
  [418] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, 0, 0),
  [420] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3, 0, 0),
  [422] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 3, 0, 0),
  [424] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_statement, 3, 0, 0),
  [426] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_rule, 4, 0, 0),
  [428] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_at_rule, 4, 0, 0),
  [430] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_media_statement, 3, 0, 0),
  [432] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_media_statement, 3, 0, 0),
  [434] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0),
  [436] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(375),
  [439] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(196),
  [442] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(174),
  [445] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(318),
  [448] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(319),
  [451] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(161),
  [454] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(162),
  [457] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(175),
  [460] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(149),
  [463] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_postcss_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(196),
  [466] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_charset_statement, 3, 0, 0),
  [468] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_charset_statement, 3, 0, 0),
  [470] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 5, 0, 0),
  [472] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_statement, 5, 0, 0),
  [474] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyframe_block_list, 3, 0, 0),
  [476] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_keyframe_block_list, 3, 0, 0),
  [478] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 5, 0, 14),
  [480] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 5, 0, 14),
  [482] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(154),
  [485] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0),
  [487] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(375),
  [490] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(195),
  [493] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(174),
  [496] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(318),
  [499] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(319),
  [502] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(161),
  [505] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(162),
  [508] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(175),
  [511] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(149),
  [514] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0), SHIFT_REPEAT(195),
  [517] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_statement, 3, 0, 0),
  [519] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_statement, 3, 0, 0),
  [521] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_postcss_statement, 3, 0, 0),
  [523] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_postcss_statement, 3, 0, 0),
  [525] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, 0, 0),
  [527] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 4, 0, 0),
  [529] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2, 0, 0),
  [531] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 6, 0, 14),
  [533] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 6, 0, 14),
  [535] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [537] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_last_declaration, 4, 0, 14),
  [539] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [541] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_media_statement, 4, 0, 0),
  [543] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_media_statement, 4, 0, 0),
  [545] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_keyframes_statement, 3, 0, 8),
  [547] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyframes_statement, 3, 0, 8),
  [549] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_supports_statement, 3, 0, 0),
  [551] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_supports_statement, 3, 0, 0),
  [553] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2, 0, 0),
  [555] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2, 0, 0),
  [557] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_at_rule, 3, 0, 0),
  [559] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_rule, 3, 0, 0),
  [561] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_statement, 4, 0, 0),
  [563] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 4, 0, 0),
  [565] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [567] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 3, 0, 0),
  [569] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 3, 0, 0),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [573] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [575] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [577] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 2, 0, 0),
  [579] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 2, 0, 0),
  [581] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 3, 0, 10),
  [583] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 3, 0, 10),
  [585] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 2, 0, 2),
  [589] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 2, 0, 2),
  [591] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [593] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_selector, 3, 0, 11),
  [595] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_selector, 3, 0, 11),
  [597] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [599] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_selector, 2, 0, 5),
  [601] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_selector, 2, 0, 5),
  [603] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2, 0, 0),
  [605] = {.entry = {.count = 1, .reusable = true}}, SHIFT(249),
  [607] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [609] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__query, 1, 0, 4),
  [611] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__query, 1, 0, 4),
  [613] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [615] = {.entry = {.count = 1, .reusable = true}}, SHIFT(237),
  [617] = {.entry = {.count = 1, .reusable = true}}, SHIFT(273),
  [619] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 4, 0, 10),
  [621] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 4, 0, 10),
  [623] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 4, 0, 0),
  [625] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 4, 0, 0),
  [627] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_selector, 4, 0, 11),
  [629] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_selector, 4, 0, 11),
  [631] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_selector, 4, 0, 15),
  [633] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute_selector, 4, 0, 15),
  [635] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_with_selector_arguments, 3, 0, 0),
  [637] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_with_selector_arguments, 3, 0, 0),
  [639] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_nth_child_arguments, 3, 0, 0),
  [641] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_nth_child_arguments, 3, 0, 0),
  [643] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 3, 0, 2),
  [645] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 3, 0, 2),
  [647] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_arguments, 3, 0, 0),
  [649] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_arguments, 3, 0, 0),
  [651] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__nth_child_pseudo_class_selector, 2, 0, 0),
  [653] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__nth_child_pseudo_class_selector, 2, 0, 0),
  [655] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_arguments, 3, 0, 0),
  [657] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_arguments, 3, 0, 0),
  [659] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_selector, 5, 0, 9),
  [661] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute_selector, 5, 0, 9),
  [663] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_class_selector, 2, 0, 0),
  [665] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_class_selector, 2, 0, 0),
  [667] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_universal_selector, 1, 0, 0),
  [669] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_universal_selector, 1, 0, 0),
  [671] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_id_selector, 2, 0, 6),
  [673] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_id_selector, 2, 0, 6),
  [675] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_with_selector_arguments, 4, 0, 0),
  [677] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_with_selector_arguments, 4, 0, 0),
  [679] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_arguments, 4, 0, 0),
  [681] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_arguments, 4, 0, 0),
  [683] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_arguments, 4, 0, 0),
  [685] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_arguments, 4, 0, 0),
  [687] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_arguments, 2, 0, 0),
  [689] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_arguments, 2, 0, 0),
  [691] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_selector, 6, 0, 15),
  [693] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute_selector, 6, 0, 15),
  [695] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_nth_child_arguments, 5, 0, 0),
  [697] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_nth_child_arguments, 5, 0, 0),
  [699] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_selector, 3, 0, 9),
  [701] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute_selector, 3, 0, 9),
  [703] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_child_selector, 2, 0, 0),
  [705] = {.entry = {.count = 1, .reusable = true}}, SHIFT(302),
  [707] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_child_selector, 2, 0, 0),
  [709] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sibling_selector, 2, 0, 0),
  [711] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sibling_selector, 2, 0, 0),
  [713] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_adjacent_sibling_selector, 2, 0, 0),
  [715] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_adjacent_sibling_selector, 2, 0, 0),
  [717] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_selector, 2, 0, 0),
  [719] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_selector, 2, 0, 0),
  [721] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_descendant_selector, 3, 0, 0),
  [723] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_descendant_selector, 3, 0, 0),
  [725] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_class_selector, 3, 0, 0),
  [727] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_class_selector, 3, 0, 0),
  [729] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_id_selector, 3, 0, 12),
  [731] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_id_selector, 3, 0, 12),
  [733] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_child_selector, 3, 0, 0),
  [735] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_child_selector, 3, 0, 0),
  [737] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sibling_selector, 3, 0, 0),
  [739] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sibling_selector, 3, 0, 0),
  [741] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_adjacent_sibling_selector, 3, 0, 0),
  [743] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_adjacent_sibling_selector, 3, 0, 0),
  [745] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_selector, 3, 0, 0),
  [747] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_selector, 3, 0, 0),
  [749] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_with_selector_arguments, 2, 0, 0),
  [751] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_with_selector_arguments, 2, 0, 0),
  [753] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_arguments, 2, 0, 0),
  [755] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_arguments, 2, 0, 0),
  [757] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_selector, 3, 0, 5),
  [759] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_selector, 3, 0, 5),
  [761] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [763] = {.entry = {.count = 1, .reusable = false}}, SHIFT(189),
  [765] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [767] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [769] = {.entry = {.count = 1, .reusable = true}}, SHIFT(282),
  [771] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [773] = {.entry = {.count = 1, .reusable = false}}, SHIFT(235),
  [775] = {.entry = {.count = 1, .reusable = false}}, SHIFT(282),
  [777] = {.entry = {.count = 1, .reusable = true}}, SHIFT(294),
  [779] = {.entry = {.count = 1, .reusable = false}}, SHIFT(294),
  [781] = {.entry = {.count = 1, .reusable = true}}, SHIFT(364),
  [783] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [785] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [787] = {.entry = {.count = 1, .reusable = true}}, SHIFT(333),
  [789] = {.entry = {.count = 1, .reusable = true}}, SHIFT(298),
  [791] = {.entry = {.count = 1, .reusable = false}}, SHIFT(223),
  [793] = {.entry = {.count = 1, .reusable = false}}, SHIFT(229),
  [795] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [797] = {.entry = {.count = 1, .reusable = false}}, SHIFT(215),
  [799] = {.entry = {.count = 1, .reusable = false}}, SHIFT(232),
  [801] = {.entry = {.count = 1, .reusable = true}}, SHIFT(254),
  [803] = {.entry = {.count = 1, .reusable = false}}, SHIFT(254),
  [805] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [807] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [809] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_integer_value, 1, 0, 0),
  [811] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_integer_value, 1, 0, 0),
  [813] = {.entry = {.count = 1, .reusable = false}}, SHIFT(179),
  [815] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float_value, 1, 0, 0),
  [817] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_float_value, 1, 0, 0),
  [819] = {.entry = {.count = 1, .reusable = false}}, SHIFT(180),
  [821] = {.entry = {.count = 1, .reusable = true}}, SHIFT(295),
  [823] = {.entry = {.count = 1, .reusable = false}}, SHIFT(295),
  [825] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [827] = {.entry = {.count = 1, .reusable = false}}, SHIFT(184),
  [829] = {.entry = {.count = 1, .reusable = true}}, SHIFT(285),
  [831] = {.entry = {.count = 1, .reusable = false}}, SHIFT(285),
  [833] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [835] = {.entry = {.count = 1, .reusable = false}}, SHIFT(203),
  [837] = {.entry = {.count = 1, .reusable = true}}, SHIFT(276),
  [839] = {.entry = {.count = 1, .reusable = false}}, SHIFT(276),
  [841] = {.entry = {.count = 1, .reusable = true}}, SHIFT(267),
  [843] = {.entry = {.count = 1, .reusable = false}}, SHIFT(267),
  [845] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [847] = {.entry = {.count = 1, .reusable = false}}, SHIFT(200),
  [849] = {.entry = {.count = 1, .reusable = true}}, SHIFT(281),
  [851] = {.entry = {.count = 1, .reusable = false}}, SHIFT(281),
  [853] = {.entry = {.count = 1, .reusable = true}}, SHIFT(265),
  [855] = {.entry = {.count = 1, .reusable = false}}, SHIFT(265),
  [857] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [859] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [861] = {.entry = {.count = 1, .reusable = true}}, SHIFT(268),
  [863] = {.entry = {.count = 1, .reusable = false}}, SHIFT(268),
  [865] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesized_value, 3, 0, 0),
  [867] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesized_value, 3, 0, 0),
  [869] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_color_value, 2, 0, 0),
  [871] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_color_value, 2, 0, 0),
  [873] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_integer_value, 2, 0, 0),
  [875] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_integer_value, 2, 0, 0),
  [877] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float_value, 2, 0, 0),
  [879] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_float_value, 2, 0, 0),
  [881] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call_expression, 2, 0, 7),
  [883] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call_expression, 2, 0, 7),
  [885] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grid_value, 3, 0, 0),
  [887] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grid_value, 3, 0, 0),
  [889] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 2, 0, 0),
  [891] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_arguments, 2, 0, 0),
  [893] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expression, 3, 0, 0),
  [895] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_expression, 3, 0, 0),
  [897] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_grid_value, 4, 0, 0),
  [899] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_grid_value, 4, 0, 0),
  [901] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 3, 0, 0),
  [903] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_arguments, 3, 0, 0),
  [905] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 4, 0, 0),
  [907] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_arguments, 4, 0, 0),
  [909] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, 0, 0),
  [911] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [913] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [915] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [917] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [919] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [921] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_last_declaration, 5, 0, 14),
  [923] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 1, 0, 0),
  [925] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 1, 0, 0),
  [927] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_postcss_statement_repeat1, 1, 0, 0),
  [929] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_postcss_statement_repeat1, 1, 0, 0),
  [931] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [933] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [935] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [937] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [939] = {.entry = {.count = 1, .reusable = false}}, SHIFT(158),
  [941] = {.entry = {.count = 1, .reusable = false}}, SHIFT(277),
  [943] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [945] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [947] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [949] = {.entry = {.count = 1, .reusable = true}}, SHIFT(368),
  [951] = {.entry = {.count = 1, .reusable = true}}, SHIFT(371),
  [953] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [955] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [957] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [959] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [961] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [963] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [965] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [967] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [969] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [971] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 1, 0, 0),
  [973] = {.entry = {.count = 1, .reusable = false}}, SHIFT(155),
  [975] = {.entry = {.count = 1, .reusable = false}}, SHIFT(160),
  [977] = {.entry = {.count = 1, .reusable = false}}, SHIFT(359),
  [979] = {.entry = {.count = 1, .reusable = false}}, SHIFT(100),
  [981] = {.entry = {.count = 1, .reusable = false}}, SHIFT(355),
  [983] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [985] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [987] = {.entry = {.count = 1, .reusable = false}}, SHIFT(349),
  [989] = {.entry = {.count = 1, .reusable = false}}, SHIFT(99),
  [991] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2, 0, 0),
  [993] = {.entry = {.count = 1, .reusable = false}}, SHIFT(173),
  [995] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [997] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [999] = {.entry = {.count = 1, .reusable = false}}, SHIFT(218),
  [1001] = {.entry = {.count = 1, .reusable = false}}, SHIFT(329),
  [1003] = {.entry = {.count = 1, .reusable = true}}, SHIFT(279),
  [1005] = {.entry = {.count = 1, .reusable = false}}, SHIFT(241),
  [1007] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [1009] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [1011] = {.entry = {.count = 1, .reusable = true}}, SHIFT(247),
  [1013] = {.entry = {.count = 1, .reusable = true}}, SHIFT(258),
  [1015] = {.entry = {.count = 1, .reusable = true}}, SHIFT(350),
  [1017] = {.entry = {.count = 1, .reusable = true}}, SHIFT(221),
  [1019] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [1021] = {.entry = {.count = 1, .reusable = true}}, SHIFT(228),
  [1023] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_keyframe_block_list_repeat1, 2, 0, 0),
  [1025] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_keyframe_block_list_repeat1, 2, 0, 0), SHIFT_REPEAT(351),
  [1028] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_keyframe_block_list_repeat1, 2, 0, 0), SHIFT_REPEAT(352),
  [1031] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [1033] = {.entry = {.count = 1, .reusable = true}}, SHIFT(351),
  [1035] = {.entry = {.count = 1, .reusable = true}}, SHIFT(352),
  [1037] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [1039] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [1041] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [1043] = {.entry = {.count = 1, .reusable = true}}, SHIFT(252),
  [1045] = {.entry = {.count = 1, .reusable = false}}, SHIFT(159),
  [1047] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [1049] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [1051] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [1053] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [1055] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [1057] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesized_query, 3, 0, 0),
  [1059] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_feature_query, 5, 0, 16),
  [1061] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_query, 3, 0, 0),
  [1063] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_grid_value_repeat1, 2, 0, 0),
  [1065] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector_query, 4, 0, 0),
  [1067] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_query, 2, 0, 0),
  [1069] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [1071] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [1073] = {.entry = {.count = 1, .reusable = true}}, SHIFT(260),
  [1075] = {.entry = {.count = 1, .reusable = true}}, SHIFT(251),
  [1077] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [1079] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [1081] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_import_statement_repeat1, 2, 0, 0),
  [1083] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [1085] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [1087] = {.entry = {.count = 1, .reusable = true}}, SHIFT(348),
  [1089] = {.entry = {.count = 1, .reusable = false}}, SHIFT(346),
  [1091] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [1093] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [1095] = {.entry = {.count = 1, .reusable = true}}, SHIFT(270),
  [1097] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [1099] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [1101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyframe_block, 2, 0, 0),
  [1103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(231),
  [1105] = {.entry = {.count = 1, .reusable = false}}, SHIFT(236),
  [1107] = {.entry = {.count = 1, .reusable = false}}, SHIFT(326),
  [1109] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_import_statement_repeat1, 2, 0, 0), SHIFT_REPEAT(221),
  [1112] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2, 0, 0), SHIFT_REPEAT(120),
  [1115] = {.entry = {.count = 1, .reusable = false}}, SHIFT(140),
  [1117] = {.entry = {.count = 1, .reusable = false}}, SHIFT(313),
  [1119] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2, 0, 0), SHIFT_REPEAT(20),
  [1122] = {.entry = {.count = 1, .reusable = true}}, SHIFT(253),
  [1124] = {.entry = {.count = 1, .reusable = false}}, SHIFT(250),
  [1126] = {.entry = {.count = 1, .reusable = false}}, SHIFT(311),
  [1128] = {.entry = {.count = 1, .reusable = false}}, SHIFT(334),
  [1130] = {.entry = {.count = 1, .reusable = false}}, SHIFT(255),
  [1132] = {.entry = {.count = 1, .reusable = false}}, SHIFT(331),
  [1134] = {.entry = {.count = 1, .reusable = false}}, SHIFT(135),
  [1136] = {.entry = {.count = 1, .reusable = false}}, SHIFT(234),
  [1138] = {.entry = {.count = 1, .reusable = false}}, SHIFT(332),
  [1140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [1142] = {.entry = {.count = 1, .reusable = false}}, SHIFT(188),
  [1144] = {.entry = {.count = 1, .reusable = false}}, SHIFT(320),
  [1146] = {.entry = {.count = 1, .reusable = false}}, SHIFT(321),
  [1148] = {.entry = {.count = 1, .reusable = false}}, SHIFT(177),
  [1150] = {.entry = {.count = 1, .reusable = false}}, SHIFT(41),
  [1152] = {.entry = {.count = 1, .reusable = false}}, SHIFT(324),
  [1154] = {.entry = {.count = 1, .reusable = false}}, SHIFT(325),
  [1156] = {.entry = {.count = 1, .reusable = false}}, SHIFT(36),
  [1158] = {.entry = {.count = 1, .reusable = false}}, SHIFT(316),
  [1160] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_value_repeat1, 2, 0, 0),
  [1162] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_value_repeat1, 2, 0, 0), SHIFT_REPEAT(331),
  [1165] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_value_repeat2, 2, 0, 0),
  [1167] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_string_value_repeat2, 2, 0, 0), SHIFT_REPEAT(332),
  [1170] = {.entry = {.count = 1, .reusable = false}}, SHIFT(314),
  [1172] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [1174] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [1176] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2, 0, 0), SHIFT_REPEAT(13),
  [1179] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_grid_value_repeat1, 2, 0, 0), SHIFT_REPEAT(167),
  [1182] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [1184] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [1186] = {.entry = {.count = 1, .reusable = true}}, SHIFT(185),
  [1188] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [1190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [1192] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 2, 0, 0),
  [1194] = {.entry = {.count = 1, .reusable = true}}, SHIFT(272),
  [1196] = {.entry = {.count = 1, .reusable = false}}, SHIFT(247),
  [1198] = {.entry = {.count = 1, .reusable = true}}, SHIFT(248),
  [1200] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [1202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [1204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [1206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [1208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(266),
  [1210] = {.entry = {.count = 1, .reusable = true}}, SHIFT(290),
  [1212] = {.entry = {.count = 1, .reusable = false}}, SHIFT(178),
  [1214] = {.entry = {.count = 1, .reusable = false}}, SHIFT(126),
  [1216] = {.entry = {.count = 1, .reusable = true}}, SHIFT(269),
  [1218] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [1220] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [1222] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [1224] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [1226] = {.entry = {.count = 1, .reusable = true}}, SHIFT(305),
  [1228] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [1230] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [1232] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [1234] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [1236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [1238] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [1240] = {.entry = {.count = 1, .reusable = true}}, SHIFT(353),
  [1242] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [1244] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [1246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [1248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(257),
  [1250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [1252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(358),
};

enum ts_external_scanner_symbol_identifiers {
  ts_external_token__descendant_operator = 0,
  ts_external_token__pseudo_class_selector_colon = 1,
  ts_external_token___error_recovery = 2,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token__descendant_operator] = sym__descendant_operator,
  [ts_external_token__pseudo_class_selector_colon] = sym__pseudo_class_selector_colon,
  [ts_external_token___error_recovery] = sym___error_recovery,
};

static const bool ts_external_scanner_states[4][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token__descendant_operator] = true,
    [ts_external_token__pseudo_class_selector_colon] = true,
    [ts_external_token___error_recovery] = true,
  },
  [2] = {
    [ts_external_token__pseudo_class_selector_colon] = true,
  },
  [3] = {
    [ts_external_token__descendant_operator] = true,
    [ts_external_token__pseudo_class_selector_colon] = true,
  },
};

#ifdef __cplusplus
extern "C" {
#endif
void *tree_sitter_css_external_scanner_create(void);
void tree_sitter_css_external_scanner_destroy(void *);
bool tree_sitter_css_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_css_external_scanner_serialize(void *, char *);
void tree_sitter_css_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_css(void) {
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
    .external_scanner = {
      &ts_external_scanner_states[0][0],
      ts_external_scanner_symbol_map,
      tree_sitter_css_external_scanner_create,
      tree_sitter_css_external_scanner_destroy,
      tree_sitter_css_external_scanner_scan,
      tree_sitter_css_external_scanner_serialize,
      tree_sitter_css_external_scanner_deserialize,
    },
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
