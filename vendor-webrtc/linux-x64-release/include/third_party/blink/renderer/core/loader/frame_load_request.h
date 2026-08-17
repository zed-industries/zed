/*
 * Copyright (C) 2003, 2006, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_LOAD_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_LOAD_REQUEST_H_

#include <optional>

#include "base/memory/ref_counted.h"
#include "base/time/time.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink.h"
#include "third_party/blink/public/common/navigation/impression.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/policy_container.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/triggering_event_info.mojom-blink.h"
#include "third_party/blink/public/mojom/loader/request_context_frame_type.mojom-blink.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-blink.h"
#include "third_party/blink/public/web/web_picture_in_picture_window_options.h"
#include "third_party/blink/public/web/web_window_features.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/frame_types.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/core/loader/navigation_policy.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"

namespace blink {

class Element;
class HTMLFormElement;
class LocalDOMWindow;
class KURL;

struct CORE_EXPORT FrameLoadRequest {
  STACK_ALLOCATED();

 public:
  FrameLoadRequest(LocalDOMWindow* origin_window, const ResourceRequest&);
  FrameLoadRequest(LocalDOMWindow* origin_window, const ResourceRequestHead&);
  FrameLoadRequest(const FrameLoadRequest&) = delete;
  FrameLoadRequest& operator=(const FrameLoadRequest&) = delete;

  LocalDOMWindow* GetOriginWindow() const { return origin_window_; }

  mojom::RequestContextFrameType GetFrameType() const { return frame_type_; }
  void SetFrameType(mojom::RequestContextFrameType frame_type) {
    frame_type_ = frame_type;
    ResolveBlobURLIfNeeded();
  }

  ResourceRequest& GetResourceRequest() { return resource_request_; }
  const ResourceRequest& GetResourceRequest() const {
    return resource_request_;
  }

  void SetClientNavigationReason(ClientNavigationReason reason) {
    client_navigation_reason_ = reason;
  }

  ClientNavigationReason GetClientNavigationReason() const {
    return client_navigation_reason_;
  }

  void SetIsContainerInitiated(bool value) { is_container_initiated_ = value; }

  bool IsContainerInitiated() const { return is_container_initiated_; }

  NavigationPolicy GetNavigationPolicy() const { return navigation_policy_; }
  void SetNavigationPolicy(NavigationPolicy navigation_policy) {
    navigation_policy_ = navigation_policy;
  }

  mojom::blink::TriggeringEventInfo GetTriggeringEventInfo() const {
    return triggering_event_info_;
  }
  void SetTriggeringEventInfo(mojom::blink::TriggeringEventInfo info) {
    DCHECK(info != mojom::blink::TriggeringEventInfo::kUnknown);
    triggering_event_info_ = info;
  }

  mojo::PendingRemote<mojom::blink::NavigationStateKeepAliveHandle>
  TakeInitiatorNavigationStateKeepAliveHandle() {
    return std::move(initiator_navigation_state_keep_alive_handle_);
  }
  void SetInitiatorNavigationStateKeepAliveHandle(
      mojo::PendingRemote<mojom::blink::NavigationStateKeepAliveHandle>
          handle) {
    initiator_navigation_state_keep_alive_handle_ = std::move(handle);
  }

  std::unique_ptr<SourceLocation> TakeSourceLocation() {
    return std::move(source_location_);
  }
  void SetSourceLocation(std::unique_ptr<SourceLocation> source_location) {
    source_location_ = std::move(source_location);
  }

  HTMLFormElement* Form() const;
  Element* GetSourceElement() const { return source_element_; }
  void SetSourceElement(Element* element) { source_element_ = element; }

  ShouldSendReferrer GetShouldSendReferrer() const {
    return should_send_referrer_;
  }

  const AtomicString& HrefTranslate() const { return href_translate_; }
  void SetHrefTranslate(const AtomicString& translate) {
    href_translate_ = translate;
  }

  // The javascript world in which this request initiated.
  const DOMWrapperWorld* JavascriptWorld() const { return world_; }

  // The BlobURLToken that should be used when fetching the resource. This
  // is needed for blob URLs, because the blob URL might be revoked before the
  // actual fetch happens, which would result in incorrect failures to fetch.
  // The token lets the browser process securely resolves the blob URL even
  // after the url has been revoked. `ResolveBlobURLIfNeeded()` must have been
  // called in `SetFrameType()` for the BlobURLToken to be available.
  mojo::PendingRemote<mojom::blink::BlobURLToken> GetBlobURLToken() const {
    if (!blob_url_token_)
      return mojo::NullRemote();
    mojo::PendingRemote<mojom::blink::BlobURLToken> result;
    blob_url_token_->data->Clone(result.InitWithNewPipeAndPassReceiver());
    return result;
  }

  void SetInputStartTime(base::TimeTicks input_start_time) {
    input_start_time_ = input_start_time;
  }

  base::TimeTicks GetInputStartTime() const { return input_start_time_; }

  base::TimeTicks GetCreationTime() const { return creation_time_; }

  const WebWindowFeatures& GetWindowFeatures() const {
    return window_features_;
  }
  void SetFeaturesForWindowOpen(const WebWindowFeatures& features) {
    window_features_ = features;
  }

  const std::optional<WebPictureInPictureWindowOptions>&
  GetPictureInPictureWindowOptions() const {
    return picture_in_picture_window_options_;
  }
  void SetPictureInPictureWindowOptions(
      const WebPictureInPictureWindowOptions& options) {
    picture_in_picture_window_options_ = options;
  }

  void SetNoOpener() { window_features_.noopener = true; }
  void SetExplicitOpener() { window_features_.explicit_opener = true; }
  void SetNoReferrer() {
    should_send_referrer_ = kNeverSendReferrer;
    resource_request_.SetReferrerString(Referrer::NoReferrer());
    resource_request_.SetReferrerPolicy(network::mojom::ReferrerPolicy::kNever);
    resource_request_.ClearHTTPOrigin();
  }

  // Impressions are set when a FrameLoadRequest is created for a click on an
  // anchor tag that has conversion measurement attributes.
  void SetImpression(const std::optional<Impression>& impression) {
    impression_ = impression;
  }

  const std::optional<blink::Impression>& Impression() const {
    return impression_;
  }

  bool CanDisplay(const KURL&) const;

  void SetInitiatorFrameToken(const LocalFrameToken& token) {
    initiator_frame_token_ = token;
  }
  const LocalFrameToken* GetInitiatorFrameToken() const;

  bool IsUnfencedTopNavigation() const { return is_unfenced_top_navigation_; }
  void SetIsUnfencedTopNavigation(bool is_unfenced_top_navigation) {
    is_unfenced_top_navigation_ = is_unfenced_top_navigation;
  }

  const KURL& GetRequestorBaseURL() const { return requestor_base_url_; }

  void SetForceHistoryPush() {
    force_history_push_ = mojom::blink::ForceHistoryPush::kYes;
  }
  mojom::blink::ForceHistoryPush ForceHistoryPush() const {
    return force_history_push_;
  }

  // This function is meant to be used in HTML/SVG attributes where dangling
  // markup injection occurs. See https://github.com/whatwg/html/pull/9309.
  const AtomicString& CleanNavigationTarget(const AtomicString& target) const;

 private:
  LocalDOMWindow* origin_window_;
  ResourceRequest resource_request_;
  AtomicString href_translate_;
  ClientNavigationReason client_navigation_reason_ =
      ClientNavigationReason::kNone;
  NavigationPolicy navigation_policy_ = kNavigationPolicyCurrentTab;
  mojom::blink::TriggeringEventInfo triggering_event_info_ =
      mojom::blink::TriggeringEventInfo::kNotFromEvent;
  Element* source_element_ = nullptr;
  ShouldSendReferrer should_send_referrer_;
  const DOMWrapperWorld* world_ = nullptr;
  scoped_refptr<base::RefCountedData<mojo::Remote<mojom::blink::BlobURLToken>>>
      blob_url_token_;
  base::TimeTicks input_start_time_;
  base::TimeTicks creation_time_;
  mojom::RequestContextFrameType frame_type_ =
      mojom::RequestContextFrameType::kNone;
  WebWindowFeatures window_features_;
  std::optional<WebPictureInPictureWindowOptions>
      picture_in_picture_window_options_;
  std::optional<blink::Impression> impression_;
  std::optional<LocalFrameToken> initiator_frame_token_;
  mojo::PendingRemote<mojom::blink::NavigationStateKeepAliveHandle>
      initiator_navigation_state_keep_alive_handle_;
  std::unique_ptr<SourceLocation> source_location_;
  KURL requestor_base_url_;

  // This is only used for navigations originating in MPArch fenced frames
  // targeting the outermost frame, which is not visible to the renderer
  // process as a remote frame.
  // TODO(crbug.com/1315802): Refactor _unfencedTop handling.
  bool is_unfenced_top_navigation_ = false;

  mojom::blink::ForceHistoryPush force_history_push_ =
      mojom::blink::ForceHistoryPush::kNo;

  // Only container-initiated navigations (e.g. iframe change src) report a
  // resource timing entry to the parent.
  bool is_container_initiated_ = false;

  // Resolves a Blob URL into a BlobURLToken if the URL is a blob URL, and
  // otherwise has no effect. It is called after the FrameType has been set.
  void ResolveBlobURLIfNeeded();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_LOAD_REQUEST_H_
