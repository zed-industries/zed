#ifndef ScriptWithExtensions_H
#define ScriptWithExtensions_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointRangeIterator.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "ScriptWithExtensionsBorrowed.d.h"

#include "ScriptWithExtensions.d.h"






ScriptWithExtensions* icu4x_ScriptWithExtensions_create_mv1(void);

typedef struct icu4x_ScriptWithExtensions_create_with_provider_mv1_result {union {ScriptWithExtensions* ok; DataError err;}; bool is_ok;} icu4x_ScriptWithExtensions_create_with_provider_mv1_result;
icu4x_ScriptWithExtensions_create_with_provider_mv1_result icu4x_ScriptWithExtensions_create_with_provider_mv1(const DataProvider* provider);

uint16_t icu4x_ScriptWithExtensions_get_script_val_mv1(const ScriptWithExtensions* self, char32_t ch);

bool icu4x_ScriptWithExtensions_has_script_mv1(const ScriptWithExtensions* self, char32_t ch, uint16_t script);

ScriptWithExtensionsBorrowed* icu4x_ScriptWithExtensions_as_borrowed_mv1(const ScriptWithExtensions* self);

CodePointRangeIterator* icu4x_ScriptWithExtensions_iter_ranges_for_script_mv1(const ScriptWithExtensions* self, uint16_t script);


void icu4x_ScriptWithExtensions_destroy_mv1(ScriptWithExtensions* self);





#endif // ScriptWithExtensions_H
