/*
 * Copyright (C) 2012 Google, Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_PARAMETERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_PARAMETERS_H_

#include "base/memory/stack_allocated.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/script/script_type.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/script/script_type.mojom-shared.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/platform/loader/fetch/cross_origin_attribute_value.h"
#include "third_party/blink/renderer/platform/loader/fetch/integrity_metadata.h"
#include "third_party/blink/renderer/platform/loader/fetch/render_blocking_behavior.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/loader/fetch/text_resource_decoder_options.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"

namespace blink {

class SecurityOrigin;

// A FetchParameters is a "parameter object" for
// ResourceFetcher::requestResource to avoid the method having too many
// arguments.
//
// This class is thread-bound. Do not copy/pass an instance across threads.
class PLATFORM_EXPORT FetchParameters {
  DISALLOW_NEW();

 public:
  enum DeferOption { kNoDefer, kLazyLoad, kIdleLoad };
  enum class SpeculativePreloadType {
    kNotSpeculative,
    kInDocument,  // The request was discovered in the main document
    kInserted     // The request was discovered in a document.write()
  };
  enum class ImageRequestBehavior {
    kNone = 0,         // No optimization.
    kDeferImageLoad,   // Defer loading the image from network. Full image
                       // might still load if the request is already-loaded or
                       // in memory cache.
    kNonBlockingImage  // The image load may continue, but must be placed in
                       // ResourceFetcher::non_blocking_loaders_.
  };

  static FetchParameters CreateForTest(ResourceRequest);

  FetchParameters(ResourceRequest, ResourceLoaderOptions);
  FetchParameters(const FetchParameters&) = delete;
  FetchParameters& operator=(const FetchParameters&) = delete;
  FetchParameters(FetchParameters&&);
  ~FetchParameters();

  ResourceRequest& MutableResourceRequest() { return resource_request_; }
  const ResourceRequest& GetResourceRequest() const {
    return resource_request_;
  }
  const KURL& Url() const { return resource_request_.Url(); }

  void SetRequestContext(mojom::blink::RequestContextType context) {
    resource_request_.SetRequestContext(context);
  }

  void SetRequestDestination(network::mojom::RequestDestination destination) {
    resource_request_.SetRequestDestination(destination);
  }

  void SetFetchPriorityHint(
      mojom::blink::FetchPriorityHint fetch_priority_hint) {
    resource_request_.SetFetchPriorityHint(fetch_priority_hint);
  }

  const TextResourceDecoderOptions& DecoderOptions() const {
    return decoder_options_;
  }
  void SetDecoderOptions(const TextResourceDecoderOptions& decoder_options) {
    decoder_options_ = decoder_options;
  }
  void OverrideContentType(
      TextResourceDecoderOptions::ContentType content_type) {
    decoder_options_.OverrideContentType(content_type);
  }
  void SetCharset(const WTF::TextEncoding& charset) {
    SetDecoderOptions(TextResourceDecoderOptions(
        TextResourceDecoderOptions::kPlainTextContent, charset));
  }

  ResourceLoaderOptions& MutableOptions() { return options_; }
  const ResourceLoaderOptions& Options() const { return options_; }

  DeferOption Defer() const { return defer_; }
  void SetDefer(DeferOption defer) { defer_ = defer; }

  std::optional<float> GetResourceWidth() const { return resource_width_; }
  void SetResourceWidth(const std::optional<float> resource_width);

  std::optional<float> GetResourceHeight() const { return resource_height_; }
  void SetResourceHeight(const std::optional<float> resource_height);

  bool IsSpeculativePreload() const {
    return speculative_preload_type_ != SpeculativePreloadType::kNotSpeculative;
  }
  SpeculativePreloadType GetSpeculativePreloadType() const {
    return speculative_preload_type_;
  }
  void SetSpeculativePreloadType(SpeculativePreloadType);

  bool IsLinkPreload() const { return options_.initiator_info.is_link_preload; }
  void SetLinkPreload(bool is_link_preload) {
    options_.initiator_info.is_link_preload = is_link_preload;
  }

  bool IsStaleRevalidation() const { return is_stale_revalidation_; }
  void SetStaleRevalidation(bool is_stale_revalidation) {
    is_stale_revalidation_ = is_stale_revalidation;
  }

  void SetContentSecurityCheck(
      network::mojom::CSPDisposition content_security_policy_option) {
    options_.content_security_policy_option = content_security_policy_option;
  }
  // Configures the request to use the "cors" mode and the credentials mode
  // specified by the crossOrigin attribute.
  void SetCrossOriginAccessControl(const SecurityOrigin*,
                                   CrossOriginAttributeValue);
  // Configures the request to use the "cors" mode and the specified
  // credentials mode.
  void SetCrossOriginAccessControl(const SecurityOrigin*,
                                   network::mojom::CredentialsMode);
  const IntegrityMetadataSet IntegrityMetadata() const {
    return options_.integrity_metadata;
  }
  void SetIntegrityMetadata(const IntegrityMetadataSet& metadata) {
    options_.integrity_metadata = metadata;
  }

  String ContentSecurityPolicyNonce() const {
    return options_.content_security_policy_nonce;
  }
  void SetContentSecurityPolicyNonce(const String& nonce) {
    options_.content_security_policy_nonce = nonce;
  }

  void SetParserDisposition(ParserDisposition parser_disposition) {
    options_.parser_disposition = parser_disposition;
  }

  void SetCacheAwareLoadingEnabled(
      CacheAwareLoadingEnabled cache_aware_loading_enabled) {
    options_.cache_aware_loading_enabled = cache_aware_loading_enabled;
  }

  void MakeSynchronous();

  ImageRequestBehavior GetImageRequestBehavior() const {
    return image_request_behavior_;
  }

  // Configures the request to defer the image and set the lazy image load bit.
  void SetLazyImageDeferred();
  void SetLazyImageNonBlocking();

  mojom::blink::ScriptType GetScriptType() const { return script_type_; }

  void SetModuleScript();

  // See documentation in blink::ResourceRequest.
  bool IsFromOriginDirtyStyleSheet() const {
    return is_from_origin_dirty_style_sheet_;
  }
  void SetFromOriginDirtyStyleSheet(bool dirty) {
    is_from_origin_dirty_style_sheet_ = dirty;
  }

  RenderBlockingBehavior GetRenderBlockingBehavior() const {
    return render_blocking_behavior_;
  }

  void SetRenderBlockingBehavior(
      RenderBlockingBehavior render_blocking_behavior) {
    render_blocking_behavior_ = render_blocking_behavior;
  }

  void SetIsPotentiallyLCPElement(bool flag) {
    is_potentially_lcp_element_ = flag;
  }

  void SetIsPotentiallyLCPInfluencer(bool flag) {
    is_potentially_lcp_influencer_ = flag;
  }

  bool IsPotentiallyLCPElement() const { return is_potentially_lcp_element_; }

  bool IsPotentiallyLCPInfluencer() const {
    return is_potentially_lcp_influencer_;
  }

  void Trace(Visitor* visitor) const { visitor->Trace(options_); }

 private:
  ResourceRequest resource_request_;
  // |decoder_options_|'s ContentType is set to |kPlainTextContent| in
  // FetchParameters but is later overridden by ResourceFactory::ContentType()
  // in ResourceFetcher::PrepareRequest() before actual use.
  TextResourceDecoderOptions decoder_options_;
  ResourceLoaderOptions options_;
  SpeculativePreloadType speculative_preload_type_ =
      SpeculativePreloadType::kNotSpeculative;
  DeferOption defer_ = DeferOption::kNoDefer;
  std::optional<float> resource_width_;
  std::optional<float> resource_height_;
  ImageRequestBehavior image_request_behavior_ = ImageRequestBehavior::kNone;
  mojom::blink::ScriptType script_type_ = mojom::blink::ScriptType::kClassic;
  bool is_stale_revalidation_ = false;
  bool is_from_origin_dirty_style_sheet_ = false;
  RenderBlockingBehavior render_blocking_behavior_ =
      RenderBlockingBehavior::kUnset;
  bool is_potentially_lcp_element_ = false;
  bool is_potentially_lcp_influencer_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_PARAMETERS_H_
