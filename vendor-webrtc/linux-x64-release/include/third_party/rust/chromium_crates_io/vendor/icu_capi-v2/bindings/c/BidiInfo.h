#ifndef BidiInfo_H
#define BidiInfo_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BidiParagraph.d.h"

#include "BidiInfo.d.h"






size_t icu4x_BidiInfo_paragraph_count_mv1(const BidiInfo* self);

BidiParagraph* icu4x_BidiInfo_paragraph_at_mv1(const BidiInfo* self, size_t n);

size_t icu4x_BidiInfo_size_mv1(const BidiInfo* self);

uint8_t icu4x_BidiInfo_level_at_mv1(const BidiInfo* self, size_t pos);


void icu4x_BidiInfo_destroy_mv1(BidiInfo* self);





#endif // BidiInfo_H
