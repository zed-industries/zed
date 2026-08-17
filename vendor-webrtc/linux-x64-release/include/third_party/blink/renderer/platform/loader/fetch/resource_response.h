/*
 * Copyright (C) 2006, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_RESPONSE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_RESPONSE_H_

#include <memory>
#include <optional>
#include <utility>

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "net/base/auth.h"
#include "net/base/ip_endpoint.h"
#include "net/http/alternate_protocol_usage.h"
#include "net/ssl/ssl_info.h"
#include "services/network/public/cpp/cors/cors_error_status.h"
#include "services/network/public/mojom/cross_origin_embedder_policy.mojom-forward.h"
#include "services/network/public/mojom/device_bound_sessions.mojom-shared.h"
#include "services/network/public/mojom/fetch_api.mojom-shared.h"
#include "services/network/public/mojom/ip_address_space.mojom-shared.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_url_response.h"
#include "third_party/blink/renderer/platform/network/http_header_map.h"
#include "third_party/blink/renderer/platform/network/http_parsers.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/date_math.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class FeatureContext;
class UnencodedDigest;
class ResourceLoadTiming;
class ServiceWorkerRouterInfo;
class UseCounter;

// A ResourceResponse is a "response" object used in blink. Conceptually
// it is https://fetch.spec.whatwg.org/#concept-response, but it contains
// a lot of blink specific fields. WebURLResponse is the "public version"
// of this class and public classes (i.e., classes in public/platform) use it.
//
// This class is thread-bound. Do not copy/pass an instance across threads.
class PLATFORM_EXPORT ResourceResponse final {
  USING_FAST_MALLOC(ResourceResponse);

 public:
  enum HTTPVersion : uint8_t {
    kHTTPVersionUnknown,
    kHTTPVersion_0_9,
    kHTTPVersion_1_0,
    kHTTPVersion_1_1,
    kHTTPVersion_2_0
  };

  ResourceResponse();
  explicit ResourceResponse(const KURL& current_request_url);
  ResourceResponse(const ResourceResponse&);
  ResourceResponse& operator=(const ResourceResponse&);
  ~ResourceResponse();

  bool IsNull() const { return is_null_; }
  bool IsHTTP() const;

  // When serving resources from a WebBundle, we might have resources whose
  // source isn't a URL (like urn:uuid), but we still need to create and
  // populate ResourceTiming entries for them, so we need to check that either
  // response has a proper request URL or whether the response is an inner
  // response of a WebBundle.
  bool ShouldPopulateResourceTiming() const;

  // The current request URL for this resource (the URL after redirects).
  // Corresponds to:
  // https://fetch.spec.whatwg.org/#concept-request-current-url
  //
  // Beware that this might not be the same the response URL, so it is usually
  // incorrect to use this in security checks. Use GetType() to determine origin
  // sameness.
  //
  // Specifically, if a service worker responded to the request for this
  // resource, it may have fetched an entirely different URL and responded with
  // that resource. WasFetchedViaServiceWorker() and ResponseUrl() can be used
  // to determine whether and how a service worker responded to the request.
  // Example service worker code:
  //
  // onfetch = (event => {
  //   if (event.request.url == 'https://abc.com')
  //     event.respondWith(fetch('https://def.com'));
  // });
  //
  // If this service worker responds to an "https://abc.com" request, then for
  // the resulting ResourceResponse, CurrentRequestUrl() is "https://abc.com",
  // WasFetchedViaServiceWorker() is true, and ResponseUrl() is
  // "https://def.com".
  const KURL& CurrentRequestUrl() const;
  void SetCurrentRequestUrl(const KURL&);

  // The response URL of this resource. Corresponds to:
  // https://fetch.spec.whatwg.org/#concept-response-url
  //
  // This returns the same URL as CurrentRequestUrl() unless a service worker
  // responded to the request. See the comments for that function.
  KURL ResponseUrl() const;

  // Returns true if this response is the result of a service worker
  // effectively calling `evt.respondWith(fetch(evt.request))`.  Specifically,
  // it returns false for synthetic constructed responses, responses fetched
  // from different URLs, and responses produced by cache_storage.
  bool IsServiceWorkerPassThrough() const;

  const AtomicString& MimeType() const;
  void SetMimeType(const AtomicString&);

  int64_t ExpectedContentLength() const;
  void SetExpectedContentLength(int64_t);

  const AtomicString& TextEncodingName() const;
  void SetTextEncodingName(const AtomicString&);

  int HttpStatusCode() const;
  void SetHttpStatusCode(int);

  const AtomicString& HttpStatusText() const;
  void SetHttpStatusText(const AtomicString&);

  const AtomicString& HttpHeaderField(const AtomicString& name) const;
  void SetHttpHeaderField(const AtomicString& name, const AtomicString& value);
  void AddHttpHeaderField(const AtomicString& name, const AtomicString& value);
  void AddHttpHeaderFieldWithMultipleValues(const AtomicString& name,
                                            const Vector<AtomicString>& values);
  void ClearHttpHeaderField(const AtomicString& name);
  const HTTPHeaderMap& HttpHeaderFields() const;

  bool IsAttachment() const;

  AtomicString HttpContentType() const;
  AtomicString GetFilteredHttpContentEncoding() const;

  // These functions return parsed values of the corresponding response headers.
  // NaN means that the header was not present or had invalid value.
  bool CacheControlContainsNoCache() const;
  bool CacheControlContainsNoStore() const;
  bool CacheControlContainsMustRevalidate() const;
  bool HasCacheValidatorFields() const;
  std::optional<base::TimeDelta> CacheControlMaxAge() const;
  std::optional<base::Time> Date(UseCounter&) const;
  std::optional<base::TimeDelta> Age() const;
  std::optional<base::Time> Expires(UseCounter&) const;
  std::optional<base::Time> LastModified(UseCounter&) const;
  // Will always return values >= 0.
  base::TimeDelta CacheControlStaleWhileRevalidate() const;
  std::optional<UnencodedDigest> UnencodedDigest(const FeatureContext*) const;

  unsigned ConnectionID() const;
  void SetConnectionID(unsigned);

  bool ConnectionReused() const;
  void SetConnectionReused(bool);

  bool WasCached() const;
  void SetWasCached(bool);

  ResourceLoadTiming* GetResourceLoadTiming() const;
  void SetResourceLoadTiming(scoped_refptr<ResourceLoadTiming>);

  HTTPVersion HttpVersion() const { return http_version_; }
  void SetHttpVersion(HTTPVersion version) { http_version_ = version; }

  int RequestId() const { return request_id_; }
  void SetRequestId(int request_id) { request_id_ = request_id; }

  bool HasMajorCertificateErrors() const {
    return has_major_certificate_errors_;
  }
  void SetHasMajorCertificateErrors(bool has_major_certificate_errors) {
    has_major_certificate_errors_ = has_major_certificate_errors;
  }

  bool HasRangeRequested() const { return has_range_requested_; }
  void SetHasRangeRequested(bool value) { has_range_requested_ = value; }

  bool TimingAllowPassed() const { return timing_allow_passed_; }
  void SetTimingAllowPassed(bool value) { timing_allow_passed_ = value; }

  SecurityStyle GetSecurityStyle() const { return security_style_; }
  void SetSecurityStyle(SecurityStyle security_style) {
    security_style_ = security_style;
  }

  const std::optional<net::SSLInfo>& GetSSLInfo() const { return ssl_info_; }
  void SetSSLInfo(const net::SSLInfo& ssl_info);

  bool EmittedExtraInfo() const { return emitted_extra_info_; }
  void SetEmittedExtraInfo(bool emitted_extra_info) {
    emitted_extra_info_ = emitted_extra_info;
  }

  bool WasFetchedViaSPDY() const { return was_fetched_via_spdy_; }
  void SetWasFetchedViaSPDY(bool value) { was_fetched_via_spdy_ = value; }

  // See network.mojom.URLResponseHead.was_fetched_via_service_worker.
  bool WasFetchedViaServiceWorker() const {
    return was_fetched_via_service_worker_;
  }
  void SetWasFetchedViaServiceWorker(bool value) {
    was_fetched_via_service_worker_ = value;
  }

  network::mojom::FetchResponseSource GetServiceWorkerResponseSource() const {
    return service_worker_response_source_;
  }

  // See network.mojom.URLResponseHead.service_worker_router_info.
  const blink::ServiceWorkerRouterInfo* GetServiceWorkerRouterInfo() const {
    return service_worker_router_info_.get();
  }
  void SetServiceWorkerRouterInfo(scoped_refptr<ServiceWorkerRouterInfo> value);

  void SetServiceWorkerResponseSource(
      network::mojom::FetchResponseSource value) {
    service_worker_response_source_ = value;
  }

  network::mojom::FetchResponseType GetType() const { return response_type_; }
  void SetType(network::mojom::FetchResponseType value) {
    response_type_ = value;
  }
  // https://html.spec.whatwg.org/C/#cors-same-origin
  bool IsCorsSameOrigin() const;
  // https://html.spec.whatwg.org/C/#cors-cross-origin
  bool IsCorsCrossOrigin() const;

  int64_t GetPadding() const { return padding_; }
  void SetPadding(int64_t padding) { padding_ = padding; }

  // See network.mojom.URLResponseHead.url_list_via_service_worker.
  const Vector<KURL>& UrlListViaServiceWorker() const {
    return url_list_via_service_worker_;
  }
  void SetUrlListViaServiceWorker(const Vector<KURL>& url_list) {
    url_list_via_service_worker_ = url_list;
  }

  const String& CacheStorageCacheName() const {
    return cache_storage_cache_name_;
  }
  void SetCacheStorageCacheName(const String& cache_storage_cache_name) {
    cache_storage_cache_name_ = cache_storage_cache_name;
  }

  const Vector<String>& CorsExposedHeaderNames() const {
    return cors_exposed_header_names_;
  }
  void SetCorsExposedHeaderNames(const Vector<String>& header_names) {
    cors_exposed_header_names_ = header_names;
  }

  bool DidServiceWorkerNavigationPreload() const {
    return did_service_worker_navigation_preload_;
  }
  void SetDidServiceWorkerNavigationPreload(bool value) {
    did_service_worker_navigation_preload_ = value;
  }

  bool DidUseSharedDictionary() const { return did_use_shared_dictionary_; }
  void SetDidUseSharedDictionary(bool value) {
    did_use_shared_dictionary_ = value;
  }

  base::Time ResponseTime() const { return response_time_; }
  void SetResponseTime(base::Time response_time) {
    response_time_ = response_time;
  }

  const net::IPEndPoint& RemoteIPEndpoint() const {
    return remote_ip_endpoint_;
  }
  void SetRemoteIPEndpoint(const net::IPEndPoint& value) {
    remote_ip_endpoint_ = value;
  }

  network::mojom::IPAddressSpace AddressSpace() const { return address_space_; }
  void SetAddressSpace(network::mojom::IPAddressSpace value) {
    address_space_ = value;
  }

  network::mojom::IPAddressSpace ClientAddressSpace() const {
    return client_address_space_;
  }
  void SetClientAddressSpace(network::mojom::IPAddressSpace value) {
    client_address_space_ = value;
  }

  network::mojom::PrivateNetworkAccessPreflightResult
  PrivateNetworkAccessPreflightResult() const {
    return private_network_access_preflight_result_;
  }
  void SetPrivateNetworkAccessPreflightResult(
      network::mojom::PrivateNetworkAccessPreflightResult result) {
    private_network_access_preflight_result_ = result;
  }

  bool WasAlpnNegotiated() const { return was_alpn_negotiated_; }
  void SetWasAlpnNegotiated(bool was_alpn_negotiated) {
    was_alpn_negotiated_ = was_alpn_negotiated;
  }

  bool HasAuthorizationCoveredByWildcardOnPreflight() const {
    return has_authorization_covered_by_wildcard_on_preflight_;
  }
  void SetHasAuthorizationCoveredByWildcardOnPreflight(bool b) {
    has_authorization_covered_by_wildcard_on_preflight_ = b;
  }

  const AtomicString& AlpnNegotiatedProtocol() const {
    return alpn_negotiated_protocol_;
  }
  void SetAlpnNegotiatedProtocol(const AtomicString& value) {
    alpn_negotiated_protocol_ = value;
  }

  net::AlternateProtocolUsage AlternateProtocolUsage() const {
    return alternate_protocol_usage_;
  }
  void SetAlternateProtocolUsage(net::AlternateProtocolUsage value) {
    alternate_protocol_usage_ = value;
  }

  net::HttpConnectionInfo ConnectionInfo() const { return connection_info_; }
  void SetConnectionInfo(net::HttpConnectionInfo value) {
    connection_info_ = value;
  }

  AtomicString ConnectionInfoString() const;

  mojom::blink::CacheState CacheState() const;
  void SetIsValidated(bool is_validated);

  int64_t EncodedDataLength() const { return encoded_data_length_; }
  void SetEncodedDataLength(int64_t value);

  int64_t EncodedBodyLength() const { return encoded_body_length_; }
  void SetEncodedBodyLength(uint64_t value);

  int64_t DecodedBodyLength() const { return decoded_body_length_; }
  void SetDecodedBodyLength(int64_t value);

  const std::optional<base::UnguessableToken>& RecursivePrefetchToken() const {
    return recursive_prefetch_token_;
  }
  void SetRecursivePrefetchToken(
      const std::optional<base::UnguessableToken>& token) {
    recursive_prefetch_token_ = token;
  }

  unsigned MemoryUsage() const {
    // average size, mostly due to URL and Header Map strings
    return 1280;
  }

  bool AsyncRevalidationRequested() const {
    return async_revalidation_requested_;
  }

  void SetAsyncRevalidationRequested(bool requested) {
    async_revalidation_requested_ = requested;
  }

  bool NetworkAccessed() const { return network_accessed_; }

  void SetNetworkAccessed(bool network_accessed) {
    network_accessed_ = network_accessed;
  }

  bool FromArchive() const { return from_archive_; }

  void SetFromArchive(bool from_archive) { from_archive_ = from_archive; }

  bool WasAlternateProtocolAvailable() const {
    return was_alternate_protocol_available_;
  }

  void SetWasAlternateProtocolAvailable(bool was_alternate_protocol_available) {
    was_alternate_protocol_available_ = was_alternate_protocol_available;
  }

  bool IsSignedExchangeInnerResponse() const {
    return is_signed_exchange_inner_response_;
  }

  void SetIsSignedExchangeInnerResponse(
      bool is_signed_exchange_inner_response) {
    is_signed_exchange_inner_response_ = is_signed_exchange_inner_response;
  }

  void SetIsWebBundleInnerResponse(bool is_web_bundle_inner_response) {
    is_web_bundle_inner_response_ = is_web_bundle_inner_response;
  }

  bool WasInPrefetchCache() const { return was_in_prefetch_cache_; }

  void SetWasInPrefetchCache(bool was_in_prefetch_cache) {
    was_in_prefetch_cache_ = was_in_prefetch_cache;
  }

  bool WasCookieInRequest() const { return was_cookie_in_request_; }

  void SetWasCookieInRequest(bool was_cookie_in_request) {
    was_cookie_in_request_ = was_cookie_in_request;
  }

  const Vector<String>& DnsAliases() const { return dns_aliases_; }

  void SetDnsAliases(Vector<String> aliases) {
    dns_aliases_ = std::move(aliases);
  }

  network::mojom::CrossOriginEmbedderPolicyValue GetCrossOriginEmbedderPolicy()
      const;

  const std::optional<net::AuthChallengeInfo>& AuthChallengeInfo() const {
    return auth_challenge_info_;
  }
  void SetAuthChallengeInfo(
      const std::optional<net::AuthChallengeInfo>& value) {
    auth_challenge_info_ = value;
  }

  bool RequestIncludeCredentials() const {
    return request_include_credentials_;
  }
  void SetRequestIncludeCredentials(bool request_include_credentials) {
    request_include_credentials_ = request_include_credentials;
  }

  bool ShouldUseSourceHashForJSCodeCache() const {
    return should_use_source_hash_for_js_code_cache_;
  }
  void SetShouldUseSourceHashForJSCodeCache(
      bool should_use_source_hash_for_js_code_cache) {
    if (should_use_source_hash_for_js_code_cache) {
      // This flag should only be set for http(s) resources, because others
      // would end up blocked in the browser process anyway (see
      // code_cache_host_impl.cc).
      CHECK(CurrentRequestUrl().ProtocolIsInHTTPFamily());
    }
    should_use_source_hash_for_js_code_cache_ =
        should_use_source_hash_for_js_code_cache;
  }

  void SetDeviceBoundSessionUsage(
      network::mojom::DeviceBoundSessionUsage usage) {
    device_bound_session_usage_ = usage;
  }
  network::mojom::DeviceBoundSessionUsage DeviceBoundSessionUsage() const {
    return device_bound_session_usage_;
  }

 private:
  void UpdateHeaderParsedState(const AtomicString& name);

  KURL current_request_url_;
  AtomicString mime_type_;
  int64_t expected_content_length_ = 0;
  AtomicString text_encoding_name_;

  unsigned connection_id_ = 0;
  int http_status_code_ = 0;
  AtomicString http_status_text_;
  HTTPHeaderMap http_header_fields_;

  // Remote IP endpoint of the socket which fetched this resource.
  net::IPEndPoint remote_ip_endpoint_;

  // The address space from which this resource was fetched.
  // https://wicg.github.io/private-network-access/#response-ip-address-space
  network::mojom::IPAddressSpace address_space_ =
      network::mojom::IPAddressSpace::kUnknown;

  // The address space of the request client.
  // https://wicg.github.io/private-network-access/#policy-container-ip-address-space
  network::mojom::IPAddressSpace client_address_space_ =
      network::mojom::IPAddressSpace::kUnknown;

  // The result of any PNA preflight sent for this request, if any.
  // TODO(https://crbug.com/1268378): Remove this once preflights are enforced.
  network::mojom::PrivateNetworkAccessPreflightResult
      private_network_access_preflight_result_ =
          network::mojom::PrivateNetworkAccessPreflightResult::kNone;

  network::mojom::DeviceBoundSessionUsage device_bound_session_usage_ =
      network::mojom::DeviceBoundSessionUsage::kUnknown;

  bool was_cached_ : 1;
  bool connection_reused_ : 1;
  bool is_null_ : 1;
  mutable bool have_parsed_age_header_ : 1;
  mutable bool have_parsed_date_header_ : 1;
  mutable bool have_parsed_expires_header_ : 1;
  mutable bool have_parsed_last_modified_header_ : 1;

  // True if the resource was retrieved by the embedder in spite of
  // certificate errors.
  bool has_major_certificate_errors_ : 1;

  // This corresponds to the range-requested flag in the Fetch spec:
  // https://fetch.spec.whatwg.org/#concept-response-range-requested-flag
  bool has_range_requested_ : 1;

  // True if the Timing-Allow-Origin check passes.
  // https://fetch.spec.whatwg.org/#concept-response-timing-allow-passed
  bool timing_allow_passed_ : 1;

  // Was the resource fetched over SPDY.  See http://dev.chromium.org/spdy
  bool was_fetched_via_spdy_ : 1;

  // Was the resource fetched over a ServiceWorker.
  bool was_fetched_via_service_worker_ : 1;

  // True if service worker navigation preload was performed due to
  // the request for this resource.
  bool did_service_worker_navigation_preload_ : 1;

  // True if a shared dictionary was used to decompress the response body.
  bool did_use_shared_dictionary_ : 1;

  // True if this resource is stale and needs async revalidation. Will only
  // possibly be set if the load_flags indicated SUPPORT_ASYNC_REVALIDATION.
  bool async_revalidation_requested_ : 1;

  // True if this resource is from an inner response of a signed exchange.
  // https://wicg.github.io/webpackage/draft-yasskin-http-origin-signed-responses.html
  bool is_signed_exchange_inner_response_ : 1;

  // True if this resource is an inner response of a WebBundle.
  bool is_web_bundle_inner_response_ : 1;

  // True if this resource is served from the prefetch cache.
  bool was_in_prefetch_cache_ : 1;

  // True if a cookie was sent in the request for this resource.
  bool was_cookie_in_request_ : 1;

  // True if this resource was loaded from the network.
  bool network_accessed_ : 1;

  // True if this resource was loaded from a MHTML archive.
  bool from_archive_ : 1;

  // True if response could use alternate protocol.
  bool was_alternate_protocol_available_ : 1;

  // True if the response was delivered after ALPN is negotiated.
  bool was_alpn_negotiated_ : 1;

  // True when there is an "authorization" header on the request and it is
  // covered by the wildcard in the preflight response.
  // TODO(crbug.com/1176753): Remove this once the investigation is done.
  bool has_authorization_covered_by_wildcard_on_preflight_ : 1;

  // Whether the resource came from the cache and validated over the network.
  bool is_validated_ : 1;

  // [spec] https://fetch.spec.whatwg.org/#response-request-includes-credentials
  // The request's |includeCredentials| value from the "HTTP-network fetch"
  // algorithm.
  // See: https://fetch.spec.whatwg.org/#concept-http-network-fetch
  bool request_include_credentials_ : 1;

  // If this response contains JavaScript, then downstream components may cache
  // the parsed bytecode, but must use a source hash comparison rather than the
  // response time when determining whether the current version of the script
  // matches the cached bytecode.
  bool should_use_source_hash_for_js_code_cache_ : 1;

  // Pre-computed padding.  This should only be non-zero if |response_type| is
  // set to kOpaque.  In addition, it is only set if the response was provided
  // by a service worker FetchEvent handler.
  int64_t padding_ = 0;

  // The time at which the resource's certificate expires. Null if there was no
  // certificate.
  base::Time cert_validity_start_;

  // The source of the resource, if it was fetched via ServiceWorker. This is
  // kUnspecified if |was_fetched_via_service_worker| is false.
  network::mojom::FetchResponseSource service_worker_response_source_ =
      network::mojom::FetchResponseSource::kUnspecified;

  // The information about the ServiceWorker Static Router that handled the
  // request. Null if there was no registered Static Routers.
  scoped_refptr<blink::ServiceWorkerRouterInfo> service_worker_router_info_;

  // https://fetch.spec.whatwg.org/#concept-response-type
  network::mojom::FetchResponseType response_type_ =
      network::mojom::FetchResponseType::kDefault;

  // HTTP version used in the response, if known.
  HTTPVersion http_version_ = kHTTPVersionUnknown;

  // Request id given to the resource by the WebUrlLoader.
  int request_id_ = 0;

  // The security style of the resource.
  // This only contains a valid value when the DevTools Network domain is
  // enabled. (Otherwise, it contains a default value of Unknown.)
  SecurityStyle security_style_ = SecurityStyle::kUnknown;

  // Security details of this request's connection.
  std::optional<net::SSLInfo> ssl_info_;

  scoped_refptr<ResourceLoadTiming> resource_load_timing_;

  mutable CacheControlHeader cache_control_header_;

  mutable std::optional<base::TimeDelta> age_;
  mutable std::optional<base::Time> date_;
  mutable std::optional<base::Time> expires_;
  mutable std::optional<base::Time> last_modified_;

  // The URL list of the response which was fetched by the ServiceWorker.
  // This is empty if the response was created inside the ServiceWorker.
  Vector<KURL> url_list_via_service_worker_;

  // The cache name of the CacheStorage from where the response is served via
  // the ServiceWorker. Null if the response isn't from the CacheStorage.
  String cache_storage_cache_name_;

  // The headers that should be exposed according to CORS. Only guaranteed
  // to be set if the response was fetched by a ServiceWorker.
  Vector<String> cors_exposed_header_names_;

  // The time at which the response headers were received.  For cached
  // responses, this time could be "far" in the past.
  base::Time response_time_;

  // ALPN negotiated protocol of the socket which fetched this resource.
  AtomicString alpn_negotiated_protocol_;

  // The reason why Chrome uses a specific transport protocol for HTTP
  // semantics.
  net::AlternateProtocolUsage alternate_protocol_usage_ =
      net::AlternateProtocolUsage::ALTERNATE_PROTOCOL_USAGE_UNSPECIFIED_REASON;

  // Information about the type of connection used to fetch this resource.
  net::HttpConnectionInfo connection_info_ = net::HttpConnectionInfo::kUNKNOWN;

  // Size of the response in bytes prior to decompression.
  int64_t encoded_data_length_ = 0;

  // Size of the response body in bytes prior to decompression.
  uint64_t encoded_body_length_ = 0;

  // Sizes of the response body in bytes after any content-encoding is
  // removed.
  int64_t decoded_body_length_ = 0;

  // This is propagated from the browser process's PrefetchURLLoader on
  // cross-origin prefetch responses. It is used to pass the token along to
  // preload header requests from these responses.
  std::optional<base::UnguessableToken> recursive_prefetch_token_;

  // Any DNS aliases for the requested URL, as read from CNAME records.
  // Includes all known aliases, e.g. from A, AAAA, or HTTPS, not just from the
  // address used for the connection, in no particular order.
  Vector<String> dns_aliases_;

  std::optional<net::AuthChallengeInfo> auth_challenge_info_;

  bool emitted_extra_info_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_RESPONSE_H_
