#ifndef Locale_H
#define Locale_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "LocaleParseError.d.h"

#include "Locale.d.h"






typedef struct icu4x_Locale_from_string_mv1_result {union {Locale* ok; LocaleParseError err;}; bool is_ok;} icu4x_Locale_from_string_mv1_result;
icu4x_Locale_from_string_mv1_result icu4x_Locale_from_string_mv1(DiplomatStringView name);

Locale* icu4x_Locale_und_mv1(void);

Locale* icu4x_Locale_clone_mv1(const Locale* self);

void icu4x_Locale_basename_mv1(const Locale* self, DiplomatWrite* write);

typedef struct icu4x_Locale_get_unicode_extension_mv1_result { bool is_ok;} icu4x_Locale_get_unicode_extension_mv1_result;
icu4x_Locale_get_unicode_extension_mv1_result icu4x_Locale_get_unicode_extension_mv1(const Locale* self, DiplomatStringView s, DiplomatWrite* write);

void icu4x_Locale_language_mv1(const Locale* self, DiplomatWrite* write);

typedef struct icu4x_Locale_set_language_mv1_result {union { LocaleParseError err;}; bool is_ok;} icu4x_Locale_set_language_mv1_result;
icu4x_Locale_set_language_mv1_result icu4x_Locale_set_language_mv1(Locale* self, DiplomatStringView s);

typedef struct icu4x_Locale_region_mv1_result { bool is_ok;} icu4x_Locale_region_mv1_result;
icu4x_Locale_region_mv1_result icu4x_Locale_region_mv1(const Locale* self, DiplomatWrite* write);

typedef struct icu4x_Locale_set_region_mv1_result {union { LocaleParseError err;}; bool is_ok;} icu4x_Locale_set_region_mv1_result;
icu4x_Locale_set_region_mv1_result icu4x_Locale_set_region_mv1(Locale* self, DiplomatStringView s);

typedef struct icu4x_Locale_script_mv1_result { bool is_ok;} icu4x_Locale_script_mv1_result;
icu4x_Locale_script_mv1_result icu4x_Locale_script_mv1(const Locale* self, DiplomatWrite* write);

typedef struct icu4x_Locale_set_script_mv1_result {union { LocaleParseError err;}; bool is_ok;} icu4x_Locale_set_script_mv1_result;
icu4x_Locale_set_script_mv1_result icu4x_Locale_set_script_mv1(Locale* self, DiplomatStringView s);

typedef struct icu4x_Locale_normalize_mv1_result {union { LocaleParseError err;}; bool is_ok;} icu4x_Locale_normalize_mv1_result;
icu4x_Locale_normalize_mv1_result icu4x_Locale_normalize_mv1(DiplomatStringView s, DiplomatWrite* write);

void icu4x_Locale_to_string_mv1(const Locale* self, DiplomatWrite* write);

bool icu4x_Locale_normalizing_eq_mv1(const Locale* self, DiplomatStringView other);

int8_t icu4x_Locale_compare_to_string_mv1(const Locale* self, DiplomatStringView other);

int8_t icu4x_Locale_compare_to_mv1(const Locale* self, const Locale* other);


void icu4x_Locale_destroy_mv1(Locale* self);





#endif // Locale_H
