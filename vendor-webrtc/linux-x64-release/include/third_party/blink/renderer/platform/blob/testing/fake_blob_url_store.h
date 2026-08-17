// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_BLOB_URL_STORE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_BLOB_URL_STORE_H_

#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-blink.h"

#include "base/unguessable_token.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/blob/blob.mojom-blink.h"
#include "third_party/blink/renderer/platform/weborigin/kurl_hash.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// Mocked BlobURLStore implementation for testing.
class FakeBlobURLStore : public mojom::blink::BlobURLStore {
 public:
  void Register(
      mojo::PendingRemote<mojom::blink::Blob>,
      const KURL&,
      // TODO(https://crbug.com/1224926): Remove this once experiment is over.
      const base::UnguessableToken& unsafe_agent_cluster_id,
      const std::optional<BlinkSchemefulSite>& unsafe_top_level_site,
      RegisterCallback) override;
  void Revoke(const KURL&) override;
  void ResolveAsURLLoaderFactory(
      const KURL&,
      mojo::PendingReceiver<network::mojom::blink::URLLoaderFactory>,
      ResolveAsURLLoaderFactoryCallback) override;
  void ResolveAsBlobURLToken(const KURL&,
                             mojo::PendingReceiver<mojom::blink::BlobURLToken>,
                             bool is_top_level_navigation,
                             ResolveAsBlobURLTokenCallback) override;
  HashMap<KURL, mojo::Remote<mojom::blink::Blob>> registrations;
  HashMap<KURL, base::UnguessableToken> agent_registrations;
  Vector<KURL> revocations;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_BLOB_URL_STORE_H_
