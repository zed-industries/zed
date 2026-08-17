#ifndef DateTime_H
#define DateTime_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Calendar.d.h"
#include "CalendarParseError.d.h"

#include "DateTime.d.h"






typedef struct icu4x_DateTime_from_string_mv1_result {union {DateTime ok; CalendarParseError err;}; bool is_ok;} icu4x_DateTime_from_string_mv1_result;
icu4x_DateTime_from_string_mv1_result icu4x_DateTime_from_string_mv1(DiplomatStringView v, const Calendar* calendar);






#endif // DateTime_H
