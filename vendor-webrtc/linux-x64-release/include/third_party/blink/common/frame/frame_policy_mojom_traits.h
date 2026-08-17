// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_COMMON_FRAME_FRAME_POLICY_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_COMMON_FRAME_FRAME_POLICY_MOJOM_TRAITS_H_

#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_mojom_traits.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-shared.h"
#include "third_party/blink/common/permissions_policy/policy_value_mojom_traits.h"
#include "third_party/blink/public/common/frame/frame_policy.h"
#include "third_party/blink/public/mojom/fenced_frame/fenced_frame.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/frame_policy.mojom-shared.h"

namespace mojo {

template <>
class BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::FramePolicyDataView, blink::FramePolicy> {
 public:
  static const std::vector<network::ParsedPermissionsPolicyDeclaration>&
  container_policy(const blink::FramePolicy& frame_policy) {
    return frame_policy.container_policy;
  }

  static network::mojom::WebSandboxFlags sandbox_flags(
      const blink::FramePolicy& frame_policy) {
    return frame_policy.sandbox_flags;
  }

  static const blink::DocumentPolicyFeatureState& required_document_policy(
      const blink::FramePolicy& frame_policy) {
    return frame_policy.required_document_policy;
  }

  static const blink::mojom::DeferredFetchPolicy& deferred_fetch_policy(
      const blink::FramePolicy& frame_policy) {
    return frame_policy.deferred_fetch_policy;
  }

  static bool Read(blink::mojom::FramePolicyDataView in,
                   blink::FramePolicy* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_COMMON_FRAME_FRAME_POLICY_MOJOM_TRAITS_H_
