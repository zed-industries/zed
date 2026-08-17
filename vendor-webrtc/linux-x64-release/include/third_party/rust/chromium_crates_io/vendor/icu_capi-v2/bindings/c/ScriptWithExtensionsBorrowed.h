#ifndef ScriptWithExtensionsBorrowed_H
#define ScriptWithExtensionsBorrowed_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointSetData.d.h"
#include "ScriptExtensionsSet.d.h"

#include "ScriptWithExtensionsBorrowed.d.h"






uint16_t icu4x_ScriptWithExtensionsBorrowed_get_script_val_mv1(const ScriptWithExtensionsBorrowed* self, char32_t ch);

ScriptExtensionsSet* icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_val_mv1(const ScriptWithExtensionsBorrowed* self, char32_t ch);

bool icu4x_ScriptWithExtensionsBorrowed_has_script_mv1(const ScriptWithExtensionsBorrowed* self, char32_t ch, uint16_t script);

CodePointSetData* icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_set_mv1(const ScriptWithExtensionsBorrowed* self, uint16_t script);


void icu4x_ScriptWithExtensionsBorrowed_destroy_mv1(ScriptWithExtensionsBorrowed* self);





#endif // ScriptWithExtensionsBorrowed_H
