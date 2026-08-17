// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client_types.h"
#include "third_party/blink/renderer/platform/graphics/paint_invalidation_reason.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/assertions.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"
#include "ui/gfx/geometry/rect.h"

#if DCHECK_IS_ON()
#include "third_party/blink/renderer/platform/json/json_values.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#endif

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class PaintArtifact;
enum class PaintPhase;

class PLATFORM_EXPORT DisplayItem {
  DISALLOW_NEW();

 public:
  enum {
    // Must be kept in sync with core/paint/PaintPhase.h.
    kPaintPhaseMax = 12,
  };

  // A display item type uniquely identifies a display item of a client.
  // Some display item types can be categorized using the following directives:
  // - In enum Type:
  //   - enum value <Category>First;
  //   - enum values of the category, first of which should equal
  //     <Category>First (for ease of maintenance, the values should be in
  //     alphabetic order);
  //   - enum value <Category>Last which should be equal to the last of the enum
  //     values of the category
  // - DEFINE_CATEGORY_METHODS(<Category>) to define is<Category>Type(Type) and
  //   is<Category>() methods.
  //
  // A category or subset of a category can contain types each of which
  // corresponds to a PaintPhase:
  // - In enum Type:
  //   - enum value <Category>[<Subset>]PaintPhaseFirst;
  //   - enum value <Category>[<Subset>]PaintPhaseLast =
  //     <Category>[<Subset>]PaintPhaseFirst + PaintPhaseMax;
  // - DEFINE_PAINT_PHASE_CONVERSION_METHOD(<Category>[<Subset>]) to define
  //   paintPhaseTo<Category>[<Subset>]Type(PaintPhase) method.
  enum Type {
    kUninitializedType,

    kDrawingFirst,
    kDrawingPaintPhaseFirst = kDrawingFirst,
    kDrawingPaintPhaseLast = kDrawingFirst + kPaintPhaseMax,
    kBoxDecorationBackground,
    kFixedAttachmentBackground,
    kCapsLockIndicator,
    kCaret,
    kColumnRules,
    kDocumentRootBackdrop,
    kDocumentBackground,
    kDragCaret,
    kForcedColorsModeBackplate,
    kSVGImage,
    kImageAreaFocusRing,
    kOverflowControls,
    kFrameOverlay,
    kPrintedContentDestinationLocations,
    kPrintedContentPDFURLRect,
    kReflectionMask,
    kResizer,
    kSVGClip,
    kSVGMask,
    kScrollCorner,
    // The following 3 types are used during cc::Scrollbar::PaintPart() only.
    // During Paint stage of document lifecycle update, we record
    // ScrollbarDisplayItem instead of DrawingItems of these types.
    kScrollbarTrackAndButtons,
    kScrollbarThumb,
    kScrollbarTickmarks,
    kSelectionTint,
    kTableCollapsedBorders,
    kWebPlugin,
    kDrawingLast = kWebPlugin,

    kForeignLayerFirst,
    kForeignLayerCanvas = kForeignLayerFirst,
    kForeignLayerDevToolsOverlay,
    kForeignLayerPlugin,
    kForeignLayerVideo,
    kForeignLayerRemoteFrame,
    kForeignLayerLinkHighlight,
    kForeignLayerViewportScroll,
    kForeignLayerViewportScrollbar,
    kForeignLayerViewTransitionContent,
    kForeignLayerLast = kForeignLayerViewTransitionContent,

    kClipPaintPhaseFirst,
    kClipPaintPhaseLast = kClipPaintPhaseFirst + kPaintPhaseMax,

    kScrollPaintPhaseFirst,
    kScrollPaintPhaseLast = kScrollPaintPhaseFirst + kPaintPhaseMax,

    kSVGTransformPaintPhaseFirst,
    kSVGTransformPaintPhaseLast = kSVGTransformPaintPhaseFirst + kPaintPhaseMax,

    kSVGEffectPaintPhaseFirst,
    kSVGEffectPaintPhaseLast = kSVGEffectPaintPhaseFirst + kPaintPhaseMax,

    // The following hit test types are for paint chunks containing hit test
    // data, when we don't have an previously set explicit chunk id when
    // creating the paint chunk, or we need dedicated paint chunk for the hit
    // test data.

    // Compositor hit testing requires that layers are created and sized to
    // include content that does not paint. Hit test data ensure a layer exists
    // and is sized properly even if no content would otherwise be painted.
    kHitTest,
    // Web plugin needs a separate id to avoid conflict with the hit test data
    // for LayoutReplaced.
    kWebPluginHitTest,

    // Used for paint chunks that contain region capture data.
    kRegionCapture,

    // Used both for specifying the paint-order scroll location, and for non-
    // composited scroll hit testing (see: hit_test_data.h).
    kScrollHitTest,
    // Used to prevent composited scrolling on the resize handle.
    kResizerScrollHitTest,
    // Used to prevent composited scrolling and set touch action region, on
    // custom scrollbars and non-composited native scrollbars.
    kScrollbarHitTest,

    // These are for paint chunks that are forced for layers.
    kLayerChunk,
    // This is used if a layer has any negative-z-index children. Otherwise the
    // foreground is in the kLayerChunk chunk.
    kLayerChunkForeground,

    // The following 2 types are For ScrollbarDisplayItem.
    kScrollbarHorizontal,
    kScrollbarVertical,

    kTypeLast = kScrollbarVertical,
  };

  static_assert(kTypeLast < (1 << 8),
                "DisplayItem::Type should fit in uint8_t");

  DisplayItem(const DisplayItem&) = delete;
  DisplayItem(DisplayItem&&) = delete;
  DisplayItem& operator=(const DisplayItem&) = delete;
  DisplayItem& operator=(DisplayItem&&) = delete;

  // Ids are for matching new DisplayItems with existing DisplayItems.
  struct Id {
    DISALLOW_NEW();
    Id(DisplayItemClientId client_id, Type type, wtf_size_t fragment = 0)
        : client_id(client_id), type(type), fragment(fragment) {}
    Id(const Id& id, wtf_size_t fragment)
        : client_id(id.client_id), type(id.type), fragment(fragment) {}

    // The no-argument version is for operator<< which is used in DCHECK and
    // unit tests.
    WTF::String ToString() const;
    // This version will output the debug name of the client.
    WTF::String ToString(const PaintArtifact&) const;

    const DisplayItemClientId client_id;
    const Type type;
    const wtf_size_t fragment;

    struct HashKey {
      DISALLOW_NEW();

     public:
      HashKey() = default;
      explicit HashKey(const DisplayItem::Id& id)
          : client_id(id.client_id), type(id.type), fragment(id.fragment) {}
      bool operator==(const HashKey& other) const {
        return client_id == other.client_id && type == other.type &&
               fragment == other.fragment;
      }

      DisplayItemClientId client_id = kInvalidDisplayItemClientId;
      DisplayItem::Type type = static_cast<DisplayItem::Type>(0);
      wtf_size_t fragment = 0;
    };

    HashKey AsHashKey() const { return HashKey(*this); }
  };

  Id GetId() const { return Id(client_id_, GetType(), fragment_); }

  DisplayItemClientId ClientId() const {
    DCHECK_NE(client_id_, kInvalidDisplayItemClientId);
    return client_id_;
  }

  // The bounding box of all pixels of this display item, in the transform space
  // of the containing paint chunk.
  const gfx::Rect& VisualRect() const { return visual_rect_; }

  RasterEffectOutset GetRasterEffectOutset() const {
    return static_cast<RasterEffectOutset>(raster_effect_outset_);
  }

  Type GetType() const { return static_cast<Type>(type_); }

  // The fragment is part of the id, to uniquely identify display items in
  // different fragments for the same client and type.
  wtf_size_t Fragment() const { return fragment_; }
  void SetFragment(wtf_size_t fragment) { fragment_ = fragment; }

// See comments of enum Type for usage of the following macros.
#define DEFINE_CATEGORY_METHODS(Category)                           \
  static constexpr bool Is##Category##Type(Type type) {             \
    return type >= k##Category##First && type <= k##Category##Last; \
  }                                                                 \
  bool Is##Category() const { return Is##Category##Type(GetType()); }

#define DEFINE_PAINT_PHASE_CONVERSION_METHOD(Category)                         \
  static constexpr Type PaintPhaseTo##Category##Type(PaintPhase paint_phase) { \
    static_assert(                                                             \
        k##Category##PaintPhaseLast - k##Category##PaintPhaseFirst ==          \
            kPaintPhaseMax,                                                    \
        "Invalid paint-phase-based category " #Category                        \
        ". See comments of DisplayItem::Type");                                \
    return static_cast<Type>(static_cast<int>(paint_phase) +                   \
                             k##Category##PaintPhaseFirst);                    \
  }

  DEFINE_CATEGORY_METHODS(Drawing)
  DEFINE_PAINT_PHASE_CONVERSION_METHOD(Drawing)

  DEFINE_CATEGORY_METHODS(ForeignLayer)

  DEFINE_PAINT_PHASE_CONVERSION_METHOD(Clip)
  DEFINE_PAINT_PHASE_CONVERSION_METHOD(Scroll)
  DEFINE_PAINT_PHASE_CONVERSION_METHOD(SVGTransform)
  DEFINE_PAINT_PHASE_CONVERSION_METHOD(SVGEffect)

  bool IsScrollbar() const {
    return type_ == kScrollbarHorizontal || type_ == kScrollbarVertical;
  }

  PaintInvalidationReason GetPaintInvalidationReason() const {
    return static_cast<PaintInvalidationReason>(paint_invalidation_reason_);
  }
  void SetPaintInvalidationReason(PaintInvalidationReason reason) {
    paint_invalidation_reason_ = static_cast<unsigned>(reason);
  }
  bool IsCacheable() const {
    return static_cast<PaintInvalidationReason>(paint_invalidation_reason_) !=
           PaintInvalidationReason::kUncacheable;
  }

  bool EqualsForUnderInvalidation(const DisplayItem& other) const;

  // True if this DisplayItem is the tombstone/"dead display item" as part of
  // moving an item from one list to another. See CreateTombstone().
  bool IsTombstone() const { return !is_not_tombstone_; }

  bool DrawsContent() const { return draws_content_; }

#if DCHECK_IS_ON()
  // A subsequence tombstone is full of zeros set by memset(0);
  bool IsSubsequenceTombstone() const {
    return !is_not_tombstone_ && client_id_ == kInvalidDisplayItemClientId;
  }
  static WTF::String TypeAsDebugString(DisplayItem::Type);
  WTF::String AsDebugString(const PaintArtifact&) const;
  WTF::String IdAsString(const PaintArtifact&) const;
  void PropertiesAsJSON(JSONObject&, const PaintArtifact&) const;
#endif

 protected:
  // Some fields are copied from |client|, because we need to access them in
  // later paint cycles when |client| may have been destroyed.
  DisplayItem(const DisplayItemClientId client_id,
              Type type,
              const gfx::Rect& visual_rect,
              RasterEffectOutset raster_effect_outset,
              PaintInvalidationReason paint_invalidation_reason,
              bool draws_content)
      : client_id_(client_id),
        visual_rect_(visual_rect),
        fragment_(0),
        paint_invalidation_reason_(
            static_cast<unsigned>(paint_invalidation_reason)),
        type_(type),
        raster_effect_outset_(static_cast<unsigned>(raster_effect_outset)),
        draws_content_(draws_content),
        is_not_tombstone_(true),
        opaqueness_(0) {}

  ~DisplayItem() = default;

 private:
  friend class DisplayItemList;

  // DisplayItemList calls this method to destruct a DisplayItem in place.
  // It knows how to destruct subclasses.
  void Destruct();

  // Used by DisplayItemList::AppendByMoving() and ReplaceLastByMoving() where
  // a tombstone DisplayItem is constructed at the source location. Only set
  // draws_content_ and is_not_tombstone_ to false, leaving other fields as-is
  // so that we can get their original values for debugging and raster
  // invalidation.
  void CreateTombstone() {
    draws_content_ = false;
    is_not_tombstone_ = false;
  }

  DisplayItemClientId client_id_;
  gfx::Rect visual_rect_;
  wtf_size_t fragment_;
  // paint_invalidation_reason_ is set during construction (or, in the case of a
  // DisplayItem copied from the cache, shortly thereafter). Once set, it is
  // never modified. It is used to inform raster invalidation.
  unsigned paint_invalidation_reason_ : 8;
  unsigned type_ : 8;
  unsigned raster_effect_outset_ : 2;
  unsigned draws_content_ : 1;
  // This is not |is_tombstone_| to allow memset(0) to clear a display item to
  // be a tombstone.
  unsigned is_not_tombstone_ : 1;

 protected:
  // For DrawingDisplayItem to save memory.
  mutable unsigned opaqueness_ : 2;
};

inline bool operator==(const DisplayItem::Id& a, const DisplayItem::Id& b) {
  return a.client_id == b.client_id && a.type == b.type &&
         a.fragment == b.fragment;
}

inline bool operator!=(const DisplayItem::Id& a, const DisplayItem::Id& b) {
  return !(a == b);
}

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, DisplayItem::Type);
// These are mainly for DCHECK and unit tests. They don't output debug names of
// DisplayItemClients. Use the argumented version of DisplayItem::Id::ToString()
// or DisplayItem::AsDebugString() if you want to see debug names.
PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const DisplayItem::Id&);
PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const DisplayItem&);

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::DisplayItem::Id::HashKey>
    : GenericHashTraits<blink::DisplayItem::Id::HashKey> {
  using Key = blink::DisplayItem::Id::HashKey;
  static constexpr bool kEmptyValueIsZero = true;
  static void ConstructDeletedValue(Key& slot) {
    const_cast<wtf_size_t&>(slot.fragment) = kNotFound;
  }
  static bool IsDeletedValue(const Key& id) { return id.fragment == kNotFound; }

  static unsigned GetHash(const Key& id) {
    unsigned hash = WTF::GetHash(id.client_id);
    WTF::AddIntToHash(hash, id.type);
    WTF::AddIntToHash(hash, id.fragment);
    return hash;
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_H_
