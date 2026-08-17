/* Alloc.h -- Memory allocation functions
2024-01-22 : Igor Pavlov : Public domain */

#ifndef ZIP7_INC_ALLOC_H
#define ZIP7_INC_ALLOC_H

#include "7zTypes.h"

EXTERN_C_BEGIN

/*
  MyFree(NULL)        : is allowed, as free(NULL)
  MyAlloc(0)          : returns NULL : but malloc(0)        is allowed to return NULL or non_NULL
  MyRealloc(NULL, 0)  : returns NULL : but realloc(NULL, 0) is allowed to return NULL or non_NULL
MyRealloc() is similar to realloc() for the following cases:
  MyRealloc(non_NULL, 0)         : returns NULL and always calls MyFree(ptr)
  MyRealloc(NULL, non_ZERO)      : returns NULL, if allocation failed
  MyRealloc(non_NULL, non_ZERO)  : returns NULL, if reallocation failed
*/

void *MyAlloc(size_t size);
void MyFree(void *address);
void *MyRealloc(void *address, size_t size);

void *z7_AlignedAlloc(size_t size);
void  z7_AlignedFree(void *p);

#ifdef _WIN32

#ifdef Z7_LARGE_PAGES
void SetLargePageSize(void);
#endif

void *MidAlloc(size_t size);
void MidFree(void *address);
void *BigAlloc(size_t size);
void BigFree(void *address);

/* #define Z7_BIG_ALLOC_IS_ZERO_FILLED */

#else

#define MidAlloc(size)    z7_AlignedAlloc(size)
#define MidFree(address)  z7_AlignedFree(address)
#define BigAlloc(size)    z7_AlignedAlloc(size)
#define BigFree(address)  z7_AlignedFree(address)

#endif

extern const ISzAlloc g_Alloc;

#ifdef _WIN32
extern const ISzAlloc g_BigAlloc;
extern const ISzAlloc g_MidAlloc;
#else
#define g_BigAlloc g_AlignedAlloc
#define g_MidAlloc g_AlignedAlloc
#endif

extern const ISzAlloc g_AlignedAlloc;


typedef struct
{
  ISzAlloc vt;
  ISzAllocPtr baseAlloc;
  unsigned numAlignBits; /* ((1 << numAlignBits) >= sizeof(void *)) */
  size_t offset;         /* (offset == (k * sizeof(void *)) && offset < (1 << numAlignBits) */
} CAlignOffsetAlloc;

void AlignOffsetAlloc_CreateVTable(CAlignOffsetAlloc *p);


EXTERN_C_END

#endif
