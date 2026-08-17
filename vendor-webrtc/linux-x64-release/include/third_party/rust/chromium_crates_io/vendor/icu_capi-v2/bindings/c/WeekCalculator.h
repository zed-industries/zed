#ifndef WeekCalculator_H
#define WeekCalculator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "Weekday.d.h"
#include "WeekendContainsDay.d.h"

#include "WeekCalculator.d.h"






typedef struct icu4x_WeekCalculator_create_mv1_result {union {WeekCalculator* ok; DataError err;}; bool is_ok;} icu4x_WeekCalculator_create_mv1_result;
icu4x_WeekCalculator_create_mv1_result icu4x_WeekCalculator_create_mv1(const Locale* locale);

typedef struct icu4x_WeekCalculator_create_with_provider_mv1_result {union {WeekCalculator* ok; DataError err;}; bool is_ok;} icu4x_WeekCalculator_create_with_provider_mv1_result;
icu4x_WeekCalculator_create_with_provider_mv1_result icu4x_WeekCalculator_create_with_provider_mv1(const DataProvider* provider, const Locale* locale);

WeekCalculator* icu4x_WeekCalculator_from_first_day_of_week_and_min_week_days_mv1(Weekday first_weekday, uint8_t min_week_days);

Weekday icu4x_WeekCalculator_first_weekday_mv1(const WeekCalculator* self);

uint8_t icu4x_WeekCalculator_min_week_days_mv1(const WeekCalculator* self);

WeekendContainsDay icu4x_WeekCalculator_weekend_mv1(const WeekCalculator* self);


void icu4x_WeekCalculator_destroy_mv1(WeekCalculator* self);





#endif // WeekCalculator_H
