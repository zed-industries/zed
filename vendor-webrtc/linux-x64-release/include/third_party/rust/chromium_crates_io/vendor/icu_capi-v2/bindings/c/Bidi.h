#ifndef Bidi_H
#define Bidi_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BidiInfo.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "ReorderedIndexMap.d.h"

#include "Bidi.d.h"






Bidi* icu4x_Bidi_create_mv1(void);

typedef struct icu4x_Bidi_create_with_provider_mv1_result {union {Bidi* ok; DataError err;}; bool is_ok;} icu4x_Bidi_create_with_provider_mv1_result;
icu4x_Bidi_create_with_provider_mv1_result icu4x_Bidi_create_with_provider_mv1(const DataProvider* provider);

BidiInfo* icu4x_Bidi_for_text_utf8_mv1(const Bidi* self, DiplomatStringView text, OptionU8 default_level);

ReorderedIndexMap* icu4x_Bidi_reorder_visual_mv1(const Bidi* self, DiplomatU8View levels);

bool icu4x_Bidi_level_is_rtl_mv1(uint8_t level);

bool icu4x_Bidi_level_is_ltr_mv1(uint8_t level);

uint8_t icu4x_Bidi_level_rtl_mv1(void);

uint8_t icu4x_Bidi_level_ltr_mv1(void);


void icu4x_Bidi_destroy_mv1(Bidi* self);





#endif // Bidi_H
