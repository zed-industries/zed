// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_SERVICE_INDICATOR_H
#define OPENSSL_HEADER_SERVICE_INDICATOR_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif

// FIPS_service_indicator_before_call and |FIPS_service_indicator_after_call|
// both currently return the same local thread counter which is slowly
// incremented whenever approved services are called. The
// |CALL_SERVICE_AND_CHECK_APPROVED| macro is strongly recommended over calling
// these functions directly.
//
// |FIPS_service_indicator_before_call| is intended to be called immediately
// before an approved service, while |FIPS_service_indicator_after_call| should
// be called immediately after. If the values returned from these two functions
// are not equal, this means that the service called inbetween is deemed to be
// approved. If the values are still the same, this means the counter has not
// been incremented, and the service called is not approved for FIPS.
//
// In non-FIPS builds, |FIPS_service_indicator_before_call| always returns zero
// and |FIPS_service_indicator_after_call| always returns one. Thus calls always
// appear to be approved. This is intended to simplify testing.
OPENSSL_EXPORT uint64_t FIPS_service_indicator_before_call(void);
OPENSSL_EXPORT uint64_t FIPS_service_indicator_after_call(void);

#if defined(__cplusplus)
}

#if !defined(BORINGSSL_NO_CXX)

extern "C++" {

// CALL_SERVICE_AND_CHECK_APPROVED runs |func| and sets |approved| to one of the
// |FIPSStatus*| values, above, depending on whether |func| invoked an
// approved service. The result of |func| becomes the result of this macro.
#define CALL_SERVICE_AND_CHECK_APPROVED(approved, func)         \
  [&] {                                                       \
    bssl::FIPSIndicatorHelper fips_indicator_helper(&approved); \
    return func;                                                \
  }()

BSSL_NAMESPACE_BEGIN

enum class FIPSStatus {
  NOT_APPROVED = 0,
  APPROVED = 1,
};

// FIPSIndicatorHelper records whether the service indicator counter advanced
// during its lifetime.
class FIPSIndicatorHelper {
 public:
  FIPSIndicatorHelper(FIPSStatus *result)
      : result_(result), before_(FIPS_service_indicator_before_call()) {
    *result_ = FIPSStatus::NOT_APPROVED;
  }

  ~FIPSIndicatorHelper() {
    uint64_t after = FIPS_service_indicator_after_call();
    if (after != before_) {
      *result_ = FIPSStatus::APPROVED;
    }
  }

  FIPSIndicatorHelper(const FIPSIndicatorHelper&) = delete;
  FIPSIndicatorHelper &operator=(const FIPSIndicatorHelper &) = delete;

 private:
  FIPSStatus *const result_;
  const uint64_t before_;
};

BSSL_NAMESPACE_END
}  // extern "C++"

#endif  // !BORINGSSL_NO_CXX
#endif  // __cplusplus

#endif  // OPENSSL_HEADER_SERVICE_INDICATOR_H
