#ifndef LocaleFallbackIterator_H
#define LocaleFallbackIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Locale.d.h"

#include "LocaleFallbackIterator.d.h"






Locale* icu4x_LocaleFallbackIterator_next_mv1(LocaleFallbackIterator* self);


void icu4x_LocaleFallbackIterator_destroy_mv1(LocaleFallbackIterator* self);





#endif // LocaleFallbackIterator_H
