#ifndef LIBZED_CORE_H
#define LIBZED_CORE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void* ZedBufferHandle;

typedef struct {
    size_t start_byte;
    size_t end_byte;
    const char* capture_name;
} ZedHighlightSpan;

typedef struct {
    ZedHighlightSpan* spans;
    size_t count;
} ZedHighlightResult;

ZedBufferHandle zed_buffer_create(const char* initial_text);
void zed_buffer_free(ZedBufferHandle buffer);
size_t zed_buffer_len(ZedBufferHandle buffer);
size_t zed_buffer_line_count(ZedBufferHandle buffer);
int zed_buffer_replace(ZedBufferHandle buffer, size_t start_byte, size_t end_byte, const char* replacement_text);
char* zed_buffer_to_string(ZedBufferHandle buffer);
void zed_string_free(char* s);
void zed_highlight_result_free(ZedHighlightResult res);

// Tree-sitter AST query
int* zed_buffer_tree_sitter_query(ZedBufferHandle buffer, const char* query, const char* capture_name);
// CRDT state synchronization
int zed_buffer_crdt_merge(ZedBufferHandle buffer, ZedBufferHandle other_buffer, const char* merge_strategy);
// LSP diagnostics
char* zed_buffer_get_diagnostics(ZedBufferHandle buffer, const char* diagnostic_type);
void zed_diagnostics_free(char* s);

#ifdef __cplusplus
}
#endif

#endif /* LIBZED_CORE_H */
