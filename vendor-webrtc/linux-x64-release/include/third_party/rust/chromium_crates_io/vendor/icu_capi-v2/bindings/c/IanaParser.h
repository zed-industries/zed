#ifndef IanaParser_H
#define IanaParser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "TimeZone.d.h"
#include "TimeZoneIterator.d.h"

#include "IanaParser.d.h"






IanaParser* icu4x_IanaParser_create_mv1(void);

typedef struct icu4x_IanaParser_create_with_provider_mv1_result {union {IanaParser* ok; DataError err;}; bool is_ok;} icu4x_IanaParser_create_with_provider_mv1_result;
icu4x_IanaParser_create_with_provider_mv1_result icu4x_IanaParser_create_with_provider_mv1(const DataProvider* provider);

TimeZone* icu4x_IanaParser_parse_mv1(const IanaParser* self, DiplomatStringView value);

TimeZoneIterator* icu4x_IanaParser_iter_mv1(const IanaParser* self);


void icu4x_IanaParser_destroy_mv1(IanaParser* self);





#endif // IanaParser_H
