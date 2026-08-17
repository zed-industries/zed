#ifndef LineBreak_H
#define LineBreak_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "LineBreak.d.h"






LineBreak icu4x_LineBreak_for_char_mv1(char32_t ch);

typedef struct icu4x_LineBreak_long_name_mv1_result {union {DiplomatStringView ok; }; bool is_ok;} icu4x_LineBreak_long_name_mv1_result;
icu4x_LineBreak_long_name_mv1_result icu4x_LineBreak_long_name_mv1(LineBreak self);

typedef struct icu4x_LineBreak_short_name_mv1_result {union {DiplomatStringView ok; }; bool is_ok;} icu4x_LineBreak_short_name_mv1_result;
icu4x_LineBreak_short_name_mv1_result icu4x_LineBreak_short_name_mv1(LineBreak self);

uint8_t icu4x_LineBreak_to_integer_value_mv1(LineBreak self);

typedef struct icu4x_LineBreak_from_integer_value_mv1_result {union {LineBreak ok; }; bool is_ok;} icu4x_LineBreak_from_integer_value_mv1_result;
icu4x_LineBreak_from_integer_value_mv1_result icu4x_LineBreak_from_integer_value_mv1(uint8_t other);






#endif // LineBreak_H
