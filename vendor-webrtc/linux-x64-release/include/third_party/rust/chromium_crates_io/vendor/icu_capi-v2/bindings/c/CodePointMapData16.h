#ifndef CodePointMapData16_H
#define CodePointMapData16_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointRangeIterator.d.h"
#include "CodePointSetData.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"

#include "CodePointMapData16.d.h"






uint16_t icu4x_CodePointMapData16_get_mv1(const CodePointMapData16* self, char32_t cp);

CodePointRangeIterator* icu4x_CodePointMapData16_iter_ranges_for_value_mv1(const CodePointMapData16* self, uint16_t value);

CodePointRangeIterator* icu4x_CodePointMapData16_iter_ranges_for_value_complemented_mv1(const CodePointMapData16* self, uint16_t value);

CodePointSetData* icu4x_CodePointMapData16_get_set_for_value_mv1(const CodePointMapData16* self, uint16_t value);

CodePointMapData16* icu4x_CodePointMapData16_create_script_mv1(void);

typedef struct icu4x_CodePointMapData16_create_script_with_provider_mv1_result {union {CodePointMapData16* ok; DataError err;}; bool is_ok;} icu4x_CodePointMapData16_create_script_with_provider_mv1_result;
icu4x_CodePointMapData16_create_script_with_provider_mv1_result icu4x_CodePointMapData16_create_script_with_provider_mv1(const DataProvider* provider);


void icu4x_CodePointMapData16_destroy_mv1(CodePointMapData16* self);





#endif // CodePointMapData16_H
