// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_VIEWPORT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_VIEWPORT_DATA_H_

#include "third_party/blink/public/mojom/page/display_cutout.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/viewport_description.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace blink {

class Document;

class ViewportData final : public GarbageCollected<ViewportData> {
 public:
  ViewportData(Document& document);
  void Trace(Visitor* visitor) const;
  void Shutdown();

  bool ShouldMergeWithLegacyDescription(ViewportDescription::Type) const;
  bool ShouldOverrideLegacyDescription(ViewportDescription::Type) const;
  CORE_EXPORT void SetViewportDescription(const ViewportDescription&);
  CORE_EXPORT ViewportDescription GetViewportDescription() const;
  Length ViewportDefaultMinWidth() const { return viewport_default_min_width_; }

  void UpdateViewportDescription();

  void SetHasComplexSafaAreaConstraint(bool value);

  // When true this will force a kCover viewport fit value which will result in
  // the document expanding into the display cutout area.
  CORE_EXPORT void SetExpandIntoDisplayCutout(bool expand);
  CORE_EXPORT bool GetExpandIntoDisplayCutout() const {
    return force_expand_display_cutout_;
  }
  mojom::ViewportFit GetCurrentViewportFitForTests() const {
    return viewport_fit_;
  }

  CORE_EXPORT void SetVirtualKeyboardOverlaysContent(bool overlays_content);
  CORE_EXPORT bool GetVirtualKeyboardOverlaysContent() const {
    return virtual_keyboard_overlays_content_;
  }

 private:
  Member<Document> document_;

  ViewportDescription viewport_description_;
  ViewportDescription legacy_viewport_description_;
  Length viewport_default_min_width_;

  // Whether overlays content was set via the virtualKeyboard API.
  bool virtual_keyboard_overlays_content_ = false;

  // Stores the current value viewport-fit value.
  mojom::ViewportFit viewport_fit_ = blink::mojom::ViewportFit::kAuto;
  bool force_expand_display_cutout_ = false;

  // Stores the current complex safe area constraint value.
  std::optional<bool> has_complex_safe_area_constraint_;

  HeapMojoAssociatedRemote<mojom::blink::DisplayCutoutHost>
      display_cutout_host_;
};

inline bool ViewportData::ShouldOverrideLegacyDescription(
    ViewportDescription::Type origin) const {
  // The different (legacy) meta tags have different priorities based on the
  // type regardless of which order they appear in the DOM. The priority is
  // given by the ViewportDescription::Type enum.
  return origin >= legacy_viewport_description_.type;
}

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_VIEWPORT_DATA_H_
