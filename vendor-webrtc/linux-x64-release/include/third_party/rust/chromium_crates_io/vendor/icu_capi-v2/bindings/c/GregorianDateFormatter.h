#ifndef GregorianDateFormatter_H
#define GregorianDateFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "DateTimeFormatterLoadError.d.h"
#include "DateTimeLength.d.h"
#include "IsoDate.d.h"
#include "Locale.d.h"

#include "GregorianDateFormatter.d.h"






typedef struct icu4x_GregorianDateFormatter_create_with_length_mv1_result {union {GregorianDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_GregorianDateFormatter_create_with_length_mv1_result;
icu4x_GregorianDateFormatter_create_with_length_mv1_result icu4x_GregorianDateFormatter_create_with_length_mv1(const Locale* locale, DateTimeLength length);

typedef struct icu4x_GregorianDateFormatter_create_with_length_and_provider_mv1_result {union {GregorianDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_GregorianDateFormatter_create_with_length_and_provider_mv1_result;
icu4x_GregorianDateFormatter_create_with_length_and_provider_mv1_result icu4x_GregorianDateFormatter_create_with_length_and_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength length);

void icu4x_GregorianDateFormatter_format_iso_mv1(const GregorianDateFormatter* self, const IsoDate* value, DiplomatWrite* write);


void icu4x_GregorianDateFormatter_destroy_mv1(GregorianDateFormatter* self);





#endif // GregorianDateFormatter_H
