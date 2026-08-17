#ifndef TimeZoneIterator_H
#define TimeZoneIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "TimeZone.d.h"

#include "TimeZoneIterator.d.h"






TimeZone* icu4x_TimeZoneIterator_next_mv1(TimeZoneIterator* self);


void icu4x_TimeZoneIterator_destroy_mv1(TimeZoneIterator* self);





#endif // TimeZoneIterator_H
