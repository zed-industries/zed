#ifndef DecimalFormatter_H
#define DecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Decimal.d.h"
#include "DecimalGroupingStrategy.d.h"
#include "Locale.d.h"

#include "DecimalFormatter.d.h"






typedef struct icu4x_DecimalFormatter_create_with_grouping_strategy_mv1_result {union {DecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_DecimalFormatter_create_with_grouping_strategy_mv1_result;
icu4x_DecimalFormatter_create_with_grouping_strategy_mv1_result icu4x_DecimalFormatter_create_with_grouping_strategy_mv1(const Locale* locale, DecimalGroupingStrategy_option grouping_strategy);

typedef struct icu4x_DecimalFormatter_create_with_grouping_strategy_and_provider_mv1_result {union {DecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_DecimalFormatter_create_with_grouping_strategy_and_provider_mv1_result;
icu4x_DecimalFormatter_create_with_grouping_strategy_and_provider_mv1_result icu4x_DecimalFormatter_create_with_grouping_strategy_and_provider_mv1(const DataProvider* provider, const Locale* locale, DecimalGroupingStrategy_option grouping_strategy);

typedef struct icu4x_DecimalFormatter_create_with_manual_data_mv1_result {union {DecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_DecimalFormatter_create_with_manual_data_mv1_result;
icu4x_DecimalFormatter_create_with_manual_data_mv1_result icu4x_DecimalFormatter_create_with_manual_data_mv1(DiplomatStringView plus_sign_prefix, DiplomatStringView plus_sign_suffix, DiplomatStringView minus_sign_prefix, DiplomatStringView minus_sign_suffix, DiplomatStringView decimal_separator, DiplomatStringView grouping_separator, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, DiplomatCharView digits, DecimalGroupingStrategy_option grouping_strategy);

void icu4x_DecimalFormatter_format_mv1(const DecimalFormatter* self, const Decimal* value, DiplomatWrite* write);


void icu4x_DecimalFormatter_destroy_mv1(DecimalFormatter* self);





#endif // DecimalFormatter_H
