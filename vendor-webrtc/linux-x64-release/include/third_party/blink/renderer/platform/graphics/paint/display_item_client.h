// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_CLIENT_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client_types.h"
#include "third_party/blink/renderer/platform/graphics/paint_invalidation_reason.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

// The class for objects that can be associated with display items. A
// DisplayItemClient object should live at least longer than the document cycle
// in which its display items are created during painting. After the document
// cycle, a pointer/reference to DisplayItemClient should be no longer
// dereferenced unless we can make sure the client is still alive.
class PLATFORM_EXPORT DisplayItemClient : public GarbageCollectedMixin {
 public:
  DisplayItemClient()
      : paint_invalidation_reason_(
            static_cast<uint8_t>(PaintInvalidationReason::kJustCreated)),
        marked_for_validation_(0) {}
  DisplayItemClient(const DisplayItemClient&) = delete;
  DisplayItemClient& operator=(const DisplayItemClient&) = delete;
  virtual ~DisplayItemClient() = default;

  DisplayItemClientId Id() const {
    return reinterpret_cast<DisplayItemClientId>(this);
  }

  virtual String DebugName() const = 0;

  // Returns the id of the DOM node associated with this DisplayItemClient, or
  // kInvalidDOMNodeId if there is no associated DOM node.
  virtual DOMNodeId OwnerNodeId() const { return kInvalidDOMNodeId; }

  // The outset will be used to inflate visual rect after the visual rect is
  // mapped into the space of the composited layer, for any special raster
  // effects that might expand the rastered pixel area.
  virtual RasterEffectOutset VisualRectOutsetForRasterEffects() const {
    return RasterEffectOutset::kNone;
  }

  // Indicates that the client will paint display items different from the ones
  // cached by PaintController. However, PaintController allows a client to
  // paint new display items that are not cached or to no longer paint some
  // cached display items without calling this method.
  // See PaintController::ClientCacheIsValid() for more details.
  void Invalidate(
      PaintInvalidationReason reason = PaintInvalidationReason::kLayout) const {
    if (reason > GetPaintInvalidationReason())
      paint_invalidation_reason_ = static_cast<uint8_t>(reason);
  }

  PaintInvalidationReason GetPaintInvalidationReason() const {
    return static_cast<PaintInvalidationReason>(paint_invalidation_reason_);
  }

  // A client is considered "just created" if its display items have never been
  // validated by any PaintController since it's created.
  bool IsJustCreated() const {
    return GetPaintInvalidationReason() ==
           PaintInvalidationReason::kJustCreated;
  }

  // Whether the client is cacheable. The uncacheable status is set when the
  // client produces any display items that skipped caching of any
  // PaintController.
  bool IsCacheable() const {
    return GetPaintInvalidationReason() !=
           PaintInvalidationReason::kUncacheable;
  }

  // True if the client's display items are cached in PaintControllers without
  // needing to update.
  bool IsValid() const {
    return GetPaintInvalidationReason() == PaintInvalidationReason::kNone;
  }

  String ToString() const;

 private:
  friend class FakeDisplayItemClient;
  friend class ObjectPaintInvalidatorTest;
  friend class PaintChunker;
  friend class PaintController;
  friend class PaintControllerCycleScope;

  void MarkForValidation() const { marked_for_validation_ = 1; }
  bool IsMarkedForValidation() const { return marked_for_validation_; }
  void Validate() const {
    paint_invalidation_reason_ =
        static_cast<uint8_t>(PaintInvalidationReason::kNone);
    marked_for_validation_ = 0;
  }

  mutable uint8_t paint_invalidation_reason_ : 7;
  mutable uint8_t marked_for_validation_ : 1;
};

class StaticDisplayItemClient
    : public GarbageCollected<StaticDisplayItemClient>,
      public DisplayItemClient {
 public:
  explicit StaticDisplayItemClient(const char* name) : name_(name) {}

  String DebugName() const override { return name_; }
  void Trace(Visitor* visitor) const override {
    DisplayItemClient::Trace(visitor);
  }

 private:
  const char* name_;
};

// Defines a StaticDisplayItemClient instance which can be used where a
// DisplayItemClient is needed but DisplayItem::Id uniqueness is guaranteed
// or not required, e.g.
// - when recording a a foreign layer,
// - when recording a DisplayItem that appears only once in the painted result,
// - when painting with a transient PaintController.
// Note: debug_name must be a literal string.
#define DEFINE_STATIC_DISPLAY_ITEM_CLIENT(name, debug_name) \
  DEFINE_STATIC_LOCAL(                                      \
      Persistent<StaticDisplayItemClient>, name,            \
      (MakeGarbageCollected<StaticDisplayItemClient>(debug_name)))

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&,
                                         const DisplayItemClient*);
PLATFORM_EXPORT std::ostream& operator<<(std::ostream&,
                                         const DisplayItemClient&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_CLIENT_H_
