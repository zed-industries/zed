#ifndef ScriptExtensionsSet_H
#define ScriptExtensionsSet_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ScriptExtensionsSet.d.h"






bool icu4x_ScriptExtensionsSet_contains_mv1(const ScriptExtensionsSet* self, uint16_t script);

size_t icu4x_ScriptExtensionsSet_count_mv1(const ScriptExtensionsSet* self);

typedef struct icu4x_ScriptExtensionsSet_script_at_mv1_result {union {uint16_t ok; }; bool is_ok;} icu4x_ScriptExtensionsSet_script_at_mv1_result;
icu4x_ScriptExtensionsSet_script_at_mv1_result icu4x_ScriptExtensionsSet_script_at_mv1(const ScriptExtensionsSet* self, size_t index);


void icu4x_ScriptExtensionsSet_destroy_mv1(ScriptExtensionsSet* self);





#endif // ScriptExtensionsSet_H
