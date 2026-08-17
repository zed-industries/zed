#ifndef UnitsConverterFactory_H
#define UnitsConverterFactory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "MeasureUnit.d.h"
#include "MeasureUnitParser.d.h"
#include "UnitsConverter.d.h"

#include "UnitsConverterFactory.d.h"






UnitsConverterFactory* icu4x_UnitsConverterFactory_create_mv1(void);

typedef struct icu4x_UnitsConverterFactory_create_with_provider_mv1_result {union {UnitsConverterFactory* ok; DataError err;}; bool is_ok;} icu4x_UnitsConverterFactory_create_with_provider_mv1_result;
icu4x_UnitsConverterFactory_create_with_provider_mv1_result icu4x_UnitsConverterFactory_create_with_provider_mv1(const DataProvider* provider);

UnitsConverter* icu4x_UnitsConverterFactory_converter_mv1(const UnitsConverterFactory* self, const MeasureUnit* from, const MeasureUnit* to);

MeasureUnitParser* icu4x_UnitsConverterFactory_parser_mv1(const UnitsConverterFactory* self);


void icu4x_UnitsConverterFactory_destroy_mv1(UnitsConverterFactory* self);





#endif // UnitsConverterFactory_H
