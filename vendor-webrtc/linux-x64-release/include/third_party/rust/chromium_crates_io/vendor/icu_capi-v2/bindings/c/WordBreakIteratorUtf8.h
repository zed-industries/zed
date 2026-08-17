#ifndef WordBreakIteratorUtf8_H
#define WordBreakIteratorUtf8_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "SegmenterWordType.d.h"

#include "WordBreakIteratorUtf8.d.h"






int32_t icu4x_WordBreakIteratorUtf8_next_mv1(WordBreakIteratorUtf8* self);

SegmenterWordType icu4x_WordBreakIteratorUtf8_word_type_mv1(const WordBreakIteratorUtf8* self);

bool icu4x_WordBreakIteratorUtf8_is_word_like_mv1(const WordBreakIteratorUtf8* self);


void icu4x_WordBreakIteratorUtf8_destroy_mv1(WordBreakIteratorUtf8* self);





#endif // WordBreakIteratorUtf8_H
