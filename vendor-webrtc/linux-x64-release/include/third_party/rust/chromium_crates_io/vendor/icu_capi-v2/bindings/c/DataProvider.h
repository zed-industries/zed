#ifndef DataProvider_H
#define DataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "LocaleFallbacker.d.h"

#include "DataProvider.d.h"






typedef struct icu4x_DataProvider_from_fs_mv1_result {union {DataProvider* ok; DataError err;}; bool is_ok;} icu4x_DataProvider_from_fs_mv1_result;
icu4x_DataProvider_from_fs_mv1_result icu4x_DataProvider_from_fs_mv1(DiplomatStringView path);

typedef struct icu4x_DataProvider_from_byte_slice_mv1_result {union {DataProvider* ok; DataError err;}; bool is_ok;} icu4x_DataProvider_from_byte_slice_mv1_result;
icu4x_DataProvider_from_byte_slice_mv1_result icu4x_DataProvider_from_byte_slice_mv1(DiplomatU8View blob);

typedef struct icu4x_DataProvider_fork_by_key_mv1_result {union { DataError err;}; bool is_ok;} icu4x_DataProvider_fork_by_key_mv1_result;
icu4x_DataProvider_fork_by_key_mv1_result icu4x_DataProvider_fork_by_key_mv1(DataProvider* self, DataProvider* other);

typedef struct icu4x_DataProvider_fork_by_locale_mv1_result {union { DataError err;}; bool is_ok;} icu4x_DataProvider_fork_by_locale_mv1_result;
icu4x_DataProvider_fork_by_locale_mv1_result icu4x_DataProvider_fork_by_locale_mv1(DataProvider* self, DataProvider* other);

typedef struct icu4x_DataProvider_enable_locale_fallback_with_mv1_result {union { DataError err;}; bool is_ok;} icu4x_DataProvider_enable_locale_fallback_with_mv1_result;
icu4x_DataProvider_enable_locale_fallback_with_mv1_result icu4x_DataProvider_enable_locale_fallback_with_mv1(DataProvider* self, const LocaleFallbacker* fallbacker);


void icu4x_DataProvider_destroy_mv1(DataProvider* self);





#endif // DataProvider_H
