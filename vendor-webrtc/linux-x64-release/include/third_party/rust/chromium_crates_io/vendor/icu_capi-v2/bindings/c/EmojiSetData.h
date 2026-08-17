#ifndef EmojiSetData_H
#define EmojiSetData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "EmojiSetData.d.h"






bool icu4x_EmojiSetData_contains_str_mv1(const EmojiSetData* self, DiplomatStringView s);

bool icu4x_EmojiSetData_contains_mv1(const EmojiSetData* self, char32_t cp);

EmojiSetData* icu4x_EmojiSetData_create_basic_mv1(void);

typedef struct icu4x_EmojiSetData_create_basic_with_provider_mv1_result {union {EmojiSetData* ok; DataError err;}; bool is_ok;} icu4x_EmojiSetData_create_basic_with_provider_mv1_result;
icu4x_EmojiSetData_create_basic_with_provider_mv1_result icu4x_EmojiSetData_create_basic_with_provider_mv1(const DataProvider* provider);


void icu4x_EmojiSetData_destroy_mv1(EmojiSetData* self);





#endif // EmojiSetData_H
