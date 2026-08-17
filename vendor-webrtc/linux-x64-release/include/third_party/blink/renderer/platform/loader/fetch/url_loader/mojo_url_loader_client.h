// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_MOJO_URL_LOADER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_MOJO_URL_LOADER_CLIENT_H_

#include <stdint.h>

#include <vector>

#include "base/functional/callback_forward.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/time/time.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "services/network/public/mojom/url_loader.mojom.h"
#include "third_party/blink/public/mojom/navigation/renderer_eviction_reason.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace net {
struct RedirectInfo;
}  // namespace net

namespace network {
struct URLLoaderCompletionStatus;
}  // namespace network

namespace blink {
class ResourceRequestSender;

// MojoURLLoaderClient is an implementation of
// network::mojom::URLLoaderClient to receive messages from a single URLLoader.
class BLINK_PLATFORM_EXPORT MojoURLLoaderClient final
    : public network::mojom::URLLoaderClient {
 public:
  MojoURLLoaderClient(
      ResourceRequestSender* resource_request_sender,
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      bool bypass_redirect_checks,
      const GURL& request_url,
      base::OnceCallback<void(mojom::blink::RendererEvictionReason)>
          evict_from_bfcache_callback,
      base::RepeatingCallback<void(size_t)>
          did_buffer_load_while_in_bfcache_callback);
  ~MojoURLLoaderClient() override;

  // Freezes the loader. See blink/renderer/platform/loader/README.md for the
  // general concept of "freezing" in the loading module. See
  // blink/public/platform/web_loader_freezing_mode.h for `mode`.
  void Freeze(LoaderFreezeMode mode);

  // network::mojom::URLLoaderClient implementation
  void OnReceiveEarlyHints(network::mojom::EarlyHintsPtr early_hints) override;
  void OnReceiveResponse(
      network::mojom::URLResponseHeadPtr response_head,
      mojo::ScopedDataPipeConsumerHandle body,
      std::optional<mojo_base::BigBuffer> cached_metadata) override;
  void OnReceiveRedirect(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr response_head) override;
  void OnUploadProgress(int64_t current_position,
                        int64_t total_size,
                        OnUploadProgressCallback ack_callback) override;
  void OnTransferSizeUpdated(int32_t transfer_size_diff) override;
  void OnComplete(const network::URLLoaderCompletionStatus& status) override;

  void EvictFromBackForwardCache(mojom::blink::RendererEvictionReason reason);
  void DidBufferLoadWhileInBackForwardCache(size_t num_bytes);
  bool CanContinueBufferingWhileInBackForwardCache();
  LoaderFreezeMode freeze_mode() const { return freeze_mode_; }

 private:
  class BodyBuffer;
  class DeferredMessage;
  class DeferredOnReceiveResponse;
  class DeferredOnReceiveRedirect;
  class DeferredOnUploadProgress;
  class DeferredOnReceiveCachedMetadata;
  class DeferredOnStartLoadingResponseBody;
  class DeferredOnComplete;

  bool NeedsStoringMessage() const;
  void StoreAndDispatch(std::unique_ptr<DeferredMessage> message);
  void OnConnectionClosed();
  const KURL& last_loaded_url() const { return last_loaded_url_; }

  // Dispatches the messages received after SetDefersLoading is called.
  void FlushDeferredMessages();

  void EvictFromBackForwardCacheDueToTimeout();
  void StopBackForwardCacheEvictionTimer();

  Vector<std::unique_ptr<DeferredMessage>> deferred_messages_;
  std::unique_ptr<BodyBuffer> body_buffer_;
  base::OneShotTimer back_forward_cache_eviction_timer_;
  base::TimeDelta back_forward_cache_timeout_;
  bool has_received_response_head_ = false;
  bool has_received_response_body_ = false;
  bool has_received_complete_ = false;
  LoaderFreezeMode freeze_mode_ = LoaderFreezeMode::kNone;
  int32_t accumulated_transfer_size_diff_during_deferred_ = 0;
  const raw_ptr<ResourceRequestSender, DanglingUntriaged>
      resource_request_sender_;
  scoped_refptr<base::SequencedTaskRunner> task_runner_;
  bool bypass_redirect_checks_ = false;
  KURL last_loaded_url_;
  base::OnceCallback<void(mojom::blink::RendererEvictionReason)>
      evict_from_bfcache_callback_;
  base::RepeatingCallback<void(size_t)>
      did_buffer_load_while_in_bfcache_callback_;

  base::WeakPtrFactory<MojoURLLoaderClient> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_MOJO_URL_LOADER_CLIENT_H_
