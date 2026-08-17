#ifndef TimeZoneAndCanonicalAndNormalizedIterator_H
#define TimeZoneAndCanonicalAndNormalizedIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "TimeZoneAndCanonicalAndNormalized.d.h"

#include "TimeZoneAndCanonicalAndNormalizedIterator.d.h"






typedef struct icu4x_TimeZoneAndCanonicalAndNormalizedIterator_next_mv1_result {union {TimeZoneAndCanonicalAndNormalized ok; }; bool is_ok;} icu4x_TimeZoneAndCanonicalAndNormalizedIterator_next_mv1_result;
icu4x_TimeZoneAndCanonicalAndNormalizedIterator_next_mv1_result icu4x_TimeZoneAndCanonicalAndNormalizedIterator_next_mv1(TimeZoneAndCanonicalAndNormalizedIterator* self);


void icu4x_TimeZoneAndCanonicalAndNormalizedIterator_destroy_mv1(TimeZoneAndCanonicalAndNormalizedIterator* self);





#endif // TimeZoneAndCanonicalAndNormalizedIterator_H
