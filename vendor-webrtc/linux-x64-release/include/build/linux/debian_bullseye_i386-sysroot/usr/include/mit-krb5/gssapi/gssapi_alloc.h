/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* To the extent possible under law, Painless Security, LLC has waived
 * all copyright and related or neighboring rights to GSS-API Memory
 * Management Header. This work is published from: United States.
 */

#ifndef GSSAPI_ALLOC_H
#define GSSAPI_ALLOC_H

#ifdef _WIN32
#include "winbase.h"
#endif
#include <string.h>

#if defined(_WIN32)

static inline void
gssalloc_free(void *value)
{
    if (value)
        HeapFree(GetProcessHeap(), 0, value);
}

static inline void *
gssalloc_malloc(size_t size)
{
    return HeapAlloc(GetProcessHeap(), 0, size);
}

static inline void *
gssalloc_calloc(size_t count, size_t size)
{
    return HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, count * size);
}

static inline void *
gssalloc_realloc(void *value, size_t size)
{
    /* Unlike realloc(), HeapReAlloc() does not work on null values. */
    if (value == NULL)
        return HeapAlloc(GetProcessHeap(), 0, size);
    return HeapReAlloc(GetProcessHeap(), 0, value, size);
}

#elif defined(DEBUG_GSSALLOC)

/* Be deliberately incompatible with malloc and free, to allow us to detect
 * mismatched malloc/gssalloc usage on Unix. */

static inline void
gssalloc_free(void *value)
{
    char *p = (char *)value - 8;

    if (value == NULL)
        return;
    if (memcmp(p, "gssalloc", 8) != 0)
        abort();
    free(p);
}

static inline void *
gssalloc_malloc(size_t size)
{
    char *p = calloc(size + 8, 1);

    memcpy(p, "gssalloc", 8);
    return p + 8;
}

static inline void *
gssalloc_calloc(size_t count, size_t size)
{
    return gssalloc_malloc(count * size);
}

static inline void *
gssalloc_realloc(void *value, size_t size)
{
    char *p = (char *)value - 8;

    if (value == NULL)
        return gssalloc_malloc(size);
    if (memcmp(p, "gssalloc", 8) != 0)
        abort();
    return (char *)realloc(p, size + 8) + 8;
}

#else /* not _WIN32 or DEBUG_GSSALLOC */

/* Normal Unix case, just use free/malloc/calloc/realloc. */

static inline void
gssalloc_free(void *value)
{
    free(value);
}

static inline void *
gssalloc_malloc(size_t size)
{
    return malloc(size);
}

static inline void *
gssalloc_calloc(size_t count, size_t size)
{
    return calloc(count, size);
}

static inline void *
gssalloc_realloc(void *value, size_t size)
{
    return realloc(value, size);
}

#endif /* not _WIN32 or DEBUG_GSSALLOC */

static inline char *
gssalloc_strdup(const char *str)
{
    size_t size = strlen(str)+1;
    char *copy = gssalloc_malloc(size);
    if (copy) {
        memcpy(copy, str, size);
        copy[size-1] = '\0';
    }
    return copy;
}

#endif
