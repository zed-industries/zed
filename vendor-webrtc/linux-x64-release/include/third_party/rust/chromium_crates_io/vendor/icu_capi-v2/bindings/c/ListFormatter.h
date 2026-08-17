#ifndef ListFormatter_H
#define ListFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "ListLength.d.h"
#include "Locale.d.h"

#include "ListFormatter.d.h"






typedef struct icu4x_ListFormatter_create_and_with_length_mv1_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} icu4x_ListFormatter_create_and_with_length_mv1_result;
icu4x_ListFormatter_create_and_with_length_mv1_result icu4x_ListFormatter_create_and_with_length_mv1(const Locale* locale, ListLength length);

typedef struct icu4x_ListFormatter_create_and_with_length_and_provider_mv1_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} icu4x_ListFormatter_create_and_with_length_and_provider_mv1_result;
icu4x_ListFormatter_create_and_with_length_and_provider_mv1_result icu4x_ListFormatter_create_and_with_length_and_provider_mv1(const DataProvider* provider, const Locale* locale, ListLength length);

typedef struct icu4x_ListFormatter_create_or_with_length_mv1_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} icu4x_ListFormatter_create_or_with_length_mv1_result;
icu4x_ListFormatter_create_or_with_length_mv1_result icu4x_ListFormatter_create_or_with_length_mv1(const Locale* locale, ListLength length);

typedef struct icu4x_ListFormatter_create_or_with_length_and_provider_mv1_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} icu4x_ListFormatter_create_or_with_length_and_provider_mv1_result;
icu4x_ListFormatter_create_or_with_length_and_provider_mv1_result icu4x_ListFormatter_create_or_with_length_and_provider_mv1(const DataProvider* provider, const Locale* locale, ListLength length);

typedef struct icu4x_ListFormatter_create_unit_with_length_mv1_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} icu4x_ListFormatter_create_unit_with_length_mv1_result;
icu4x_ListFormatter_create_unit_with_length_mv1_result icu4x_ListFormatter_create_unit_with_length_mv1(const Locale* locale, ListLength length);

typedef struct icu4x_ListFormatter_create_unit_with_length_and_provider_mv1_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} icu4x_ListFormatter_create_unit_with_length_and_provider_mv1_result;
icu4x_ListFormatter_create_unit_with_length_and_provider_mv1_result icu4x_ListFormatter_create_unit_with_length_and_provider_mv1(const DataProvider* provider, const Locale* locale, ListLength length);

void icu4x_ListFormatter_format_utf8_mv1(const ListFormatter* self, DiplomatStringsView list, DiplomatWrite* write);

void icu4x_ListFormatter_format_utf16_mv1(const ListFormatter* self, DiplomatStrings16View list, DiplomatWrite* write);


void icu4x_ListFormatter_destroy_mv1(ListFormatter* self);





#endif // ListFormatter_H
