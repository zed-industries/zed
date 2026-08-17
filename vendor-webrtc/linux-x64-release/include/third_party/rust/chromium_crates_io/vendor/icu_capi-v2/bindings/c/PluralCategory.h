#ifndef PluralCategory_H
#define PluralCategory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "PluralCategory.d.h"






typedef struct icu4x_PluralCategory_get_for_cldr_string_mv1_result {union {PluralCategory ok; }; bool is_ok;} icu4x_PluralCategory_get_for_cldr_string_mv1_result;
icu4x_PluralCategory_get_for_cldr_string_mv1_result icu4x_PluralCategory_get_for_cldr_string_mv1(DiplomatStringView s);






#endif // PluralCategory_H
