// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_WEB_BUNDLE_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_WEB_BUNDLE_LOADER_H_

#include "services/network/public/mojom/web_bundle_handle.mojom-blink.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink.h"
#include "third_party/blink/renderer/core/html/cross_origin_attribute.h"
#include "third_party/blink/renderer/core/loader/threadable_loader_client.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver_set.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

class SecurityOrigin;
class SubresourceWebBundle;
class Document;
class ThreadableLoader;

// A loader which is used to load a resource from webbundle.
class WebBundleLoader : public GarbageCollected<WebBundleLoader>,
                        public ThreadableLoaderClient,
                        public network::mojom::blink::WebBundleHandle {
 public:
  WebBundleLoader(SubresourceWebBundle& subresource_web_bundle,
                  Document& document,
                  const KURL& url,
                  network::mojom::CredentialsMode credentials_mode);

  void Trace(Visitor* visitor) const override;

  bool HasLoaded() const { return load_state_ == LoadState::kSuccess; }
  bool HasFailed() const { return load_state_ == LoadState::kFailed; }

  // ThreadableLoaderClient
  void DidStartLoadingResponseBody(BytesConsumer& consumer) override;
  void DidFail(uint64_t, const ResourceError&) override;
  void DidFailRedirectCheck(uint64_t) override;

  // network::mojom::blink::WebBundleHandle
  void Clone(mojo::PendingReceiver<network::mojom::blink::WebBundleHandle>
                 receiver) override;
  void OnWebBundleError(network::mojom::blink::WebBundleErrorType type,
                        const String& message) override;
  void OnWebBundleLoadFinished(bool success) override;

  const KURL& url() const { return url_; }
  scoped_refptr<const SecurityOrigin> GetSecurityOrigin() const {
    return security_origin_;
  }
  const base::UnguessableToken& WebBundleToken() const {
    return web_bundle_token_;
  }

  void ClearReceivers();

 private:
  enum class LoadState { kInProgress, kFailed, kSuccess };

  void DidFailInternal();

  Member<SubresourceWebBundle> subresource_web_bundle_;
  Member<ThreadableLoader> loader_;
  LoadState load_state_ = LoadState::kInProgress;
  KURL url_;
  scoped_refptr<const SecurityOrigin> security_origin_;
  base::UnguessableToken web_bundle_token_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  // We need ReceiverSet here because WebBundleHandle is cloned when
  // ResourceRequest is copied.
  HeapMojoReceiverSet<network::mojom::blink::WebBundleHandle, WebBundleLoader>
      receivers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_WEB_BUNDLE_LOADER_H_
