// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_POLICY_CONTAINER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_POLICY_CONTAINER_H_

#include <vector>

#include "services/network/public/mojom/cross_origin_embedder_policy.mojom-shared.h"
#include "services/network/public/mojom/ip_address_space.mojom-shared.h"
#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/policy_container.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_content_security_policy_struct.h"

namespace blink {

// TODO(antoniosartori): Remove this when CommitNavigation IPC will be handled
// directly in blink.
struct WebPolicyContainerPolicies {
  network::mojom::CrossOriginEmbedderPolicyValue cross_origin_embedder_policy =
      network::mojom::CrossOriginEmbedderPolicyValue::kNone;
  network::mojom::ReferrerPolicy referrer_policy =
      network::mojom::ReferrerPolicy::kDefault;
  std::vector<WebContentSecurityPolicy> content_security_policies;
  bool is_credentialless = false;
  network::mojom::WebSandboxFlags sandbox_flags =
      network::mojom::WebSandboxFlags::kNone;
  network::mojom::IPAddressSpace ip_address_space =
      network::mojom::IPAddressSpace::kUnknown;
  bool can_navigate_top_without_user_gesture = true;
  // An extra bit ensuring that the document cannot be cross-origin-isolated
  // when it's false. Note that it is a necessary condition but not a sufficient
  // condition on its own.
  bool allow_cross_origin_isolation = false;
  bool cross_origin_isolation_enabled_by_dip = false;
};

// TODO(antoniosartori): Remove this when CommitNavigation IPC will be handled
// directly in blink.
struct WebPolicyContainer {
  WebPolicyContainer() = default;

  WebPolicyContainer(
      WebPolicyContainerPolicies policies,
      CrossVariantMojoAssociatedRemote<mojom::PolicyContainerHostInterfaceBase>
          remote)
      : policies(std::move(policies)), remote(std::move(remote)) {}

  WebPolicyContainerPolicies policies;
  CrossVariantMojoAssociatedRemote<mojom::PolicyContainerHostInterfaceBase>
      remote;
};

struct WebPolicyContainerBindParams {
  CrossVariantMojoAssociatedReceiver<mojom::PolicyContainerHostInterfaceBase>
      receiver;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_POLICY_CONTAINER_H_
