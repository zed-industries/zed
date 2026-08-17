#ifndef DateTimeFormatter_H
#define DateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "Date.d.h"
#include "DateTimeAlignment.d.h"
#include "DateTimeFormatterLoadError.d.h"
#include "DateTimeLength.d.h"
#include "DateTimeMismatchedCalendarError.d.h"
#include "IsoDate.d.h"
#include "Locale.d.h"
#include "Time.d.h"
#include "TimePrecision.d.h"
#include "YearStyle.d.h"

#include "DateTimeFormatter.d.h"






typedef struct icu4x_DateTimeFormatter_create_dt_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_dt_mv1_result;
icu4x_DateTimeFormatter_create_dt_mv1_result icu4x_DateTimeFormatter_create_dt_mv1(const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

typedef struct icu4x_DateTimeFormatter_create_dt_with_provider_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_dt_with_provider_mv1_result;
icu4x_DateTimeFormatter_create_dt_with_provider_mv1_result icu4x_DateTimeFormatter_create_dt_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

typedef struct icu4x_DateTimeFormatter_create_mdt_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_mdt_mv1_result;
icu4x_DateTimeFormatter_create_mdt_mv1_result icu4x_DateTimeFormatter_create_mdt_mv1(const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

typedef struct icu4x_DateTimeFormatter_create_mdt_with_provider_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_mdt_with_provider_mv1_result;
icu4x_DateTimeFormatter_create_mdt_with_provider_mv1_result icu4x_DateTimeFormatter_create_mdt_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

typedef struct icu4x_DateTimeFormatter_create_ymdt_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_ymdt_mv1_result;
icu4x_DateTimeFormatter_create_ymdt_mv1_result icu4x_DateTimeFormatter_create_ymdt_mv1(const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_DateTimeFormatter_create_ymdt_with_provider_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_ymdt_with_provider_mv1_result;
icu4x_DateTimeFormatter_create_ymdt_with_provider_mv1_result icu4x_DateTimeFormatter_create_ymdt_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_DateTimeFormatter_create_det_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_det_mv1_result;
icu4x_DateTimeFormatter_create_det_mv1_result icu4x_DateTimeFormatter_create_det_mv1(const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

typedef struct icu4x_DateTimeFormatter_create_det_with_provider_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_det_with_provider_mv1_result;
icu4x_DateTimeFormatter_create_det_with_provider_mv1_result icu4x_DateTimeFormatter_create_det_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

typedef struct icu4x_DateTimeFormatter_create_mdet_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_mdet_mv1_result;
icu4x_DateTimeFormatter_create_mdet_mv1_result icu4x_DateTimeFormatter_create_mdet_mv1(const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

typedef struct icu4x_DateTimeFormatter_create_mdet_with_provider_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_mdet_with_provider_mv1_result;
icu4x_DateTimeFormatter_create_mdet_with_provider_mv1_result icu4x_DateTimeFormatter_create_mdet_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

typedef struct icu4x_DateTimeFormatter_create_ymdet_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_ymdet_mv1_result;
icu4x_DateTimeFormatter_create_ymdet_mv1_result icu4x_DateTimeFormatter_create_ymdet_mv1(const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_DateTimeFormatter_create_ymdet_with_provider_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_ymdet_with_provider_mv1_result;
icu4x_DateTimeFormatter_create_ymdet_with_provider_mv1_result icu4x_DateTimeFormatter_create_ymdet_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_DateTimeFormatter_create_et_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_et_mv1_result;
icu4x_DateTimeFormatter_create_et_mv1_result icu4x_DateTimeFormatter_create_et_mv1(const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

typedef struct icu4x_DateTimeFormatter_create_et_with_provider_mv1_result {union {DateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_et_with_provider_mv1_result;
icu4x_DateTimeFormatter_create_et_with_provider_mv1_result icu4x_DateTimeFormatter_create_et_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, TimePrecision_option time_precision, DateTimeAlignment_option alignment);

void icu4x_DateTimeFormatter_format_iso_mv1(const DateTimeFormatter* self, const IsoDate* date, const Time* time, DiplomatWrite* write);

typedef struct icu4x_DateTimeFormatter_format_same_calendar_mv1_result {union { DateTimeMismatchedCalendarError err;}; bool is_ok;} icu4x_DateTimeFormatter_format_same_calendar_mv1_result;
icu4x_DateTimeFormatter_format_same_calendar_mv1_result icu4x_DateTimeFormatter_format_same_calendar_mv1(const DateTimeFormatter* self, const Date* date, const Time* time, DiplomatWrite* write);


void icu4x_DateTimeFormatter_destroy_mv1(DateTimeFormatter* self);





#endif // DateTimeFormatter_H
