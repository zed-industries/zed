/*
 * Copyright (C) 2006 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2009 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_SVG_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_SVG_IMAGE_H_

#include <optional>

#include "base/gtest_prod_util.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/scheduler/public/agent_group_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "ui/gfx/geometry/size.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Document;
class Element;
class IsolatedSVGDocumentHost;
class LayoutSVGRoot;
class LocalFrame;
class Node;
class Page;
class SVGImageChromeClient;
class SVGImageForContainer;
class SVGSVGElement;
class SVGViewSpec;
struct NaturalSizingInfo;

// A collection of "viewport defining" parameters for an SVGImage.
class SVGImageViewInfo final : public GarbageCollected<SVGImageViewInfo> {
 public:
  SVGImageViewInfo(const SVGViewSpec* view_spec, Element* target)
      : view_spec_(view_spec), target_(target) {}

  const SVGViewSpec* ViewSpec() const { return view_spec_; }
  Element* Target() const { return target_; }

  void Trace(Visitor*) const;

 private:
  Member<const SVGViewSpec> view_spec_;
  Member<Element> target_;
};

// SVGImage does not use Skia to draw images (as BitmapImage does) but instead
// handles drawing itself. Internally, SVGImage creates a
// IsolatedSVGDocumentHost and reuses the existing paint code in Blink to draw
// the image. Because a single SVGImage can be referenced by multiple
// containers (see: SVGImageForContainer.h), each call to SVGImage::draw() may
// require (re-)laying out the inner SVGDocument.
//
// Using Page was an architectural hack and has surprising side-effects. Ideally
// SVGImage would use a lighter container around an SVGDocument that does not
// have the full Page machinery but still has the sandboxing security guarantees
// needed by SVGImage.
class CORE_EXPORT SVGImage final : public Image {
 public:
  static scoped_refptr<SVGImage> Create(ImageObserver* observer,
                                        bool is_multipart = false) {
    return base::AdoptRef(new SVGImage(observer, is_multipart));
  }

  static bool IsInSVGImage(const Node*);

  bool IsSVGImage() const override { return true; }
  gfx::Size SizeWithConfig(SizeConfig) const override;

  void CheckLoaded() const;
  bool CurrentFrameHasSingleSecurityOrigin() const override;

  void StartAnimation() override;
  void ResetAnimation() override;
  void RestoreAnimation();

  // Does the SVG image/document contain any animations?
  bool MaybeAnimated() override;

  // Advances an animated image. This will trigger an animation update for CSS
  // and advance the SMIL timeline by one frame.
  void AdvanceAnimationForTesting() override;
  SVGImageChromeClient& ChromeClientForTesting();

  static gfx::PointF OffsetForCurrentFrame(const gfx::RectF& dst_rect,
                                           const gfx::RectF& src_rect);

  // Service CSS and SMIL animations.
  void ServiceAnimations(base::TimeTicks monotonic_animation_start_time);

  // Update use counters for the given document after an image finishes loading
  // in that document.
  void UpdateUseCountersAfterLoad(const Document&) const;

  void MaybeRecordSvgImageProcessingTime(const Document&);

  PaintImage PaintImageForCurrentFrame() override;

  void SetPreferredColorScheme(
      mojom::blink::PreferredColorScheme preferred_color_scheme);

  // Introspective service hatch for mask-image. Don't abuse for anything else.
  Element* GetResourceElement(const AtomicString& id) const;

 protected:
  // Whether or not size is available yet.
  bool IsSizeAvailable() override;

 private:
  // Accesses |document_host_|.
  friend class SVGImageChromeClient;
  // Forwards calls to the various *ForContainer methods and other parts of
  // the the Image interface.
  friend class SVGImageForContainer;
  // Forwards calls to the sizing methods.
  friend class SVGImageView;

  SVGImage(ImageObserver*, bool is_multipart);
  ~SVGImage() override;

  // Parse and create an SVGImageViewInfo from the provided fragment string.
  // Returns nullptr if no valid view specifier is found.
  const SVGImageViewInfo* CreateViewInfo(const String& fragment) const;

  // Apply a view specifier.
  void ApplyViewInfo(const SVGImageViewInfo*);

  // Get the intrinsic dimensions (width, height and aspect ratio) from this
  // SVGImage. Returns true if successful.
  std::optional<NaturalSizingInfo> GetNaturalDimensions(
      const SVGViewSpec*) const;

  String FilenameExtension() const override;

  const AtomicString& MimeType() const override;

  SizeAvailability DataChanged(bool all_data_received) override;

  // FIXME: SVGImages are underreporting decoded sizes and will be unable
  // to prune because these functions are not implemented yet.
  void DestroyDecodedData() override {}

  // FIXME: Implement this to be less conservative.
  bool CurrentFrameKnownToBeOpaque() override { return false; }

  class DrawInfo {
    STACK_ALLOCATED();

   public:
    DrawInfo(const gfx::SizeF& container_size,
             float zoom,
             const SVGImageViewInfo* viewinfo,
             bool is_dark_mode_enabled);

    gfx::SizeF CalculateResidualScale() const;
    float Zoom() const { return zoom_; }
    const gfx::SizeF& ContainerSize() const { return container_size_; }
    const gfx::Size& RoundedContainerSize() const {
      return rounded_container_size_;
    }
    const SVGImageViewInfo* View() const { return viewinfo_; }
    bool IsDarkModeEnabled() const { return is_dark_mode_enabled_; }

   private:
    const gfx::SizeF container_size_;
    const gfx::Size rounded_container_size_;
    const float zoom_;
    const SVGImageViewInfo* viewinfo_;
    const bool is_dark_mode_enabled_;
  };

  void Draw(cc::PaintCanvas*,
            const cc::PaintFlags&,
            const gfx::RectF& dst_rect,
            const gfx::RectF& src_rect,
            const ImageDrawOptions&) override;
  void DrawForContainer(const DrawInfo&,
                        cc::PaintCanvas*,
                        const cc::PaintFlags&,
                        const gfx::RectF& dst_rect,
                        const gfx::RectF& src_rect);
  void DrawPatternForContainer(const DrawInfo&,
                               GraphicsContext&,
                               const cc::PaintFlags&,
                               const gfx::RectF& dst_rect,
                               const ImageTilingInfo&);
  void PopulatePaintRecordForCurrentFrameForContainer(const DrawInfo&,
                                                      PaintImageBuilder&);

  // Paints the current frame. Returns new PaintRecord. |cull_rect| is an
  // optional additional cull rect.
  std::optional<PaintRecord> PaintRecordForCurrentFrame(
      const DrawInfo&,
      const gfx::Rect* cull_rect);

  void DrawInternal(const DrawInfo&,
                    cc::PaintCanvas*,
                    const cc::PaintFlags&,
                    const gfx::RectF& dst_rect,
                    const gfx::RectF& unzoomed_src_rect);
  bool ApplyShader(cc::PaintFlags&,
                   const SkMatrix& local_matrix,
                   const gfx::RectF& src_rect,
                   const ImageDrawOptions&) override;
  bool ApplyShaderForContainer(const DrawInfo&,
                               cc::PaintFlags&,
                               const gfx::RectF& src_rect,
                               const SkMatrix& local_matrix);
  bool ApplyShaderInternal(const DrawInfo&,
                           cc::PaintFlags&,
                           const gfx::RectF& unzoomed_src_rect,
                           const SkMatrix& local_matrix);

  void StopAnimation();
  void ScheduleTimelineRewind();
  void FlushPendingTimelineRewind();

  void NotifyAsyncLoadCompleted();

  LocalFrame* GetFrame() const;
  SVGSVGElement* RootElement() const;
  LayoutSVGRoot* LayoutRoot() const;

  Page* GetPageForTesting();

  Persistent<SVGImageChromeClient> chrome_client_;
  Persistent<IsolatedSVGDocumentHost> document_host_;
  Persistent<AgentGroupScheduler> agent_group_scheduler_;

  PhysicalSize intrinsic_size_;
  bool has_pending_timeline_rewind_;

  int data_change_count_ = 0;
  base::TimeDelta data_change_elapsed_time_;

  base::WeakPtrFactory<SVGImage> weak_ptr_factory_{this};
  FRIEND_TEST_ALL_PREFIXES(ElementFragmentAnchorTest,
                           SVGDocumentDoesntCreateFragment);
  FRIEND_TEST_ALL_PREFIXES(SVGImageTest, SupportsSubsequenceCaching);
  FRIEND_TEST_ALL_PREFIXES(SVGImageTest, LayoutShiftTrackerDisabled);
  FRIEND_TEST_ALL_PREFIXES(SVGImageTest, SetSizeOnVisualViewport);
  FRIEND_TEST_ALL_PREFIXES(SVGImageTest, IsSizeAvailable);
  FRIEND_TEST_ALL_PREFIXES(SVGImageTest, DisablesSMILEvents);
};

template <>
struct DowncastTraits<SVGImage> {
  static bool AllowFrom(const Image& image) { return image.IsSVGImage(); }
};

class ImageObserverDisabler {
  STACK_ALLOCATED();

 public:
  explicit ImageObserverDisabler(Image* image) : image_(image) {
    image_->SetImageObserverDisabled(true);
  }

  ImageObserverDisabler(const ImageObserverDisabler&) = delete;
  ImageObserverDisabler& operator=(const ImageObserverDisabler&) = delete;

  ~ImageObserverDisabler() { image_->SetImageObserverDisabled(false); }

 private:
  Image* image_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_SVG_IMAGE_H_
