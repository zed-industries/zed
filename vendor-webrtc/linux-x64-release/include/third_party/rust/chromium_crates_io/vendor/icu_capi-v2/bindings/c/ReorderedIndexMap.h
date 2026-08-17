#ifndef ReorderedIndexMap_H
#define ReorderedIndexMap_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ReorderedIndexMap.d.h"






DiplomatUsizeView icu4x_ReorderedIndexMap_as_slice_mv1(const ReorderedIndexMap* self);

size_t icu4x_ReorderedIndexMap_len_mv1(const ReorderedIndexMap* self);

bool icu4x_ReorderedIndexMap_is_empty_mv1(const ReorderedIndexMap* self);

size_t icu4x_ReorderedIndexMap_get_mv1(const ReorderedIndexMap* self, size_t index);


void icu4x_ReorderedIndexMap_destroy_mv1(ReorderedIndexMap* self);





#endif // ReorderedIndexMap_H
