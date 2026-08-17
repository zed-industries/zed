#include <stddef.h>

#ifndef _STDLIB_H
#define _STDLIB_H 1

void* rust_zstd_wasm_shim_malloc(size_t size);
void* rust_zstd_wasm_shim_calloc(size_t nmemb, size_t size);
void rust_zstd_wasm_shim_free(void* ptr);
void rust_zstd_wasm_shim_qsort(void* base, size_t nitems, size_t size,
                               int (*compar)(const void*, const void*));

#define malloc(size) rust_zstd_wasm_shim_malloc(size)
#define calloc(nmemb, size) rust_zstd_wasm_shim_calloc(nmemb, size)
#define free(ptr) rust_zstd_wasm_shim_free(ptr)
#define qsort(base, nitems, size, compar) \
  rust_zstd_wasm_shim_qsort(base, nitems, size, compar)

#endif  // _STDLIB_H
