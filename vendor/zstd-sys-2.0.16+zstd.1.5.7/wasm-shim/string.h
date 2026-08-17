#include <stdlib.h>

#ifndef	_STRING_H
#define	_STRING_H	1

int rust_zstd_wasm_shim_memcmp(const void *str1, const void *str2, size_t n);
void *rust_zstd_wasm_shim_memcpy(void *restrict dest, const void *restrict src, size_t n);
void *rust_zstd_wasm_shim_memmove(void *dest, const void *src, size_t n);
void *rust_zstd_wasm_shim_memset(void *dest, int c, size_t n);

inline int memcmp(const void *str1, const void *str2, size_t n) {
    return rust_zstd_wasm_shim_memcmp(str1, str2, n);
}

inline void *memcpy(void *restrict dest, const void *restrict src, size_t n) {
	return rust_zstd_wasm_shim_memcpy(dest, src, n);
}

inline void *memmove(void *dest, const void *src, size_t n) {
	return rust_zstd_wasm_shim_memmove(dest, src, n);
}

inline void *memset(void *dest, int c, size_t n) {
	return rust_zstd_wasm_shim_memset(dest, c, n);
}

#endif // _STRING_H
