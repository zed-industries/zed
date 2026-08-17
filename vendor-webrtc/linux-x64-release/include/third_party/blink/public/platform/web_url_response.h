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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_URL_RESPONSE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_URL_RESPONSE_H_

#include <memory>
#include <optional>
#include <vector>

#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "net/base/auth.h"
#include "net/base/ip_endpoint.h"
#include "net/cert/ct_policy_status.h"
#include "net/http/alternate_protocol_usage.h"
#include "net/http/http_connection_info.h"
#include "third_party/blink/public/common/security/security_style.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"

namespace network {
namespace mojom {
enum class AlternateProtocolUsage;
enum class DeviceBoundSessionUsage;
enum class FetchResponseSource;
enum class FetchResponseType : int32_t;
enum class IPAddressSpace : int32_t;
enum class PrivateNetworkAccessPreflightResult;
class URLResponseHead;
class LoadTimingInfo;
class ServiceWorkerRouterInfo;
}  // namespace mojom
}  // namespace network

namespace net {
class SSLInfo;
}

namespace blink {

class ResourceResponse;
class WebHTTPHeaderVisitor;
class WebURL;

class BLINK_PLATFORM_EXPORT WebURLResponse {
 public:
  enum HTTPVersion {
    kHTTPVersionUnknown,
    kHTTPVersion_0_9,
    kHTTPVersion_1_0,
    kHTTPVersion_1_1,
    kHTTPVersion_2_0
  };

  static WebURLResponse Create(const WebURL& url,
                               const network::mojom::URLResponseHead& head,
                               bool report_security_info,
                               int request_id);

  ~WebURLResponse();

  WebURLResponse();
  WebURLResponse(const WebURLResponse&);
  explicit WebURLResponse(const WebURL& current_request_url);
  WebURLResponse& operator=(const WebURLResponse&);

  bool IsNull() const;

  // The current request URL for this resource (the URL after redirects).
  // Corresponds to:
  // https://fetch.spec.whatwg.org/#concept-request-current-url
  //
  // It is usually wrong to use this for security checks. See detailed
  // documentation at blink::ResourceResponse::CurrentRequestUrl().
  WebURL CurrentRequestUrl() const;
  void SetCurrentRequestUrl(const WebURL&);

  // The response URL of this resource. Corresponds to:
  // https://fetch.spec.whatwg.org/#concept-response-url
  //
  // This may be the empty URL. See detailed documentation at
  // blink::ResourceResponse::ResponseUrl().
  WebURL ResponseUrl() const;

  void SetConnectionID(unsigned);

  void SetConnectionReused(bool);

  void SetLoadTiming(const network::mojom::LoadTimingInfo&);

  base::Time ResponseTime() const;
  void SetResponseTime(base::Time);

  WebString MimeType() const;
  void SetMimeType(const WebString&);

  int64_t ExpectedContentLength() const;
  void SetExpectedContentLength(int64_t);

  void SetTextEncodingName(const WebString&);

  HTTPVersion HttpVersion() const;
  void SetHttpVersion(HTTPVersion);

  int RequestId() const;
  void SetRequestId(int);

  int HttpStatusCode() const;
  void SetHttpStatusCode(int);

  WebString HttpStatusText() const;
  void SetHttpStatusText(const WebString&);

  bool EmittedExtraInfo() const;
  void SetEmittedExtraInfo(bool);

  WebString HttpHeaderField(const WebString& name) const;
  void SetHttpHeaderField(const WebString& name, const WebString& value);
  void AddHttpHeaderField(const WebString& name, const WebString& value);
  void ClearHttpHeaderField(const WebString& name);
  void VisitHttpHeaderFields(WebHTTPHeaderVisitor*) const;

  void SetHasMajorCertificateErrors(bool);
  void SetHasRangeRequested(bool);
  void SetTimingAllowPassed(bool);
  bool TimingAllowPassed() const;

  void SetSecurityStyle(SecurityStyle);

  void SetSSLInfo(const net::SSLInfo&);

  void SetAsyncRevalidationRequested(bool);
  void SetNetworkAccessed(bool);

#if INSIDE_BLINK
  const ResourceResponse& ToResourceResponse() const;
#endif

  // Flag whether this request was served from the disk cache entry.
  void SetWasCached(bool);

  // Flag whether this request was loaded via the SPDY protocol or not.
  // SPDY is an experimental web protocol, see http://dev.chromium.org/spdy
  bool WasFetchedViaSPDY() const;
  void SetWasFetchedViaSPDY(bool);

  // Flag whether this request was loaded via a ServiceWorker.
  // See network.mojom.URLResponseHead.was_fetched_via_service_worker.
  bool WasFetchedViaServiceWorker() const;
  void SetWasFetchedViaServiceWorker(bool);

  // Set when this request was loaded via a ServiceWorker.
  // See network.mojom.URLResponseHead.service_worker_response_source.
  network::mojom::FetchResponseSource GetServiceWorkerResponseSource() const;
  void SetServiceWorkerResponseSource(network::mojom::FetchResponseSource);

  // See network.mojom.URLResponseHead.static_routing_info.
  void SetServiceWorkerRouterInfo(
      const network::mojom::ServiceWorkerRouterInfo&);

  // Flag whether a shared dictionary was used to decompress the response body.
  void SetDidUseSharedDictionary(bool);

  // https://fetch.spec.whatwg.org/#concept-response-type
  void SetType(network::mojom::FetchResponseType);
  network::mojom::FetchResponseType GetType() const;

  // Pre-computed padding.  This should only be non-zero if the type is
  // kOpaque.  In addition, it is only set for responses provided by a
  // service worker FetchEvent handler.
  void SetPadding(int64_t);
  int64_t GetPadding() const;

  // The URL list of the Response object the ServiceWorker passed to
  // respondWith().
  // See network.mojom.URLResponseHead.url_list_via_service_worker.
  void SetUrlListViaServiceWorker(const std::vector<WebURL>&);
  // Returns true if the URL list is not empty.
  bool HasUrlListViaServiceWorker() const;

  // The cache name of the CacheStorage from where the response is served via
  // the ServiceWorker. Null if the response isn't from the CacheStorage.
  WebString CacheStorageCacheName() const;
  void SetCacheStorageCacheName(const WebString&);

  // The headers that should be exposed according to CORS. Only guaranteed
  // to be set if the response was served by a ServiceWorker.
  std::vector<WebString> CorsExposedHeaderNames() const;
  void SetCorsExposedHeaderNames(const std::vector<WebString>&);

  // Whether service worker navigation preload occurred.
  // See network.mojom.URLResponseHead.did_navigation_preload.
  void SetDidServiceWorkerNavigationPreload(bool);

  // Remote IP endpoint of the socket which fetched this resource.
  net::IPEndPoint RemoteIPEndpoint() const;
  void SetRemoteIPEndpoint(const net::IPEndPoint&);

  // Address space from which this resource was fetched.
  network::mojom::IPAddressSpace AddressSpace() const;
  void SetAddressSpace(network::mojom::IPAddressSpace);

  network::mojom::IPAddressSpace ClientAddressSpace() const;
  void SetClientAddressSpace(network::mojom::IPAddressSpace);

  // Information about any preflight sent for this resource.
  // TODO(https://crbug.com/1268378): Remove this once preflights are enforced.
  network::mojom::PrivateNetworkAccessPreflightResult
  PrivateNetworkAccessPreflightResult() const;
  void SetPrivateNetworkAccessPreflightResult(
      network::mojom::PrivateNetworkAccessPreflightResult);

  // ALPN negotiated protocol of the socket which fetched this resource.
  bool WasAlpnNegotiated() const;
  void SetWasAlpnNegotiated(bool);
  WebString AlpnNegotiatedProtocol() const;
  void SetAlpnNegotiatedProtocol(const WebString&);
  void SetAlternateProtocolUsage(net::AlternateProtocolUsage);

  bool HasAuthorizationCoveredByWildcardOnPreflight() const;
  void SetHasAuthorizationCoveredByWildcardOnPreflight(bool);

  // Whether the response could use alternate protocol.
  bool WasAlternateProtocolAvailable() const;
  void SetWasAlternateProtocolAvailable(bool);

  // Information about the type of connection used to fetch this resource.
  net::HttpConnectionInfo ConnectionInfo() const;
  void SetConnectionInfo(net::HttpConnectionInfo);

  // Whether the response was cached and validated over the network.
  void SetIsValidated(bool);

  // Original size of the response before decompression.
  void SetEncodedDataLength(int64_t);

  // Original size of the response body before decompression.
  int64_t EncodedBodyLength() const;
  void SetEncodedBodyLength(uint64_t);

  void SetIsSignedExchangeInnerResponse(bool);
  void SetIsWebBundleInnerResponse(bool);
  void SetWasInPrefetchCache(bool);
  void SetWasCookieInRequest(bool);
  void SetRecursivePrefetchToken(const std::optional<base::UnguessableToken>&);

  // Whether this resource is from a MHTML archive.
  bool FromArchive() const;

  // Sets any DNS aliases for the requested URL. The alias chain order is
  // expected to be in reverse, from canonical name (i.e. address record name)
  // through to query name.
  void SetDnsAliases(const std::vector<WebString>&);

  void SetAuthChallengeInfo(const std::optional<net::AuthChallengeInfo>&);
  const std::optional<net::AuthChallengeInfo>& AuthChallengeInfo() const;

  // The request's |includeCredentials| value from the "HTTP-network fetch"
  // algorithm.
  // See: https://fetch.spec.whatwg.org/#concept-http-network-fetch
  void SetRequestIncludeCredentials(bool);
  bool RequestIncludeCredentials() const;

  void SetShouldUseSourceHashForJSCodeCache(bool);
  bool ShouldUseSourceHashForJSCodeCache() const;

  void SetWasFetchedViaCache(bool);

  void SetDeviceBoundSessionUsage(network::mojom::DeviceBoundSessionUsage);
  network::mojom::DeviceBoundSessionUsage DeviceBoundSessionUsage() const;

  // Whether the request was actually deferred by any device bound sessions.
  void SetWasDeferredByDeviceBoundSession(bool);
  bool WasDeferredByDeviceBoundSession() const;

#if INSIDE_BLINK
 protected:
  // Permit subclasses to set arbitrary ResourceResponse pointer as
  // |resource_response_|. |owned_resource_response_| is not set in this case.
  explicit WebURLResponse(ResourceResponse&);
#endif

 private:
  // If this instance owns a ResourceResponse then |owned_resource_response_|
  // is non-null and |resource_response_| points to the ResourceResponse
  // instance it contains.
  const std::unique_ptr<ResourceResponse> owned_resource_response_;

  // Should never be null.
  const raw_ptr<ResourceResponse> resource_response_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_URL_RESPONSE_H_
