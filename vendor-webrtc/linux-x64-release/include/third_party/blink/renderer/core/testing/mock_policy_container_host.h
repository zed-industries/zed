// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_POLICY_CONTAINER_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_POLICY_CONTAINER_HOST_H_

#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "services/network/public/mojom/content_security_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/mojom/frame/policy_container.mojom-blink.h"

namespace blink {

class MockPolicyContainerHost : public mojom::blink::PolicyContainerHost {
 public:
  MOCK_METHOD(void,
              SetReferrerPolicy,
              (network::mojom::ReferrerPolicy),
              (override));
  MOCK_METHOD(void,
              AddContentSecurityPolicies,
              (Vector<network::mojom::blink::ContentSecurityPolicyPtr>),
              (override));
  MockPolicyContainerHost() = default;

  // Wrapper around AssociatedReceiver::BindNewEndpointAndPassDedicatedRemote.
  mojo::PendingAssociatedRemote<mojom::blink::PolicyContainerHost>
  BindNewEndpointAndPassDedicatedRemote();

  // Wrapper around AssociatedReceiver::FlushForTesting.
  void FlushForTesting();

  // This does the same as BindNewEndpointAndPassDedicatedRemote, but allows the
  // remote to be created first and the receiver to be passed in.
  void BindWithNewEndpoint(
      mojo::PendingAssociatedReceiver<mojom::blink::PolicyContainerHost>
          receiver);

 private:
  mojo::AssociatedReceiver<mojom::blink::PolicyContainerHost> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_POLICY_CONTAINER_HOST_H_
