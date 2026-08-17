/*
 * Copyright (C) 2012 Motorola Mobility Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_PUBLIC_URL_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_PUBLIC_URL_MANAGER_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/associated_remote.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "services/network/public/mojom/url_loader_factory.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KURL;
class ExecutionContext;
class GlobalStorageAccessHandle;
class URLRegistry;
class URLRegistrable;

class CORE_EXPORT PublicURLManager final
    : public GarbageCollected<PublicURLManager>,
      public ExecutionContextLifecycleObserver {
 public:
  explicit PublicURLManager(ExecutionContext*);
  explicit PublicURLManager(
      base::PassKey<GlobalStorageAccessHandle>,
      ExecutionContext*,
      mojo::PendingAssociatedRemote<mojom::blink::BlobURLStore>);

  // Generates a new Blob URL and registers the URLRegistrable to the
  // corresponding URLRegistry with the Blob URL. Returns the serialization
  // of the Blob URL.
  String RegisterURL(URLRegistrable*);
  // Revokes the given URL.
  void Revoke(const KURL&);
  // When mojo Blob URLs are enabled this resolves the provided URL to a
  // factory capable of creating loaders for the specific URL.
  void Resolve(const KURL&,
               mojo::PendingReceiver<network::mojom::blink::URLLoaderFactory>);
  // When mojo Blob URLs are enabled this resolves the provided URL to a mojom
  // BlobURLToken. This token can be used by the browser process to securely
  // lookup what blob a URL used to refer to, even after the URL is revoked.
  // If the URL fails to resolve the request will simply be disconnected.
  void ResolveAsBlobURLToken(const KURL&,
                             mojo::PendingReceiver<mojom::blink::BlobURLToken>,
                             bool is_top_level_navigation);

  // ExecutionContextLifecycleObserver interface.
  void ContextDestroyed() override;

  void Trace(Visitor*) const override;

  void SetURLStoreForTesting(
      HeapMojoAssociatedRemote<mojom::blink::BlobURLStore> frame_url_store) {
    frame_url_store_ = std::move(frame_url_store);
  }

  // Returns a reference to the BlobURLStore from either `frame_url_store_` or
  // `worker_url_store_` (depending on the context).
  mojom::blink::BlobURLStore& GetBlobURLStore();

 private:
  typedef String URLString;
  // Map from URLs to the URLRegistry they are registered with.
  typedef HashMap<URLString, URLRegistry*> URLToRegistryMap;
  URLToRegistryMap url_to_registry_;
  HashSet<URLString> mojo_urls_;

  bool is_stopped_ = false;

  // For a frame context or from a main thread worklet context, a
  // navigation-associated interface is used to preserve message ordering.
  // Remotes corresponding to that interface get stored in `frame_url_store_`.
  // For workers and threaded worklets, the interface is exposed via
  // `BrowserInterfaceBroker`s, and the remotes for that are stored in
  // `worker_url_store_`.
  HeapMojoAssociatedRemote<mojom::blink::BlobURLStore> frame_url_store_;
  HeapMojoRemote<mojom::blink::BlobURLStore> worker_url_store_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_PUBLIC_URL_MANAGER_H_
