#ifndef TREE_SITTER_RUNTIME_H_
#define TREE_SITTER_RUNTIME_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdlib.h>
#include <stdbool.h>

typedef unsigned short TSSymbol;
typedef struct TSLanguage TSLanguage;
typedef struct TSDocument TSDocument;

typedef enum {
  TSInputEncodingUTF8,
  TSInputEncodingUTF16,
} TSInputEncoding;

typedef struct {
  void *payload;
  const char *(*read_fn)(void *payload, size_t *bytes_read);
  int (*seek_fn)(void *payload, size_t character, size_t byte);
  TSInputEncoding encoding;
} TSInput;

typedef enum {
  TSDebugTypeParse,
  TSDebugTypeLex,
} TSDebugType;

typedef struct {
  void *payload;
  void (*debug_fn)(void *payload, TSDebugType, const char *);
} TSDebugger;

typedef struct {
  size_t position;
  size_t chars_inserted;
  size_t chars_removed;
} TSInputEdit;

typedef struct {
  size_t row;
  size_t column;
} TSPoint;

typedef struct {
  const void *data;
  size_t offset[3];
} TSNode;

typedef struct {
  TSSymbol value;
  bool done;
  void *data;
} TSSymbolIterator;

size_t ts_node_start_char(TSNode);
size_t ts_node_start_byte(TSNode);
TSPoint ts_node_start_point(TSNode);
size_t ts_node_end_char(TSNode);
size_t ts_node_end_byte(TSNode);
TSPoint ts_node_end_point(TSNode);
TSSymbol ts_node_symbol(TSNode);
TSSymbolIterator ts_node_symbols(TSNode);
void ts_symbol_iterator_next(TSSymbolIterator *);
const char *ts_node_name(TSNode, const TSDocument *);
char *ts_node_string(TSNode, const TSDocument *);
bool ts_node_eq(TSNode, TSNode);
bool ts_node_is_named(TSNode);
bool ts_node_has_changes(TSNode);
TSNode ts_node_parent(TSNode);
TSNode ts_node_child(TSNode, size_t);
TSNode ts_node_named_child(TSNode, size_t);
size_t ts_node_child_count(TSNode);
size_t ts_node_named_child_count(TSNode);
TSNode ts_node_next_sibling(TSNode);
TSNode ts_node_next_named_sibling(TSNode);
TSNode ts_node_prev_sibling(TSNode);
TSNode ts_node_prev_named_sibling(TSNode);
TSNode ts_node_descendant_for_range(TSNode, size_t, size_t);
TSNode ts_node_named_descendant_for_range(TSNode, size_t, size_t);

TSDocument *ts_document_make();
void ts_document_free(TSDocument *);
const TSLanguage *ts_document_language(TSDocument *);
void ts_document_set_language(TSDocument *, const TSLanguage *);
TSInput ts_document_input(TSDocument *);
void ts_document_set_input(TSDocument *, TSInput);
void ts_document_set_input_string(TSDocument *, const char *);
TSDebugger ts_document_debugger(const TSDocument *);
void ts_document_set_debugger(TSDocument *, TSDebugger);
void ts_document_print_debugging_graphs(TSDocument *, bool);
void ts_document_edit(TSDocument *, TSInputEdit);
int ts_document_parse(TSDocument *);
void ts_document_invalidate(TSDocument *);
TSNode ts_document_root_node(const TSDocument *);
size_t ts_document_parse_count(const TSDocument *);

size_t ts_language_symbol_count(const TSLanguage *);
const char *ts_language_symbol_name(const TSLanguage *, TSSymbol);

#define ts_builtin_sym_error ((TSSymbol)-1)
#define ts_builtin_sym_end 0
#define ts_builtin_sym_start 1

#ifdef __cplusplus
}
#endif

#endif  // TREE_SITTER_RUNTIME_H_
