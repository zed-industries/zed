// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_SECURITY_CONTEXT_INIT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_SECURITY_CONTEXT_INIT_H_

#include <optional>

#include "base/types/optional_ref.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "third_party/blink/public/common/fenced_frame/redacted_fenced_frame_config.h"
#include "third_party/blink/public/common/frame/frame_policy.h"
#include "third_party/blink/public/common/permissions_policy/document_policy.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/web_feature.h"
#include "third_party/blink/renderer/core/permissions_policy/policy_helper.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class LocalFrame;
class ResourceResponse;

class CORE_EXPORT SecurityContextInit {
  STACK_ALLOCATED();

 public:
  explicit SecurityContextInit(ExecutionContext*);

  // Init |permissions_policy_| and |report_only_permissions_policy_| by copying
  // state from another security context instance.
  // Used to carry permissions policy information from previous document
  // to current document during XSLT navigation, because XSLT navigation
  // does not have header information available.
  void InitPermissionsPolicyFrom(const SecurityContext& other);

  // Init |document_policy_| and |report_only_document_policy_| by copying
  // state from another security context instance.
  // Used to carry document policy information from previous document
  // to current document during XSLT navigation, because XSLT navigation
  // does not have header information available.
  void InitDocumentPolicyFrom(const SecurityContext& other);

  void ApplyPermissionsPolicy(
      LocalFrame& frame,
      const ResourceResponse& response,
      const FramePolicy& frame_policy,
      const std::optional<network::ParsedPermissionsPolicy>&
          isolated_app_policy,
      const base::optional_ref<const FencedFrame::RedactedFencedFrameProperties>
          fenced_frame_properties,
      const KURL& document_url);
  void ApplyDocumentPolicy(
      DocumentPolicy::ParsedDocumentPolicy& document_policy,
      const String& report_only_document_policy_header);

  const network::ParsedPermissionsPolicy& PermissionsPolicyHeader() const {
    return permissions_policy_header_;
  }

 private:
  ExecutionContext* execution_context_ = nullptr;
  network::ParsedPermissionsPolicy permissions_policy_header_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_SECURITY_CONTEXT_INIT_H_
