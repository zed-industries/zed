// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CONTROLLER_H_

#include <memory>
#include <optional>
#include <utility>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/memory/ptr_util.h"
#include "cc/input/hit_test_opaqueness.h"
#include "cc/input/layer_selection_bound.h"
#include "cc/paint/element_id.h"
#include "third_party/blink/renderer/platform/geometry/infinite_int_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_list.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_artifact.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_chunk.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_chunker.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_under_invalidation_checker.h"
#include "third_party/blink/renderer/platform/graphics/paint/region_capture_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "ui/gfx/geometry/rect.h"

class SkTextBlob;

namespace blink {

enum class PaintBenchmarkMode {
  kNormal,
  kForceRasterInvalidationAndConvert,
  kForcePaintArtifactCompositorUpdate,
  // Tests PaintController performance of moving cached subsequences.
  kForcePaint,
  // Tests performance of core paint tree walk and moving cached display items.
  kSubsequenceCachingDisabled,
  // Tests performance of full repaint.
  kCachingDisabled,
};

// FrameFirstPaint stores first-paint, text or image painted for the
// corresponding frame. They are never reset to false. First-paint is defined in
// https://github.com/WICG/paint-timing. It excludes default background paint.
struct FrameFirstPaint {
  DISALLOW_NEW();
  explicit FrameFirstPaint(const void* frame)
      : frame(frame),
        first_painted(false),
        text_painted(false),
        image_painted(false) {}

  const void* frame = nullptr;
  bool first_painted : 1;
  bool text_painted : 1;
  bool image_painted : 1;
};

struct SubsequenceMarkers {
  DISALLOW_NEW();
  DisplayItemClientId client_id = kInvalidDisplayItemClientId;
  // The start and end (not included) index of paint chunks in this
  // subsequence.
  wtf_size_t start_chunk_index = 0;
  wtf_size_t end_chunk_index = 0;
  bool is_moved_from_cached_subsequence = false;
};

struct SubsequencesData {
  DISALLOW_NEW();
  // Map a client to the index into |tree|.
  HashMap<DisplayItemClientId, wtf_size_t> map;
  // A pre-order list of the subsequence tree.
  Vector<SubsequenceMarkers> tree;
};

class PaintController;

class PLATFORM_EXPORT PaintControllerPersistentData
    : public GarbageCollected<PaintControllerPersistentData> {
 public:
  void Trace(Visitor* visitor) const {
    visitor->Trace(current_paint_artifact_);
  }

  // Returns the approximate memory usage owned by by this data.
  size_t ApproximateUnsharedMemoryUsage() const;

  // Get the artifact generated after the last commit.
  const PaintArtifact& GetPaintArtifact() const {
    return *current_paint_artifact_;
  }
  const DisplayItemList& GetDisplayItemList() const {
    return GetPaintArtifact().GetDisplayItemList();
  }
  const PaintChunks& GetPaintChunks() const {
    return GetPaintArtifact().GetPaintChunks();
  }

  void InvalidateAllForTesting();

  bool ClientCacheIsValid(const DisplayItemClient&) const;

  wtf_size_t GetSubsequenceIndex(DisplayItemClientId) const;
  const SubsequenceMarkers* GetSubsequenceMarkers(DisplayItemClientId) const;

 private:
  friend class PaintController;

  void CommitNewDisplayItems(PaintArtifact& new_paint_artifact,
                             SubsequencesData new_subsequences);

  Member<PaintArtifact> current_paint_artifact_ =
      MakeGarbageCollected<PaintArtifact>();
  SubsequencesData current_subsequences_;

  bool cache_is_all_invalid_ = true;
};

// Responsible for processing display items as they are produced, and producing
// a final paint artifact when complete. This class includes logic for caching,
// cache invalidation, and merging.
class PLATFORM_EXPORT PaintController {
  STACK_ALLOCATED();

 public:
  explicit PaintController(bool record_debug_info = false,
                           PaintControllerPersistentData* = nullptr,
                           PaintBenchmarkMode = PaintBenchmarkMode::kNormal);
  PaintController(const PaintController&) = delete;
  PaintController& operator=(const PaintController&) = delete;
  ~PaintController();

  bool HasPersistentData() const { return persistent_data_; }

  void SetRecordDebugInfo(bool record_debug_info) {
    record_debug_info_ = record_debug_info;
  }

  // Provide a new set of paint chunk properties to apply to recorded display
  // items. If id is nullptr, the id of the first display item will be used as
  // the id of the paint chunk if needed.
  void UpdateCurrentPaintChunkProperties(const PaintChunk::Id&,
                                         const DisplayItemClient&,
                                         const PropertyTreeStateOrAlias&);
  void UpdateCurrentPaintChunkProperties(const PropertyTreeStateOrAlias&);
  const PropertyTreeStateOrAlias& CurrentPaintChunkProperties() const {
    return paint_chunker_.CurrentPaintChunkProperties();
  }

  void SetCurrentEffectivelyInvisible(bool invisible) {
    paint_chunker_.SetCurrentEffectivelyInvisible(invisible);
  }
  bool CurrentEffectivelyInvisible() const {
    return paint_chunker_.CurrentEffectivelyInvisible();
  }
  void EnsureChunk();

  bool CurrentChunkIsNonEmptyAndTransparentToHitTest() const {
    return paint_chunker_.CurrentChunkIsNonEmptyAndTransparentToHitTest();
  }
  void RecordHitTestData(const DisplayItemClient&,
                         const gfx::Rect&,
                         TouchAction,
                         bool blocking_wheel,
                         cc::HitTestOpaqueness,
                         DisplayItem::Type type = DisplayItem::kHitTest);

  void RecordRegionCaptureData(const DisplayItemClient& client,
                               const RegionCaptureCropId& crop_id,
                               const gfx::Rect& rect);

  void RecordScrollHitTestData(
      const DisplayItemClient&,
      DisplayItem::Type,
      const TransformPaintPropertyNode* scroll_translation,
      const gfx::Rect& scroll_hit_test_rect,
      cc::HitTestOpaqueness,
      const gfx::Rect& scrolling_contents_cull_rect = InfiniteIntRect());

  void RecordSelection(std::optional<PaintedSelectionBound> start,
                       std::optional<PaintedSelectionBound> end,
                       String debug_info);
  void RecordAnySelectionWasPainted() {
    paint_chunker_.RecordAnySelectionWasPainted();
  }

  wtf_size_t NumNewChunks() const {
    return new_paint_artifact_->GetPaintChunks().size();
  }
  const gfx::Rect& LastChunkBounds() const {
    return new_paint_artifact_->GetPaintChunks().back().bounds;
  }
  const TraceablePropertyTreeStateOrAlias& LastChunkProperties() const {
    return new_paint_artifact_->GetPaintChunks().back().properties;
  }

  void MarkClientForValidation(const DisplayItemClient& client);

  template <typename DisplayItemClass, typename... Args>
  void CreateAndAppend(const DisplayItemClient& client, Args&&... args) {
    MarkClientForValidation(client);
    DisplayItemClass& display_item =
        new_paint_artifact_->GetDisplayItemList()
            .AllocateAndConstruct<DisplayItemClass>(
                client.Id(), std::forward<Args>(args)...);
    display_item.SetFragment(current_fragment_);
    ProcessNewItem(client, display_item);
  }

  // Tries to find the cached display item corresponding to the given
  // parameters. If found, appends the cached display item to the new display
  // list and returns true. Otherwise returns false.
  bool UseCachedItemIfPossible(const DisplayItemClient&, DisplayItem::Type);

#if DCHECK_IS_ON()
  void AssertLastCheckedCachedItem(const DisplayItemClient&, DisplayItem::Type);
#endif

  // Returns the SkTextBlob in DrawTextBlobOp in
  // MatchingCachedItemToBeRepainted() if it exists and can be reused for
  // repainting.
  sk_sp<SkTextBlob> CachedTextBlob() const;

  // Tries to find the cached subsequence corresponding to the given parameters.
  // If found, copies the cache subsequence to the new display list and returns
  // true. Otherwise returns false.
  bool UseCachedSubsequenceIfPossible(const DisplayItemClient&);

  // Returns the index of the new subsequence.
  wtf_size_t BeginSubsequence(const DisplayItemClient&);
  // The |subsequence_index| parameter should be the return value of the
  // corresponding BeginSubsequence().
  void EndSubsequence(wtf_size_t subsequence_index);

  void BeginSkippingCache() {
    if (persistent_data_) {
      ++skipping_cache_count_;
    }
  }
  void EndSkippingCache() {
    if (persistent_data_) {
      CHECK_GT(skipping_cache_count_, 0u);
      --skipping_cache_count_;
    }
  }
  bool IsSkippingCache() const {
    return !persistent_data_ || skipping_cache_count_;
  }

  // Must be called when a painting is finished. If associated with persistent
  // data, updates the current paint artifact in the persistent data with the
  // new paintings. Returns the new paint artifact (which is mainly for
  // transient (i.e. no persistent data) usages).
  const PaintArtifact& CommitNewDisplayItems();

  bool IsCheckingUnderInvalidationForTesting() const;

  void SetFirstPainted();
  void SetTextPainted();
  void SetImagePainted();

#if DCHECK_IS_ON()
  void ShowCompactDebugData() const;
  String DebugDataAsString(
      DisplayItemList::JsonOption = DisplayItemList::kDefault) const;
  void ShowDebugData() const;
  void ShowDebugDataWithPaintRecords() const;
#endif

  void BeginFrame(const void* frame);
  FrameFirstPaint EndFrame(const void* frame);

  // The current fragment will be part of the ids of all display items and
  // paint chunks, to uniquely identify display items in different fragments
  // for the same client and type.
  wtf_size_t CurrentFragment() const { return current_fragment_; }
  void SetCurrentFragment(wtf_size_t fragment) { current_fragment_ = fragment; }

  class CounterForTesting {
    STACK_ALLOCATED();
   public:
    CounterForTesting() {
      DCHECK(!PaintController::counter_for_testing_);
      PaintController::counter_for_testing_ = this;
    }
    ~CounterForTesting() {
      DCHECK_EQ(this, PaintController::counter_for_testing_);
      PaintController::counter_for_testing_ = nullptr;
    }
    void Reset() { num_cached_items = num_cached_subsequences = 0; }

    size_t num_cached_items = 0;
    size_t num_cached_subsequences = 0;
  };

 private:
  friend class PaintControllerTestBase;
  friend class PaintControllerPaintTestBase;
  friend class PaintControllerPersistentData;
  friend class PaintUnderInvalidationChecker;

  const PaintArtifact& CurrentPaintArtifact() const {
    DCHECK(persistent_data_);
    return *persistent_data_->current_paint_artifact_;
  }
  PaintArtifact& CurrentPaintArtifact() {
    DCHECK(persistent_data_);
    return *persistent_data_->current_paint_artifact_;
  }
  const DisplayItemList& CurrentDisplayItemList() const {
    return CurrentPaintArtifact().GetDisplayItemList();
  }
  DisplayItemList& CurrentDisplayItemList() {
    return CurrentPaintArtifact().GetDisplayItemList();
  }
  const PaintChunks& CurrentPaintChunks() const {
    return CurrentPaintArtifact().GetPaintChunks();
  }
  PaintChunks& CurrentPaintChunks() {
    return CurrentPaintArtifact().GetPaintChunks();
  }
  const SubsequencesData& CurrentSubsequences() const {
    DCHECK(persistent_data_);
    return persistent_data_->current_subsequences_;
  }
  SubsequencesData& CurrentSubsequences() {
    DCHECK(persistent_data_);
    return persistent_data_->current_subsequences_;
  }

  void RecordDebugInfo(const DisplayItemClient&);

  // True if all display items associated with the client are validly cached.
  // However, the current algorithm allows the following situations even if
  // ClientCacheIsValid() is true for a client during painting:
  // 1. The client paints a new display item that is not cached:
  //    UseCachedItemIfPossible() returns false for the display item and the
  //    newly painted display item will be added into the cache. This situation
  //    has slight performance hit (see FindOutOfOrderCachedItemForward()) so we
  //    print a warning in the situation and should keep it rare.
  // 2. the client no longer paints a display item that is cached: the cached
  //    display item will be removed. This doesn't affect performance.
  bool ClientCacheIsValid(const DisplayItemClient&) const;

  // Set new item state (cache skipping, etc) for the last new display item.
  void ProcessNewItem(const DisplayItemClient&, DisplayItem&);

  // This can only be called if the previous UseCachedItemIfPossible() returned
  // false. Returns the cached display item that was matched in the previous
  // UseCachedItemIfPossible() for an invalidated DisplayItemClient for
  // non-layout reason, or nullptr if there is no such item.
  const DisplayItem* MatchingCachedItemToBeRepainted() const;

  void CheckNewItem(DisplayItem&);
  void CheckNewChunkId(const PaintChunk::Id&);
  void CheckNewChunk();

  // Maps a display item id to the index of the display item or the paint chunk.
  using IdIndexMap = HashMap<DisplayItem::Id::HashKey, wtf_size_t>;

  static wtf_size_t FindItemFromIdIndexMap(const DisplayItem::Id&,
                                           const IdIndexMap&,
                                           const DisplayItemList&);
  static void AddToIdIndexMap(const DisplayItem::Id&,
                              wtf_size_t index,
                              IdIndexMap&);

  wtf_size_t FindCachedItem(const DisplayItemClient&, const DisplayItem::Id&);
  wtf_size_t FindOutOfOrderCachedItemForward(const DisplayItemClient&,
                                             const DisplayItem::Id&);
  void AppendSubsequenceByMoving(const DisplayItemClient&,
                                 wtf_size_t subsequence_index,
                                 wtf_size_t start_chunk_index,
                                 wtf_size_t end_chunk_index);

  wtf_size_t GetSubsequenceIndex(DisplayItemClientId id) const {
    return persistent_data_->GetSubsequenceIndex(id);
  }
  const SubsequenceMarkers* GetSubsequenceMarkers(
      DisplayItemClientId id) const {
    return persistent_data_->GetSubsequenceMarkers(id);
  }

  void ValidateNewChunkClient(const DisplayItemClient&);

  PaintUnderInvalidationChecker& EnsureUnderInvalidationChecker();
  ALWAYS_INLINE bool IsCheckingUnderInvalidation() const;

#if DCHECK_IS_ON()
  void ShowDebugDataInternal(DisplayItemList::JsonOption) const;
#endif

  PaintControllerPersistentData* const persistent_data_;

  // Data being used to build the next paint artifact. It's not null until
  // CommitNewDisplayItems().
  PaintArtifact* new_paint_artifact_ = MakeGarbageCollected<PaintArtifact>();
  PaintChunker paint_chunker_;
  HeapVector<Member<const DisplayItemClient>> clients_to_validate_;

  // Saves the original persistent_data_->current_paint_artifact_ after calling
  // CommitNewDisplayItems. The data will be cleared when this PaintController
  // is destructed to reduce GC overhead.
  PaintArtifact* old_paint_artifact_ = nullptr;

  // A stack recording current frames' first paints.
  Vector<FrameFirstPaint> frame_first_paints_;

  bool record_debug_info_ = false;

  unsigned skipping_cache_count_ = 0;

  wtf_size_t num_cached_new_items_ = 0;
  wtf_size_t num_cached_new_subsequences_ = 0;

  // Maps from ids to indices of valid cacheable display items in
  // current_paint_artifact_.GetDisplayItemList() that have not been matched by
  // requests of cached display items (using UseCachedItemIfPossible() and
  // UseCachedSubsequenceIfPossible()) during sequential matching. The indexed
  // items will be matched by later out-of-order requests of cached display
  // items. This ensures that when out-of-order cached display items are
  // requested, we only traverse at most once over the current display list
  // looking for potential matches. Thus we can ensure that the algorithm runs
  // in linear time.
  IdIndexMap out_of_order_item_id_index_map_;

  wtf_size_t current_fragment_ = 0;

  // The next item in the current list for sequential match.
  wtf_size_t next_item_to_match_ = 0;

  // The next item in the current list to be indexed for out-of-order cache
  // requests.
  wtf_size_t next_item_to_index_ = 0;

  wtf_size_t last_matching_item_ = kNotFound;
  PaintInvalidationReason last_matching_client_invalidation_reason_ =
      PaintInvalidationReason::kNone;

#if DCHECK_IS_ON()
  wtf_size_t num_indexed_items_ = 0;
  wtf_size_t num_sequential_matches_ = 0;
  wtf_size_t num_out_of_order_matches_ = 0;

  // This is used to check duplicated ids during CreateAndAppend().
  IdIndexMap new_display_item_id_index_map_;
  // This is used to check duplicated ids for new paint chunks.
  IdIndexMap new_paint_chunk_id_index_map_;

  DisplayItem::Id::HashKey last_checked_cached_item_id_;
#endif

  const PaintBenchmarkMode benchmark_mode_;

  std::optional<PaintUnderInvalidationChecker> under_invalidation_checker_;

  SubsequencesData new_subsequences_;

  static CounterForTesting* counter_for_testing_;

  class PaintArtifactAsJSON;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CONTROLLER_H_
