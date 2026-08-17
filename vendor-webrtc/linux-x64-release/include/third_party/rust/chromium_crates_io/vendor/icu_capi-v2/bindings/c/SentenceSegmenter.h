#ifndef SentenceSegmenter_H
#define SentenceSegmenter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "SentenceBreakIteratorLatin1.d.h"
#include "SentenceBreakIteratorUtf16.d.h"
#include "SentenceBreakIteratorUtf8.d.h"

#include "SentenceSegmenter.d.h"






SentenceSegmenter* icu4x_SentenceSegmenter_create_mv1(void);

typedef struct icu4x_SentenceSegmenter_create_with_content_locale_mv1_result {union {SentenceSegmenter* ok; DataError err;}; bool is_ok;} icu4x_SentenceSegmenter_create_with_content_locale_mv1_result;
icu4x_SentenceSegmenter_create_with_content_locale_mv1_result icu4x_SentenceSegmenter_create_with_content_locale_mv1(const Locale* locale);

typedef struct icu4x_SentenceSegmenter_create_with_content_locale_and_provider_mv1_result {union {SentenceSegmenter* ok; DataError err;}; bool is_ok;} icu4x_SentenceSegmenter_create_with_content_locale_and_provider_mv1_result;
icu4x_SentenceSegmenter_create_with_content_locale_and_provider_mv1_result icu4x_SentenceSegmenter_create_with_content_locale_and_provider_mv1(const DataProvider* provider, const Locale* locale);

SentenceBreakIteratorUtf8* icu4x_SentenceSegmenter_segment_utf8_mv1(const SentenceSegmenter* self, DiplomatStringView input);

SentenceBreakIteratorUtf16* icu4x_SentenceSegmenter_segment_utf16_mv1(const SentenceSegmenter* self, DiplomatString16View input);

SentenceBreakIteratorLatin1* icu4x_SentenceSegmenter_segment_latin1_mv1(const SentenceSegmenter* self, DiplomatU8View input);


void icu4x_SentenceSegmenter_destroy_mv1(SentenceSegmenter* self);





#endif // SentenceSegmenter_H
