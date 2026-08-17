// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_RASTER_INVALIDATION_TRACKING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_RASTER_INVALIDATION_TRACKING_H_

#include <optional>

#include "cc/base/region.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client_types.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/graphics/paint_invalidation_reason.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/json/json_values.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/skia/include/core/SkColor.h"
#include "ui/gfx/geometry/rect.h"

namespace cc {
struct LayerDebugInfo;
}

namespace blink {

struct RasterInvalidationInfo {
  DISALLOW_NEW();

  // This is for comparison only. Don't dereference because the client may have
  // died.
  DisplayItemClientId client_id = kInvalidDisplayItemClientId;
  String client_debug_name;
  // For CAP, this is set in PaintArtifactCompositor when converting chunk
  // raster invalidations to cc raster invalidations.
  gfx::Rect rect;
  PaintInvalidationReason reason = PaintInvalidationReason::kLayout;
};

inline bool operator==(const RasterInvalidationInfo& a,
                       const RasterInvalidationInfo& b) {
  return a.client_id == b.client_id &&
         a.client_debug_name == b.client_debug_name && a.rect == b.rect &&
         a.reason == b.reason;
}
inline bool operator!=(const RasterInvalidationInfo& a,
                       const RasterInvalidationInfo& b) {
  return !(a == b);
}

inline std::ostream& operator<<(std::ostream& os,
                                const RasterInvalidationInfo& info) {
  return os << info.client_id << ":" << info.client_debug_name
            << " rect=" << info.rect.ToString() << " reason=" << info.reason;
}

struct RasterUnderInvalidation {
  DISALLOW_NEW();
  int x;
  int y;
  // TODO(https://crbug.com/1351544): This class should use SkColor4f.
  SkColor old_pixel;
  SkColor new_pixel;
};

class PLATFORM_EXPORT RasterInvalidationTracking
    : public GarbageCollected<RasterInvalidationTracking> {
 public:
  void Trace(Visitor*) const {}

  // When RuntimeEnabledFeatures::PaintUnderInvalidationCheckingEnabled() and
  // SimulateRasterUnderInvalidation(true) is called, all changed pixels will
  // be reported as raster under-invalidations. Used to visually test raster
  // under-invalidation checking feature.
  static void SimulateRasterUnderInvalidations(bool enable);

  // Whether we should always track because RuntimeEnabledFeatures::
  // PaintUnderInvalidationCheckingEnabled() is true, or we are tracing
  // "disabled-by-default-blink.invalidation" category.
  static bool ShouldAlwaysTrack();

  static bool IsTracingRasterInvalidations();

  void AddInvalidation(DisplayItemClientId,
                       const String& debug_name,
                       const gfx::Rect&,
                       PaintInvalidationReason);
  bool HasInvalidations() const { return !invalidations_.empty(); }
  const Vector<RasterInvalidationInfo>& Invalidations() const {
    return invalidations_;
  }
  void ClearInvalidations() { invalidations_.clear(); }

  // Compares the last recording against |new_record|, by rastering both into
  // bitmaps. If there are any differences outside of invalidated regions,
  // the corresponding pixels in UnderInvalidationRecord() will be drawn in
  // dark red. The caller can overlay UnderInvalidationRecord() onto the
  // original drawings to show the under raster invalidations.
  void CheckUnderInvalidations(const String& layer_debug_name,
                               PaintRecord new_record,
                               const gfx::Rect& new_interest_rect);

  void AsJSON(JSONObject*, bool detailed) const;

  void AddToLayerDebugInfo(cc::LayerDebugInfo&) const;

  // The record containing under-invalidated pixels in dark red.
  PaintRecord UnderInvalidationRecord() const {
    return under_invalidation_record_;
  }

 private:
  Vector<RasterInvalidationInfo> invalidations_;

  // The following fields are for raster under-invalidation detection.
  std::optional<PaintRecord> last_painted_record_;
  gfx::Rect last_interest_rect_;
  cc::Region invalidation_region_since_last_paint_;
  Vector<RasterUnderInvalidation> under_invalidations_;
  PaintRecord under_invalidation_record_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_RASTER_INVALIDATION_TRACKING_H_
