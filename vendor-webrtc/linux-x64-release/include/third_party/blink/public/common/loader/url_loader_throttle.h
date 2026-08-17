// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_URL_LOADER_THROTTLE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_URL_LOADER_THROTTLE_H_

#include <string>
#include <string_view>
#include <vector>

#include "base/memory/raw_ptr.h"
#include "base/types/strong_alias.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "net/base/request_priority.h"
#include "services/network/public/mojom/url_loader.mojom-forward.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "services/network/public/mojom/web_client_hints_types.mojom.h"
#include "third_party/blink/public/common/common_export.h"

class GURL;

namespace net {
class HttpRequestHeaders;
struct RedirectInfo;
}

namespace network {
struct ResourceRequest;
struct URLLoaderCompletionStatus;
}  // namespace network

namespace blink {

// A URLLoaderThrottle gets notified at various points during the process of
// loading a resource. At each stage, it has the opportunity to defer the
// resource load.
//
// Note that while a single throttle deferring a load at any given step will
// block the load from progressing further until a subsequent Delegate::Resume()
// call is made, it does NOT prevent subsequent throttles from processing the
// same step of the request if multiple throttles are affecting the load.
class BLINK_COMMON_EXPORT URLLoaderThrottle {
 public:
  // An interface for the throttle implementation to resume (when deferred) or
  // cancel the resource load. Please note that these methods could be called
  // in-band (i.e., inside URLLoaderThrottle notification methods such as
  // WillStartRequest), or out-of-band.
  //
  // It is guaranteed that throttles calling these methods won't be destroyed
  // synchronously.
  class BLINK_COMMON_EXPORT Delegate {
   public:
    // Cancels the resource load with the specified error code and an optional,
    // application-defined reason description.
    virtual void CancelWithError(int error_code,
                                 std::string_view custom_reason = "") = 0;

    // Cancels the resource load with the specified error code and an optional,
    // application-defined reason description with optional extended_reason().
    virtual void CancelWithExtendedError(int error_code,
                                         int extended_reason_code,
                                         std::string_view custom_reason = "") {}

    // Resumes the deferred resource load. It is a no-op if the resource load is
    // not deferred or has already been canceled.
    virtual void Resume() = 0;

    // Updates the response head which is deferred to be sent. This method needs
    // to be called when the response is deferred on
    // URLLoaderThrottle::WillProcessResponse() and before calling
    // Delegate::Resume().
    virtual void UpdateDeferredResponseHead(
        network::mojom::URLResponseHeadPtr new_response_head,
        mojo::ScopedDataPipeConsumerHandle body);

    // Replaces the URLLoader and URLLoaderClient endpoints held by the
    // ThrottlingURLLoader instance.
    virtual void InterceptResponse(
        mojo::PendingRemote<network::mojom::URLLoader> new_loader,
        mojo::PendingReceiver<network::mojom::URLLoaderClient>
            new_client_receiver,
        mojo::PendingRemote<network::mojom::URLLoader>* original_loader,
        mojo::PendingReceiver<network::mojom::URLLoaderClient>*
            original_client_receiver,
        mojo::ScopedDataPipeConsumerHandle* body);

    // Indicates a restart did occur due to a Critical-CH HTTP Header.
    virtual void DidRestartForCriticalClientHint() {}

   protected:
    virtual ~Delegate();
  };

  virtual ~URLLoaderThrottle();

  // Detaches this object from the current sequence in preparation for a move to
  // a different sequence. If this method is called it must be before any of the
  // Will* methods below and may only be called once.
  virtual void DetachFromCurrentSequence();

  // Called exactly once before the resource request is started.
  //
  // |request| needs to be modified before the callback returns (i.e.
  // asynchronously touching the pointer in defer case is not valid)
  // When |request->url| is modified it will make an internal redirect, which
  // might have some side-effects: drop upload streams data might be dropped,
  // redirect count may be reached.
  //
  // Implementations should be aware that throttling can happen multiple times
  // for the same |request|, even after one instance of the same throttle
  // subclass already modified the request. This happens, e.g., when a service
  // worker initially elects to handle a request but then later falls back to
  // network, so new throttles are created for another URLLoaderFactory to
  // handle the request.
  virtual void WillStartRequest(network::ResourceRequest* request, bool* defer);

  // If non-null is returned a histogram will be logged using this name when the
  // throttle defers the navigation in WillStartRequest().
  virtual const char* NameForLoggingWillStartRequest();

  // Called when the request was redirected.  |redirect_info| contains the
  // redirect responses's HTTP status code and some information about the new
  // request that will be sent if the redirect is followed, including the new
  // URL and new method.
  //
  // Request headers added to |to_be_removed_request_headers| will be removed
  // before the redirect is followed. Headers added to
  // |modified_request_headers| and |modified_cors_exempt_request_headers| will
  // be merged into the existing request headers and cors_exempt_headers before
  // the redirect is followed.
  //
  // If |redirect_info->new_url| is modified by a throttle, the request will be
  // redirected to the new URL. Only one throttle can update this and it must
  // have the same origin as |redirect_info->new_url|. See the comments for
  // WillStartRequest on possible side-effects.
  virtual void WillRedirectRequest(
      net::RedirectInfo* redirect_info,
      const network::mojom::URLResponseHead& response_head,
      bool* defer,
      std::vector<std::string>* to_be_removed_request_headers,
      net::HttpRequestHeaders* modified_request_headers,
      net::HttpRequestHeaders* modified_cors_exempt_request_headers);

  // Called when the response headers and meta data are available.
  // TODO(776312): Migrate this URL to URLResponseHead.
  virtual void WillProcessResponse(
      const GURL& response_url,
      network::mojom::URLResponseHead* response_head,
      bool* defer);

  // If non-null is returned a histogram will be logged using this name when the
  // throttle defers the navigation in WillProcessResponse().
  virtual const char* NameForLoggingWillProcessResponse();

  // When `*restart_with_url_reset` is set to true in
  // `BeforeWillProcessResponse` or `BeforeWillRedirectRequest`, the caller
  // should restart the URL loader using the original URL before modified by
  // WillStartRequest(). When a URL loader is restarted, throttles will NOT have
  // their WillStartRequest() method called again - that is only called for the
  // initial request start.
  using RestartWithURLReset =
      base::StrongAlias<struct RestartWithURLResetTag, bool>;

  // Called prior WillProcessResponse() to allow throttles to restart the URL
  // load by setting `RestartWithURLReset` to true.
  //
  // Having this method separate from WillProcessResponse() ensures that
  // WillProcessResponse() is called at most once even in the presence of
  // restarts.
  virtual void BeforeWillProcessResponse(
      const GURL& response_url,
      const network::mojom::URLResponseHead& response_head,
      RestartWithURLReset* restart_with_url_reset);

  // Called prior WillRedirectRequest() to allow throttles to restart the URL
  // load by setting `RestartWithURLReset` to true.
  //
  // Having this method separate from WillRedirectRequest() ensures that
  // WillRedirectRequest() is called at most once per redirect even in the
  // presence of restarts.
  //
  // Note: restarting with the url reset triggers an internal redirect, which
  // will cause this to be run again. Ensure that this doesn't cause loops.
  virtual void BeforeWillRedirectRequest(
      net::RedirectInfo* redirect_info,
      const network::mojom::URLResponseHead& response_head,
      RestartWithURLReset* restart_with_url_reset,
      std::vector<std::string>* to_be_removed_request_headers,
      net::HttpRequestHeaders* modified_request_headers,
      net::HttpRequestHeaders* modified_cors_exempt_request_headers);

  // Called if there is a non-OK net::Error in the completion status.
  virtual void WillOnCompleteWithError(
      const network::URLLoaderCompletionStatus& status);

  void set_delegate(Delegate* delegate) { delegate_ = delegate; }

 protected:
  URLLoaderThrottle();

  raw_ptr<Delegate, DanglingUntriaged> delegate_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_URL_LOADER_THROTTLE_H_
