#ifndef GeneralCategory_H
#define GeneralCategory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "GeneralCategoryGroup.d.h"

#include "GeneralCategory.d.h"






GeneralCategory icu4x_GeneralCategory_for_char_mv1(char32_t ch);

typedef struct icu4x_GeneralCategory_long_name_mv1_result {union {DiplomatStringView ok; }; bool is_ok;} icu4x_GeneralCategory_long_name_mv1_result;
icu4x_GeneralCategory_long_name_mv1_result icu4x_GeneralCategory_long_name_mv1(GeneralCategory self);

typedef struct icu4x_GeneralCategory_short_name_mv1_result {union {DiplomatStringView ok; }; bool is_ok;} icu4x_GeneralCategory_short_name_mv1_result;
icu4x_GeneralCategory_short_name_mv1_result icu4x_GeneralCategory_short_name_mv1(GeneralCategory self);

uint8_t icu4x_GeneralCategory_to_integer_value_mv1(GeneralCategory self);

GeneralCategoryGroup icu4x_GeneralCategory_to_group_mv1(GeneralCategory self);

typedef struct icu4x_GeneralCategory_from_integer_value_mv1_result {union {GeneralCategory ok; }; bool is_ok;} icu4x_GeneralCategory_from_integer_value_mv1_result;
icu4x_GeneralCategory_from_integer_value_mv1_result icu4x_GeneralCategory_from_integer_value_mv1(uint8_t other);






#endif // GeneralCategory_H
