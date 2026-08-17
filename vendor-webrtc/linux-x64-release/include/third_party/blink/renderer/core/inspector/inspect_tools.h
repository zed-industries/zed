// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECT_TOOLS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECT_TOOLS_H_

#include <vector>

#include <v8-inspector.h>
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/inspector/inspector_overlay_agent.h"
#include "third_party/blink/renderer/core/inspector/node_content_visibility_state.h"

namespace blink {

class WebMouseEvent;
class WebPointerEvent;

// -----------------------------------------------------------------------------

class SearchingForNodeTool : public InspectTool {
 public:
  SearchingForNodeTool(InspectorOverlayAgent* overlay,
                       OverlayFrontend* frontend,
                       InspectorDOMAgent* dom_agent,
                       bool ua_shadow,
                       const std::vector<uint8_t>& highlight_config);
  SearchingForNodeTool(const SearchingForNodeTool&) = delete;
  SearchingForNodeTool& operator=(const SearchingForNodeTool&) = delete;

  void Trace(Visitor* visitor) const override;

 private:
  bool HandleInputEvent(LocalFrameView* frame_view,
                        const WebInputEvent& input_event,
                        bool* swallow_next_mouse_up) override;
  bool HandleMouseDown(const WebMouseEvent& event,
                       bool* swallow_next_mouse_up) override;
  bool HandleMouseMove(const WebMouseEvent& event) override;
  bool HandleGestureTapEvent(const WebGestureEvent&) override;
  bool HandlePointerEvent(const WebPointerEvent&) override;
  void Draw(float scale) override;
  void NodeHighlightRequested(Node*);
  bool SupportsPersistentOverlays() override;
  String GetOverlayName() override;

  Member<InspectorDOMAgent> dom_agent_;
  bool ua_shadow_;

  NodeContentVisibilityState content_visibility_state_ =
      NodeContentVisibilityState::kNone;

  Member<Node> hovered_node_;
  Member<Node> event_target_node_;
  std::unique_ptr<InspectorHighlightConfig> highlight_config_;
  InspectorHighlightContrastInfo contrast_info_;
  bool omit_tooltip_ = false;
};

// -----------------------------------------------------------------------------

class QuadHighlightTool : public InspectTool {
 public:
  QuadHighlightTool(InspectorOverlayAgent* overlay,
                    OverlayFrontend* frontend,
                    std::unique_ptr<gfx::QuadF> quad,
                    Color color,
                    Color outline_color);
  QuadHighlightTool(const QuadHighlightTool&) = delete;
  QuadHighlightTool& operator=(const QuadHighlightTool&) = delete;

 private:
  bool ForwardEventsToOverlay() override;
  bool HideOnHideHighlight() override;
  void Draw(float scale) override;
  String GetOverlayName() override;
  std::unique_ptr<gfx::QuadF> quad_;
  Color color_;
  Color outline_color_;
};

// -----------------------------------------------------------------------------

class NodeHighlightTool : public InspectTool {
 public:
  NodeHighlightTool(InspectorOverlayAgent* overlay,
                    OverlayFrontend* frontend,
                    Member<Node> node,
                    String selector_list,
                    std::unique_ptr<InspectorHighlightConfig> highlight_config);
  NodeHighlightTool(const NodeHighlightTool&) = delete;
  NodeHighlightTool& operator=(const NodeHighlightTool&) = delete;

  std::unique_ptr<protocol::DictionaryValue> GetNodeInspectorHighlightAsJson(
      bool append_element_info,
      bool append_distance_info) const;

  void Trace(Visitor* visitor) const override;

 private:
  bool ForwardEventsToOverlay() override;
  bool SupportsPersistentOverlays() override;
  bool HideOnMouseMove() override;
  bool HideOnHideHighlight() override;
  void Draw(float scale) override;
  void DrawNode();
  void DrawMatchingSelector();
  String GetOverlayName() override;

  NodeContentVisibilityState content_visibility_state_ =
      NodeContentVisibilityState::kNone;
  Member<Node> node_;
  String selector_list_;
  std::unique_ptr<InspectorHighlightConfig> highlight_config_;
  InspectorHighlightContrastInfo contrast_info_;
};

// -----------------------------------------------------------------------------

class SourceOrderTool : public InspectTool {
 public:
  SourceOrderTool(
      InspectorOverlayAgent* overlay,
      OverlayFrontend* frontend,
      Node* node,
      std::unique_ptr<InspectorSourceOrderConfig> source_order_config);
  SourceOrderTool(const SourceOrderTool&) = delete;
  SourceOrderTool& operator=(const SourceOrderTool&) = delete;
  std::unique_ptr<protocol::DictionaryValue>
  GetNodeInspectorSourceOrderHighlightAsJson() const;

  void Trace(Visitor* visitor) const override;

 private:
  bool HideOnHideHighlight() override;
  bool HideOnMouseMove() override;
  void Draw(float scale) override;
  void DrawNode(Node* node, int source_order_position);
  void DrawParentNode();
  String GetOverlayName() override;

  Member<Node> node_;
  std::unique_ptr<InspectorSourceOrderConfig> source_order_config_;
};

// -----------------------------------------------------------------------------
using GridConfigs = HeapHashMap<WeakMember<Node>,
                                std::unique_ptr<InspectorGridHighlightConfig>>;
using FlexContainerConfigs =
    HeapHashMap<WeakMember<Node>,
                std::unique_ptr<InspectorFlexContainerHighlightConfig>>;
using ScrollSnapConfigs =
    HeapHashMap<WeakMember<Node>,
                std::unique_ptr<InspectorScrollSnapContainerHighlightConfig>>;
using ContainerQueryConfigs = HeapHashMap<
    WeakMember<Node>,
    std::unique_ptr<InspectorContainerQueryContainerHighlightConfig>>;
using IsolatedElementConfigs =
    HeapHashMap<WeakMember<Element>,
                std::unique_ptr<InspectorIsolationModeHighlightConfig>>;

class PersistentTool : public InspectTool {
  using InspectTool::InspectTool;

 public:
  PersistentTool(const PersistentTool&) = delete;
  PersistentTool& operator=(const PersistentTool&) = delete;

  void Draw(float scale) override;
  bool IsEmpty();
  void SetGridConfigs(GridConfigs);
  void SetFlexContainerConfigs(FlexContainerConfigs);
  void SetScrollSnapConfigs(ScrollSnapConfigs);
  void SetContainerQueryConfigs(ContainerQueryConfigs);
  void SetIsolatedElementConfigs(IsolatedElementConfigs);

  std::unique_ptr<protocol::DictionaryValue> GetGridInspectorHighlightsAsJson()
      const;

  void Trace(Visitor* visitor) const override;

 private:
  bool ForwardEventsToOverlay() override;
  bool HideOnMouseMove() override;
  bool HideOnHideHighlight() override;
  String GetOverlayName() override;
  void Dispatch(const ScriptValue& message,
                ExceptionState& exception_state) override;

  GridConfigs grid_node_highlights_;
  FlexContainerConfigs flex_container_configs_;
  ScrollSnapConfigs scroll_snap_configs_;
  ContainerQueryConfigs container_query_configs_;
  IsolatedElementConfigs isolated_element_configs_;
};

// -----------------------------------------------------------------------------

class NearbyDistanceTool : public InspectTool {
 public:
  NearbyDistanceTool(const NearbyDistanceTool&) = delete;
  NearbyDistanceTool& operator=(const NearbyDistanceTool&) = delete;
  void Trace(Visitor* visitor) const override;

 private:
  using InspectTool::InspectTool;

  bool HandleMouseDown(const WebMouseEvent& event,
                       bool* swallow_next_mouse_up) override;
  bool HandleMouseMove(const WebMouseEvent& event) override;
  bool HandleMouseUp(const WebMouseEvent& event) override;
  void Draw(float scale) override;
  String GetOverlayName() override;

  Member<Node> hovered_node_;
};

// -----------------------------------------------------------------------------

class ShowViewSizeTool : public InspectTool {
  using InspectTool::InspectTool;

 public:
  ShowViewSizeTool(const ShowViewSizeTool&) = delete;
  ShowViewSizeTool& operator=(const ShowViewSizeTool&) = delete;

 private:
  bool ForwardEventsToOverlay() override;
  void Draw(float scale) override;
  String GetOverlayName() override;
};

// -----------------------------------------------------------------------------

class ScreenshotTool : public InspectTool {
 public:
  ScreenshotTool(InspectorOverlayAgent* overlay, OverlayFrontend* frontend);
  ScreenshotTool(const ScreenshotTool&) = delete;
  ScreenshotTool& operator=(const ScreenshotTool&) = delete;

 private:
  void Dispatch(const ScriptValue& message,
                ExceptionState& exception_state) override;
  String GetOverlayName() override;
};

// -----------------------------------------------------------------------------

class PausedInDebuggerTool : public InspectTool {
 public:
  PausedInDebuggerTool(InspectorOverlayAgent* overlay,
                       OverlayFrontend* frontend,
                       v8_inspector::V8InspectorSession* v8_session,
                       const String& message)
      : InspectTool(overlay, frontend),
        v8_session_(v8_session),
        message_(message) {}
  PausedInDebuggerTool(const PausedInDebuggerTool&) = delete;
  PausedInDebuggerTool& operator=(const PausedInDebuggerTool&) = delete;

 private:
  void Draw(float scale) override;
  void Dispatch(const ScriptValue& message,
                ExceptionState& exception_state) override;
  String GetOverlayName() override;
  v8_inspector::V8InspectorSession* v8_session_;
  String message_;
};

// -----------------------------------------------------------------------------

class WindowControlsOverlayTool : public InspectTool {
 public:
  WindowControlsOverlayTool(
      InspectorOverlayAgent* overlay,
      OverlayFrontend* frontend,
      std::unique_ptr<protocol::DictionaryValue> wco_config);
  WindowControlsOverlayTool(const WindowControlsOverlayTool&) = delete;
  WindowControlsOverlayTool& operator=(const WindowControlsOverlayTool&) =
      delete;

 private:
  void Draw(float scale) override;
  String GetOverlayName() override;

  std::unique_ptr<protocol::DictionaryValue> wco_config_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECT_TOOLS_H_
