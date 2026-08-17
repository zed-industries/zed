// Copyright 2019 The Chromium Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef BSSL_PKI_REVOCATION_UTIL_H_
#define BSSL_PKI_REVOCATION_UTIL_H_

#include <cstdint>
#include <optional>

#include <openssl/base.h>

BSSL_NAMESPACE_BEGIN

namespace der {
struct GeneralizedTime;
}

// Returns true if a revocation status with |this_update| field and potentially
// a |next_update| field, is valid at POSIX time |verify_time_epoch_seconds| and
// not older than |max_age_seconds| seconds, if specified. Expressed
// differently, returns true if |this_update <= verify_time < next_update|, and
// |this_update >= verify_time - max_age|.
[[nodiscard]] OPENSSL_EXPORT bool CheckRevocationDateValid(
    const der::GeneralizedTime &this_update,
    const der::GeneralizedTime *next_update, int64_t verify_time_epoch_seconds,
    std::optional<int64_t> max_age_seconds);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_REVOCATION_UTIL_H_
