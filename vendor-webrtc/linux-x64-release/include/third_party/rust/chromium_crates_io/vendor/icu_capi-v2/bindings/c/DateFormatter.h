#ifndef DateFormatter_H
#define DateFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Calendar.d.h"
#include "DataProvider.d.h"
#include "Date.d.h"
#include "DateTimeFormatError.d.h"
#include "DateTimeFormatterLoadError.d.h"
#include "DateTimeLength.d.h"
#include "IsoDate.d.h"
#include "Locale.d.h"

#include "DateFormatter.d.h"






typedef struct icu4x_DateFormatter_create_with_length_mv1_result {union {DateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateFormatter_create_with_length_mv1_result;
icu4x_DateFormatter_create_with_length_mv1_result icu4x_DateFormatter_create_with_length_mv1(const Locale* locale, DateTimeLength length);

typedef struct icu4x_DateFormatter_create_with_length_and_provider_mv1_result {union {DateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateFormatter_create_with_length_and_provider_mv1_result;
icu4x_DateFormatter_create_with_length_and_provider_mv1_result icu4x_DateFormatter_create_with_length_and_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength length);

typedef struct icu4x_DateFormatter_format_mv1_result {union { DateTimeFormatError err;}; bool is_ok;} icu4x_DateFormatter_format_mv1_result;
icu4x_DateFormatter_format_mv1_result icu4x_DateFormatter_format_mv1(const DateFormatter* self, const Date* value, DiplomatWrite* write);

typedef struct icu4x_DateFormatter_format_iso_mv1_result {union { DateTimeFormatError err;}; bool is_ok;} icu4x_DateFormatter_format_iso_mv1_result;
icu4x_DateFormatter_format_iso_mv1_result icu4x_DateFormatter_format_iso_mv1(const DateFormatter* self, const IsoDate* value, DiplomatWrite* write);

Calendar* icu4x_DateFormatter_calendar_mv1(const DateFormatter* self);


void icu4x_DateFormatter_destroy_mv1(DateFormatter* self);





#endif // DateFormatter_H
