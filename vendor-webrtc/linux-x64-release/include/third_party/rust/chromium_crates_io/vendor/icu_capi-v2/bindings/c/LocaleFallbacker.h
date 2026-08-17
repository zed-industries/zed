#ifndef LocaleFallbacker_H
#define LocaleFallbacker_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "LocaleFallbackConfig.d.h"
#include "LocaleFallbackerWithConfig.d.h"

#include "LocaleFallbacker.d.h"






LocaleFallbacker* icu4x_LocaleFallbacker_create_mv1(void);

typedef struct icu4x_LocaleFallbacker_create_with_provider_mv1_result {union {LocaleFallbacker* ok; DataError err;}; bool is_ok;} icu4x_LocaleFallbacker_create_with_provider_mv1_result;
icu4x_LocaleFallbacker_create_with_provider_mv1_result icu4x_LocaleFallbacker_create_with_provider_mv1(const DataProvider* provider);

LocaleFallbacker* icu4x_LocaleFallbacker_without_data_mv1(void);

LocaleFallbackerWithConfig* icu4x_LocaleFallbacker_for_config_mv1(const LocaleFallbacker* self, LocaleFallbackConfig config);


void icu4x_LocaleFallbacker_destroy_mv1(LocaleFallbacker* self);





#endif // LocaleFallbacker_H
