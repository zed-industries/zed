/* memxor.h
 *
 */

#ifndef NETTLE_MEMXOR_H_INCLUDED
#define NETTLE_MEMXOR_H_INCLUDED

#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define memxor nettle_memxor
#define memxor3 nettle_memxor3

void *memxor(void *dst, const void *src, size_t n);
void *memxor3(void *dst, const void *a, const void *b, size_t n);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_MEMXOR_H_INCLUDED */
