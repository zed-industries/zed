#ifndef LocaleExpander_H
#define LocaleExpander_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "TransformResult.d.h"

#include "LocaleExpander.d.h"






LocaleExpander* icu4x_LocaleExpander_create_common_mv1(void);

typedef struct icu4x_LocaleExpander_create_common_with_provider_mv1_result {union {LocaleExpander* ok; DataError err;}; bool is_ok;} icu4x_LocaleExpander_create_common_with_provider_mv1_result;
icu4x_LocaleExpander_create_common_with_provider_mv1_result icu4x_LocaleExpander_create_common_with_provider_mv1(const DataProvider* provider);

LocaleExpander* icu4x_LocaleExpander_create_extended_mv1(void);

typedef struct icu4x_LocaleExpander_create_extended_with_provider_mv1_result {union {LocaleExpander* ok; DataError err;}; bool is_ok;} icu4x_LocaleExpander_create_extended_with_provider_mv1_result;
icu4x_LocaleExpander_create_extended_with_provider_mv1_result icu4x_LocaleExpander_create_extended_with_provider_mv1(const DataProvider* provider);

TransformResult icu4x_LocaleExpander_maximize_mv1(const LocaleExpander* self, Locale* locale);

TransformResult icu4x_LocaleExpander_minimize_mv1(const LocaleExpander* self, Locale* locale);

TransformResult icu4x_LocaleExpander_minimize_favor_script_mv1(const LocaleExpander* self, Locale* locale);


void icu4x_LocaleExpander_destroy_mv1(LocaleExpander* self);





#endif // LocaleExpander_H
