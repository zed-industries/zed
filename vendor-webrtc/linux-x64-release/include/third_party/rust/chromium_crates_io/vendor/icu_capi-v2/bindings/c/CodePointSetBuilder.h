#ifndef CodePointSetBuilder_H
#define CodePointSetBuilder_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointSetData.d.h"

#include "CodePointSetBuilder.d.h"






CodePointSetBuilder* icu4x_CodePointSetBuilder_create_mv1(void);

CodePointSetData* icu4x_CodePointSetBuilder_build_mv1(CodePointSetBuilder* self);

void icu4x_CodePointSetBuilder_complement_mv1(CodePointSetBuilder* self);

bool icu4x_CodePointSetBuilder_is_empty_mv1(const CodePointSetBuilder* self);

void icu4x_CodePointSetBuilder_add_char_mv1(CodePointSetBuilder* self, char32_t ch);

void icu4x_CodePointSetBuilder_add_inclusive_range_mv1(CodePointSetBuilder* self, char32_t start, char32_t end);

void icu4x_CodePointSetBuilder_add_set_mv1(CodePointSetBuilder* self, const CodePointSetData* data);

void icu4x_CodePointSetBuilder_remove_char_mv1(CodePointSetBuilder* self, char32_t ch);

void icu4x_CodePointSetBuilder_remove_inclusive_range_mv1(CodePointSetBuilder* self, char32_t start, char32_t end);

void icu4x_CodePointSetBuilder_remove_set_mv1(CodePointSetBuilder* self, const CodePointSetData* data);

void icu4x_CodePointSetBuilder_retain_char_mv1(CodePointSetBuilder* self, char32_t ch);

void icu4x_CodePointSetBuilder_retain_inclusive_range_mv1(CodePointSetBuilder* self, char32_t start, char32_t end);

void icu4x_CodePointSetBuilder_retain_set_mv1(CodePointSetBuilder* self, const CodePointSetData* data);

void icu4x_CodePointSetBuilder_complement_char_mv1(CodePointSetBuilder* self, char32_t ch);

void icu4x_CodePointSetBuilder_complement_inclusive_range_mv1(CodePointSetBuilder* self, char32_t start, char32_t end);

void icu4x_CodePointSetBuilder_complement_set_mv1(CodePointSetBuilder* self, const CodePointSetData* data);


void icu4x_CodePointSetBuilder_destroy_mv1(CodePointSetBuilder* self);





#endif // CodePointSetBuilder_H
