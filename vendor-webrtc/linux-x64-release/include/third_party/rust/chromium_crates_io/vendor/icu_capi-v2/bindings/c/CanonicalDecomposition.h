#ifndef CanonicalDecomposition_H
#define CanonicalDecomposition_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Decomposed.d.h"

#include "CanonicalDecomposition.d.h"






CanonicalDecomposition* icu4x_CanonicalDecomposition_create_mv1(void);

typedef struct icu4x_CanonicalDecomposition_create_with_provider_mv1_result {union {CanonicalDecomposition* ok; DataError err;}; bool is_ok;} icu4x_CanonicalDecomposition_create_with_provider_mv1_result;
icu4x_CanonicalDecomposition_create_with_provider_mv1_result icu4x_CanonicalDecomposition_create_with_provider_mv1(const DataProvider* provider);

Decomposed icu4x_CanonicalDecomposition_decompose_mv1(const CanonicalDecomposition* self, char32_t c);


void icu4x_CanonicalDecomposition_destroy_mv1(CanonicalDecomposition* self);





#endif // CanonicalDecomposition_H
