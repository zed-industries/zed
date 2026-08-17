/*
 * Copyright (C) 2007, 2008, 2009, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_HTML_VIDEO_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_HTML_VIDEO_ELEMENT_H_

#include "third_party/blink/public/platform/web_media_player.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_image_source.h"
#include "third_party/blink/renderer/core/html/html_image_loader.h"
#include "third_party/blink/renderer/core/html/media/html_media_element.h"
#include "third_party/blink/renderer/core/imagebitmap/image_bitmap_source.h"
#include "third_party/blink/renderer/platform/graphics/canvas_resource_provider.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class ImageBitmapOptions;
class IntersectionObserverEntry;
class MediaCustomControlsFullscreenDetector;
class MediaVideoVisibilityTracker;
class MediaRemotingInterstitial;
class PictureInPictureInterstitial;
class StaticBitmapImage;
class VideoWakeLock;

class CORE_EXPORT HTMLVideoElement final
    : public HTMLMediaElement,
      public CanvasImageSource,
      public ImageBitmapSource,
      public Supplementable<HTMLVideoElement> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const int kNoAlreadyUploadedFrame = -1;

  explicit HTMLVideoElement(Document&);
  void Trace(Visitor*) const override;

  bool HasPendingActivity() const final;

  // Node override.
  Node::InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  unsigned videoWidth() const;
  unsigned videoHeight() const;

  gfx::Size videoVisibleSize() const;

  // Fullscreen
  void EnterFullscreen();
  void DidEnterFullscreen();
  void DidExitFullscreen();

  // Statistics
  unsigned webkitDecodedFrameCount() const;
  unsigned webkitDroppedFrameCount() const;

  // Used by canvas to gain raw pixel access
  //
  // |paint_flags| is optional. If unspecified, its blend mode defaults to kSrc.
  void PaintCurrentFrame(cc::PaintCanvas*,
                         const gfx::Rect&,
                         const cc::PaintFlags* paint_flags) const;

  bool HasAvailableVideoFrame() const;
  bool HasReadableVideoFrame() const;

  void OnFirstFrame(base::TimeTicks frame_time,
                    size_t bytes_to_first_frame) final;

  KURL PosterImageURL() const override;

  // Returns whether the current poster image URL is the default for the
  // document.
  // TODO(1190335): Remove this once default poster image URL is removed.
  bool IsDefaultPosterImageURL() const;

  // Helper for GetSourceImageForCanvas() and other external callers who want a
  // StaticBitmapImage of the current VideoFrame. If `allow_accelerated_images`
  // is set to false a software backed CanvasResourceProvider will be used to
  // produce the StaticBitmapImage. If `size` is specified, the image will be
  // scaled to it, otherwise the image will be in its natural size. If
  // `reinterpret_as_srgb` is true, then reinterpret the video as thought it
  // is in sRGB color space.
  scoped_refptr<StaticBitmapImage> CreateStaticBitmapImage(
      bool allow_accelerated_images = true,
      std::optional<gfx::Size> size = std::nullopt,
      bool reinterpret_as_srgb = false);

  // CanvasImageSource implementation
  scoped_refptr<Image> GetSourceImageForCanvas(FlushReason,
                                               SourceImageStatus*,
                                               const gfx::SizeF&) override;
  bool IsVideoElement() const override { return true; }
  bool WouldTaintOrigin() const override;
  gfx::SizeF ElementSize(const gfx::SizeF&,
                         const RespectImageOrientationEnum) const override;
  const KURL& SourceURL() const override { return currentSrc(); }
  bool IsHTMLVideoElement() const override { return true; }
  // Video elements currently always go through RAM when used as a canvas image
  // source.
  bool IsAccelerated() const override { return false; }

  // ImageBitmapSource implementation
  ImageBitmapSourceStatus CheckUsability() const override;
  ScriptPromise<ImageBitmap> CreateImageBitmap(
      ScriptState*,
      std::optional<gfx::Rect> crop_rect,
      const ImageBitmapOptions*,
      ExceptionState&) override;

  // WebMediaPlayerClient implementation.
  void OnRequestVideoFrameCallback() final;
  void SetCcLayer(cc::Layer*) final;
  void StyleDidChange(const ComputedStyle* old_style,
                      const ComputedStyle& new_style);

  bool IsPersistent() const;

  bool IsRemotingInterstitialVisible() const;

  void MediaRemotingStarted(const WebString& remote_device_friendly_name) final;
  bool SupportsPictureInPicture() const final;
  void MediaRemotingStopped(int error_code) final;
  WebMediaPlayer::DisplayType GetDisplayType() const final;
  bool IsInAutoPIP() const final;
  void DidPlayerMediaPositionStateChange(double playback_rate,
                                         base::TimeDelta duration,
                                         base::TimeDelta position,
                                         bool end_of_media) final;
  void OnPictureInPictureStateChange() final;
  void SetPersistentState(bool persistent) final;

  // Used by the PictureInPictureController as callback when the video element
  // enters or exits Picture-in-Picture state.
  void OnEnteredPictureInPicture();
  void OnExitedPictureInPicture();

  void SetIsEffectivelyFullscreen(blink::WebFullscreenVideoStatus);
  void SetIsDominantVisibleContent(bool is_dominant);

  bool IsRichlyEditableForAccessibility() const override { return false; }

  void RecordVideoOcclusionState(std::string_view occlusion_state) const final;

  VideoWakeLock* wake_lock_for_tests() const { return wake_lock_.Get(); }

  MediaVideoVisibilityTracker* visibility_tracker_for_tests() const {
    return visibility_tracker_.Get();
  }

 protected:
  // EventTarget overrides.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

  void OnWebMediaPlayerCreated() final;
  void OnWebMediaPlayerCleared() final;

  void AttributeChanged(const AttributeModificationParams& params) override;

 private:
  friend class MediaCustomControlsFullscreenDetectorTest;
  friend class HTMLMediaElementEventListenersTest;
  friend class HTMLVideoElementPersistentTest;
  friend class VideoFillingViewportTest;
  friend class HTMLVideoElementTest;

  // ExecutionContextLifecycleStateObserver functions.
  void ContextDestroyed() final;

  bool LayoutObjectIsNeeded(const DisplayStyle&) const override;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  void AttachLayoutTree(AttachContext&) override;
  void UpdatePosterImage();
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;
  bool IsURLAttribute(const Attribute&) const override;
  const AtomicString ImageSourceURL() const override;

  void OnPlay() final;
  void OnLoadStarted() final;
  void OnLoadFinished() final;

  // Wrapper for the |MediaVideoVisibilityTracker|
  // |UpdateVisibilityTrackerState| method. |UpdateVisibilityTrackerState| is
  // called only if the |visibility_tracker_| exists.
  void UpdateVideoVisibilityTracker() final;

  // Video-specific overrides for part of the media::mojom::MediaPlayer
  // interface, fully implemented in the parent class HTMLMediaElement.
  void RequestEnterPictureInPicture() final;
  void RequestMediaRemoting() final;
  void RequestVisibility(
      HTMLMediaElement::RequestVisibilityCallback request_visibility_cb) final;

  void DidMoveToNewDocument(Document& old_document) override;

  void UpdatePictureInPictureAvailability();

  void OnIntersectionChangedForLazyLoad(
      const HeapVector<Member<IntersectionObserverEntry>>& entries);

  void SetPersistentStateInternal(bool persistent);

  // Creates a |MediaVideoVisibilityTracker| if one does not already exist.
  void CreateVisibilityTrackerIfNeeded();

  void ReportVisibility(bool meets_visibility_threshold);

  void ResetCache(TimerBase*);

  Member<HTMLImageLoader> image_loader_;
  Member<MediaCustomControlsFullscreenDetector>
      custom_controls_fullscreen_detector_;
  Member<VideoWakeLock> wake_lock_;

  Member<MediaRemotingInterstitial> remoting_interstitial_;
  Member<PictureInPictureInterstitial> picture_in_picture_interstitial_;

  AtomicString default_poster_url_;

  // Tracks visibility of playing videos, taking into account both: viewport
  // intersection and occluding elements.
  Member<MediaVideoVisibilityTracker> visibility_tracker_;

  // Represents whether the video is 'persistent'. It is used for videos with
  // custom controls that are in auto-pip (Android). This boolean is used by a
  // CSS rule.
  bool is_persistent_ : 1;

  // Whether the video is currently in auto-pip (Android). It is not similar to
  // a video being in regular Picture-in-Picture mode.
  bool is_auto_picture_in_picture_ : 1;

  // Whether this element is in overlay fullscreen mode.
  bool in_overlay_fullscreen_video_ : 1;

  // Whether the video element should be considered as fullscreen with regards
  // to display type and other UI features. This does not mean the DOM element
  // is fullscreen.
  bool is_effectively_fullscreen_ : 1;

  bool video_has_played_ : 1;

  // True, if the video element occupies most of the viewport.
  bool mostly_filling_viewport_ : 1;

  // Used to fulfill blink::Image requests (CreateImage(),
  // GetSourceImageForCanvas(), etc). Created on demand.
  std::unique_ptr<CanvasResourceProvider> resource_provider_;
  bool allow_accelerated_images_ = true;
  HeapTaskRunnerTimer<HTMLVideoElement> cache_deleting_timer_;

  // Paint flags set based on CSS properties, which must be propagated to the
  // cc::Layer.
  cc::PaintFlags::FilterQuality filter_quality_ =
      cc::PaintFlags::FilterQuality::kLow;
  cc::PaintFlags::DynamicRangeLimitMixture dynamic_range_limit_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_HTML_VIDEO_ELEMENT_H_
