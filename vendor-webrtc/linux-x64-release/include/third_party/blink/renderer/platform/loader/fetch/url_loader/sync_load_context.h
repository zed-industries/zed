// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_SYNC_LOAD_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_SYNC_LOAD_CONTEXT_H_

#include <vector>

#include "base/memory/raw_ptr.h"
#include "base/synchronization/waitable_event_watcher.h"
#include "base/task/single_thread_task_runner.h"
#include "base/timer/timer.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "net/traffic_annotation/network_traffic_annotation.h"
#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "third_party/blink/public/mojom/blob/blob_registry.mojom-blink.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/resource_request_client.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/resource_request_sender.h"

namespace base {
class WaitableEvent;
}

namespace network {
struct ResourceRequest;
}

namespace blink {
class ResourceLoadInfoNotifierWrapper;
class URLLoaderThrottle;
struct SyncLoadResponse;

// This class owns the context necessary to perform an asynchronous request
// while the main thread is blocked so that it appears to be synchronous.
// There are a couple of modes to load a request:
//   1) kDataPipe; body is received on a data pipe passed on
//      OnStartLoadingResponseBody(), and the body is set to response_.data.
//   2) kBlob: body is received on a data pipe passed on
//      OnStartLoadingResponseBody(), and wraps the data pipe with a
//      SerializedBlobPtr.
class BLINK_PLATFORM_EXPORT SyncLoadContext : public ResourceRequestClient {
 public:
  // Begins a new asynchronous request on whatever sequence this method is
  // called on. |completed_event| will be signalled when the request is complete
  // and |response| will be populated with the response data. |abort_event|
  // will be signalled from the main thread to abort the sync request on a
  // worker thread when the worker thread is being terminated.
  // The pointer whose address is `context_for_redirect` is held by the caller
  // that is blocked on this method, so it will remain valid until the operation
  // completes. If there are redirects, `context_for_redirect` will point to the
  // callee context.
  // If |download_to_blob_registry| is not null, it is used to
  // redirect the download to a blob, with the resulting blob populated in
  // |response|.
  static void StartAsyncWithWaitableEvent(
      std::unique_ptr<network::ResourceRequest> request,
      scoped_refptr<base::SingleThreadTaskRunner> loading_task_runner,
      const net::NetworkTrafficAnnotationTag& traffic_annotation,
      uint32_t loader_options,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_url_loader_factory,
      std::vector<std::unique_ptr<URLLoaderThrottle>> throttles,
      SyncLoadResponse* response,
      SyncLoadContext** context_for_redirect,
      base::WaitableEvent* completed_event,
      base::WaitableEvent* abort_event,
      base::TimeDelta timeout,
      mojo::PendingRemote<mojom::blink::BlobRegistry> download_to_blob_registry,
      const Vector<String>& cors_exempt_header_list,
      std::unique_ptr<ResourceLoadInfoNotifierWrapper>
          resource_load_info_notifier_wrapper);

  SyncLoadContext(const SyncLoadContext&) = delete;
  SyncLoadContext& operator=(const SyncLoadContext&) = delete;
  ~SyncLoadContext() override;

  void FollowRedirect(std::vector<std::string> removed_headers,
                      net::HttpRequestHeaders modified_headers);
  void CancelRedirect();

 private:
  friend class SyncLoadContextTest;

  SyncLoadContext(
      network::ResourceRequest* request,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          url_loader_factory,
      SyncLoadResponse* response,
      SyncLoadContext** context_for_redirect,
      base::WaitableEvent* completed_event,
      base::WaitableEvent* abort_event,
      base::TimeDelta timeout,
      mojo::PendingRemote<mojom::blink::BlobRegistry> download_to_blob_registry,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  // ResourceRequestClient implementation:
  void OnUploadProgress(uint64_t position, uint64_t size) override;
  void OnReceivedRedirect(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr head,
      FollowRedirectCallback follow_redirect_callback) override;
  void OnReceivedResponse(
      network::mojom::URLResponseHeadPtr head,
      mojo::ScopedDataPipeConsumerHandle body,
      std::optional<mojo_base::BigBuffer> cached_metadata) override;
  void OnTransferSizeUpdated(int transfer_size_diff) override;
  void OnCompletedRequest(
      const network::URLLoaderCompletionStatus& status) override;
  void OnFinishCreatingBlob(const scoped_refptr<BlobDataHandle>& blob);

  void OnBodyReadable(MojoResult, const mojo::HandleSignalsState&);

  void OnAbort(base::WaitableEvent* event);
  void OnTimeout();

  void CompleteRequest();
  bool Completed() const;

  // This raw pointer will remain valid for the lifetime of this object because
  // it remains on the stack until |event_| is signaled.
  // Set to null after CompleteRequest() is called.
  raw_ptr<SyncLoadResponse> response_;

  // Used when handling a redirect. It is set in OnReceivedRedirect(), and
  // called when FollowRedirect() is called from the original thread.
  FollowRedirectCallback follow_redirect_callback_;

  // This raw pointer will be set to `this` when receiving redirects on
  // independent thread and set to nullptr in `FollowRedirect()` or
  // `CancelRedirect()` on the same thread after `redirect_or_response_event_`
  // is signaled, which protects it against race condition.
  raw_ptr<SyncLoadContext*> context_for_redirect_;

  enum class Mode { kInitial, kDataPipe, kBlob };
  Mode mode_ = Mode::kInitial;

  // Used when Mode::kDataPipe.
  mojo::ScopedDataPipeConsumerHandle body_handle_;
  mojo::SimpleWatcher body_watcher_;

  // State necessary to run a request on an independent thread.
  scoped_refptr<network::SharedURLLoaderFactory> url_loader_factory_;
  std::unique_ptr<ResourceRequestSender> resource_request_sender_;

  // State for downloading to a blob.
  mojo::Remote<mojom::blink::BlobRegistry> download_to_blob_registry_;
  bool blob_response_started_ = false;
  bool blob_finished_ = false;
  bool request_completed_ = false;

  // True when the request contains Authorization header.
  // TODO(https://crbug.com/1393520): Remove this field once we get enough
  // stats to make a decision.
  bool has_authorization_header_ = false;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  class SignalHelper;
  std::unique_ptr<SignalHelper> signals_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_SYNC_LOAD_CONTEXT_H_
