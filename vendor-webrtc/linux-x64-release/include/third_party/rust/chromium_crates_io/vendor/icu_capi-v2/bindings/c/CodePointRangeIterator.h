#ifndef CodePointRangeIterator_H
#define CodePointRangeIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointRangeIteratorResult.d.h"

#include "CodePointRangeIterator.d.h"






CodePointRangeIteratorResult icu4x_CodePointRangeIterator_next_mv1(CodePointRangeIterator* self);


void icu4x_CodePointRangeIterator_destroy_mv1(CodePointRangeIterator* self);





#endif // CodePointRangeIterator_H
