// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_REMOTE_SECURITY_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_REMOTE_SECURITY_CONTEXT_H_

#include "services/network/public/cpp/permissions_policy/permissions_policy.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/security_context.h"

namespace blink {

class CORE_EXPORT RemoteSecurityContext final : public SecurityContext {
 public:
  RemoteSecurityContext();

  void SetReplicatedOrigin(scoped_refptr<SecurityOrigin>);
  void ResetReplicatedContentSecurityPolicy();
  void ResetAndEnforceSandboxFlags(
      network::mojom::blink::WebSandboxFlags flags);

  // Constructs the enforcement PermissionsPolicy struct for this security
  // context. The resulting PermissionsPolicy is a combination of:
  //   * |parsed_header|: from the PermissionsPolicy part of the response
  //   headers.
  //   * |container_policy|: from <iframe>'s allow attribute.
  //   * |parent_permissions_policy|: which is the current state of permissions
  //   policies in a parent browsing context (frame).
  // Note that |parent_permissions_policy| is null, and |container_policy| is
  // empty for a top-level security context.
  void InitializePermissionsPolicy(
      const network::ParsedPermissionsPolicy& parsed_header,
      const network::ParsedPermissionsPolicy& container_policy,
      const network::PermissionsPolicy* parent_permissions_policy);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_REMOTE_SECURITY_CONTEXT_H_
