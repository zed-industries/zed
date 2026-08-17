#ifndef Collator_H
#define Collator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CollatorOptionsV1.d.h"
#include "CollatorResolvedOptionsV1.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"

#include "Collator.d.h"






typedef struct icu4x_Collator_create_v1_mv1_result {union {Collator* ok; DataError err;}; bool is_ok;} icu4x_Collator_create_v1_mv1_result;
icu4x_Collator_create_v1_mv1_result icu4x_Collator_create_v1_mv1(const Locale* locale, CollatorOptionsV1 options);

typedef struct icu4x_Collator_create_v1_with_provider_mv1_result {union {Collator* ok; DataError err;}; bool is_ok;} icu4x_Collator_create_v1_with_provider_mv1_result;
icu4x_Collator_create_v1_with_provider_mv1_result icu4x_Collator_create_v1_with_provider_mv1(const DataProvider* provider, const Locale* locale, CollatorOptionsV1 options);

int8_t icu4x_Collator_compare_utf8_mv1(const Collator* self, DiplomatStringView left, DiplomatStringView right);

int8_t icu4x_Collator_compare_utf16_mv1(const Collator* self, DiplomatString16View left, DiplomatString16View right);

CollatorResolvedOptionsV1 icu4x_Collator_resolved_options_v1_mv1(const Collator* self);


void icu4x_Collator_destroy_mv1(Collator* self);





#endif // Collator_H
