#ifndef CanonicalCombiningClassMap_H
#define CanonicalCombiningClassMap_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "CanonicalCombiningClassMap.d.h"






CanonicalCombiningClassMap* icu4x_CanonicalCombiningClassMap_create_mv1(void);

typedef struct icu4x_CanonicalCombiningClassMap_create_with_provider_mv1_result {union {CanonicalCombiningClassMap* ok; DataError err;}; bool is_ok;} icu4x_CanonicalCombiningClassMap_create_with_provider_mv1_result;
icu4x_CanonicalCombiningClassMap_create_with_provider_mv1_result icu4x_CanonicalCombiningClassMap_create_with_provider_mv1(const DataProvider* provider);

uint8_t icu4x_CanonicalCombiningClassMap_get_mv1(const CanonicalCombiningClassMap* self, char32_t ch);


void icu4x_CanonicalCombiningClassMap_destroy_mv1(CanonicalCombiningClassMap* self);





#endif // CanonicalCombiningClassMap_H
