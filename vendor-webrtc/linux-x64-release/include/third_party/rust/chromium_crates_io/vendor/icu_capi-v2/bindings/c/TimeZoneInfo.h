#ifndef TimeZoneInfo_H
#define TimeZoneInfo_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "IsoDate.d.h"
#include "IsoDateTime.d.h"
#include "Time.d.h"
#include "TimeZone.d.h"
#include "TimeZoneVariant.d.h"
#include "UtcOffset.d.h"
#include "UtcOffsetCalculator.d.h"

#include "TimeZoneInfo.d.h"






TimeZoneInfo* icu4x_TimeZoneInfo_utc_mv1(void);

TimeZoneInfo* icu4x_TimeZoneInfo_from_parts_mv1(const TimeZone* time_zone_id, const UtcOffset* offset, TimeZoneVariant_option zone_variant);

TimeZone* icu4x_TimeZoneInfo_time_zone_id_mv1(const TimeZoneInfo* self);

TimeZoneInfo* icu4x_TimeZoneInfo_at_time_mv1(const TimeZoneInfo* self, const IsoDate* date, const Time* time);

typedef struct icu4x_TimeZoneInfo_local_time_mv1_result {union {IsoDateTime ok; }; bool is_ok;} icu4x_TimeZoneInfo_local_time_mv1_result;
icu4x_TimeZoneInfo_local_time_mv1_result icu4x_TimeZoneInfo_local_time_mv1(const TimeZoneInfo* self);

TimeZoneInfo* icu4x_TimeZoneInfo_with_zone_variant_mv1(const TimeZoneInfo* self, TimeZoneVariant time_zone_variant);

typedef struct icu4x_TimeZoneInfo_infer_zone_variant_mv1_result { bool is_ok;} icu4x_TimeZoneInfo_infer_zone_variant_mv1_result;
icu4x_TimeZoneInfo_infer_zone_variant_mv1_result icu4x_TimeZoneInfo_infer_zone_variant_mv1(TimeZoneInfo* self, const UtcOffsetCalculator* offset_calculator);

typedef struct icu4x_TimeZoneInfo_zone_variant_mv1_result {union {TimeZoneVariant ok; }; bool is_ok;} icu4x_TimeZoneInfo_zone_variant_mv1_result;
icu4x_TimeZoneInfo_zone_variant_mv1_result icu4x_TimeZoneInfo_zone_variant_mv1(const TimeZoneInfo* self);


void icu4x_TimeZoneInfo_destroy_mv1(TimeZoneInfo* self);





#endif // TimeZoneInfo_H
