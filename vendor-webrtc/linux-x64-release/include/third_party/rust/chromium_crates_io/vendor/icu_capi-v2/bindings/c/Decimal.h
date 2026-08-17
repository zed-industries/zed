#ifndef Decimal_H
#define Decimal_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "FixedDecimalParseError.d.h"
#include "FixedDecimalRoundingIncrement.d.h"
#include "FixedDecimalSign.d.h"
#include "FixedDecimalSignDisplay.d.h"
#include "FixedDecimalSignedRoundingMode.d.h"

#include "Decimal.d.h"






Decimal* icu4x_Decimal_from_int32_mv1(int32_t v);

Decimal* icu4x_Decimal_from_uint32_mv1(uint32_t v);

Decimal* icu4x_Decimal_from_int64_mv1(int64_t v);

Decimal* icu4x_Decimal_from_uint64_mv1(uint64_t v);

typedef struct icu4x_Decimal_from_double_with_integer_precision_mv1_result {union {Decimal* ok; }; bool is_ok;} icu4x_Decimal_from_double_with_integer_precision_mv1_result;
icu4x_Decimal_from_double_with_integer_precision_mv1_result icu4x_Decimal_from_double_with_integer_precision_mv1(double f);

typedef struct icu4x_Decimal_from_double_with_lower_magnitude_mv1_result {union {Decimal* ok; }; bool is_ok;} icu4x_Decimal_from_double_with_lower_magnitude_mv1_result;
icu4x_Decimal_from_double_with_lower_magnitude_mv1_result icu4x_Decimal_from_double_with_lower_magnitude_mv1(double f, int16_t magnitude);

typedef struct icu4x_Decimal_from_double_with_significant_digits_mv1_result {union {Decimal* ok; }; bool is_ok;} icu4x_Decimal_from_double_with_significant_digits_mv1_result;
icu4x_Decimal_from_double_with_significant_digits_mv1_result icu4x_Decimal_from_double_with_significant_digits_mv1(double f, uint8_t digits);

typedef struct icu4x_Decimal_from_double_with_round_trip_precision_mv1_result {union {Decimal* ok; }; bool is_ok;} icu4x_Decimal_from_double_with_round_trip_precision_mv1_result;
icu4x_Decimal_from_double_with_round_trip_precision_mv1_result icu4x_Decimal_from_double_with_round_trip_precision_mv1(double f);

typedef struct icu4x_Decimal_from_string_mv1_result {union {Decimal* ok; FixedDecimalParseError err;}; bool is_ok;} icu4x_Decimal_from_string_mv1_result;
icu4x_Decimal_from_string_mv1_result icu4x_Decimal_from_string_mv1(DiplomatStringView v);

uint8_t icu4x_Decimal_digit_at_mv1(const Decimal* self, int16_t magnitude);

int16_t icu4x_Decimal_magnitude_start_mv1(const Decimal* self);

int16_t icu4x_Decimal_magnitude_end_mv1(const Decimal* self);

int16_t icu4x_Decimal_nonzero_magnitude_start_mv1(const Decimal* self);

int16_t icu4x_Decimal_nonzero_magnitude_end_mv1(const Decimal* self);

bool icu4x_Decimal_is_zero_mv1(const Decimal* self);

void icu4x_Decimal_multiply_pow10_mv1(Decimal* self, int16_t power);

FixedDecimalSign icu4x_Decimal_sign_mv1(const Decimal* self);

void icu4x_Decimal_set_sign_mv1(Decimal* self, FixedDecimalSign sign);

void icu4x_Decimal_apply_sign_display_mv1(Decimal* self, FixedDecimalSignDisplay sign_display);

void icu4x_Decimal_trim_start_mv1(Decimal* self);

void icu4x_Decimal_trim_end_mv1(Decimal* self);

void icu4x_Decimal_trim_end_if_integer_mv1(Decimal* self);

void icu4x_Decimal_pad_start_mv1(Decimal* self, int16_t position);

void icu4x_Decimal_pad_end_mv1(Decimal* self, int16_t position);

void icu4x_Decimal_set_max_position_mv1(Decimal* self, int16_t position);

void icu4x_Decimal_round_mv1(Decimal* self, int16_t position);

void icu4x_Decimal_ceil_mv1(Decimal* self, int16_t position);

void icu4x_Decimal_expand_mv1(Decimal* self, int16_t position);

void icu4x_Decimal_floor_mv1(Decimal* self, int16_t position);

void icu4x_Decimal_trunc_mv1(Decimal* self, int16_t position);

void icu4x_Decimal_round_with_mode_mv1(Decimal* self, int16_t position, FixedDecimalSignedRoundingMode mode);

void icu4x_Decimal_round_with_mode_and_increment_mv1(Decimal* self, int16_t position, FixedDecimalSignedRoundingMode mode, FixedDecimalRoundingIncrement increment);

typedef struct icu4x_Decimal_concatenate_end_mv1_result { bool is_ok;} icu4x_Decimal_concatenate_end_mv1_result;
icu4x_Decimal_concatenate_end_mv1_result icu4x_Decimal_concatenate_end_mv1(Decimal* self, Decimal* other);

void icu4x_Decimal_to_string_mv1(const Decimal* self, DiplomatWrite* write);


void icu4x_Decimal_destroy_mv1(Decimal* self);





#endif // Decimal_H
