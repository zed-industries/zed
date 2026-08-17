#ifndef UtcOffset_H
#define UtcOffset_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "UtcOffset.d.h"






typedef struct icu4x_UtcOffset_from_seconds_mv1_result {union {UtcOffset* ok; }; bool is_ok;} icu4x_UtcOffset_from_seconds_mv1_result;
icu4x_UtcOffset_from_seconds_mv1_result icu4x_UtcOffset_from_seconds_mv1(int32_t seconds);

UtcOffset* icu4x_UtcOffset_from_eighths_of_hour_mv1(int8_t eighths_of_hour);

typedef struct icu4x_UtcOffset_from_string_mv1_result {union {UtcOffset* ok; }; bool is_ok;} icu4x_UtcOffset_from_string_mv1_result;
icu4x_UtcOffset_from_string_mv1_result icu4x_UtcOffset_from_string_mv1(DiplomatStringView offset);

int8_t icu4x_UtcOffset_eighths_of_hour_mv1(const UtcOffset* self);

int32_t icu4x_UtcOffset_seconds_mv1(const UtcOffset* self);

bool icu4x_UtcOffset_is_non_negative_mv1(const UtcOffset* self);

bool icu4x_UtcOffset_is_zero_mv1(const UtcOffset* self);

int32_t icu4x_UtcOffset_hours_part_mv1(const UtcOffset* self);

uint32_t icu4x_UtcOffset_minutes_part_mv1(const UtcOffset* self);

uint32_t icu4x_UtcOffset_seconds_part_mv1(const UtcOffset* self);


void icu4x_UtcOffset_destroy_mv1(UtcOffset* self);





#endif // UtcOffset_H
