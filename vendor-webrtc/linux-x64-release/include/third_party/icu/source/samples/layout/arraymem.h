/*
 *
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 *
 * (C) Copyright IBM Corp. 1998-2007 - All Rights Reserved
 *
 */

#ifndef __ARRAYMEM_H
#define __ARRAYMEM_H

#include <stdlib.h>

#define ARRAY_SIZE(array) (sizeof array / sizeof array[0])

#define ARRAY_COPY(dst, src, count) memcpy((void *) (dst), (void *) (src), (count) * sizeof (src)[0])

#define NEW_ARRAY(type,count) (type *) malloc((count) * sizeof(type))

#define DELETE_ARRAY(array) free((void *) (array))

#define GROW_ARRAY(array,newSize) realloc((void *) (array), (newSize) * sizeof (array)[0])

#endif
