#ifndef NoCalendarFormatter_H
#define NoCalendarFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "DateTimeFormatterLoadError.d.h"
#include "DateTimeLength.d.h"
#include "Locale.d.h"
#include "Time.d.h"

#include "NoCalendarFormatter.d.h"






typedef struct icu4x_NoCalendarFormatter_create_with_length_mv1_result {union {NoCalendarFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NoCalendarFormatter_create_with_length_mv1_result;
icu4x_NoCalendarFormatter_create_with_length_mv1_result icu4x_NoCalendarFormatter_create_with_length_mv1(const Locale* locale, DateTimeLength length);

typedef struct icu4x_NoCalendarFormatter_create_with_length_and_provider_mv1_result {union {NoCalendarFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NoCalendarFormatter_create_with_length_and_provider_mv1_result;
icu4x_NoCalendarFormatter_create_with_length_and_provider_mv1_result icu4x_NoCalendarFormatter_create_with_length_and_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength length);

void icu4x_NoCalendarFormatter_format_mv1(const NoCalendarFormatter* self, const Time* value, DiplomatWrite* write);


void icu4x_NoCalendarFormatter_destroy_mv1(NoCalendarFormatter* self);





#endif // NoCalendarFormatter_H
