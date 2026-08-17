// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_NAVIGATION_POLICY_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_NAVIGATION_POLICY_MOJOM_TRAITS_H_

#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/navigation/navigation_policy.h"
#include "third_party/blink/public/mojom/navigation/navigation_policy.mojom-shared.h"
#include "third_party/blink/public/mojom/navigation/navigation_policy.mojom.h"

namespace mojo {

template <>
class BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::NavigationDownloadTypesDataView,
                 blink::NavigationDownloadPolicy::NavigationDownloadTypes> {
 public:
  static bool view_source(
      const blink::NavigationDownloadPolicy::NavigationDownloadTypes& types);
  static bool interstitial(
      const blink::NavigationDownloadPolicy::NavigationDownloadTypes& types);
  static bool opener_cross_origin(
      const blink::NavigationDownloadPolicy::NavigationDownloadTypes& types);
  static bool ad_frame_no_gesture(
      const blink::NavigationDownloadPolicy::NavigationDownloadTypes& types);
  static bool ad_frame(
      const blink::NavigationDownloadPolicy::NavigationDownloadTypes& types);
  static bool sandbox(
      const blink::NavigationDownloadPolicy::NavigationDownloadTypes& types);
  static bool no_gesture(
      const blink::NavigationDownloadPolicy::NavigationDownloadTypes& types);

  static bool Read(
      blink::mojom::NavigationDownloadTypesDataView in,
      blink::NavigationDownloadPolicy::NavigationDownloadTypes* out);
};

template <>
class BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::NavigationDownloadPolicyDataView,
                 blink::NavigationDownloadPolicy> {
 public:
  static blink::mojom::NavigationDownloadTypesPtr observed_types(
      const blink::NavigationDownloadPolicy& download_policy);

  static blink::mojom::NavigationDownloadTypesPtr disallowed_types(
      const blink::NavigationDownloadPolicy& download_policy);

  static bool Read(blink::mojom::NavigationDownloadPolicyDataView in,
                   blink::NavigationDownloadPolicy* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_NAVIGATION_POLICY_MOJOM_TRAITS_H_
