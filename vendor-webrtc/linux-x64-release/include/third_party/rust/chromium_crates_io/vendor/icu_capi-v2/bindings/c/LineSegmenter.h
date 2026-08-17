#ifndef LineSegmenter_H
#define LineSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "LineBreakIteratorLatin1.d.h"
#include "LineBreakIteratorUtf16.d.h"
#include "LineBreakIteratorUtf8.d.h"
#include "LineBreakOptionsV2.d.h"
#include "Locale.d.h"

#include "LineSegmenter.d.h"






LineSegmenter* icu4x_LineSegmenter_create_auto_mv1(void);

LineSegmenter* icu4x_LineSegmenter_create_lstm_mv1(void);

LineSegmenter* icu4x_LineSegmenter_create_dictionary_mv1(void);

LineSegmenter* icu4x_LineSegmenter_create_auto_with_options_v2_mv1(const Locale* content_locale, LineBreakOptionsV2 options);

typedef struct icu4x_LineSegmenter_create_auto_with_options_v2_and_provider_mv1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_auto_with_options_v2_and_provider_mv1_result;
icu4x_LineSegmenter_create_auto_with_options_v2_and_provider_mv1_result icu4x_LineSegmenter_create_auto_with_options_v2_and_provider_mv1(const DataProvider* provider, const Locale* content_locale, LineBreakOptionsV2 options);

LineSegmenter* icu4x_LineSegmenter_create_lstm_with_options_v2_mv1(const Locale* content_locale, LineBreakOptionsV2 options);

typedef struct icu4x_LineSegmenter_create_lstm_with_options_v2_and_provider_mv1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_lstm_with_options_v2_and_provider_mv1_result;
icu4x_LineSegmenter_create_lstm_with_options_v2_and_provider_mv1_result icu4x_LineSegmenter_create_lstm_with_options_v2_and_provider_mv1(const DataProvider* provider, const Locale* content_locale, LineBreakOptionsV2 options);

LineSegmenter* icu4x_LineSegmenter_create_dictionary_with_options_v2_mv1(const Locale* content_locale, LineBreakOptionsV2 options);

typedef struct icu4x_LineSegmenter_create_dictionary_with_options_v2_and_provider_mv1_result {union {LineSegmenter* ok; DataError err;}; bool is_ok;} icu4x_LineSegmenter_create_dictionary_with_options_v2_and_provider_mv1_result;
icu4x_LineSegmenter_create_dictionary_with_options_v2_and_provider_mv1_result icu4x_LineSegmenter_create_dictionary_with_options_v2_and_provider_mv1(const DataProvider* provider, const Locale* content_locale, LineBreakOptionsV2 options);

LineBreakIteratorUtf8* icu4x_LineSegmenter_segment_utf8_mv1(const LineSegmenter* self, DiplomatStringView input);

LineBreakIteratorUtf16* icu4x_LineSegmenter_segment_utf16_mv1(const LineSegmenter* self, DiplomatString16View input);

LineBreakIteratorLatin1* icu4x_LineSegmenter_segment_latin1_mv1(const LineSegmenter* self, DiplomatU8View input);


void icu4x_LineSegmenter_destroy_mv1(LineSegmenter* self);





#endif // LineSegmenter_H
