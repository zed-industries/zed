// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SECURITY_CONTEXT_INSECURE_REQUEST_POLICY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SECURITY_CONTEXT_INSECURE_REQUEST_POLICY_H_

#include <bitset>

#include "base/check_op.h"
#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-shared.h"

namespace blink {
namespace mojom {

inline constexpr InsecureRequestPolicy operator&(InsecureRequestPolicy a,
                                                 InsecureRequestPolicy b) {
  return static_cast<InsecureRequestPolicy>(static_cast<int>(a) &
                                            static_cast<int>(b));
}

inline constexpr InsecureRequestPolicy operator|(InsecureRequestPolicy a,
                                                 InsecureRequestPolicy b) {
  InsecureRequestPolicy result = static_cast<InsecureRequestPolicy>(
      static_cast<int>(a) | static_cast<int>(b));
  DCHECK_LE(result, InsecureRequestPolicy::kMaxInsecureRequestPolicy);
  return result;
}

inline InsecureRequestPolicy& operator|=(InsecureRequestPolicy& a,
                                         InsecureRequestPolicy b) {
  return a = a | b;
}

inline constexpr InsecureRequestPolicy operator~(InsecureRequestPolicy flags) {
  return static_cast<InsecureRequestPolicy>(~static_cast<int>(flags));
}

}  // namespace mojom
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SECURITY_CONTEXT_INSECURE_REQUEST_POLICY_H_
