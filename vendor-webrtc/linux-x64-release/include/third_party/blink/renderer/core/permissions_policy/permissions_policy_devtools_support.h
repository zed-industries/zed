// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_PERMISSIONS_POLICY_DEVTOOLS_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_PERMISSIONS_POLICY_DEVTOOLS_SUPPORT_H_

#include <optional>

#include "services/network/public/mojom/permissions_policy/permissions_policy_feature.mojom-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
class Frame;

// The reason for a feature to be disallowed.
enum class PermissionsPolicyBlockReason {
  // Feature's allowlist declaration can be overridden either in HTTP header,
  // or in iframe attribute.
  kHeader,
  kIframeAttribute,
  // All permissions are disabled by default for fenced frames, irrespective of
  // headers.
  kInFencedFrameTree,
  // Feature is not specified in an Isolated App's Web App Manifest and will be
  // disabled.
  kInIsolatedApp,
};

struct PermissionsPolicyBlockLocator {
  // FrameId used in devtools protocol.
  String frame_id;
  // Note: Attribute declaration is on frame's owner element, which is
  // technically above 1 level in the frame tree.
  PermissionsPolicyBlockReason reason;
};

// Traces the root reason for a feature to be disabled in a frame.
// Returns std::nullopt when the feature is enabled in the frame.
CORE_EXPORT std::optional<PermissionsPolicyBlockLocator>
TracePermissionsPolicyBlockSource(Frame*,
                                  network::mojom::PermissionsPolicyFeature);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_PERMISSIONS_POLICY_DEVTOOLS_SUPPORT_H_
