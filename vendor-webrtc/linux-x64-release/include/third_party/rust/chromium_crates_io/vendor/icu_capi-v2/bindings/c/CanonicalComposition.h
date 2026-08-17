#ifndef CanonicalComposition_H
#define CanonicalComposition_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "CanonicalComposition.d.h"






CanonicalComposition* icu4x_CanonicalComposition_create_mv1(void);

typedef struct icu4x_CanonicalComposition_create_with_provider_mv1_result {union {CanonicalComposition* ok; DataError err;}; bool is_ok;} icu4x_CanonicalComposition_create_with_provider_mv1_result;
icu4x_CanonicalComposition_create_with_provider_mv1_result icu4x_CanonicalComposition_create_with_provider_mv1(const DataProvider* provider);

char32_t icu4x_CanonicalComposition_compose_mv1(const CanonicalComposition* self, char32_t starter, char32_t second);


void icu4x_CanonicalComposition_destroy_mv1(CanonicalComposition* self);





#endif // CanonicalComposition_H
