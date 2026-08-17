#ifndef Calendar_H
#define Calendar_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AnyCalendarKind.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"

#include "Calendar.d.h"






typedef struct icu4x_Calendar_create_for_locale_mv1_result {union {Calendar* ok; DataError err;}; bool is_ok;} icu4x_Calendar_create_for_locale_mv1_result;
icu4x_Calendar_create_for_locale_mv1_result icu4x_Calendar_create_for_locale_mv1(const Locale* locale);

typedef struct icu4x_Calendar_create_for_kind_mv1_result {union {Calendar* ok; DataError err;}; bool is_ok;} icu4x_Calendar_create_for_kind_mv1_result;
icu4x_Calendar_create_for_kind_mv1_result icu4x_Calendar_create_for_kind_mv1(AnyCalendarKind kind);

typedef struct icu4x_Calendar_create_for_locale_with_provider_mv1_result {union {Calendar* ok; DataError err;}; bool is_ok;} icu4x_Calendar_create_for_locale_with_provider_mv1_result;
icu4x_Calendar_create_for_locale_with_provider_mv1_result icu4x_Calendar_create_for_locale_with_provider_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_Calendar_create_for_kind_with_provider_mv1_result {union {Calendar* ok; DataError err;}; bool is_ok;} icu4x_Calendar_create_for_kind_with_provider_mv1_result;
icu4x_Calendar_create_for_kind_with_provider_mv1_result icu4x_Calendar_create_for_kind_with_provider_mv1(const DataProvider* provider, AnyCalendarKind kind);

AnyCalendarKind icu4x_Calendar_kind_mv1(const Calendar* self);


void icu4x_Calendar_destroy_mv1(Calendar* self);





#endif // Calendar_H
