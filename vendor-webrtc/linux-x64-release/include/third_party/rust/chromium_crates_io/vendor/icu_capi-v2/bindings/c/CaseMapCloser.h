#ifndef CaseMapCloser_H
#define CaseMapCloser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointSetBuilder.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"

#include "CaseMapCloser.d.h"






typedef struct icu4x_CaseMapCloser_create_mv1_result {union {CaseMapCloser* ok; DataError err;}; bool is_ok;} icu4x_CaseMapCloser_create_mv1_result;
icu4x_CaseMapCloser_create_mv1_result icu4x_CaseMapCloser_create_mv1(void);

typedef struct icu4x_CaseMapCloser_create_with_provider_mv1_result {union {CaseMapCloser* ok; DataError err;}; bool is_ok;} icu4x_CaseMapCloser_create_with_provider_mv1_result;
icu4x_CaseMapCloser_create_with_provider_mv1_result icu4x_CaseMapCloser_create_with_provider_mv1(const DataProvider* provider);

void icu4x_CaseMapCloser_add_case_closure_to_mv1(const CaseMapCloser* self, char32_t c, CodePointSetBuilder* builder);

bool icu4x_CaseMapCloser_add_string_case_closure_to_mv1(const CaseMapCloser* self, DiplomatStringView s, CodePointSetBuilder* builder);


void icu4x_CaseMapCloser_destroy_mv1(CaseMapCloser* self);





#endif // CaseMapCloser_H
