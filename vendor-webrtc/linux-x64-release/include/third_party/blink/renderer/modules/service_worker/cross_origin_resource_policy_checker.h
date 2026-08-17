// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_CROSS_ORIGIN_RESOURCE_POLICY_CHECKER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_CROSS_ORIGIN_RESOURCE_POLICY_CHECKER_H_

#include "mojo/public/cpp/bindings/remote.h"
#include "services/network/public/cpp/cross_origin_embedder_policy.h"
#include "services/network/public/mojom/cross_origin_embedder_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/cross_origin_embedder_policy.mojom.h"
#include "services/network/public/mojom/document_isolation_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/document_isolation_policy.mojom.h"
#include "services/network/public/mojom/fetch_api.mojom.h"

namespace url {
class Origin;
}  // namespace url

namespace blink {

class Response;

// Contains the COEP policy, the DocumentIsolationPolicy and the reporters for
// the controllee and does CORP validation based on it. The lifetime is bound
// with the Mojo connection between the controllee and the service worker.
class CrossOriginResourcePolicyChecker {
 public:
  // |reporter| can be null if reporting is not necessary.
  CrossOriginResourcePolicyChecker(
      network::CrossOriginEmbedderPolicy coep,
      mojo::PendingRemote<
          network::mojom::blink::CrossOriginEmbedderPolicyReporter>
          coep_reporter,
      network::DocumentIsolationPolicy document_isolation_policy,
      mojo::PendingRemote<
          network::mojom::blink::DocumentIsolationPolicyReporter> dip_reporter);

  CrossOriginResourcePolicyChecker(const CrossOriginResourcePolicyChecker&) =
      delete;
  CrossOriginResourcePolicyChecker& operator=(
      const CrossOriginResourcePolicyChecker&) = delete;

  bool IsBlocked(const url::Origin& initiator_origin,
                 network::mojom::RequestMode request_mode,
                 network::mojom::RequestDestination request_destination,
                 const Response& response);

  base::WeakPtr<CrossOriginResourcePolicyChecker> GetWeakPtr();

 private:
  const network::CrossOriginEmbedderPolicy coep_;
  mojo::Remote<network::mojom::CrossOriginEmbedderPolicyReporter>
      coep_reporter_;
  const network::DocumentIsolationPolicy document_isolation_policy_;
  mojo::Remote<network::mojom::DocumentIsolationPolicyReporter> dip_reporter_;

  base::WeakPtrFactory<CrossOriginResourcePolicyChecker> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_CROSS_ORIGIN_RESOURCE_POLICY_CHECKER_H_
