#ifndef TitlecaseMapper_H
#define TitlecaseMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "TitlecaseOptionsV1.d.h"

#include "TitlecaseMapper.d.h"






typedef struct icu4x_TitlecaseMapper_create_mv1_result {union {TitlecaseMapper* ok; DataError err;}; bool is_ok;} icu4x_TitlecaseMapper_create_mv1_result;
icu4x_TitlecaseMapper_create_mv1_result icu4x_TitlecaseMapper_create_mv1(void);

typedef struct icu4x_TitlecaseMapper_create_with_provider_mv1_result {union {TitlecaseMapper* ok; DataError err;}; bool is_ok;} icu4x_TitlecaseMapper_create_with_provider_mv1_result;
icu4x_TitlecaseMapper_create_with_provider_mv1_result icu4x_TitlecaseMapper_create_with_provider_mv1(const DataProvider* provider);

void icu4x_TitlecaseMapper_titlecase_segment_v1_mv1(const TitlecaseMapper* self, DiplomatStringView s, const Locale* locale, TitlecaseOptionsV1 options, DiplomatWrite* write);


void icu4x_TitlecaseMapper_destroy_mv1(TitlecaseMapper* self);





#endif // TitlecaseMapper_H
