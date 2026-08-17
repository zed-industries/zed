#ifndef BidiClass_H
#define BidiClass_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BidiClass.d.h"






BidiClass icu4x_BidiClass_for_char_mv1(char32_t ch);

typedef struct icu4x_BidiClass_long_name_mv1_result {union {DiplomatStringView ok; }; bool is_ok;} icu4x_BidiClass_long_name_mv1_result;
icu4x_BidiClass_long_name_mv1_result icu4x_BidiClass_long_name_mv1(BidiClass self);

typedef struct icu4x_BidiClass_short_name_mv1_result {union {DiplomatStringView ok; }; bool is_ok;} icu4x_BidiClass_short_name_mv1_result;
icu4x_BidiClass_short_name_mv1_result icu4x_BidiClass_short_name_mv1(BidiClass self);

uint8_t icu4x_BidiClass_to_integer_value_mv1(BidiClass self);

typedef struct icu4x_BidiClass_from_integer_value_mv1_result {union {BidiClass ok; }; bool is_ok;} icu4x_BidiClass_from_integer_value_mv1_result;
icu4x_BidiClass_from_integer_value_mv1_result icu4x_BidiClass_from_integer_value_mv1(uint8_t other);






#endif // BidiClass_H
