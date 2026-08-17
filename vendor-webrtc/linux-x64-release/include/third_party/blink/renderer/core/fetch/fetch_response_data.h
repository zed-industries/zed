// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_RESPONSE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_RESPONSE_DATA_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "net/http/http_connection_info.h"
#include "services/network/public/mojom/fetch_api.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_response.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fetch/body_stream_buffer.h"
#include "third_party/blink/renderer/core/fetch/fetch_request_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_response.h"
#include "third_party/blink/renderer/platform/network/http_header_set.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExceptionState;
class FetchHeaderList;
class ScriptState;

class CORE_EXPORT FetchResponseData final
    : public GarbageCollected<FetchResponseData> {
 public:
  // "A response can have an associated termination reason which is one of
  // end-user abort, fatal, and timeout."
  enum TerminationReason {
    kEndUserAbortTermination,
    kFatalTermination,
    kTimeoutTermination
  };

  static FetchResponseData* Create();
  static FetchResponseData* CreateNetworkErrorResponse();
  static FetchResponseData* CreateWithBuffer(BodyStreamBuffer*);

  FetchResponseData(network::mojom::FetchResponseType,
                    network::mojom::FetchResponseSource,
                    uint16_t,
                    AtomicString);
  FetchResponseData(const FetchResponseData&) = delete;
  FetchResponseData& operator=(const FetchResponseData&) = delete;

  FetchResponseData* CreateBasicFilteredResponse() const;
  FetchResponseData* CreateCorsFilteredResponse(
      const HTTPHeaderSet& exposed_headers) const;
  FetchResponseData* CreateOpaqueFilteredResponse() const;
  FetchResponseData* CreateOpaqueRedirectFilteredResponse() const;

  FetchResponseData* InternalResponse() { return internal_response_.Get(); }
  const FetchResponseData* InternalResponse() const {
    return internal_response_.Get();
  }

  FetchResponseData* Clone(ScriptState*, ExceptionState& exception_state);

  network::mojom::FetchResponseType GetType() const { return type_; }
  network::mojom::FetchResponseSource ResponseSource() const {
    return response_source_;
  }
  const KURL* Url() const;
  uint16_t Status() const { return status_; }
  uint16_t InternalStatus() const;
  AtomicString StatusMessage() const { return status_message_; }
  FetchHeaderList* HeaderList() const { return header_list_.Get(); }
  FetchHeaderList* InternalHeaderList() const;
  BodyStreamBuffer* Buffer() const { return buffer_.Get(); }
  String MimeType() const;
  // Returns the BodyStreamBuffer of |m_internalResponse| if any. Otherwise,
  // returns |m_buffer|.
  BodyStreamBuffer* InternalBuffer() const;
  String InternalMIMEType() const;
  base::Time ResponseTime() const { return response_time_; }
  String CacheStorageCacheName() const { return cache_storage_cache_name_; }
  const HTTPHeaderSet& CorsExposedHeaderNames() const {
    return cors_exposed_header_names_;
  }
  bool HasRangeRequested() const { return has_range_requested_; }
  bool RequestIncludeCredentials() const;

  int64_t GetPadding() const { return padding_; }
  void SetPadding(int64_t padding) { padding_ = padding; }
  void SetResponseSource(network::mojom::FetchResponseSource response_source) {
    response_source_ = response_source;
  }
  void SetURLList(const Vector<KURL>&);
  const Vector<KURL>& UrlList() const { return url_list_; }
  const Vector<KURL>& InternalURLList() const;

  void SetStatus(uint16_t status) { status_ = status; }
  void SetStatusMessage(AtomicString status_message) {
    status_message_ = status_message;
  }
  void SetMimeType(const String& type) { mime_type_ = type; }
  void SetRequestMethod(const AtomicString& method) {
    request_method_ = method;
  }
  void SetResponseTime(base::Time response_time) {
    response_time_ = response_time;
  }
  void SetCacheStorageCacheName(const String& cache_storage_cache_name) {
    cache_storage_cache_name_ = cache_storage_cache_name;
  }
  void SetCorsExposedHeaderNames(const HTTPHeaderSet& header_names) {
    cors_exposed_header_names_ = header_names;
  }
  void SetConnectionInfo(net::HttpConnectionInfo connection_info) {
    connection_info_ = connection_info;
  }
  void SetAlpnNegotiatedProtocol(AtomicString alpn_negotiated_protocol) {
    alpn_negotiated_protocol_ = alpn_negotiated_protocol;
  }
  void SetWasFetchedViaSpdy(bool was_fetched_via_spdy) {
    was_fetched_via_spdy_ = was_fetched_via_spdy;
  }
  void SetHasRangeRequested(bool has_range_requested) {
    has_range_requested_ = has_range_requested;
  }
  void SetAuthChallengeInfo(
      const std::optional<net::AuthChallengeInfo>& auth_challenge_info);
  void SetRequestIncludeCredentials(bool request_include_credentials);

  // If the type is Default, replaces |buffer_|.
  // If the type is Basic or CORS, replaces |buffer_| and
  // |internal_response_->buffer_|.
  // If the type is Error or Opaque, does nothing.
  void ReplaceBodyStreamBuffer(BodyStreamBuffer*);

  // Does not contain the blob response body.
  mojom::blink::FetchAPIResponsePtr PopulateFetchAPIResponse(
      const KURL& request_url);

  // Initialize non-body data from the given |response|.
  void InitFromResourceResponse(
      ExecutionContext* context,
      network::mojom::FetchResponseType response_type,
      const Vector<KURL>& request_url_list,
      const AtomicString& request_method,
      network::mojom::CredentialsMode request_credentials,
      const ResourceResponse& response);

  void Trace(Visitor*) const;

 private:
  network::mojom::FetchResponseType type_;
  int64_t padding_;
  network::mojom::FetchResponseSource response_source_;
  std::unique_ptr<TerminationReason> termination_reason_;
  Vector<KURL> url_list_;
  uint16_t status_;
  AtomicString status_message_;
  Member<FetchHeaderList> header_list_;
  Member<FetchResponseData> internal_response_;
  Member<BodyStreamBuffer> buffer_;
  String mime_type_;
  AtomicString request_method_;
  base::Time response_time_;
  String cache_storage_cache_name_;
  HTTPHeaderSet cors_exposed_header_names_;
  net::HttpConnectionInfo connection_info_ = net::HttpConnectionInfo::kUNKNOWN;
  AtomicString alpn_negotiated_protocol_;
  // |auth_challenge_info_| is a std::unique_ptr instead of std::optional
  // |because this member is empty in most cases.
  std::unique_ptr<net::AuthChallengeInfo> auth_challenge_info_;

  bool was_fetched_via_spdy_ : 1;
  bool has_range_requested_ : 1;
  // The request's |includeCredentials| value from the "HTTP-network fetch"
  // algorithm.
  // See: https://fetch.spec.whatwg.org/#concept-http-network-fetch
  bool request_include_credentials_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_RESPONSE_DATA_H_
