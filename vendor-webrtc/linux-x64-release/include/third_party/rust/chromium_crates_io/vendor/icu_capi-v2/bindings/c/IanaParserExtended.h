#ifndef IanaParserExtended_H
#define IanaParserExtended_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "TimeZoneAndCanonicalAndNormalized.d.h"
#include "TimeZoneAndCanonicalAndNormalizedIterator.d.h"
#include "TimeZoneAndCanonicalIterator.d.h"

#include "IanaParserExtended.d.h"






IanaParserExtended* icu4x_IanaParserExtended_create_mv1(void);

typedef struct icu4x_IanaParserExtended_create_with_provider_mv1_result {union {IanaParserExtended* ok; DataError err;}; bool is_ok;} icu4x_IanaParserExtended_create_with_provider_mv1_result;
icu4x_IanaParserExtended_create_with_provider_mv1_result icu4x_IanaParserExtended_create_with_provider_mv1(const DataProvider* provider);

TimeZoneAndCanonicalAndNormalized icu4x_IanaParserExtended_parse_mv1(const IanaParserExtended* self, DiplomatStringView value);

TimeZoneAndCanonicalIterator* icu4x_IanaParserExtended_iter_mv1(const IanaParserExtended* self);

TimeZoneAndCanonicalAndNormalizedIterator* icu4x_IanaParserExtended_iter_all_mv1(const IanaParserExtended* self);


void icu4x_IanaParserExtended_destroy_mv1(IanaParserExtended* self);





#endif // IanaParserExtended_H
