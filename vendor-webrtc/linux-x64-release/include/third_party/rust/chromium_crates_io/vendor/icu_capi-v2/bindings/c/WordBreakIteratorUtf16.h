#ifndef WordBreakIteratorUtf16_H
#define WordBreakIteratorUtf16_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "SegmenterWordType.d.h"

#include "WordBreakIteratorUtf16.d.h"






int32_t icu4x_WordBreakIteratorUtf16_next_mv1(WordBreakIteratorUtf16* self);

SegmenterWordType icu4x_WordBreakIteratorUtf16_word_type_mv1(const WordBreakIteratorUtf16* self);

bool icu4x_WordBreakIteratorUtf16_is_word_like_mv1(const WordBreakIteratorUtf16* self);


void icu4x_WordBreakIteratorUtf16_destroy_mv1(WordBreakIteratorUtf16* self);





#endif // WordBreakIteratorUtf16_H
