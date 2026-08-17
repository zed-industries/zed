#ifndef TimeZoneAndCanonicalIterator_H
#define TimeZoneAndCanonicalIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "TimeZoneAndCanonical.d.h"

#include "TimeZoneAndCanonicalIterator.d.h"






typedef struct icu4x_TimeZoneAndCanonicalIterator_next_mv1_result {union {TimeZoneAndCanonical ok; }; bool is_ok;} icu4x_TimeZoneAndCanonicalIterator_next_mv1_result;
icu4x_TimeZoneAndCanonicalIterator_next_mv1_result icu4x_TimeZoneAndCanonicalIterator_next_mv1(TimeZoneAndCanonicalIterator* self);


void icu4x_TimeZoneAndCanonicalIterator_destroy_mv1(TimeZoneAndCanonicalIterator* self);





#endif // TimeZoneAndCanonicalIterator_H
