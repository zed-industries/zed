// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_RESOURCE_REQUEST_SENDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_RESOURCE_REQUEST_SENDER_H_

#include <stdint.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "base/functional/callback_forward.h"
#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "base/time/time.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "net/base/host_port_pair.h"
#include "net/base/net_errors.h"
#include "net/base/request_priority.h"
#include "net/http/http_request_headers.h"
#include "net/traffic_annotation/network_traffic_annotation.h"
#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "services/network/public/mojom/fetch_api.mojom-forward.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/common/loader/url_loader_throttle.h"
#include "third_party/blink/public/mojom/blob/blob_registry.mojom-blink.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/resource_load_info.mojom-shared.h"
#include "third_party/blink/public/mojom/loader/resource_load_info.mojom.h"
#include "third_party/blink/public/mojom/navigation/renderer_eviction_reason.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class WaitableEvent;
}

namespace net {
struct RedirectInfo;
}

namespace network {
struct ResourceRequest;
struct URLLoaderCompletionStatus;
}  // namespace network

namespace blink {
class CodeCacheFetcher;
class CodeCacheHost;
class ResourceLoadInfoNotifierWrapper;
class ThrottlingURLLoader;
class MojoURLLoaderClient;
class ResourceRequestClient;
struct SyncLoadResponse;

// This class creates a PendingRequestInfo object and handles sending a resource
// request asynchronously or synchronously, and it's owned by
// URLLoader::Context or SyncLoadContext.
class BLINK_PLATFORM_EXPORT ResourceRequestSender {
 public:
  ResourceRequestSender();
  ResourceRequestSender(const ResourceRequestSender&) = delete;
  ResourceRequestSender& operator=(const ResourceRequestSender&) = delete;
  virtual ~ResourceRequestSender();

  // Call this method to load the resource synchronously (i.e., in one shot).
  // This is an alternative to the StartAsync method. Be warned that this method
  // will block the calling thread until the resource is fully downloaded or an
  // error occurs. It could block the calling thread for a long time, so only
  // use this if you really need it!  There is also no way for the caller to
  // interrupt this method. Errors are reported via the status field of the
  // response parameter.
  //
  // |timeout| is used to abort the sync request on timeouts. TimeDelta::Max()
  // is interpreted as no-timeout.
  // If |download_to_blob_registry| is not null, it is used to redirect the
  // download to a blob.
  virtual void SendSync(
      std::unique_ptr<network::ResourceRequest> request,
      const net::NetworkTrafficAnnotationTag& traffic_annotation,
      uint32_t loader_options,
      SyncLoadResponse* response,
      scoped_refptr<network::SharedURLLoaderFactory> url_loader_factory,
      std::vector<std::unique_ptr<URLLoaderThrottle>> throttles,
      base::TimeDelta timeout,
      const Vector<String>& cors_exempt_header_list,
      base::WaitableEvent* terminate_sync_load_event,
      mojo::PendingRemote<mojom::blink::BlobRegistry> download_to_blob_registry,
      scoped_refptr<ResourceRequestClient> client,
      std::unique_ptr<ResourceLoadInfoNotifierWrapper>
          resource_load_info_notifier_wrapper);

  // Call this method to initiate the request. If this method succeeds, then
  // the client's methods will be called asynchronously to report various
  // events. Returns the request id. |url_loader_factory| must be non-null.
  //
  // You need to pass a non-null |loading_task_runner| to specify task queue to
  // execute loading tasks on.
  virtual int SendAsync(
      std::unique_ptr<network::ResourceRequest> request,
      scoped_refptr<base::SequencedTaskRunner> loading_task_runner,
      const net::NetworkTrafficAnnotationTag& traffic_annotation,
      uint32_t loader_options,
      const Vector<String>& cors_exempt_header_list,
      scoped_refptr<ResourceRequestClient> client,
      scoped_refptr<network::SharedURLLoaderFactory> url_loader_factory,
      std::vector<std::unique_ptr<URLLoaderThrottle>> throttles,
      std::unique_ptr<ResourceLoadInfoNotifierWrapper>
          resource_load_info_notifier_wrapper,
      CodeCacheHost* code_cache_host,
      base::OnceCallback<void(mojom::blink::RendererEvictionReason)>
          evict_from_bfcache_callback,
      base::RepeatingCallback<void(size_t)>
          did_buffer_load_while_in_bfcache_callback);

  // Cancels the current request and `request_info_` will be released.
  virtual void Cancel(scoped_refptr<base::SequencedTaskRunner> task_runner);

  // Freezes the loader. See blink/renderer/platform/loader/README.md for the
  // general concept of "freezing" in the loading module. See
  // blink/public/platform/web_loader_freezing_mode.h for `mode`.
  virtual void Freeze(LoaderFreezeMode mode);

  // Indicates the priority of the specified request changed.
  void DidChangePriority(net::RequestPriority new_priority,
                         int intra_priority_value);

  virtual void DeletePendingRequest(
      scoped_refptr<base::SequencedTaskRunner> task_runner);

  // Called when the transfer size is updated.
  virtual void OnTransferSizeUpdated(int32_t transfer_size_diff);

  // Called as upload progress is made.
  virtual void OnUploadProgress(int64_t position, int64_t size);

  // Called when response headers are available.
  virtual void OnReceivedResponse(
      network::mojom::URLResponseHeadPtr response_head,
      mojo::ScopedDataPipeConsumerHandle body,
      std::optional<mojo_base::BigBuffer> cached_metadata,
      base::TimeTicks response_ipc_arrival_time);

  // Called when a redirect occurs.
  virtual void OnReceivedRedirect(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr response_head,
      base::TimeTicks redirect_ipc_arrival_time);

  // Called when the response is complete.
  virtual void OnRequestComplete(
      const network::URLLoaderCompletionStatus& status,
      base::TimeTicks complete_ipc_arrival_time);

 private:
  friend class URLLoaderClientImpl;
  friend class URLResponseBodyConsumer;

  struct PendingRequestInfo {
    PendingRequestInfo(scoped_refptr<ResourceRequestClient> client,
                       network::mojom::RequestDestination request_destination,
                       const KURL& request_url,
                       std::unique_ptr<ResourceLoadInfoNotifierWrapper>
                           resource_load_info_notifier_wrapper);

    ~PendingRequestInfo();

    scoped_refptr<ResourceRequestClient> client;
    network::mojom::RequestDestination request_destination;
    LoaderFreezeMode freeze_mode = LoaderFreezeMode::kNone;
    // Original requested url.
    KURL url;
    // The url, method and referrer of the latest response even in case of
    // redirection.
    KURL response_url;
    bool has_pending_redirect = false;
    base::TimeTicks local_request_start;
    base::TimeTicks local_response_start;
    base::TimeTicks remote_request_start;
    net::LoadTimingInfo load_timing_info;
    bool redirect_requires_loader_restart = false;
    // Network error code the request completed with, or net::ERR_IO_PENDING if
    // it's not completed. Used both to distinguish completion from
    // cancellation, and to log histograms.
    int net_error = net::ERR_IO_PENDING;

    std::unique_ptr<ThrottlingURLLoader> url_loader;
    std::unique_ptr<MojoURLLoaderClient> url_loader_client;

    // The Client Hints headers that need to be removed from a redirect.
    //
    // May also include the `Shared-Storage-Writable` header in the case that
    // permission has been revoked on a redirect.
    std::vector<std::string> removed_headers
        ALLOW_DISCOURAGED_TYPE("Matches Chrome net API");

    // Headers that need to be added or updated, e.g. the
    // `Shared-Storage-Writable` header in the case that permission has been
    // restored on a redirect.
    net::HttpRequestHeaders modified_headers;

    // Used to notify the loading stats.
    std::unique_ptr<ResourceLoadInfoNotifierWrapper>
        resource_load_info_notifier_wrapper;

    // Set to true when the request was frozen. This is used not to record
    // histograms for frozen requests. Note: Even if the request was unfreezed,
    // we don't resume recording histograms because tasks are deferred in
    // MojoURLLoaderClient.
    bool ignore_for_histogram = false;
  };

  // Called as a callback for ResourceRequestClient::OnReceivedRedirect().
  void OnFollowRedirectCallback(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr response_head,
      std::vector<std::string> removed_headers,
      net::HttpRequestHeaders modified_headers);

  // Follows redirect, if any, for the given request.
  void FollowPendingRedirect(PendingRequestInfo* request_info);

  // Converts remote times in the response head to local times. Returns the
  // converted response start time.
  base::TimeTicks ToLocalURLResponseHead(
      const PendingRequestInfo& request_info,
      network::mojom::URLResponseHead& response_head) const;

  void DidReceiveCachedCode();

  bool ShouldDeferTask() const;

  void MaybeRunPendingTasks();

  // The instance is created on StartAsync() or StartSync(), and it's deleted
  // when the response has finished, or when the request is canceled.
  std::unique_ptr<PendingRequestInfo> request_info_;

  // Set to true when an operation integral to resource loading latency is
  // delayed waiting on a response from the code cache.
  bool latency_critical_operation_deferred_ = false;
  bool used_code_cache_fetcher_ = false;

  scoped_refptr<base::SequencedTaskRunner> loading_task_runner_;

  // `pending_tasks_` are queued while waiting for the response from the
  // IsolatedCode Cache Host. Ideally for the code health, we should not have
  // such deferring logic. However, it is difficult because the current code for
  // ScriptCachedMetadataHandler is written with the assumption that metadata
  // comes first.
  WTF::Vector<base::OnceClosure> pending_tasks_;

  scoped_refptr<CodeCacheFetcher> code_cache_fetcher_;

  base::WeakPtrFactory<ResourceRequestSender> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_RESOURCE_REQUEST_SENDER_H_
