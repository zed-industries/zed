#ifndef WordBreakIteratorLatin1_H
#define WordBreakIteratorLatin1_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "SegmenterWordType.d.h"

#include "WordBreakIteratorLatin1.d.h"






int32_t icu4x_WordBreakIteratorLatin1_next_mv1(WordBreakIteratorLatin1* self);

SegmenterWordType icu4x_WordBreakIteratorLatin1_word_type_mv1(const WordBreakIteratorLatin1* self);

bool icu4x_WordBreakIteratorLatin1_is_word_like_mv1(const WordBreakIteratorLatin1* self);


void icu4x_WordBreakIteratorLatin1_destroy_mv1(WordBreakIteratorLatin1* self);





#endif // WordBreakIteratorLatin1_H
