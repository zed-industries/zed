// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SHARED_STORAGE_SHARED_STORAGE_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SHARED_STORAGE_SHARED_STORAGE_UTILS_H_

#include <cstdlib>
#include <string_view>

#include "third_party/blink/public/common/common_export.h"

namespace blink {

static constexpr char kSharedStorageModuleScriptNotLoadedErrorMessage[] =
    "The module script hasn't been loaded.";

static constexpr char kSharedStorageOperationNotFoundErrorMessage[] =
    "Cannot find operation name.";

static constexpr char
    kSharedStorageEmptyOperationDefinitionInstanceErrorMessage[] =
        "Empty operation definition instance.";

static constexpr char kSharedStorageCannotDeserializeDataErrorMessage[] =
    "Cannot deserialize data.";

static constexpr char kSharedStorageEmptyScriptResultErrorMessage[] =
    "empty script result.";

static constexpr char kSharedStorageReturnValueToIntErrorMessage[] =
    "Promise did not resolve to an uint32 number.";

static constexpr char kSharedStorageReturnValueOutOfRangeErrorMessage[] =
    "Promise resolved to a number outside the length of the input urls.";

// Scope from which a shared storage call is made.
// TODO(https://crbug.com/380291909): Implement DevTools event notifications for
// shared storage access from PA worklet access.
enum class SharedStorageAccessScope {
  kWindow,
  kSharedStorageWorklet,
  kProtectedAudienceWorklet,
  kHeader,
};

// Whether or not the worklet ever entered keep-alive, and if so, the reason the
// keep-alive was terminated. Recorded to UMA; always add new values to the end
// and do not reorder or delete values from this list.
enum class SharedStorageWorkletDestroyedStatus {
  kDidNotEnterKeepAlive = 0,
  kKeepAliveEndedDueToOperationsFinished = 1,
  kKeepAliveEndedDueToTimeout = 2,
  kOther = 3,

  // Keep this at the end and equal to the last entry.
  kMaxValue = kOther,
};

// Error type encountered by worklet.
// Recorded to UMA; always add new values to the end and do not reorder or
// delete values from this list. Instead of deleting an obsolete value "kFoo",
// for example, insert "OBSOLETE_" in the value's name after the initial "k" to
// get "kOBSOLETE_foo".
enum class SharedStorageWorkletErrorType {
  kAddModuleWebVisible = 0,
  kOBSOLETE_AddModuleNonWebVisible = 1,  // Replaced by finer-grained types.
  kRunWebVisible = 2,
  kOBSOLETE_RunNonWebVisible = 3,  // Replaced by finer-grained types.
  kSelectURLWebVisible = 4,
  kOBSOLETE_SelectURLNonWebVisible = 5,  // Replaced by finer-grained types.
  kSuccess = 6,
  kOBSOLETE_AddModuleNonWebVisibleMulipleWorkletsDisabled = 7,
  kOBSOLETE_AddModuleNonWebVisibleCrossOriginWorkletsDisabled = 8,
  kAddModuleNonWebVisibleCrossOriginSharedStorageDisabled = 9,
  kAddModuleNonWebVisibleOther = 10,
  kRunNonWebVisibleInvalidContextId = 11,
  kRunNonWebVisibleKeepAliveFalse = 12,
  kRunNonWebVisibleCrossOriginSharedStorageDisabled = 13,
  kRunNonWebVisibleModuleScriptNotLoaded = 14,
  kRunNonWebVisibleOperationNotFound = 15,
  kRunNonWebVisibleEmptyOperationDefinitionInstance = 16,
  kRunNonWebVisibleCannotDeserializeData = 17,
  kRunNonWebVisibleEmptyScriptResult = 18,
  kRunNonWebVisibleOther = 19,
  kSelectURLNonWebVisibleInvalidURLArrayLength = 20,
  kSelectURLNonWebVisibleInvalidFencedFrameURL = 21,
  kSelectURLNonWebVisibleInvalidReportingURL = 22,
  kSelectURLNonWebVisibleInvalidContextId = 23,
  kSelectURLNonWebVisibleKeepAliveFalse = 24,
  kSelectURLNonWebVisibleCrossOriginSharedStorageDisabled = 25,
  kSelectURLNonWebVisibleModuleScriptNotLoaded = 26,
  kSelectURLNonWebVisibleOperationNotFound = 27,
  kSelectURLNonWebVisibleEmptyOperationDefinitionInstance = 28,
  kSelectURLNonWebVisibleCannotDeserializeData = 29,
  kSelectURLNonWebVisibleEmptyScriptResult = 30,
  kSelectURLNonWebVisibleReturnValueToInt = 31,
  kSelectURLNonWebVisibleReturnValueOutOfRange = 32,
  kSelectURLNonWebVisibleUnexpectedIndexReturned = 33,
  kSelectURLNonWebVisibleInsufficientBudget = 34,
  kSelectURLNonWebVisibleOther = 35,
  kRunNonWebVisibleInvalidFilteringIdMaxBytes = 36,
  kSelectURLNonWebVisibleInvalidFilteringIdMaxBytes = 37,
  kAddModuleNonWebVisibleCustomDataOriginDisabled = 38,

  // Keep this at the end and equal to the last entry.
  kMaxValue = kAddModuleNonWebVisibleCustomDataOriginDisabled,
};

// Whether or not there is sufficient budget for the `selectURL()` call, and if
// there is insufficient budget, which of the budgets fell short. Note that
// budgets are checked in the order: site navigation budget, overall pageload
// budget, site pageload budget. Recorded to UMA; always add new values to the
// end and do not reorder or delete values from this list.
enum class SharedStorageSelectUrlBudgetStatus {
  kSufficientBudget = 0,
  kInsufficientSiteNavigationBudget = 1,
  kInsufficientOverallPageloadBudget = 2,
  kInsufficientSitePageloadBudget = 3,
  kOther = 4,

  // Keep this at the end and equal to the last entry.
  kMaxValue = kOther,
};

// Whether the length of the urls input parameter (of the
// sharedStorage.runURLSelectionOperation method) is valid.
BLINK_COMMON_EXPORT bool IsValidSharedStorageURLsArrayLength(size_t length);

// Logs histogram of the calling method and error type for worklet errors.
BLINK_COMMON_EXPORT void LogSharedStorageWorkletError(
    SharedStorageWorkletErrorType error_type);

// Logs histogram of the `selectURL()` budget status.
BLINK_COMMON_EXPORT void LogSharedStorageSelectURLBudgetStatus(
    SharedStorageSelectUrlBudgetStatus budget_status);

// Whether `privateAggregation` should be exposed to `SharedStorageWorklet`
// scope.
BLINK_COMMON_EXPORT bool ShouldDefinePrivateAggregationInSharedStorage();

// Whether the `context_id` is valid UTF-8 and has a valid length.
BLINK_COMMON_EXPORT bool IsValidPrivateAggregationContextId(
    std::string_view context_id);

// Maximum allowed length of the context_id string.
static constexpr int kPrivateAggregationApiContextIdMaxLength = 64;

// Whether the `filtering_id_max_bytes` has a valid value.
BLINK_COMMON_EXPORT bool IsValidPrivateAggregationFilteringIdMaxBytes(
    size_t filtering_id_max_bytes);

static constexpr size_t kPrivateAggregationApiDefaultFilteringIdMaxBytes = 1;
static constexpr size_t kPrivateAggregationApiMaxFilteringIdMaxBytes = 8;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SHARED_STORAGE_SHARED_STORAGE_UTILS_H_
