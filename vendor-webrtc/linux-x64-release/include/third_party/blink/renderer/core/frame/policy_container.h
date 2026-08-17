// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_POLICY_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_POLICY_CONTAINER_H_

#include "mojo/public/cpp/bindings/associated_remote.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/network/public/mojom/content_security_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/policy_container.mojom-blink.h"
#include "third_party/blink/public/platform/web_policy_container.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

// PolicyContainer serves as a container for several security policies to be
// applied to a document. It is constructed at commit time with the data passed
// by the RenderFrameHost. It is Blink's counterpart of the PolicyContainerHost,
// which is held by the RenderFrameHost. Some document policies of the policy
// container can be updated also by Blink (this generally happens when Blink
// parses meta tags). The corresponding setters trigger also an update in the
// corresponding PolicyContainerHost via a mojo IPC.
class CORE_EXPORT PolicyContainer {
  USING_FAST_MALLOC(PolicyContainer);

 public:
  PolicyContainer() = delete;
  PolicyContainer(
      mojo::PendingAssociatedRemote<mojom::blink::PolicyContainerHost> remote,
      mojom::blink::PolicyContainerPoliciesPtr policies);
  PolicyContainer(const PolicyContainer&) = delete;
  PolicyContainer& operator=(const PolicyContainer&) = delete;
  ~PolicyContainer() = default;

  static std::unique_ptr<PolicyContainer> CreateEmpty();
  static std::unique_ptr<PolicyContainer> CreateFromWebPolicyContainer(
      std::unique_ptr<WebPolicyContainer> container);

  // Change the Referrer Policy and sync the new policy with the corresponding
  // PolicyContainerHost.
  void UpdateReferrerPolicy(network::mojom::blink::ReferrerPolicy policy);
  network::mojom::blink::ReferrerPolicy GetReferrerPolicy() const;

  // Append |policies| to the list of Content Security Policy and sync them with
  // the PolicyContainerHost.
  void AddContentSecurityPolicies(
      Vector<network::mojom::blink::ContentSecurityPolicyPtr> policies);

  const mojom::blink::PolicyContainerPolicies& GetPolicies() const;

 private:
  mojom::blink::PolicyContainerPoliciesPtr policies_;

  mojo::AssociatedRemote<mojom::blink::PolicyContainerHost>
      policy_container_host_remote_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_POLICY_CONTAINER_H_
