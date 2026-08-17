#ifndef LocaleDirectionality_H
#define LocaleDirectionality_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "LocaleDirection.d.h"

#include "LocaleDirectionality.d.h"






LocaleDirectionality* icu4x_LocaleDirectionality_create_common_mv1(void);

typedef struct icu4x_LocaleDirectionality_create_common_with_provider_mv1_result {union {LocaleDirectionality* ok; DataError err;}; bool is_ok;} icu4x_LocaleDirectionality_create_common_with_provider_mv1_result;
icu4x_LocaleDirectionality_create_common_with_provider_mv1_result icu4x_LocaleDirectionality_create_common_with_provider_mv1(const DataProvider* provider);

LocaleDirectionality* icu4x_LocaleDirectionality_create_extended_mv1(void);

typedef struct icu4x_LocaleDirectionality_create_extended_with_provider_mv1_result {union {LocaleDirectionality* ok; DataError err;}; bool is_ok;} icu4x_LocaleDirectionality_create_extended_with_provider_mv1_result;
icu4x_LocaleDirectionality_create_extended_with_provider_mv1_result icu4x_LocaleDirectionality_create_extended_with_provider_mv1(const DataProvider* provider);

LocaleDirection icu4x_LocaleDirectionality_get_mv1(const LocaleDirectionality* self, const Locale* locale);

bool icu4x_LocaleDirectionality_is_left_to_right_mv1(const LocaleDirectionality* self, const Locale* locale);

bool icu4x_LocaleDirectionality_is_right_to_left_mv1(const LocaleDirectionality* self, const Locale* locale);


void icu4x_LocaleDirectionality_destroy_mv1(LocaleDirectionality* self);





#endif // LocaleDirectionality_H
