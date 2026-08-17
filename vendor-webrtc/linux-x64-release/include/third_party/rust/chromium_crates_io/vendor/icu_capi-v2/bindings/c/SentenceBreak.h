#ifndef SentenceBreak_H
#define SentenceBreak_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "SentenceBreak.d.h"






SentenceBreak icu4x_SentenceBreak_for_char_mv1(char32_t ch);

typedef struct icu4x_SentenceBreak_long_name_mv1_result {union {DiplomatStringView ok; }; bool is_ok;} icu4x_SentenceBreak_long_name_mv1_result;
icu4x_SentenceBreak_long_name_mv1_result icu4x_SentenceBreak_long_name_mv1(SentenceBreak self);

typedef struct icu4x_SentenceBreak_short_name_mv1_result {union {DiplomatStringView ok; }; bool is_ok;} icu4x_SentenceBreak_short_name_mv1_result;
icu4x_SentenceBreak_short_name_mv1_result icu4x_SentenceBreak_short_name_mv1(SentenceBreak self);

uint8_t icu4x_SentenceBreak_to_integer_value_mv1(SentenceBreak self);

typedef struct icu4x_SentenceBreak_from_integer_value_mv1_result {union {SentenceBreak ok; }; bool is_ok;} icu4x_SentenceBreak_from_integer_value_mv1_result;
icu4x_SentenceBreak_from_integer_value_mv1_result icu4x_SentenceBreak_from_integer_value_mv1(uint8_t other);






#endif // SentenceBreak_H
