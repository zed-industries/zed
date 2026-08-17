// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_CONTROLLER_PAINT_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_CONTROLLER_PAINT_TEST_H_

#include "base/check_op.h"
#include "cc/paint/paint_op.h"
#include "cc/paint/paint_op_buffer_iterator.h"
#include "third_party/blink/renderer/core/dom/events/add_event_listener_options_resolved.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/core/editing/frame_selection.h"
#include "third_party/blink/renderer/core/frame/local_frame_view.h"
#include "third_party/blink/renderer/core/layout/layout_view.h"
#include "third_party/blink/renderer/core/paint/cull_rect_updater.h"
#include "third_party/blink/renderer/core/paint/paint_layer.h"
#include "third_party/blink/renderer/core/paint/paint_layer_scrollable_area.h"
#include "third_party/blink/renderer/core/testing/core_unit_test_helper.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/graphics/paint/cull_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_chunk_subset.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_controller_test.h"
#include "third_party/blink/renderer/platform/testing/paint_test_configurations.h"

namespace blink {

class PaintControllerPaintTestBase : public RenderingTest {
 public:
  PaintControllerPaintTestBase(LocalFrameClient* local_frame_client = nullptr)
      : RenderingTest(local_frame_client) {}

 protected:
  LayoutView& GetLayoutView() const { return *GetDocument().GetLayoutView(); }
  PaintControllerPersistentData& GetPersistentData() const {
    return GetDocument().View()->GetPaintControllerPersistentDataForTesting();
  }

  void SetUp() override {
    EnableCompositing();
    RenderingTest::SetUp();
  }

  const DisplayItemClient& ViewScrollingBackgroundClient() {
    return GetLayoutView()
        .GetScrollableArea()
        ->GetScrollingBackgroundDisplayItemClient();
  }

  void UpdateAllLifecyclePhasesExceptPaint(bool update_cull_rects = true) {
    GetDocument().View()->UpdateAllLifecyclePhasesExceptPaint(
        DocumentUpdateReason::kTest);
    if (update_cull_rects) {
      // Run CullRectUpdater to ease testing of cull rects and repaint flags of
      // PaintLayers on cull rect change.
      UpdateCullRects();
    }
  }

  void UpdateCullRects() {
    DCHECK_EQ(GetDocument().Lifecycle().GetState(),
              DocumentLifecycle::kPrePaintClean);
    CullRectUpdater(*GetLayoutView().Layer()).Update();
  }

  void PaintContents(const gfx::Rect& interest_rect) {
    GetDocument().View()->PaintForTest(CullRect(interest_rect));
  }

  void InvalidateAll() {
    GetPersistentData().InvalidateAllForTesting();
    GetLayoutView().Layer()->SetNeedsRepaint();
  }

  bool ClientCacheIsValid(const DisplayItemClient& client) {
    return GetPersistentData().ClientCacheIsValid(client);
  }

  const SubsequenceMarkers* GetSubsequenceMarkers(
      const DisplayItemClient& client) {
    return GetPersistentData().GetSubsequenceMarkers(client.Id());
  }

  static bool IsNotContentType(DisplayItem::Type type) {
    return type == DisplayItem::kFrameOverlay ||
           type == DisplayItem::kForeignLayerLinkHighlight ||
           type == DisplayItem::kForeignLayerViewportScroll ||
           type == DisplayItem::kForeignLayerViewportScrollbar;
  }

  // Excludes display items for LayoutView non-scrolling background, visual
  // viewport, overlays, etc. Includes LayoutView scrolling background.
  DisplayItemRange ContentDisplayItems() {
    const auto& display_item_list = GetPersistentData().GetDisplayItemList();
    wtf_size_t begin_index = 0;
    wtf_size_t end_index = display_item_list.size();
    while (begin_index < end_index &&
           UNSAFE_TODO(display_item_list[begin_index]).ClientId() ==
               GetLayoutView().Id()) {
      begin_index++;
    }
    while (end_index > begin_index &&
           IsNotContentType(
               UNSAFE_TODO(display_item_list[end_index - 1]).GetType())) {
      end_index--;
    }
    return UNSAFE_TODO(display_item_list.ItemsInRange(begin_index, end_index));
  }

  // Excludes paint chunks for LayoutView non-scrolling background and scroll
  // hit test, visual viewport, overlays, etc. Includes LayoutView scrolling
  // background.
  PaintChunkSubset ContentPaintChunks() {
    const auto& chunks = GetPersistentData().GetPaintChunks();
    wtf_size_t begin_index = 0;
    wtf_size_t end_index = chunks.size();
    while (begin_index < end_index) {
      DisplayItemClientId client_id = chunks[begin_index].id.client_id;
      if (client_id != GetLayoutView().Id() &&
          client_id != GetLayoutView().Layer()->Id()) {
        break;
      }
      begin_index++;
    }
    while (end_index > begin_index &&
           IsNotContentType(chunks[end_index - 1].id.type)) {
      end_index--;
    }
    const auto& artifact = GetPersistentData().GetPaintArtifact();
    PaintChunkSubset subset(artifact, chunks[begin_index]);
    for (wtf_size_t i = begin_index + 1; i < end_index; i++) {
      subset.Merge(PaintChunkSubset(artifact, chunks[i]));
    }
    return subset;
  }

  class MockEventListener final : public NativeEventListener {
   public:
    void Invoke(ExecutionContext*, Event*) override {}
  };

  void SetWheelEventListener(const char* element_id) {
    auto* element = GetDocument().getElementById(AtomicString(element_id));
    auto* listener = MakeGarbageCollected<MockEventListener>();
    auto* resolved_options =
        MakeGarbageCollected<AddEventListenerOptionsResolved>();
    resolved_options->setPassive(false);
    element->addEventListener(event_type_names::kWheel, listener,
                              resolved_options);
    UpdateAllLifecyclePhasesForTest();
  }
};

class PaintControllerPaintTest : public PaintTestConfigurations,
                                 public PaintControllerPaintTestBase {
 public:
  PaintControllerPaintTest(LocalFrameClient* local_frame_client = nullptr)
      : PaintControllerPaintTestBase(local_frame_client) {}
};

// Shorter names for frequently used display item types in core/ tests.
const DisplayItem::Type kBackgroundChunkType =
    DisplayItem::PaintPhaseToDrawingType(PaintPhase::kBlockBackground);
const DisplayItem::Type kHitTestChunkType =
    DisplayItem::PaintPhaseToDrawingType(PaintPhase::kSelfBlockBackgroundOnly);
const DisplayItem::Type kScrollingBackgroundChunkType =
    DisplayItem::PaintPhaseToClipType(PaintPhase::kSelfBlockBackgroundOnly);
const DisplayItem::Type kClippedContentsBackgroundChunkType =
    DisplayItem::PaintPhaseToClipType(
        PaintPhase::kDescendantBlockBackgroundsOnly);

#define VIEW_SCROLLING_BACKGROUND_DISPLAY_ITEM   \
  IsSameId(ViewScrollingBackgroundClient().Id(), \
           DisplayItem::kDocumentBackground)

// Checks for view scrolling background chunk in common case that there is only
// one display item in the chunk and no hit test rects.
#define VIEW_SCROLLING_BACKGROUND_CHUNK_COMMON                      \
  IsPaintChunk(0, 1,                                                \
               PaintChunk::Id(ViewScrollingBackgroundClient().Id(), \
                              DisplayItem::kDocumentBackground),    \
               GetLayoutView().FirstFragment().ContentsProperties())

// This version also checks the following additional parameters:
//   wtf_size_t display_item_count,
//   const HitTestData* hit_test_data,
//   (optional) const gfx::Rect& bounds
#define VIEW_SCROLLING_BACKGROUND_CHUNK(display_item_count, ...)     \
  IsPaintChunk(0, display_item_count,                                \
               PaintChunk::Id(ViewScrollingBackgroundClient().Id(),  \
                              DisplayItem::kDocumentBackground),     \
               GetLayoutView().FirstFragment().ContentsProperties(), \
               __VA_ARGS__)

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_CONTROLLER_PAINT_TEST_H_
