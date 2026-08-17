// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_THROTTLING_URL_LOADER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_THROTTLING_URL_LOADER_H_

#include <memory>
#include <optional>
#include <string_view>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/time/time.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/bindings/unique_receiver_set.h"
#include "services/network/public/cpp/resource_request.h"
#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "services/network/public/mojom/accept_ch_frame_observer.mojom.h"
#include "services/network/public/mojom/url_loader.mojom.h"
#include "services/network/public/mojom/url_loader_factory.mojom-forward.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "services/network/public/mojom/web_client_hints_types.mojom.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/loader/url_loader_throttle.h"

namespace base {
class SequencedTaskRunner;
}

namespace blink {

// ThrottlingURLLoader is a wrapper around the
// network::mojom::URLLoader[Factory] interfaces. It applies a list of
// URLLoaderThrottle instances which could defer, resume restart or cancel the
// URL loading. If the Mojo connection fails during the request it is canceled
// with net::ERR_ABORTED.
class BLINK_COMMON_EXPORT ThrottlingURLLoader
    : public network::mojom::URLLoaderClient {
 public:
  // A delegate to override the handling actions to some of the URL loading
  // stages from the calls of `network::mojom::URLLoaderClient`, received by
  // `client_receiver_`. For the stages not provided by this delegate, e.g.
  // `OnTransferSizeUpdated()`, `OnReceiveEarlyHints()`, `OnUploadProgress()`,
  // they follow the default implementation within ThrottlingURLLoader.
  // See also
  // https://docs.google.com/document/d/1RKPgoLBrrLZBPn01XtwHJiLlH9rA7nIRXQJIR7BUqJA/edit#heading=h.y1og20bzkuf7
  class ClientReceiverDelegate {
   public:
    // Called at the end of `ThrottlingURLLoader::OnReceiveRedirect()`.
    // It allows the delegate to decide how to proceed with the redirect instead
    // of simply calling `forwarding_client_`.
    virtual void EndReceiveRedirect(
        const net::RedirectInfo& redirect_info,
        network::mojom::URLResponseHeadPtr response_head) = 0;
    // Called at the beginning of `ThrottlingURLLoader::OnReceiveResponse()`,
    // and overrides all of its behavior.
    // This method receive the same params as if they are coming directly from
    // network::mojom::URLLoaderClient.
    virtual void OnReceiveResponse(
        network::mojom::URLResponseHeadPtr response_head,
        mojo::ScopedDataPipeConsumerHandle body,
        std::optional<mojo_base::BigBuffer> cached_metadata) = 0;
    // Called at the beginning of `ThrottlingURLLoader::OnComplete()`, and
    // overrides all of its behavior.
    // This method receive the same params as if they are coming directly from
    // network::mojom::URLLoaderClient.
    virtual void OnComplete(
        const network::URLLoaderCompletionStatus& status) = 0;
    // Called when a loading stage is cancelled by throttles or due to mojo
    // disconnection. `status` is internally constructed by ThrottlingURLLoader
    // when the latter decides to terminate.
    virtual void CancelWithStatus(
        const network::URLLoaderCompletionStatus& status) = 0;
  };
  // Reason used when resetting the URLLoader to follow a redirect.
  static const char kFollowRedirectReason[];

  // |url_request| can be mutated by this function, and doesn't need to stay
  // alive after calling this function.
  //
  // |client| must stay alive during the lifetime of the returned object. Please
  // note that the request may not start immediately since it could be deferred
  // by throttles.
  //
  // |client_receiver_delegate| if provided, must stay alive during the lifetime
  // of the returned object. When set, the following behaviors of the returned
  // object will be overridden by the delegate:
  // - `OnReceiveRedirect()` will execute custom logic at its end, instead of
  //   calling `forwarding_client_`.
  // - `OnReceiveResponse()` and `OnComplete()` will be entirely replaced by the
  //   delegate's ones.
  // - `CancelWithExtendedError()` will execute custom logic at its end, instead
  //   of calling `forwarding_client_`.
  // In addition, |client_receiver_delegate| is not orthogonal to |client|, in
  // that the latter is still necessary for the returned object to run the
  // throttle-related logic.
  // Note that once |client_receiver_delegate| is set, the relevant throttle
  // callbacks like BeforeWillProcessResponse(), WillProcessResponse(), and
  // WillOnCompleteWithError(), will not be triggered by the returned object.
  static std::unique_ptr<ThrottlingURLLoader> CreateLoaderAndStart(
      scoped_refptr<network::SharedURLLoaderFactory> factory,
      std::vector<std::unique_ptr<URLLoaderThrottle>> throttles,
      int32_t request_id,
      uint32_t options,
      network::ResourceRequest* url_request,
      network::mojom::URLLoaderClient* client,
      const net::NetworkTrafficAnnotationTag& traffic_annotation,
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      std::optional<std::vector<std::string>> cors_exempt_header_list =
          std::nullopt,
      ClientReceiverDelegate* client_receiver_delegate = nullptr);

  ThrottlingURLLoader(const ThrottlingURLLoader&) = delete;
  ThrottlingURLLoader& operator=(const ThrottlingURLLoader&) = delete;
  ~ThrottlingURLLoader() override;

  // Follows a redirect, calling CreateLoaderAndStart() on the factory. This
  // is useful if the factory uses different loaders for different URLs.
  void FollowRedirectForcingRestart();
  // This should be called if the loader will be recreated to follow a redirect
  // instead of calling FollowRedirect(). This can be used if a loader is
  // implementing similar logic to FollowRedirectForcingRestart(). If this is
  // called, a future request for the redirect should be guaranteed to be sent
  // with the same request_id.
  // `removed_headers`, `modified_headers` and `modified_cors_exempt_headers`
  // will be merged to corresponding members in the ThrottlingURLLoader, and
  // then apply updates against `resource_request`.
  void ResetForFollowRedirect(
      network::ResourceRequest& resource_request,
      const std::vector<std::string>& removed_headers,
      const net::HttpRequestHeaders& modified_headers,
      const net::HttpRequestHeaders& modified_cors_exempt_headers);

  void FollowRedirect(
      const std::vector<std::string>& removed_headers,
      const net::HttpRequestHeaders& modified_headers,
      const net::HttpRequestHeaders& modified_cors_exempt_headers);
  void SetPriority(net::RequestPriority priority, int32_t intra_priority_value);

  // Disconnect the forwarding URLLoaderClient and the URLLoader. Returns the
  // datapipe endpoints.
  network::mojom::URLLoaderClientEndpointsPtr Unbind();

  void CancelWithError(int error_code, std::string_view custom_reason);

  void CancelWithExtendedError(int error_code,
                               int extended_reason_code,
                               std::string_view custom_reason);

  bool response_intercepted() const { return response_intercepted_; }

  // Indicates a restart did occur due to a Critical-CH HTTP Header.
  void DidRestartForCriticalClientHint() {
    critical_ch_restart_time_ = base::TimeTicks::Now();
  }

 private:
  class ForwardingThrottleDelegate;

  ThrottlingURLLoader(
      std::vector<std::unique_ptr<URLLoaderThrottle>> throttles,
      network::mojom::URLLoaderClient* client,
      const net::NetworkTrafficAnnotationTag& traffic_annotation,
      ClientReceiverDelegate* client_receiver_delegate);

  void Start(scoped_refptr<network::SharedURLLoaderFactory> factory,
             int32_t request_id,
             uint32_t options,
             network::ResourceRequest* url_request,
             scoped_refptr<base::SequencedTaskRunner> task_runner,
             std::optional<std::vector<std::string>> cors_exempt_header_list);

  void StartNow();
  void RestartWithURLResetNow();

  // Processes the result of a URLLoaderThrottle call. If it's deferred, adds
  // the throttle to the blocking set and updates |*should_defer| accordingly.
  // Returns |true| if the request should continue to be processed (regardless
  // of whether it's been deferred) or |false| if it's been cancelled.
  bool HandleThrottleResult(URLLoaderThrottle* throttle,
                            bool throttle_deferred = false,
                            bool* should_defer = nullptr);

  // Stops a given throttle from deferring the request. If this was not the last
  // deferring throttle, the request remains deferred. Otherwise it resumes
  // progress.
  void StopDeferringForThrottle(URLLoaderThrottle* throttle);

  // network::mojom::URLLoaderClient implementation:
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

  void OnClientConnectionError();

  void Resume();
  void SetPriority(net::RequestPriority priority);
  void UpdateRequestHeaders(network::ResourceRequest& resource_request);
  void UpdateDeferredResponseHead(
      network::mojom::URLResponseHeadPtr new_response_head,
      mojo::ScopedDataPipeConsumerHandle body);
  void InterceptResponse(
      mojo::PendingRemote<network::mojom::URLLoader> new_loader,
      mojo::PendingReceiver<network::mojom::URLLoaderClient>
          new_client_receiver,
      mojo::PendingRemote<network::mojom::URLLoader>* original_loader,
      mojo::PendingReceiver<network::mojom::URLLoaderClient>*
          original_client_receiver,
      mojo::ScopedDataPipeConsumerHandle* body);

  // Disconnects the client connection and releases the URLLoader.
  void DisconnectClient(std::string_view custom_description);

  enum DeferredStage {
    DEFERRED_NONE,
    DEFERRED_START,
    DEFERRED_REDIRECT,
    DEFERRED_RESPONSE,
  };
  const char* GetStageNameForHistogram(DeferredStage stage);

  DeferredStage deferred_stage_ = DEFERRED_NONE;
  bool loader_completed_ = false;
  bool did_receive_response_ = false;

  struct ThrottleEntry {
    ThrottleEntry(ThrottlingURLLoader* loader,
                  std::unique_ptr<URLLoaderThrottle> the_throttle);
    ThrottleEntry(ThrottleEntry&& other);
    ThrottleEntry& operator=(ThrottleEntry&& other);
    ~ThrottleEntry();

    std::unique_ptr<URLLoaderThrottle> throttle;
    std::unique_ptr<ForwardingThrottleDelegate> delegate;
  };

  std::vector<ThrottleEntry> throttles_;
  std::map<URLLoaderThrottle*, /*start=*/base::Time> deferring_throttles_;

  // NOTE: This may point to a native implementation (instead of a Mojo proxy
  // object). And it is possible that the implementation of |forwarding_client_|
  // destroys this object synchronously when this object is calling into it.
  const raw_ptr<network::mojom::URLLoaderClient, DanglingUntriaged>
      forwarding_client_;
  mojo::Remote<network::mojom::URLLoader> url_loader_;

  mojo::Receiver<network::mojom::URLLoaderClient> client_receiver_{this};

  // A delegate to override some of the message handling for `client_receiver_`.
  // If provided, its lifetime must be >= `this`.
  raw_ptr<ClientReceiverDelegate> client_receiver_delegate_;

  struct StartInfo {
    StartInfo(
        scoped_refptr<network::SharedURLLoaderFactory> in_url_loader_factory,
        int32_t in_request_id,
        uint32_t in_options,
        network::ResourceRequest* in_url_request,
        scoped_refptr<base::SequencedTaskRunner> in_task_runner,
        std::optional<std::vector<std::string>> in_cors_exempt_header_list);
    ~StartInfo();

    const scoped_refptr<network::SharedURLLoaderFactory> url_loader_factory;
    const int32_t request_id;
    const uint32_t options;

    network::ResourceRequest url_request;
    // |task_runner| is used to set up |client_receiver_|.
    const scoped_refptr<base::SequencedTaskRunner> task_runner;
    const std::optional<std::vector<std::string>> cors_exempt_header_list;
  };
  // Holds any info needed to start or restart the request. Used when start is
  // deferred or when FollowRedirectForcingRestart() is called.
  std::unique_ptr<StartInfo> start_info_;

  struct ResponseInfo {
    explicit ResponseInfo(network::mojom::URLResponseHeadPtr in_response_head);
    ~ResponseInfo();

    network::mojom::URLResponseHeadPtr response_head;
  };
  // Set if response is deferred.
  std::unique_ptr<ResponseInfo> response_info_;
  mojo::ScopedDataPipeConsumerHandle body_;
  std::optional<mojo_base::BigBuffer> cached_metadata_;

  struct RedirectInfo {
    RedirectInfo(const net::RedirectInfo& in_redirect_info,
                 network::mojom::URLResponseHeadPtr in_response_head);
    ~RedirectInfo();

    net::RedirectInfo redirect_info;
    network::mojom::URLResponseHeadPtr response_head;
  };
  // Set if redirect is deferred.
  std::unique_ptr<RedirectInfo> redirect_info_;

  struct PriorityInfo {
    PriorityInfo(net::RequestPriority in_priority,
                 int32_t in_intra_priority_value);

    net::RequestPriority priority;
    int32_t intra_priority_value;
  };
  // Set if request is deferred and SetPriority() is called.
  std::unique_ptr<PriorityInfo> priority_info_;

  // If
  // - A throttle changed the URL in WillStartRequest(), or
  // - `RestartWithURLReset` is set to true in `BeforeWillProcessResponse()` or
  //   `BeforeWillRedirectRequest()`,
  // a synthesized redirect to the modified URL is dispatched.
  // 1. `throttle_will_start_redirect_url_` is set when the synthesized redirect
  //    is scheduled.
  // 2. The actual redirect is started in the first half of `StartNow()`.
  // 3. `throttle_will_start_redirect_url_` is reset when the redirect is done.
  GURL throttle_will_start_redirect_url_;

  // Set if a throttle changed the URL in WillRedirectRequest.
  // Only supported with the network service.
  GURL throttle_will_redirect_redirect_url_;

  // The first URL seen by the throttle.
  GURL original_url_;

  const net::NetworkTrafficAnnotationTag traffic_annotation_;

  uint32_t inside_delegate_calls_ = 0;

  // The latest request URL from where we expect a response
  GURL response_url_;

  bool response_intercepted_ = false;

  std::vector<std::string> removed_headers_;
  net::HttpRequestHeaders modified_headers_;
  net::HttpRequestHeaders modified_cors_exempt_headers_;

  base::TimeTicks critical_ch_restart_time_;

  base::WeakPtrFactory<ThrottlingURLLoader> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_THROTTLING_URL_LOADER_H_
