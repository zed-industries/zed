#ifndef Time_H
#define Time_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CalendarError.d.h"
#include "CalendarParseError.d.h"

#include "Time.d.h"






typedef struct icu4x_Time_create_mv1_result {union {Time* ok; CalendarError err;}; bool is_ok;} icu4x_Time_create_mv1_result;
icu4x_Time_create_mv1_result icu4x_Time_create_mv1(uint8_t hour, uint8_t minute, uint8_t second, uint32_t subsecond);

typedef struct icu4x_Time_from_string_mv1_result {union {Time* ok; CalendarParseError err;}; bool is_ok;} icu4x_Time_from_string_mv1_result;
icu4x_Time_from_string_mv1_result icu4x_Time_from_string_mv1(DiplomatStringView v);

typedef struct icu4x_Time_midnight_mv1_result {union {Time* ok; CalendarError err;}; bool is_ok;} icu4x_Time_midnight_mv1_result;
icu4x_Time_midnight_mv1_result icu4x_Time_midnight_mv1(void);

uint8_t icu4x_Time_hour_mv1(const Time* self);

uint8_t icu4x_Time_minute_mv1(const Time* self);

uint8_t icu4x_Time_second_mv1(const Time* self);

uint32_t icu4x_Time_subsecond_mv1(const Time* self);


void icu4x_Time_destroy_mv1(Time* self);





#endif // Time_H
