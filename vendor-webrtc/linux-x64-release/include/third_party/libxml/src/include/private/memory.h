#ifndef XML_MEMORY_H_PRIVATE__
#define XML_MEMORY_H_PRIVATE__

#include "../../libxml.h"

#include <limits.h>
#include <stddef.h>

#ifndef SIZE_MAX
  #define SIZE_MAX ((size_t) -1)
#endif

#define XML_MAX_ITEMS 1000000000 /* 1 billion */

XML_HIDDEN void
xmlInitMemoryInternal(void);
XML_HIDDEN void
xmlCleanupMemoryInternal(void);

/**
 * xmlGrowCapacity:
 * @array:  pointer to array
 * @capacity:  pointer to capacity (in/out)
 * @elemSize:  size of an element in bytes
 * @min:  elements in initial allocation
 * @max:  maximum elements in the array
 *
 * Grow an array by at least one element, checking for overflow.
 *
 * Returns the new array size on success, -1 on failure.
 */
static XML_INLINE int
xmlGrowCapacity(int capacity, size_t elemSize, int min, int max) {
    int extra;

    if (capacity <= 0) {
#ifdef FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION
        (void) min;
        return(1);
#else
        return(min);
#endif
    }

    if ((capacity >= max) ||
        ((size_t) capacity > SIZE_MAX / 2 / elemSize))
        return(-1);

    /* Grow by 50% */
    extra = (capacity + 1) / 2;

    if (capacity > max - extra)
        return(max);

    return(capacity + extra);
}

#endif /* XML_MEMORY_H_PRIVATE__ */
