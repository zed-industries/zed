// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_PRELOAD_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_PRELOAD_REQUEST_H_

#include <memory>

#include "base/memory/ptr_util.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/script/script_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/script.h"
#include "third_party/blink/renderer/platform/loader/fetch/cross_origin_attribute_value.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_parameters.h"
#include "third_party/blink/renderer/platform/loader/fetch/integrity_metadata.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher.h"
#include "third_party/blink/renderer/platform/weborigin/security_policy.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"

namespace blink {

class Document;
class Resource;
enum class ResourceType : uint8_t;

class CORE_EXPORT PreloadRequest {
  USING_FAST_MALLOC(PreloadRequest);

 public:
  class CORE_EXPORT ExclusionInfo : public RefCounted<ExclusionInfo> {
    USING_FAST_MALLOC(ExclusionInfo);

   public:
    ExclusionInfo(const KURL& document_url,
                  HashSet<KURL> scopes,
                  HashSet<KURL> resources);
    virtual ~ExclusionInfo();

    // Disallow copy and assign.
    ExclusionInfo(const ExclusionInfo&) = delete;
    ExclusionInfo& operator=(const ExclusionInfo&) = delete;

    const KURL& document_url() const { return document_url_; }
    const HashSet<KURL>& scopes() const { return scopes_; }
    const HashSet<KURL>& resources() const { return resources_; }

    bool ShouldExclude(const KURL& base_url, const String& resource_url) const;

   private:
    const KURL document_url_;
    const HashSet<KURL> scopes_;
    const HashSet<KURL> resources_;
  };

  enum RequestType {
    kRequestTypePreload,
    kRequestTypePreconnect,
    kRequestTypeLinkRelPreload
  };

  static std::unique_ptr<PreloadRequest> CreateIfNeeded(
      const String& initiator_name,
      const String& resource_url,
      const KURL& base_url,
      ResourceType resource_type,
      const network::mojom::ReferrerPolicy referrer_policy,
      ResourceFetcher::IsImageSet is_image_set,
      const ExclusionInfo* exclusion_info,
      const std::optional<float> resource_width = std::nullopt,
      const std::optional<float> resource_height = std::nullopt,
      RequestType request_type = kRequestTypePreload);

  Resource* Start(Document*);

  void SetInitiatorPosition(const TextPosition& position) {
    initiator_position_ = position;
  }

  void SetDefer(FetchParameters::DeferOption defer) { defer_ = defer; }
  FetchParameters::DeferOption DeferOption() const { return defer_; }

  void SetCharset(const String& charset) { charset_ = charset; }
  void SetCrossOrigin(CrossOriginAttributeValue cross_origin) {
    cross_origin_ = cross_origin;
  }
  CrossOriginAttributeValue CrossOrigin() const { return cross_origin_; }

  void SetFetchPriorityHint(
      mojom::blink::FetchPriorityHint fetch_priority_hint) {
    fetch_priority_hint_ = fetch_priority_hint;
  }
  mojom::blink::FetchPriorityHint FetchPriorityHint() const {
    return fetch_priority_hint_;
  }

  void SetNonce(const String& nonce) { nonce_ = nonce; }
  const String& Nonce() const { return nonce_; }

  ResourceType GetResourceType() const { return resource_type_; }

  const String& ResourceURL() const { return resource_url_; }
  const KURL& BaseURL() const { return base_url_; }
  bool IsPreconnect() const { return request_type_ == kRequestTypePreconnect; }
  network::mojom::ReferrerPolicy GetReferrerPolicy() const {
    return referrer_policy_;
  }

  void SetScriptType(mojom::blink::ScriptType script_type) {
    script_type_ = script_type;
  }

  // Only scripts and css stylesheets need to have integrity set on preloads.
  // This is because neither resource keeps raw data around to redo an
  // integrity check. A resource in memory cache needs integrity
  // data cached to match an outgoing request.
  void SetIntegrityMetadata(const IntegrityMetadataSet& metadata_set) {
    integrity_metadata_ = metadata_set;
  }
  const IntegrityMetadataSet& IntegrityMetadataForTestingOnly() const {
    return integrity_metadata_;
  }
  void SetFromInsertionScanner(const bool from_insertion_scanner) {
    from_insertion_scanner_ = from_insertion_scanner;
  }

  bool IsImageSetForTestingOnly() const {
    return is_image_set_ == ResourceFetcher::kImageIsImageSet;
  }

  void SetRenderBlockingBehavior(
      RenderBlockingBehavior render_blocking_behavior) {
    render_blocking_behavior_ = render_blocking_behavior;
  }

  RenderBlockingBehavior GetRenderBlockingBehavior() {
    return render_blocking_behavior_;
  }

  bool IsAttributionReportingEligibleImgOrScript() const {
    return is_attribution_reporting_eligible_img_or_script_;
  }

  void SetAttributionReportingEligibleImgOrScript(bool eligible) {
    is_attribution_reporting_eligible_img_or_script_ = eligible;
  }

  void SetIsPotentiallyLCPElement(bool flag) {
    is_potentially_lcp_element_ = flag;
  }

  void SetIsPotentiallyLCPInfluencer(bool flag) {
    is_potentially_lcp_influencer_ = flag;
  }

  void SetSharedStorageWritableOptedIn(bool opted_in) {
    shared_storage_writable_opted_in_ = opted_in;
  }

  // Set whether the preload request is eligible for the Browsing Topics API.
  //
  // See https://github.com/patcg-individual-drafts/topics/blob/main/README.md
  // for the latest version of the Topics API explainer.
  void SetBrowsingTopicsEligible(bool flag) {
    browsing_topics_eligible_ = flag;
  }

  bool IsPotentiallyLCPElement() const { return is_potentially_lcp_element_; }

  bool IsPotentiallyLCPInfluencer() const {
    return is_potentially_lcp_influencer_;
  }

  std::optional<float> GetResourceWidth() const { return resource_width_; }
  std::optional<float> GetResourceHeight() const { return resource_height_; }

 private:
  PreloadRequest(const String& initiator_name,
                 const String& resource_url,
                 const KURL& base_url,
                 ResourceType resource_type,
                 const std::optional<float> resource_width,
                 const std::optional<float> resource_height,
                 RequestType request_type,
                 const network::mojom::ReferrerPolicy referrer_policy,
                 ResourceFetcher::IsImageSet is_image_set)
      : initiator_name_(initiator_name),
        resource_url_(resource_url),
        base_url_(base_url),
        resource_type_(resource_type),
        resource_width_(resource_width),
        resource_height_(resource_height),
        request_type_(request_type),
        referrer_policy_(referrer_policy),
        is_image_set_(is_image_set) {}

  KURL CompleteURL(Document*);

  const String initiator_name_;
  TextPosition initiator_position_{TextPosition::MinimumPosition()};
  const String resource_url_;
  const KURL base_url_;
  String charset_;
  const ResourceType resource_type_;
  mojom::blink::ScriptType script_type_ = mojom::blink::ScriptType::kClassic;
  CrossOriginAttributeValue cross_origin_ = kCrossOriginAttributeNotSet;
  mojom::blink::FetchPriorityHint fetch_priority_hint_ =
      mojom::blink::FetchPriorityHint::kAuto;
  String nonce_;
  FetchParameters::DeferOption defer_ = FetchParameters::kNoDefer;
  const std::optional<float> resource_width_;
  const std::optional<float> resource_height_;
  const RequestType request_type_;
  const network::mojom::ReferrerPolicy referrer_policy_;
  IntegrityMetadataSet integrity_metadata_;
  RenderBlockingBehavior render_blocking_behavior_ =
      RenderBlockingBehavior::kUnset;
  bool from_insertion_scanner_ = false;
  const ResourceFetcher::IsImageSet is_image_set_;
  bool is_lazy_load_image_enabled_ = false;
  base::TimeTicks creation_time_ = base::TimeTicks::Now();
  bool is_attribution_reporting_eligible_img_or_script_ = false;
  bool is_potentially_lcp_element_ = false;
  bool is_potentially_lcp_influencer_ = false;
  bool shared_storage_writable_opted_in_ = false;
  bool browsing_topics_eligible_ = false;
};

typedef Vector<std::unique_ptr<PreloadRequest>> PreloadRequestStream;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_PRELOAD_REQUEST_H_
