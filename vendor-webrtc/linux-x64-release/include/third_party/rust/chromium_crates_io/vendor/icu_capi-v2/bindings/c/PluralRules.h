#ifndef PluralRules_H
#define PluralRules_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "PluralCategories.d.h"
#include "PluralCategory.d.h"
#include "PluralOperands.d.h"

#include "PluralRules.d.h"






typedef struct icu4x_PluralRules_create_cardinal_mv1_result {union {PluralRules* ok; DataError err;}; bool is_ok;} icu4x_PluralRules_create_cardinal_mv1_result;
icu4x_PluralRules_create_cardinal_mv1_result icu4x_PluralRules_create_cardinal_mv1(const Locale* locale);

typedef struct icu4x_PluralRules_create_cardinal_with_provider_mv1_result {union {PluralRules* ok; DataError err;}; bool is_ok;} icu4x_PluralRules_create_cardinal_with_provider_mv1_result;
icu4x_PluralRules_create_cardinal_with_provider_mv1_result icu4x_PluralRules_create_cardinal_with_provider_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_PluralRules_create_ordinal_mv1_result {union {PluralRules* ok; DataError err;}; bool is_ok;} icu4x_PluralRules_create_ordinal_mv1_result;
icu4x_PluralRules_create_ordinal_mv1_result icu4x_PluralRules_create_ordinal_mv1(const Locale* locale);

typedef struct icu4x_PluralRules_create_ordinal_with_provider_mv1_result {union {PluralRules* ok; DataError err;}; bool is_ok;} icu4x_PluralRules_create_ordinal_with_provider_mv1_result;
icu4x_PluralRules_create_ordinal_with_provider_mv1_result icu4x_PluralRules_create_ordinal_with_provider_mv1(const DataProvider* provider, const Locale* locale);

PluralCategory icu4x_PluralRules_category_for_mv1(const PluralRules* self, const PluralOperands* op);

PluralCategories icu4x_PluralRules_categories_mv1(const PluralRules* self);


void icu4x_PluralRules_destroy_mv1(PluralRules* self);





#endif // PluralRules_H
