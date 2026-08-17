#ifndef WordSegmenter_H
#define WordSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "WordBreakIteratorLatin1.d.h"
#include "WordBreakIteratorUtf16.d.h"
#include "WordBreakIteratorUtf8.d.h"

#include "WordSegmenter.d.h"






WordSegmenter* icu4x_WordSegmenter_create_auto_mv1(void);

typedef struct icu4x_WordSegmenter_create_auto_with_content_locale_mv1_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} icu4x_WordSegmenter_create_auto_with_content_locale_mv1_result;
icu4x_WordSegmenter_create_auto_with_content_locale_mv1_result icu4x_WordSegmenter_create_auto_with_content_locale_mv1(const Locale* locale);

typedef struct icu4x_WordSegmenter_create_auto_with_content_locale_and_provider_mv1_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} icu4x_WordSegmenter_create_auto_with_content_locale_and_provider_mv1_result;
icu4x_WordSegmenter_create_auto_with_content_locale_and_provider_mv1_result icu4x_WordSegmenter_create_auto_with_content_locale_and_provider_mv1(const DataProvider* provider, const Locale* locale);

WordSegmenter* icu4x_WordSegmenter_create_lstm_mv1(void);

typedef struct icu4x_WordSegmenter_create_lstm_with_content_locale_mv1_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} icu4x_WordSegmenter_create_lstm_with_content_locale_mv1_result;
icu4x_WordSegmenter_create_lstm_with_content_locale_mv1_result icu4x_WordSegmenter_create_lstm_with_content_locale_mv1(const Locale* locale);

typedef struct icu4x_WordSegmenter_create_lstm_with_content_locale_and_provider_mv1_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} icu4x_WordSegmenter_create_lstm_with_content_locale_and_provider_mv1_result;
icu4x_WordSegmenter_create_lstm_with_content_locale_and_provider_mv1_result icu4x_WordSegmenter_create_lstm_with_content_locale_and_provider_mv1(const DataProvider* provider, const Locale* locale);

WordSegmenter* icu4x_WordSegmenter_create_dictionary_mv1(void);

typedef struct icu4x_WordSegmenter_create_dictionary_with_content_locale_mv1_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} icu4x_WordSegmenter_create_dictionary_with_content_locale_mv1_result;
icu4x_WordSegmenter_create_dictionary_with_content_locale_mv1_result icu4x_WordSegmenter_create_dictionary_with_content_locale_mv1(const Locale* locale);

typedef struct icu4x_WordSegmenter_create_dictionary_with_content_locale_and_provider_mv1_result {union {WordSegmenter* ok; DataError err;}; bool is_ok;} icu4x_WordSegmenter_create_dictionary_with_content_locale_and_provider_mv1_result;
icu4x_WordSegmenter_create_dictionary_with_content_locale_and_provider_mv1_result icu4x_WordSegmenter_create_dictionary_with_content_locale_and_provider_mv1(const DataProvider* provider, const Locale* locale);

WordBreakIteratorUtf8* icu4x_WordSegmenter_segment_utf8_mv1(const WordSegmenter* self, DiplomatStringView input);

WordBreakIteratorUtf16* icu4x_WordSegmenter_segment_utf16_mv1(const WordSegmenter* self, DiplomatString16View input);

WordBreakIteratorLatin1* icu4x_WordSegmenter_segment_latin1_mv1(const WordSegmenter* self, DiplomatU8View input);


void icu4x_WordSegmenter_destroy_mv1(WordSegmenter* self);





#endif // WordSegmenter_H
