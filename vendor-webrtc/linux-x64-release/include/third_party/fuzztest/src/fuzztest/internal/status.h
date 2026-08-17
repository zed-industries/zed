#ifndef FUZZTEST_FUZZTEST_INTERNAL_STATUS_H_
#define FUZZTEST_FUZZTEST_INTERNAL_STATUS_H_

#include "absl/status/status.h"
#include "absl/strings/string_view.h"

// Prefix status error message with `prefix`, if `status` is not OK.
absl::Status Prefix(const absl::Status& status, absl::string_view prefix);

// Postfix status error message with `postfix`, if `status` is not OK.
absl::Status Postfix(const absl::Status& status, absl::string_view postfix);

#endif  // FUZZTEST_FUZZTEST_INTERNAL_STATUS_H_
