// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_REFERRER_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_REFERRER_UTILS_H_

#include "net/url_request/referrer_policy.h"
#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

class ReferrerUtils {
 public:
  static BLINK_COMMON_EXPORT network::mojom::ReferrerPolicy
  NetToMojoReferrerPolicy(net::ReferrerPolicy net_policy);

  static BLINK_COMMON_EXPORT net::ReferrerPolicy GetDefaultNetReferrerPolicy();

  // The ReferrerPolicy enum contains a member kDefault, which is not a real
  // referrer policy, but instead Blink falls back to
  // kStrictOriginWhenCrossOrigin when a certain condition is met. The function
  // below is provided so that a referrer policy which may be kDefault can be
  // resolved to a valid value.
  static BLINK_COMMON_EXPORT network::mojom::ReferrerPolicy
      MojoReferrerPolicyResolveDefault(network::mojom::ReferrerPolicy);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_REFERRER_UTILS_H_
