#ifndef ZonedIsoDateTime_H
#define ZonedIsoDateTime_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CalendarParseError.d.h"
#include "IanaParser.d.h"
#include "UtcOffsetCalculator.d.h"

#include "ZonedIsoDateTime.d.h"






typedef struct icu4x_ZonedIsoDateTime_from_string_mv1_result {union {ZonedIsoDateTime ok; CalendarParseError err;}; bool is_ok;} icu4x_ZonedIsoDateTime_from_string_mv1_result;
icu4x_ZonedIsoDateTime_from_string_mv1_result icu4x_ZonedIsoDateTime_from_string_mv1(DiplomatStringView v, const IanaParser* iana_parser, const UtcOffsetCalculator* offset_calculator);






#endif // ZonedIsoDateTime_H
