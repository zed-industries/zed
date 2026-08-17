#ifndef WindowsParser_H
#define WindowsParser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "TimeZone.d.h"

#include "WindowsParser.d.h"






WindowsParser* icu4x_WindowsParser_create_mv1(void);

typedef struct icu4x_WindowsParser_create_with_provider_mv1_result {union {WindowsParser* ok; DataError err;}; bool is_ok;} icu4x_WindowsParser_create_with_provider_mv1_result;
icu4x_WindowsParser_create_with_provider_mv1_result icu4x_WindowsParser_create_with_provider_mv1(const DataProvider* provider);

TimeZone* icu4x_WindowsParser_parse_mv1(const WindowsParser* self, DiplomatStringView value, DiplomatStringView region);


void icu4x_WindowsParser_destroy_mv1(WindowsParser* self);





#endif // WindowsParser_H
