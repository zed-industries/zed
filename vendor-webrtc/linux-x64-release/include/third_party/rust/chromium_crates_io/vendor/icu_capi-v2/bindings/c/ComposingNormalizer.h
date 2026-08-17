#ifndef ComposingNormalizer_H
#define ComposingNormalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "ComposingNormalizer.d.h"






ComposingNormalizer* icu4x_ComposingNormalizer_create_nfc_mv1(void);

typedef struct icu4x_ComposingNormalizer_create_nfc_with_provider_mv1_result {union {ComposingNormalizer* ok; DataError err;}; bool is_ok;} icu4x_ComposingNormalizer_create_nfc_with_provider_mv1_result;
icu4x_ComposingNormalizer_create_nfc_with_provider_mv1_result icu4x_ComposingNormalizer_create_nfc_with_provider_mv1(const DataProvider* provider);

ComposingNormalizer* icu4x_ComposingNormalizer_create_nfkc_mv1(void);

typedef struct icu4x_ComposingNormalizer_create_nfkc_with_provider_mv1_result {union {ComposingNormalizer* ok; DataError err;}; bool is_ok;} icu4x_ComposingNormalizer_create_nfkc_with_provider_mv1_result;
icu4x_ComposingNormalizer_create_nfkc_with_provider_mv1_result icu4x_ComposingNormalizer_create_nfkc_with_provider_mv1(const DataProvider* provider);

void icu4x_ComposingNormalizer_normalize_mv1(const ComposingNormalizer* self, DiplomatStringView s, DiplomatWrite* write);

bool icu4x_ComposingNormalizer_is_normalized_utf8_mv1(const ComposingNormalizer* self, DiplomatStringView s);

bool icu4x_ComposingNormalizer_is_normalized_utf16_mv1(const ComposingNormalizer* self, DiplomatString16View s);

size_t icu4x_ComposingNormalizer_is_normalized_utf8_up_to_mv1(const ComposingNormalizer* self, DiplomatStringView s);

size_t icu4x_ComposingNormalizer_is_normalized_utf16_up_to_mv1(const ComposingNormalizer* self, DiplomatString16View s);


void icu4x_ComposingNormalizer_destroy_mv1(ComposingNormalizer* self);





#endif // ComposingNormalizer_H
