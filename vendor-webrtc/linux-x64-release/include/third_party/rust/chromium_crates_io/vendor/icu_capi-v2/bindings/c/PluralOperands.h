#ifndef PluralOperands_H
#define PluralOperands_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Decimal.d.h"
#include "FixedDecimalParseError.d.h"

#include "PluralOperands.d.h"






typedef struct icu4x_PluralOperands_from_string_mv1_result {union {PluralOperands* ok; FixedDecimalParseError err;}; bool is_ok;} icu4x_PluralOperands_from_string_mv1_result;
icu4x_PluralOperands_from_string_mv1_result icu4x_PluralOperands_from_string_mv1(DiplomatStringView s);

PluralOperands* icu4x_PluralOperands_from_int64_mv1(int64_t i);

PluralOperands* icu4x_PluralOperands_from_fixed_decimal_mv1(const Decimal* x);


void icu4x_PluralOperands_destroy_mv1(PluralOperands* self);





#endif // PluralOperands_H
