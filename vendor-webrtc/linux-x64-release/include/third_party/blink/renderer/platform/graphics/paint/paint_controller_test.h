// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CONTROLLER_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CONTROLLER_TEST_H_

#include "base/auto_reset.h"
#include "base/dcheck_is_on.h"
#include "base/functional/function_ref.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/platform/graphics/paint/drawing_recorder.h"
#include "third_party/blink/renderer/platform/graphics/paint/hit_test_data.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_controller.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/testing/fake_display_item_client.h"
#include "third_party/blink/renderer/platform/testing/paint_property_test_helpers.h"

namespace blink {

class GraphicsContext;

class PaintControllerForTest : public PaintController {
 public:
  explicit PaintControllerForTest()
      : PaintController(/*record_debug_info=*/true) {}
  explicit PaintControllerForTest(
      PaintControllerPersistentData& persistent_data)
      : PaintController(/*record_debug_info=*/true, &persistent_data) {}
};

class AutoCommitPaintController : public PaintControllerForTest {
 public:
  using PaintControllerForTest::PaintControllerForTest;
  ~AutoCommitPaintController() { CommitNewDisplayItems(); }
};

class PaintControllerTestBase : public testing::Test {
 public:
  enum DrawResult {
    kCached,
    kPaintedNew,
    kRepaintedCachedItem,
  };

  static DrawResult Draw(GraphicsContext& context,
                         const DisplayItemClient& client,
                         DisplayItem::Type type,
                         base::FunctionRef<void()> draw_function);

  static DrawResult DrawNothing(GraphicsContext& context,
                                const DisplayItemClient& client,
                                DisplayItem::Type type) {
    return Draw(context, client, type, [&]() {
      DrawingRecorder recorder(context, client, type, gfx::Rect());
    });
  }

  static DrawResult DrawRect(GraphicsContext& context,
                             const DisplayItemClient& client,
                             DisplayItem::Type type,
                             const gfx::Rect& bounds) {
    return Draw(context, client, type, [&]() {
      DrawingRecorder recorder(context, client, type, bounds);
      context.FillRect(bounds, AutoDarkMode::Disabled());
    });
  }

 protected:
  PaintControllerTestBase()
      : root_paint_property_client_(
            MakeGarbageCollected<FakeDisplayItemClient>("root")),
        root_paint_chunk_id_(root_paint_property_client_->Id(),
                             DisplayItem::kUninitializedType) {}

  void SetUp() override { GTEST_FLAG_SET(death_test_style, "threadsafe"); }

  void InitRootChunk(PaintController& paint_controller) const {
    paint_controller.UpdateCurrentPaintChunkProperties(
        root_paint_chunk_id_, *root_paint_property_client_,
        DefaultPaintChunkProperties());
  }
  const PaintChunk::Id DefaultRootChunkId() const {
    return root_paint_chunk_id_;
  }

  PaintControllerPersistentData& GetPersistentData() {
    CHECK(persistent_data_);
    return *persistent_data_;
  }

  static const PaintArtifact& GetNewPaintArtifact(
      const PaintController& controller) {
    // This can only be called before CommitNewDisplayItems().
    DCHECK(controller.new_paint_artifact_);
    return *controller.new_paint_artifact_;
  }

  static wtf_size_t NumCachedNewItems(const PaintController& controller) {
    return controller.num_cached_new_items_;
  }
  static wtf_size_t NumCachedNewSubsequences(
      const PaintController& controller) {
    return controller.num_cached_new_subsequences_;
  }
#if DCHECK_IS_ON()
  static wtf_size_t NumIndexedItems(const PaintController& controller) {
    return controller.num_indexed_items_;
  }
  static wtf_size_t NumSequentialMatches(const PaintController& controller) {
    return controller.num_sequential_matches_;
  }
  static wtf_size_t NumOutOfOrderMatches(const PaintController& controller) {
    return controller.num_out_of_order_matches_;
  }
#endif

  void InvalidateAll() { persistent_data_->InvalidateAllForTesting(); }

  const SubsequenceMarkers* GetSubsequenceMarkers(
      const DisplayItemClient& client) {
    return persistent_data_->GetSubsequenceMarkers(client.Id());
  }

  bool ClientCacheIsValid(const DisplayItemClient& client) const {
    return persistent_data_->ClientCacheIsValid(client);
  }

  static bool ClientCacheIsValid(const PaintController& paint_controller,
                                 const DisplayItemClient& client) {
    return paint_controller.ClientCacheIsValid(client);
  }

 private:
  Persistent<FakeDisplayItemClient> root_paint_property_client_;
  PaintChunk::Id root_paint_chunk_id_;
  Persistent<PaintControllerPersistentData> persistent_data_ =
      MakeGarbageCollected<PaintControllerPersistentData>();
};

// Matcher for checking display item list. Sample usage:
// EXPECT_THAT(paint_controller.GetDisplayItemList(),
//             ElementsAre(IsSameId(client1, kBackgroundType),
//                         IsSameId(client2, kForegroundType)));
MATCHER_P(IsSameId, id, "") {
  return arg.GetId() == id;
}
MATCHER_P2(IsSameId, client_id, type, "") {
  return arg.GetId() == DisplayItem::Id(client_id, type);
}
MATCHER_P3(IsSameId, client_id, type, fragment, "") {
  return arg.GetId() == DisplayItem::Id(client_id, type, fragment);
}

// Matcher for checking paint chunks. Sample usage:
// EXPACT_THAT(paint_controller.GetPaintChunks(),
//             ELementsAre(IsPaintChunk(0, 1, id1, properties1),
//                         IsPaintChunk(1, 3, id2, properties2)));
inline bool CheckChunk(const PaintChunk& chunk,
                       wtf_size_t begin,
                       wtf_size_t end) {
  return chunk.begin_index == begin && chunk.end_index == end;
}
inline bool CheckChunk(const PaintChunk& chunk,
                       wtf_size_t begin,
                       wtf_size_t end,
                       const PaintChunk::Id& id,
                       const PropertyTreeStateOrAlias& properties,
                       const HitTestData* hit_test_data = nullptr,
                       const gfx::Rect* bounds = nullptr) {
  return chunk.begin_index == begin && chunk.end_index == end &&
         chunk.id == id && chunk.properties == properties &&
         ((!chunk.hit_test_data && !hit_test_data) ||
          (chunk.hit_test_data && hit_test_data &&
           *chunk.hit_test_data == *hit_test_data)) &&
         (!bounds || chunk.bounds == *bounds);
}
MATCHER_P2(IsPaintChunk, begin, end, "") {
  return CheckChunk(arg, begin, end);
}
MATCHER_P4(IsPaintChunk, begin, end, id, properties, "") {
  return CheckChunk(arg, begin, end, id, properties);
}
MATCHER_P5(IsPaintChunk, begin, end, id, properties, hit_test_data, "") {
  return CheckChunk(arg, begin, end, id, properties, hit_test_data);
}
MATCHER_P6(IsPaintChunk,
           begin,
           end,
           id,
           properties,
           hit_test_data,
           bounds,
           "") {
  return CheckChunk(arg, begin, end, id, properties, hit_test_data, &bounds);
}

// Shorter names for frequently used display item types in tests.
const DisplayItem::Type kBackgroundType = DisplayItem::kBoxDecorationBackground;
const DisplayItem::Type kForegroundType =
    static_cast<DisplayItem::Type>(DisplayItem::kDrawingPaintPhaseFirst + 5);
const DisplayItem::Type kClipType = DisplayItem::kClipPaintPhaseFirst;

#define EXPECT_SUBSEQUENCE(client, expected_start_chunk_index,     \
                           expected_end_chunk_index)               \
  do {                                                             \
    auto* subsequence = GetSubsequenceMarkers(client);             \
    ASSERT_NE(nullptr, subsequence);                               \
    EXPECT_EQ(static_cast<wtf_size_t>(expected_start_chunk_index), \
              subsequence->start_chunk_index);                     \
    EXPECT_EQ(static_cast<wtf_size_t>(expected_end_chunk_index),   \
              subsequence->end_chunk_index);                       \
  } while (false)

#define EXPECT_SUBSEQUENCE_FROM_CHUNK(client, start_chunk_iterator, \
                                      chunk_count)                  \
  EXPECT_SUBSEQUENCE(                                               \
      client, (start_chunk_iterator).IndexInPaintArtifact(),        \
      (start_chunk_iterator).IndexInPaintArtifact() + chunk_count)

#define EXPECT_NO_SUBSEQUENCE(client) \
  EXPECT_EQ(nullptr, GetSubsequenceMarkers(client))

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CONTROLLER_TEST_H_
