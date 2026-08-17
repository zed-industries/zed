/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_URL_REQUEST_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_URL_REQUEST_H_

#include <memory>
#include <optional>

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "net/base/request_priority.h"
#include "third_party/blink/public/platform/web_common.h"
#include "ui/base/page_transition_types.h"

// TODO(crbug.com/922875): Need foo.mojom.shared-forward.h.
namespace network {
namespace mojom {
enum class CorsPreflightPolicy : int32_t;
enum class CredentialsMode : int32_t;
enum class RedirectMode : int32_t;
enum class ReferrerPolicy : int32_t;
enum class RequestMode : int32_t;
enum class RequestDestination : int32_t;
}  // namespace mojom

class OptionalTrustTokenParams;
}  // namespace network

namespace net {
class SiteForCookies;
}  // namespace net

namespace blink {

namespace mojom {
enum class FetchCacheMode : int32_t;
enum class RequestContextType : int32_t;
enum class RequestContextFrameType : int32_t;
}  // namespace mojom

class ResourceRequest;
class WebHTTPBody;
class WebHTTPHeaderVisitor;
class WebURLRequestExtraData;
class WebSecurityOrigin;
class WebString;
class WebURL;

class BLINK_PLATFORM_EXPORT WebURLRequest {
 public:
  // The enum values should remain synchronized with the enum
  // WebURLRequestPriority in tools/metrics/histograms.enums.xml.
  enum class Priority {
    kUnresolved = -1,
    kVeryLow,
    kLow,
    kMedium,
    kHigh,
    kVeryHigh,
    kLowest = kVeryLow,
    kHighest = kVeryHigh,
  };
  static net::RequestPriority ConvertToNetPriority(WebURLRequest::Priority);

  ~WebURLRequest();
  WebURLRequest();
  WebURLRequest(const WebURLRequest&) = delete;
  WebURLRequest(WebURLRequest&&);
  explicit WebURLRequest(const WebURL&);
  WebURLRequest& operator=(const WebURLRequest&) = delete;
  WebURLRequest& operator=(WebURLRequest&&);
  void CopyFrom(const WebURLRequest&);

  bool IsNull() const;

  WebURL Url() const;
  void SetUrl(const WebURL&);

  // Used to implement third-party cookie blocking.
  const net::SiteForCookies& SiteForCookies() const;
  void SetSiteForCookies(const net::SiteForCookies&);

  std::optional<WebSecurityOrigin> TopFrameOrigin() const;
  void SetTopFrameOrigin(const WebSecurityOrigin&);

  // https://fetch.spec.whatwg.org/#concept-request-origin
  WebSecurityOrigin RequestorOrigin() const;
  void SetRequestorOrigin(const WebSecurityOrigin&);

  // The origin of the isolated world - set if this is a fetch/XHR initiated by
  // an isolated world.
  WebSecurityOrigin IsolatedWorldOrigin() const;

  mojom::FetchCacheMode GetCacheMode() const;
  void SetCacheMode(mojom::FetchCacheMode);

  base::TimeDelta TimeoutInterval() const;

  WebString HttpMethod() const;
  void SetHttpMethod(const WebString&);

  WebString HttpContentType() const;

  bool IsFormSubmission() const;

  WebString HttpHeaderField(const WebString& name) const;
  // It's not possible to set the referrer header using this method. Use
  // SetReferrerString instead.
  void SetHttpHeaderField(const WebString& name, const WebString& value);
  void AddHttpHeaderField(const WebString& name, const WebString& value);
  void ClearHttpHeaderField(const WebString& name);
  void VisitHttpHeaderFields(WebHTTPHeaderVisitor*) const;

  WebHTTPBody HttpBody() const;
  void SetHttpBody(const WebHTTPBody&);

  WebHTTPBody AttachedCredential() const;
  void SetAttachedCredential(const WebHTTPBody&);

  // Controls whether upload progress events are generated when a request
  // has a body.
  bool ReportUploadProgress() const;
  void SetReportUploadProgress(bool);

  mojom::RequestContextType GetRequestContext() const;
  void SetRequestContext(mojom::RequestContextType);

  network::mojom::RequestDestination GetRequestDestination() const;
  void SetRequestDestination(network::mojom::RequestDestination);

  void SetReferrerString(const WebString& referrer);
  void SetReferrerPolicy(network::mojom::ReferrerPolicy referrer_policy);

  WebString ReferrerString() const;
  network::mojom::ReferrerPolicy GetReferrerPolicy() const;

  // Sets an HTTP origin header if it is empty and the HTTP method of the
  // request requires it.
  void SetHttpOriginIfNeeded(const WebSecurityOrigin&);

  // True if the request was user initiated.
  bool HasUserGesture() const;
  void SetHasUserGesture(bool);

  bool HasTextFragmentToken() const;

  // A consumer controlled value intended to be used to identify the
  // requestor.
  int RequestorID() const;
  void SetRequestorID(int);

  // True if the requestor wants to receive the response body as a stream.
  bool UseStreamOnResponse() const;
  void SetUseStreamOnResponse(bool);

  // True if the request can work after the fetch group is terminated.
  bool GetKeepalive() const;
  void SetKeepalive(bool);

  // True if the service workers should not get events for the request.
  bool GetSkipServiceWorker() const;
  void SetSkipServiceWorker(bool);

  // True if corresponding AppCache group should be resetted.
  bool ShouldResetAppCache() const;
  void SetShouldResetAppCache(bool);

  // The request mode which will be passed to the ServiceWorker.
  network::mojom::RequestMode GetMode() const;
  void SetMode(network::mojom::RequestMode);

  // True if the request is for a favicon.
  bool GetFavicon() const;
  void SetFavicon(bool);

  // The credentials mode which will be passed to the ServiceWorker.
  network::mojom::CredentialsMode GetCredentialsMode() const;
  void SetCredentialsMode(network::mojom::CredentialsMode);

  // The redirect mode which is used in Fetch API.
  network::mojom::RedirectMode GetRedirectMode() const;
  void SetRedirectMode(network::mojom::RedirectMode);

  // Extra data associated with the underlying resource request. Resource
  // requests can be copied. If non-null, each copy of a resource requests
  // holds a pointer to the extra data, and the extra data pointer will be
  // deleted when the last resource request is destroyed. Setting the extra
  // data pointer will cause the underlying resource request to be
  // dissociated from any existing non-null extra data pointer.
  const scoped_refptr<WebURLRequestExtraData>& GetURLRequestExtraData() const;
  void SetURLRequestExtraData(scoped_refptr<WebURLRequestExtraData>);

  // The request is downloaded to the network cache, but not rendered or
  // executed.
  bool IsDownloadToNetworkCacheOnly() const;
  void SetDownloadToNetworkCacheOnly(bool);

  Priority GetPriority() const;
  void SetPriority(Priority);

  network::mojom::CorsPreflightPolicy GetCorsPreflightPolicy() const;

  // If this request was created from an anchor with a download attribute, this
  // is the value provided there.
  std::optional<WebString> GetSuggestedFilename() const;

  // Returns true if this request is tagged as an ad. This is done using various
  // heuristics so it is not expected to be 100% accurate.
  bool IsAdResource() const;

  // Should be set to true if this request (including redirects) should be
  // upgraded to HTTPS due to an Upgrade-Insecure-Requests requirement.
  void SetUpgradeIfInsecure(bool);

  // Returns true if request (including redirects) should be upgraded to HTTPS
  // due to an Upgrade-Insecure-Requests requirement.
  bool UpgradeIfInsecure() const;

  bool SupportsAsyncRevalidation() const;

  // Returns true when the request is for revalidation.
  bool IsRevalidating() const;

  // Returns the DevTools ID to throttle the network request.
  const std::optional<base::UnguessableToken>& GetDevToolsToken() const;

  // Remembers 'X-Requested-With' header value. Blink should not set this header
  // value until CORS checks are done to avoid running checks even against
  // headers that are internally set.
  const WebString GetRequestedWithHeader() const;
  void SetRequestedWithHeader(const WebString&);

  // Remembers 'Purpose' header value. Blink should not set this header value
  // until CORS checks are done to avoid running checks even against headers
  // that are internally set.
  const WebString GetPurposeHeader() const;

  // https://fetch.spec.whatwg.org/#concept-request-window
  // See network::ResourceRequest::fetch_window_id for details.
  const base::UnguessableToken& GetFetchWindowId() const;
  void SetFetchWindowId(const base::UnguessableToken&);

  std::optional<WebString> GetDevToolsId() const;

  int GetLoadFlagsForWebUrlRequest() const;

  bool IsFromOriginDirtyStyleSheet() const;

  std::optional<base::UnguessableToken> RecursivePrefetchToken() const;

  // Specifies a Trust Tokens protocol operation to execute alongside the
  // request's load (https://github.com/wicg/trust-token-api).
  network::OptionalTrustTokenParams TrustTokenParams() const;

  std::optional<WebURL> WebBundleUrl() const;
  std::optional<base::UnguessableToken> WebBundleToken() const;

#if INSIDE_BLINK
  ResourceRequest& ToMutableResourceRequest();
  const ResourceRequest& ToResourceRequest() const;

 protected:
  // Permit subclasses to set arbitrary ResourceRequest pointer as
  // |resource_request_|. |owned_resource_request_| is not set in this case.
  explicit WebURLRequest(ResourceRequest&);
#endif

 private:
  // If this instance owns a ResourceRequest then |owned_resource_request_|
  // is non-null and |resource_request_| points to the ResourceRequest
  // instance it contains.
  std::unique_ptr<ResourceRequest> owned_resource_request_;

  // Should never be null.
  raw_ptr<ResourceRequest, DanglingUntriaged> resource_request_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_URL_REQUEST_H_
