#ifndef LocaleCanonicalizer_H
#define LocaleCanonicalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "TransformResult.d.h"

#include "LocaleCanonicalizer.d.h"






LocaleCanonicalizer* icu4x_LocaleCanonicalizer_create_common_mv1(void);

typedef struct icu4x_LocaleCanonicalizer_create_common_with_provider_mv1_result {union {LocaleCanonicalizer* ok; DataError err;}; bool is_ok;} icu4x_LocaleCanonicalizer_create_common_with_provider_mv1_result;
icu4x_LocaleCanonicalizer_create_common_with_provider_mv1_result icu4x_LocaleCanonicalizer_create_common_with_provider_mv1(const DataProvider* provider);

LocaleCanonicalizer* icu4x_LocaleCanonicalizer_create_extended_mv1(void);

typedef struct icu4x_LocaleCanonicalizer_create_extended_with_provider_mv1_result {union {LocaleCanonicalizer* ok; DataError err;}; bool is_ok;} icu4x_LocaleCanonicalizer_create_extended_with_provider_mv1_result;
icu4x_LocaleCanonicalizer_create_extended_with_provider_mv1_result icu4x_LocaleCanonicalizer_create_extended_with_provider_mv1(const DataProvider* provider);

TransformResult icu4x_LocaleCanonicalizer_canonicalize_mv1(const LocaleCanonicalizer* self, Locale* locale);


void icu4x_LocaleCanonicalizer_destroy_mv1(LocaleCanonicalizer* self);





#endif // LocaleCanonicalizer_H
