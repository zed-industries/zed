#ifndef LocaleDisplayNamesFormatter_H
#define LocaleDisplayNamesFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "DisplayNamesOptionsV1.d.h"
#include "Locale.d.h"

#include "LocaleDisplayNamesFormatter.d.h"






typedef struct icu4x_LocaleDisplayNamesFormatter_create_v1_mv1_result {union {LocaleDisplayNamesFormatter* ok; DataError err;}; bool is_ok;} icu4x_LocaleDisplayNamesFormatter_create_v1_mv1_result;
icu4x_LocaleDisplayNamesFormatter_create_v1_mv1_result icu4x_LocaleDisplayNamesFormatter_create_v1_mv1(const Locale* locale, DisplayNamesOptionsV1 options);

typedef struct icu4x_LocaleDisplayNamesFormatter_create_v1_with_provider_mv1_result {union {LocaleDisplayNamesFormatter* ok; DataError err;}; bool is_ok;} icu4x_LocaleDisplayNamesFormatter_create_v1_with_provider_mv1_result;
icu4x_LocaleDisplayNamesFormatter_create_v1_with_provider_mv1_result icu4x_LocaleDisplayNamesFormatter_create_v1_with_provider_mv1(const DataProvider* provider, const Locale* locale, DisplayNamesOptionsV1 options);

void icu4x_LocaleDisplayNamesFormatter_of_mv1(const LocaleDisplayNamesFormatter* self, const Locale* locale, DiplomatWrite* write);


void icu4x_LocaleDisplayNamesFormatter_destroy_mv1(LocaleDisplayNamesFormatter* self);





#endif // LocaleDisplayNamesFormatter_H
