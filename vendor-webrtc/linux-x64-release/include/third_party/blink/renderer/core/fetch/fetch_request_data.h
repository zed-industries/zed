// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_REQUEST_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_REQUEST_DATA_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/unguessable_token.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/network/public/mojom/attribution.mojom-blink.h"
#include "services/network/public/mojom/fetch_api.mojom-blink-forward.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/trust_tokens.mojom-blink.h"
#include "services/network/public/mojom/url_loader_factory.mojom-blink.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/fetch/body_stream_buffer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_priority.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class FetchHeaderList;
class SecurityOrigin;
class ScriptState;

class CORE_EXPORT FetchRequestData final
    : public GarbageCollected<FetchRequestData> {
 public:
  enum class ForServiceWorkerFetchEvent { kFalse, kTrue };

  static FetchRequestData* Create(ScriptState*,
                                  mojom::blink::FetchAPIRequestPtr,
                                  ForServiceWorkerFetchEvent);
  FetchRequestData* Clone(ScriptState*, ExceptionState&);
  FetchRequestData* Pass(ScriptState*, ExceptionState&);

  explicit FetchRequestData(ExecutionContext* execution_context);
  FetchRequestData(const FetchRequestData&) = delete;
  FetchRequestData& operator=(const FetchRequestData&) = delete;
  ~FetchRequestData();

  void SetMethod(AtomicString method) { method_ = method; }
  const AtomicString& Method() const { return method_; }
  void SetURL(const KURL& url) { url_ = url; }
  const KURL& Url() const { return url_; }
  network::mojom::RequestDestination Destination() const {
    return destination_;
  }
  void SetDestination(network::mojom::RequestDestination destination) {
    destination_ = destination;
  }
  scoped_refptr<const SecurityOrigin> Origin() const { return origin_; }
  void SetOrigin(scoped_refptr<const SecurityOrigin> origin) {
    origin_ = std::move(origin);
  }
  const WTF::Vector<KURL>& NavigationRedirectChain() const {
    return navigation_redirect_chain_;
  }
  void SetNavigationRedirectChain(const WTF::Vector<KURL>& value) {
    navigation_redirect_chain_ = value;
  }
  scoped_refptr<const SecurityOrigin> IsolatedWorldOrigin() const {
    return isolated_world_origin_;
  }
  void SetIsolatedWorldOrigin(
      scoped_refptr<const SecurityOrigin> isolated_world_origin) {
    isolated_world_origin_ = std::move(isolated_world_origin);
  }
  const AtomicString& ReferrerString() const { return referrer_string_; }
  void SetReferrerString(const AtomicString& s) { referrer_string_ = s; }
  network::mojom::ReferrerPolicy GetReferrerPolicy() const {
    return referrer_policy_;
  }
  void SetReferrerPolicy(network::mojom::ReferrerPolicy p) {
    referrer_policy_ = p;
  }
  void SetMode(network::mojom::RequestMode mode) { mode_ = mode; }
  network::mojom::RequestMode Mode() const { return mode_; }
  void SetTargetAddressSpace(
      network::mojom::IPAddressSpace target_address_space) {
    target_address_space_ = target_address_space;
  }
  network::mojom::IPAddressSpace TargetAddressSpace() const {
    return target_address_space_;
  }
  void SetCredentials(network::mojom::CredentialsMode credentials) {
    credentials_ = credentials;
  }
  network::mojom::CredentialsMode Credentials() const { return credentials_; }
  void SetCacheMode(mojom::blink::FetchCacheMode cache_mode) {
    cache_mode_ = cache_mode;
  }
  mojom::blink::FetchCacheMode CacheMode() const { return cache_mode_; }
  void SetRedirect(network::mojom::RedirectMode redirect) {
    redirect_ = redirect;
  }
  network::mojom::RedirectMode Redirect() const { return redirect_; }
  void SetFetchPriorityHint(
      mojom::blink::FetchPriorityHint fetch_priority_hint) {
    fetch_priority_hint_ = fetch_priority_hint;
  }
  mojom::blink::FetchPriorityHint FetchPriorityHint() const {
    return fetch_priority_hint_;
  }
  FetchHeaderList* HeaderList() const { return header_list_.Get(); }
  void SetHeaderList(FetchHeaderList* header_list) {
    header_list_ = header_list;
  }
  BodyStreamBuffer* Buffer() const { return buffer_.Get(); }
  void SetBuffer(BodyStreamBuffer* buffer, uint64_t length = 0) {
    buffer_ = buffer;
    buffer_byte_length_ = length;
  }
  uint64_t BufferByteLength() const { return buffer_byte_length_; }
  String MimeType() const { return mime_type_; }
  void SetMimeType(const String& type) { mime_type_ = type; }
  String Integrity() const { return integrity_; }
  void SetIntegrity(const String& integrity) { integrity_ = integrity; }
  ResourceLoadPriority Priority() const { return priority_; }
  void SetPriority(ResourceLoadPriority priority) { priority_ = priority; }

  // The original destination of a request passed through by a service worker.
  void SetOriginalDestination(network::mojom::RequestDestination value) {
    original_destination_ = value;
  }
  network::mojom::RequestDestination OriginalDestination() const {
    return original_destination_;
  }

  bool Keepalive() const { return keepalive_; }
  void SetKeepalive(bool b) { keepalive_ = b; }

  bool BrowsingTopics() const { return browsing_topics_; }
  void SetBrowsingTopics(bool b) { browsing_topics_ = b; }

  bool AdAuctionHeaders() const { return ad_auction_headers_; }
  void SetAdAuctionHeaders(bool b) { ad_auction_headers_ = b; }

  bool SharedStorageWritable() const { return shared_storage_writable_; }
  void SetSharedStorageWritable(bool shared_storage_writable) {
    shared_storage_writable_ = shared_storage_writable;
  }

  bool IsHistoryNavigation() const { return is_history_navigation_; }
  void SetIsHistoryNavigation(bool b) { is_history_navigation_ = b; }

  network::mojom::blink::URLLoaderFactory* URLLoaderFactory() const {
    return url_loader_factory_.is_bound() ? url_loader_factory_.get() : nullptr;
  }
  void SetURLLoaderFactory(
      mojo::PendingRemote<network::mojom::blink::URLLoaderFactory> factory) {
    url_loader_factory_.Bind(
        std::move(factory),
        execution_context_->GetTaskRunner(TaskType::kNetworking));
  }
  const base::UnguessableToken& WindowId() const { return window_id_; }
  void SetWindowId(const base::UnguessableToken& id) { window_id_ = id; }

  const std::optional<network::mojom::blink::TrustTokenParams>&
  TrustTokenParams() const {
    return trust_token_params_;
  }
  void SetTrustTokenParams(
      std::optional<network::mojom::blink::TrustTokenParams>
          trust_token_params) {
    trust_token_params_ = std::move(trust_token_params);
  }

  network::mojom::AttributionReportingEligibility
  AttributionReportingEligibility() const {
    return attribution_reporting_eligibility_;
  }
  void SetAttributionReportingEligibility(
      network::mojom::AttributionReportingEligibility eligibility) {
    attribution_reporting_eligibility_ = eligibility;
  }

  network::mojom::AttributionSupport AttributionSupport() const {
    return attribution_reporting_support_;
  }
  void SetAttributionReportingSupport(
      network::mojom::AttributionSupport support) {
    attribution_reporting_support_ = support;
  }

  base::UnguessableToken ServiceWorkerRaceNetworkRequestToken() const {
    return service_worker_race_network_request_token_;
  }
  void SetServiceWorkerRaceNetworkRequestToken(
      const base::UnguessableToken& token) {
    service_worker_race_network_request_token_ = token;
  }

  void Trace(Visitor*) const;

 private:
  FetchRequestData* CloneExceptBody();

  AtomicString method_ = http_names::kGET;
  KURL url_;
  Member<FetchHeaderList> header_list_ =
      MakeGarbageCollected<FetchHeaderList>();
  // FIXME: Support m_skipServiceWorkerFlag;
  network::mojom::RequestDestination destination_ =
      network::mojom::RequestDestination::kEmpty;
  scoped_refptr<const SecurityOrigin> origin_;
  WTF::Vector<KURL> navigation_redirect_chain_;
  scoped_refptr<const SecurityOrigin> isolated_world_origin_;
  // FIXME: Support m_forceOriginHeaderFlag;
  AtomicString referrer_string_;
  network::mojom::ReferrerPolicy referrer_policy_ =
      network::mojom::ReferrerPolicy::kDefault;
  // FIXME: Support m_authenticationFlag;
  // FIXME: Support m_synchronousFlag;
  network::mojom::RequestMode mode_ = network::mojom::RequestMode::kNoCors;
  network::mojom::IPAddressSpace target_address_space_ =
      network::mojom::IPAddressSpace::kUnknown;
  network::mojom::CredentialsMode credentials_ =
      network::mojom::CredentialsMode::kOmit;
  // TODO(yiyix): |cache_mode_| is exposed but does not yet affect fetch
  // behavior. We must transfer the mode to the network layer and service
  // worker.
  mojom::blink::FetchCacheMode cache_mode_ =
      mojom::blink::FetchCacheMode::kDefault;
  network::mojom::RedirectMode redirect_ =
      network::mojom::RedirectMode::kFollow;
  mojom::blink::FetchPriorityHint fetch_priority_hint_ =
      mojom::blink::FetchPriorityHint::kAuto;
  std::optional<network::mojom::blink::TrustTokenParams> trust_token_params_;
  // FIXME: Support m_useURLCredentialsFlag;
  // FIXME: Support m_redirectCount;
  Member<BodyStreamBuffer> buffer_;
  uint64_t buffer_byte_length_ = 0;
  String mime_type_;
  String integrity_;
  ResourceLoadPriority priority_ = ResourceLoadPriority::kUnresolved;
  network::mojom::RequestDestination original_destination_ =
      network::mojom::RequestDestination::kEmpty;
  bool keepalive_ = false;
  bool browsing_topics_ = false;
  bool ad_auction_headers_ = false;
  bool shared_storage_writable_ = false;
  bool is_history_navigation_ = false;
  network::mojom::AttributionReportingEligibility
      attribution_reporting_eligibility_ =
          network::mojom::AttributionReportingEligibility::kUnset;
  network::mojom::AttributionSupport attribution_reporting_support_ =
      network::mojom::AttributionSupport::kUnset;
  // A specific factory that should be used for this request instead of whatever
  // the system would otherwise decide to use to load this request.
  // Currently used for blob: URLs, to ensure they can still be loaded even if
  // the URL got revoked after creating the request.
  HeapMojoRemote<network::mojom::blink::URLLoaderFactory> url_loader_factory_;
  base::UnguessableToken window_id_;
  Member<ExecutionContext> execution_context_;

  // A token set only when the fetch request is initiated with ServiceWorker
  // RaceNetworkRequest(crbug.com/1420517). When the request is cloned, this
  // member shouldn't be copied to the new request.
  base::UnguessableToken service_worker_race_network_request_token_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_REQUEST_DATA_H_
