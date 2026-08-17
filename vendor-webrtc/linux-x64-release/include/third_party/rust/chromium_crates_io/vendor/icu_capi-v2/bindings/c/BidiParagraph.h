#ifndef BidiParagraph_H
#define BidiParagraph_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BidiDirection.d.h"

#include "BidiParagraph.d.h"






bool icu4x_BidiParagraph_set_paragraph_in_text_mv1(BidiParagraph* self, size_t n);

BidiDirection icu4x_BidiParagraph_direction_mv1(const BidiParagraph* self);

size_t icu4x_BidiParagraph_size_mv1(const BidiParagraph* self);

size_t icu4x_BidiParagraph_range_start_mv1(const BidiParagraph* self);

size_t icu4x_BidiParagraph_range_end_mv1(const BidiParagraph* self);

typedef struct icu4x_BidiParagraph_reorder_line_mv1_result { bool is_ok;} icu4x_BidiParagraph_reorder_line_mv1_result;
icu4x_BidiParagraph_reorder_line_mv1_result icu4x_BidiParagraph_reorder_line_mv1(const BidiParagraph* self, size_t range_start, size_t range_end, DiplomatWrite* write);

uint8_t icu4x_BidiParagraph_level_at_mv1(const BidiParagraph* self, size_t pos);


void icu4x_BidiParagraph_destroy_mv1(BidiParagraph* self);





#endif // BidiParagraph_H
