#ifndef UtcOffsetCalculator_H
#define UtcOffsetCalculator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "IsoDate.d.h"
#include "Time.d.h"
#include "TimeZone.d.h"
#include "UtcOffsets.d.h"

#include "UtcOffsetCalculator.d.h"






UtcOffsetCalculator* icu4x_UtcOffsetCalculator_create_mv1(void);

typedef struct icu4x_UtcOffsetCalculator_create_with_provider_mv1_result {union {UtcOffsetCalculator* ok; DataError err;}; bool is_ok;} icu4x_UtcOffsetCalculator_create_with_provider_mv1_result;
icu4x_UtcOffsetCalculator_create_with_provider_mv1_result icu4x_UtcOffsetCalculator_create_with_provider_mv1(const DataProvider* provider);

typedef struct icu4x_UtcOffsetCalculator_compute_offsets_from_time_zone_mv1_result {union {UtcOffsets ok; }; bool is_ok;} icu4x_UtcOffsetCalculator_compute_offsets_from_time_zone_mv1_result;
icu4x_UtcOffsetCalculator_compute_offsets_from_time_zone_mv1_result icu4x_UtcOffsetCalculator_compute_offsets_from_time_zone_mv1(const UtcOffsetCalculator* self, const TimeZone* time_zone, const IsoDate* local_date, const Time* local_time);


void icu4x_UtcOffsetCalculator_destroy_mv1(UtcOffsetCalculator* self);





#endif // UtcOffsetCalculator_H
