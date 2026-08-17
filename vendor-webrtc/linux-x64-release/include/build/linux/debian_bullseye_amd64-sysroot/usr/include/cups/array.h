/*
 * Sorted array definitions for CUPS.
 *
 * Copyright 2007-2010 by Apple Inc.
 * Copyright 1997-2007 by Easy Software Products.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more information.
 */

#ifndef _CUPS_ARRAY_H_
#  define _CUPS_ARRAY_H_

/*
 * Include necessary headers...
 */

#  include "versioning.h"
#  include <stdlib.h>


/*
 * C++ magic...
 */

#  ifdef __cplusplus
extern "C" {
#  endif /* __cplusplus */


/*
 * Types and structures...
 */

typedef struct _cups_array_s cups_array_t;
					/**** CUPS array type ****/
typedef int (*cups_array_func_t)(void *first, void *second, void *data);
					/**** Array comparison function ****/
typedef int (*cups_ahash_func_t)(void *element, void *data);
					/**** Array hash function ****/
typedef void *(*cups_acopy_func_t)(void *element, void *data);
					/**** Array element copy function ****/
typedef void (*cups_afree_func_t)(void *element, void *data);
					/**** Array element free function ****/


/*
 * Functions...
 */

extern int		cupsArrayAdd(cups_array_t *a, void *e) _CUPS_API_1_2;
extern void		cupsArrayClear(cups_array_t *a) _CUPS_API_1_2;
extern int		cupsArrayCount(cups_array_t *a) _CUPS_API_1_2;
extern void		*cupsArrayCurrent(cups_array_t *a) _CUPS_API_1_2;
extern void		cupsArrayDelete(cups_array_t *a) _CUPS_API_1_2;
extern cups_array_t	*cupsArrayDup(cups_array_t *a) _CUPS_API_1_2;
extern void		*cupsArrayFind(cups_array_t *a, void *e) _CUPS_API_1_2;
extern void		*cupsArrayFirst(cups_array_t *a) _CUPS_API_1_2;
extern int		cupsArrayGetIndex(cups_array_t *a) _CUPS_API_1_3;
extern int		cupsArrayGetInsert(cups_array_t *a) _CUPS_API_1_3;
extern void		*cupsArrayIndex(cups_array_t *a, int n) _CUPS_API_1_2;
extern int		cupsArrayInsert(cups_array_t *a, void *e) _CUPS_API_1_2;
extern void		*cupsArrayLast(cups_array_t *a) _CUPS_API_1_2;
extern cups_array_t	*cupsArrayNew(cups_array_func_t f, void *d) _CUPS_API_1_2;
extern cups_array_t	*cupsArrayNew2(cups_array_func_t f, void *d,
			               cups_ahash_func_t h, int hsize) _CUPS_API_1_3;
extern cups_array_t	*cupsArrayNew3(cups_array_func_t f, void *d,
			               cups_ahash_func_t h, int hsize,
				       cups_acopy_func_t cf,
				       cups_afree_func_t ff) _CUPS_API_1_5;
extern void		*cupsArrayNext(cups_array_t *a) _CUPS_API_1_2;
extern void		*cupsArrayPrev(cups_array_t *a) _CUPS_API_1_2;
extern int		cupsArrayRemove(cups_array_t *a, void *e) _CUPS_API_1_2;
extern void		*cupsArrayRestore(cups_array_t *a) _CUPS_API_1_2;
extern int		cupsArraySave(cups_array_t *a) _CUPS_API_1_2;
extern void		*cupsArrayUserData(cups_array_t *a) _CUPS_API_1_2;

#  ifdef __cplusplus
}
#  endif /* __cplusplus */
#endif /* !_CUPS_ARRAY_H_ */
