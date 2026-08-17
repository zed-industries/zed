// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_PROPERTY_TREE_BUILDER_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_PROPERTY_TREE_BUILDER_TEST_H_

#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/core/paint/paint_controller_paint_test.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/testing/unit_test_helpers.h"

namespace blink {

class ClipPaintPropertyNode;
class GeometryMapperTransformCache;
class ScrollPaintPropertyNode;
class TransformPaintPropertyNode;

class PaintPropertyTreeBuilderTest : public PaintControllerPaintTest {
 public:
  PaintPropertyTreeBuilderTest()
      : PaintControllerPaintTest(
            MakeGarbageCollected<SingleChildLocalFrameClient>()) {}

 protected:
  void LoadTestData(const char* file_name);

  // These helpers return paint property nodes associated with a document (the
  // main frame's document if not otherwise specified).
  const TransformPaintPropertyNode* DocPreTranslation(
      const Document* = nullptr);
  const TransformPaintPropertyNode* DocScrollTranslation(
      const Document* = nullptr);
  const ClipPaintPropertyNode* DocContentClip(const Document* = nullptr);
  const ScrollPaintPropertyNode* DocScroll(const Document* = nullptr);

  // Return the local border box's paint offset. For more details, see
  // ObjectPaintProperties::localBorderBoxProperties().
  PhysicalOffset PaintOffset(const LayoutObject*);

  const ObjectPaintProperties* PaintPropertiesForElement(const char* name);

  const GeometryMapperTransformCache& GetTransformCache(
      const TransformPaintPropertyNode&);

  static unsigned NumFragments(const LayoutObject* obj) {
    return obj->FragmentList().size();
  }

  static const FragmentData& FragmentAt(const LayoutObject* obj,
                                        unsigned index) {
    return obj->FragmentList().at(index);
  }

 private:
  void SetUp() override;
};

// Used when LayoutClipRect and PaintClipRect are the same.
// |expected_arg| can be gfx::RectF or FloatRoundedRect.
#define EXPECT_CLIP_RECT(expected_arg, clip_node)                     \
  do {                                                                \
    FloatRoundedRect expected((expected_arg));                        \
    ASSERT_TRUE(clip_node);                                           \
    EXPECT_EQ(expected.Rect(), (clip_node)->LayoutClipRect().Rect()); \
    EXPECT_EQ(expected.IsRounded(),                                   \
              (clip_node)->LayoutClipRect().HasRadius());             \
    EXPECT_EQ(expected, (clip_node)->PaintClipRect());                \
  } while (false)

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_PROPERTY_TREE_BUILDER_TEST_H_
