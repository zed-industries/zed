#ifndef CaseMapper_H
#define CaseMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointSetBuilder.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "TitlecaseOptionsV1.d.h"

#include "CaseMapper.d.h"






CaseMapper* icu4x_CaseMapper_create_mv1(void);

typedef struct icu4x_CaseMapper_create_with_provider_mv1_result {union {CaseMapper* ok; DataError err;}; bool is_ok;} icu4x_CaseMapper_create_with_provider_mv1_result;
icu4x_CaseMapper_create_with_provider_mv1_result icu4x_CaseMapper_create_with_provider_mv1(const DataProvider* provider);

void icu4x_CaseMapper_lowercase_mv1(const CaseMapper* self, DiplomatStringView s, const Locale* locale, DiplomatWrite* write);

void icu4x_CaseMapper_uppercase_mv1(const CaseMapper* self, DiplomatStringView s, const Locale* locale, DiplomatWrite* write);

void icu4x_CaseMapper_titlecase_segment_with_only_case_data_v1_mv1(const CaseMapper* self, DiplomatStringView s, const Locale* locale, TitlecaseOptionsV1 options, DiplomatWrite* write);

void icu4x_CaseMapper_fold_mv1(const CaseMapper* self, DiplomatStringView s, DiplomatWrite* write);

void icu4x_CaseMapper_fold_turkic_mv1(const CaseMapper* self, DiplomatStringView s, DiplomatWrite* write);

void icu4x_CaseMapper_add_case_closure_to_mv1(const CaseMapper* self, char32_t c, CodePointSetBuilder* builder);

char32_t icu4x_CaseMapper_simple_lowercase_mv1(const CaseMapper* self, char32_t ch);

char32_t icu4x_CaseMapper_simple_uppercase_mv1(const CaseMapper* self, char32_t ch);

char32_t icu4x_CaseMapper_simple_titlecase_mv1(const CaseMapper* self, char32_t ch);

char32_t icu4x_CaseMapper_simple_fold_mv1(const CaseMapper* self, char32_t ch);

char32_t icu4x_CaseMapper_simple_fold_turkic_mv1(const CaseMapper* self, char32_t ch);


void icu4x_CaseMapper_destroy_mv1(CaseMapper* self);





#endif // CaseMapper_H
