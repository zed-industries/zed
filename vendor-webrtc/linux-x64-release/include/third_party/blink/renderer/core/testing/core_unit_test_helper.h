// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_CORE_UNIT_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_CORE_UNIT_TEST_HELPER_H_

#include <gtest/gtest.h>

#include <memory>

#include "cc/layers/layer.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/core/frame/local_frame_client.h"
#include "third_party/blink/renderer/core/frame/local_frame_view.h"
#include "third_party/blink/renderer/core/frame/settings.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_rect.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/hit_test_result.h"
#include "third_party/blink/renderer/core/layout/inline/inline_node.h"
#include "third_party/blink/renderer/core/layout/layout_view.h"
#include "third_party/blink/renderer/core/loader/empty_clients.h"
#include "third_party/blink/renderer/core/testing/page_test_base.h"
#include "third_party/blink/renderer/platform/testing/geometry_test_helpers.h"
#include "third_party/blink/renderer/platform/testing/layer_tree_host_embedder.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PaintLayer;

class SingleChildLocalFrameClient final : public EmptyLocalFrameClient {
 public:
  explicit SingleChildLocalFrameClient() = default;

  void Trace(Visitor* visitor) const override {
    EmptyLocalFrameClient::Trace(visitor);
  }

  // LocalFrameClient overrides:
  LocalFrame* CreateFrame(const AtomicString& name,
                          HTMLFrameOwnerElement*) override;
};

class LocalFrameClientWithParent final : public EmptyLocalFrameClient {
 public:
  explicit LocalFrameClientWithParent(LocalFrame* parent) : parent_(parent) {}

  void Trace(Visitor* visitor) const override {
    visitor->Trace(parent_);
    EmptyLocalFrameClient::Trace(visitor);
  }

  // FrameClient overrides:
  void Detached(FrameDetachType) override;

 private:
  Member<LocalFrame> parent_;
};

// RenderingTestChromeClient ensures that we have a LayerTreeHost which allows
// testing property tree creation.
class RenderingTestChromeClient : public EmptyChromeClient {
 public:
  void SetUp() {
    // Runtime flags can affect LayerTreeHost's settings so this needs to be
    // recreated for each test.
    layer_tree_ = std::make_unique<LayerTreeHostEmbedder>();
    device_emulation_transform_ = gfx::Transform();
  }

  bool HasLayer(const cc::Layer& layer) {
    return layer.layer_tree_host() == layer_tree_->layer_tree_host();
  }

  void AttachRootLayer(scoped_refptr<cc::Layer> layer,
                       LocalFrame* local_root) override {
    layer_tree_->layer_tree_host()->SetRootLayer(std::move(layer));
  }

  cc::LayerTreeHost* layer_tree_host() {
    return layer_tree_->layer_tree_host();
  }

  void SetDeviceEmulationTransform(const gfx::Transform& t) {
    device_emulation_transform_ = t;
  }
  gfx::Transform GetDeviceEmulationTransform() const override {
    return device_emulation_transform_;
  }

  void InjectScrollbarGestureScroll(
      LocalFrame& local_frame,
      const gfx::Vector2dF& delta,
      ui::ScrollGranularity granularity,
      CompositorElementId scrollable_area_element_id,
      WebInputEvent::Type injected_type) override;

  void ScheduleAnimation(const LocalFrameView*,
                         base::TimeDelta,
                         bool) override {
    animation_scheduled_ = true;
  }
  bool AnimationScheduled() const { return animation_scheduled_; }
  void UnsetAnimationScheduled() { animation_scheduled_ = false; }

 private:
  std::unique_ptr<LayerTreeHostEmbedder> layer_tree_;
  gfx::Transform device_emulation_transform_;
  bool animation_scheduled_ = false;
};

class RenderingTest : public PageTestBase {
  USING_FAST_MALLOC(RenderingTest);

 public:
  RenderingTest(base::test::TaskEnvironment::TimeSource time_source);
  virtual FrameSettingOverrideFunction SettingOverrider() const {
    return nullptr;
  }
  virtual RenderingTestChromeClient& GetChromeClient() const;

  explicit RenderingTest(LocalFrameClient* = nullptr);

  const Node* HitTest(int x, int y);
  const HitTestResult::NodeSet& RectBasedHitTest(const PhysicalRect& rect);

 protected:
  void SetUp() override;
  void TearDown() override;

  LayoutView& GetLayoutView() const {
    return *GetDocument().View()->GetLayoutView();
  }

  LocalFrame& ChildFrame() {
    return *To<LocalFrame>(GetFrame().Tree().FirstChild());
  }
  Document& ChildDocument() { return *ChildFrame().GetDocument(); }

  void SetChildFrameHTML(const String&);

  void RunDocumentLifecycle() {
    GetDocument().View()->SetParentVisible(true);
    GetDocument().View()->SetSelfVisible(true);
    UpdateAllLifecyclePhasesForTest();
  }

  LayoutObject* GetLayoutObjectByElementId(const char* id) const {
    const auto* element = GetElementById(id);
    return element ? element->GetLayoutObject() : nullptr;
  }

  LayoutBox* GetLayoutBoxByElementId(const char* id) const {
    return To<LayoutBox>(GetLayoutObjectByElementId(id));
  }

  LayoutBlockFlow* GetLayoutBlockFlowByElementId(const char* id) const {
    return To<LayoutBlockFlow>(GetLayoutObjectByElementId(id));
  }

  InlineNode GetInlineNodeByElementId(const char* id) const {
    return InlineNode(GetLayoutBlockFlowByElementId(id));
  }

  PaintLayer* GetPaintLayerByElementId(const char* id) {
    return To<LayoutBoxModelObject>(GetLayoutObjectByElementId(id))->Layer();
  }

  const DisplayItemClient* GetDisplayItemClientFromLayoutObject(
      LayoutObject* obj) const {
    return obj;
  }

  const DisplayItemClient* GetDisplayItemClientFromElementId(
      const char* id) const {
    return GetDisplayItemClientFromLayoutObject(GetLayoutObjectByElementId(id));
  }

  // Create a `ConstraintSpace` for the given available inline size. The
  // available block sizes is `LayoutUnit::Max()`.
  ConstraintSpace ConstraintSpaceForAvailableSize(LayoutUnit inline_size) const;

 private:
  Persistent<LocalFrameClient> local_frame_client_;
};

// These constructors are for convenience of tests to construct these geometries
// from integers.
constexpr LogicalOffset::LogicalOffset(int inline_offset, int block_offset)
    : inline_offset(inline_offset), block_offset(block_offset) {}
constexpr LogicalSize::LogicalSize(int inline_size, int block_size)
    : inline_size(inline_size), block_size(block_size) {}
constexpr LogicalRect::LogicalRect(int inline_offset,
                                   int block_offset,
                                   int inline_size,
                                   int block_size)
    : offset(inline_offset, block_offset), size(inline_size, block_size) {}
constexpr PhysicalRect::PhysicalRect(int left, int top, int width, int height)
    : offset(left, top), size(width, height) {}

// Returns the rect that should have raster invalidated whenever this object
// changes. The rect is in the coordinate space of the document's scrolling
// contents. This method deals with outlines and overflow.
PhysicalRect VisualRectInDocument(const LayoutObject& object,
                                  VisualRectFlags = kDefaultVisualRectFlags);

// Returns the rect that should have raster invalidated whenever the specified
// object changes. The rect is in the object's local physical coordinate space.
// This is for non-SVG objects and LayoutSVGRoot only. SVG objects (except
// LayoutSVGRoot) should use VisualRectInLocalSVGCoordinates() and map with
// SVG transforms instead.
PhysicalRect LocalVisualRect(const LayoutObject& object);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_CORE_UNIT_TEST_HELPER_H_
