/*
 * Copyright (C) 2005, 2006, 2011 Apple Inc. All rights reserved.
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
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOADER_H_

#include <memory>
#include <variant>

#include "base/containers/span.h"
#include "base/feature_list.h"
#include "base/gtest_prod_util.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "services/network/public/mojom/fetch_api.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/blob/blob_registry.mojom-blink.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/loader/fetch/data_pipe_bytes_consumer.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_context.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_scheduler.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/loader/fetch/response_body_loader_client.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/url_loader.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/url_loader_client.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"

namespace base {
class UnguessableToken;
}

namespace blink {

class ResourceError;
class ResourceFetcher;
class ResponseBodyLoader;

// Struct for keeping variables used in testing CNAME alias info bundled
// together.
struct CnameAliasInfoForTesting {
  bool has_aliases = false;
  bool was_ad_tagged_based_on_alias = false;
  bool was_blocked_based_on_alias = false;
  int list_length = 0;
  int invalid_count = 0;
  int redundant_count = 0;
};

// A ResourceLoader is created for each Resource by the ResourceFetcher when it
// needs to load the specified resource. A ResourceLoader creates a
// URLLoader and loads the resource using it. Any per-load logic should be
// implemented in this class basically.
class PLATFORM_EXPORT ResourceLoader final
    : public GarbageCollected<ResourceLoader>,
      public ResourceLoadSchedulerClient,
      protected URLLoaderClient,
      public mojom::blink::ProgressClient,
      private ResponseBodyLoaderClient {
  USING_PRE_FINALIZER(ResourceLoader, Dispose);

 public:
  // Assumes ResourceFetcher and Resource are non-null.
  ResourceLoader(ResourceFetcher*,
                 ResourceLoadScheduler*,
                 Resource*,
                 ContextLifecycleNotifier*,
                 ResourceRequestBody request_body = ResourceRequestBody(),
                 uint32_t inflight_keepalive_bytes = 0);
  ~ResourceLoader() override;
  void Trace(Visitor*) const override;

  void Start();

  void ScheduleCancel();
  void Cancel();

  void SetDefersLoading(LoaderFreezeMode);

  void DidChangePriority(ResourceLoadPriority, int intra_priority_value);

  bool IsCacheAwareLoadingActivated() const {
    return is_cache_aware_loading_activated_;
  }

  ResourceFetcher* Fetcher() { return fetcher_.Get(); }
  bool ShouldBeKeptAliveWhenDetached() const;

  void AbortResponseBodyLoading();

  // URLLoaderClient
  //
  // A succesful load will consist of:
  // 0+  WillFollowRedirect()
  // 0+  DidSendData()
  // 1   DidReceiveResponse()
  // 0+  DidReceiveTransferSizeUpdate()
  // 1   DidFinishLoading()
  // A failed load is indicated by 1 DidFail(), which can occur at any time
  // before DidFinishLoading(), including synchronous inside one of the other
  // callbacks via ResourceLoader::cancel()
  bool WillFollowRedirect(const WebURL& new_url,
                          const net::SiteForCookies& new_site_for_cookies,
                          const WebString& new_referrer,
                          network::mojom::ReferrerPolicy new_referrer_policy,
                          const WebString& new_method,
                          const WebURLResponse& passed_redirect_response,
                          bool& has_devtools_request_id,
                          std::vector<std::string>* removed_headers,
                          net::HttpRequestHeaders& modified_headers,
                          bool insecure_scheme_was_upgraded) override;
  void DidSendData(uint64_t bytes_sent,
                   uint64_t total_bytes_to_be_sent) override;
  void DidReceiveResponse(
      const WebURLResponse&,
      std::variant<mojo::ScopedDataPipeConsumerHandle, SegmentedBuffer>,
      std::optional<mojo_base::BigBuffer> cached_metadata) override;
  void DidReceiveDataForTesting(base::span<const char> data) override;
  void DidReceiveTransferSizeUpdate(int transfer_size_diff) override;
  void DidFinishLoading(base::TimeTicks response_end_time,
                        int64_t encoded_data_length,
                        uint64_t encoded_body_length,
                        int64_t decoded_body_length) override;
  void DidFail(const WebURLError&,
               base::TimeTicks response_end_time,
               int64_t encoded_data_length,
               uint64_t encoded_body_length,
               int64_t decoded_body_length) override;
  void CountFeature(blink::mojom::WebFeature) override;

  void HandleError(const ResourceError&);

  void DidFinishLoadingFirstPartInMultipart();

  scoped_refptr<base::SingleThreadTaskRunner> GetLoadingTaskRunner();

  void CancelIfWebBundleTokenMatches(
      const base::UnguessableToken& web_bundle_token);

  const FeatureContext* GetFeatureContext() const {
    return Context().GetFeatureContext();
  }

 private:
  friend class SubresourceIntegrityTest;
  friend class ResourceLoaderIsolatedCodeCacheTest;
  friend class ResourceLoaderSubresourceFilterCnameAliasTest;

  // ResourceLoadSchedulerClient.
  void Run() override;

  // ResponseBodyLoaderClient implementation.
  void DidReceiveData(base::span<const char> data) override;
  void DidReceiveDecodedData(
      const String& data,
      std::unique_ptr<ParkableStringImpl::SecureDigest> digest) override;
  void DidFinishLoadingBody() override;
  void DidFailLoadingBody() override;
  void DidCancelLoadingBody() override;

  void DidReceiveDataImpl(
      std::variant<SegmentedBuffer, base::span<const char>> data);

  bool ShouldFetchCodeCache();
  void StartFetch();

  void Release(ResourceLoadScheduler::ReleaseOption,
               const ResourceLoadScheduler::TrafficReportHints&);

  // This method is currently only used for cache-aware loading, other users
  // should be careful not to break ResourceLoader state.
  void Restart();

  FetchContext& Context() const;

  // Returns true during resource load is happening. Methods as
  // a URLLoaderClient should not be invoked if this returns false.
  bool IsLoading() const;

  void CancelForRedirectAccessCheckError(const KURL&,
                                         ResourceRequestBlockedReason);
  void RequestSynchronously();
  void RequestAsynchronously();
  void Dispose();

  void DidReceiveResponseInternal(
      const ResourceResponse&,
      std::optional<mojo_base::BigBuffer> cached_metadata);

  void DidStartLoadingResponseBodyInternal(BytesConsumer& bytes_consumer);

  void CancelTimerFired(TimerBase*);

  void OnProgress(uint64_t delta) override;
  void FinishedCreatingBlob(const scoped_refptr<BlobDataHandle>&);

  std::optional<ResourceRequestBlockedReason> CheckResponseNosniff(
      mojom::blink::RequestContextType,
      const ResourceResponse&);

  // Processes Data URL in ResourceLoader instead of using |loader_|.
  void HandleDataUrl();

  // If enabled, performs SubresourceFilter checks for any DNS aliases found for
  // the requested URL, which may result in ad-tagging the ResourceRequest.
  // Returns true if the request should be blocked based on these checks.
  bool ShouldBlockRequestBasedOnSubresourceFilterDnsAliasCheck(
      const Vector<String>& dns_aliases,
      const KURL& request_url,
      const KURL& original_url,
      ResourceType resource_type,
      const ResourceRequestHead& initial_request,
      const ResourceLoaderOptions& options,
      const ResourceRequest::RedirectInfo redirect_info);

  // Increments the right UseCounter for the given PNA preflight result, if any.
  void CountPrivateNetworkAccessPreflightResult(
      network::mojom::PrivateNetworkAccessPreflightResult result);

  // The request object which will be passed to URLLoader. This is not used when
  // the request URL is a data URL.
  std::unique_ptr<network::ResourceRequest> network_resource_request_;

  // Used only for non-data URL request.
  std::unique_ptr<URLLoader> loader_;

  ResourceLoadScheduler::ClientId scheduler_client_id_;
  Member<ResourceFetcher> fetcher_;
  Member<ResourceLoadScheduler> scheduler_;
  Member<Resource> resource_;
  ResourceRequestBody request_body_;
  Member<ResponseBodyLoader> response_body_loader_;
  Member<DataPipeBytesConsumer::CompletionNotifier>
      data_pipe_completion_notifier_;

  // https://fetch.spec.whatwg.org/#concept-request-response-tainting
  network::mojom::FetchResponseType response_tainting_ =
      network::mojom::FetchResponseType::kBasic;
  uint32_t inflight_keepalive_bytes_;
  bool is_cache_aware_loading_activated_;

  blink::HeapMojoAssociatedReceiver<mojom::blink::ProgressClient,
                                    blink::ResourceLoader>
      progress_receiver_;
  bool blob_finished_ = false;
  bool blob_response_started_ = false;
  bool has_seen_end_of_body_ = false;
  // If DidFinishLoading is called while downloading to a blob before the blob
  // is finished, we might have to defer actually handling the event. This
  // struct is used to store the information needed to refire DidFinishLoading
  // when the blob is finished too.
  struct DeferredFinishLoadingInfo {
    base::TimeTicks response_end_time;
  };
  std::optional<DeferredFinishLoadingInfo> deferred_finish_loading_info_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_for_body_loader_;

  LoaderFreezeMode freeze_mode_ = LoaderFreezeMode::kNone;
  // True if the next call of SetDefersLoading(kNotDeferred) needs to invoke
  // HandleDataURL().
  bool defers_handling_data_url_ = false;

  HeapTaskRunnerTimer<ResourceLoader> cancel_timer_;

  FrameScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;

  base::TimeTicks response_end_time_for_error_cases_;

  int64_t received_body_length_from_service_worker_ = 0;
  CnameAliasInfoForTesting cname_alias_info_for_testing_;
  bool finished_ = false;

  // This is used to keep the body handle of 304 Not Modified response until
  // Blink receives the URLLoaderClient's OnComplete IPC.
  mojo::ScopedDataPipeConsumerHandle empty_body_handle_for_revalidation_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOADER_H_
