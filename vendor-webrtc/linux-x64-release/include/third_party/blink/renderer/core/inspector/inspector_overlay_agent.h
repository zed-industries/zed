/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_OVERLAY_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_OVERLAY_AGENT_H_

#include <v8-inspector.h>
#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/accessibility/ax_context.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/dom_node_ids.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_dom_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_highlight.h"
#include "third_party/blink/renderer/core/inspector/inspector_overlay_host.h"
#include "third_party/blink/renderer/core/inspector/protocol/overlay.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/quad_f.h"

namespace cc {
class Layer;
class LayerTreeDebugState;
}

namespace blink {

class FrameOverlay;
class GraphicsContext;
class InspectedFrames;
class InspectorDOMAgent;
class LocalFrame;
class LocalFrameView;
class Node;
class Page;
class WebGestureEvent;
class WebKeyboardEvent;
class WebMouseEvent;
class WebLocalFrameImpl;
class WebPointerEvent;

class InspectorOverlayAgent;
class PersistentTool;

using OverlayFrontend = protocol::Overlay::Metainfo::FrontendClass;

// Overlay names returned by GetOverlayName().
class OverlayNames {
 public:
  static const char* OVERLAY_HIGHLIGHT;
  static const char* OVERLAY_PERSISTENT;
  static const char* OVERLAY_SOURCE_ORDER;
  static const char* OVERLAY_DISTANCES;
  static const char* OVERLAY_VIEWPORT_SIZE;
  static const char* OVERLAY_SCREENSHOT;
  static const char* OVERLAY_PAUSED;
  static const char* OVERLAY_WINDOW_CONTROLS_OVERLAY;
};

class CORE_EXPORT InspectTool : public GarbageCollected<InspectTool> {
 public:
  InspectTool(InspectorOverlayAgent* overlay, OverlayFrontend* frontend)
      : overlay_(overlay), frontend_(frontend) {}
  virtual ~InspectTool() = default;

  virtual String GetOverlayName() = 0;
  virtual bool HandleInputEvent(LocalFrameView* frame_view,
                                const WebInputEvent& input_event,
                                bool* swallow_next_mouse_up);
  virtual bool HandleMouseEvent(const WebMouseEvent&,
                                bool* swallow_next_mouse_up);
  virtual bool HandleMouseDown(const WebMouseEvent&,
                               bool* swallow_next_mouse_up);
  virtual bool HandleMouseUp(const WebMouseEvent&);
  virtual bool HandleMouseMove(const WebMouseEvent&);
  virtual bool HandleGestureTapEvent(const WebGestureEvent&);
  virtual bool HandlePointerEvent(const WebPointerEvent&);
  virtual bool HandleKeyboardEvent(const WebKeyboardEvent&);
  virtual bool ForwardEventsToOverlay();
  virtual bool SupportsPersistentOverlays();
  virtual void Draw(float scale) {}
  virtual void Dispatch(const ScriptValue& message,
                        ExceptionState& exception_state) {}
  virtual void Trace(Visitor* visitor) const;
  virtual bool HideOnHideHighlight();
  virtual bool HideOnMouseMove();

 protected:
  Member<InspectorOverlayAgent> overlay_;
  OverlayFrontend* frontend_ = nullptr;
};

class CORE_EXPORT Hinge final : public GarbageCollected<Hinge> {
 public:
  Hinge(gfx::QuadF quad,
        Color color,
        Color outline_color,
        InspectorOverlayAgent* overlay);
  Hinge(const Hinge&) = delete;
  Hinge& operator=(const Hinge&) = delete;
  ~Hinge() = default;
  String GetOverlayName();
  void Draw(float scale);
  void Trace(Visitor* visitor) const;

 private:
  gfx::QuadF quad_;
  Color content_color_;
  Color outline_color_;
  Member<InspectorOverlayAgent> overlay_;
};

class CORE_EXPORT InspectorOverlayAgent final
    : public InspectorBaseAgent<protocol::Overlay::Metainfo>,
      public InspectorOverlayHost::Delegate {
 public:
  static std::unique_ptr<InspectorGridHighlightConfig> ToGridHighlightConfig(
      protocol::Overlay::GridHighlightConfig*);
  static std::unique_ptr<InspectorFlexContainerHighlightConfig>
  ToFlexContainerHighlightConfig(
      protocol::Overlay::FlexContainerHighlightConfig*);
  static std::unique_ptr<InspectorScrollSnapContainerHighlightConfig>
  ToScrollSnapContainerHighlightConfig(
      protocol::Overlay::ScrollSnapContainerHighlightConfig*);
  static std::unique_ptr<InspectorContainerQueryContainerHighlightConfig>
  ToContainerQueryContainerHighlightConfig(
      protocol::Overlay::ContainerQueryContainerHighlightConfig*);
  static std::unique_ptr<InspectorFlexItemHighlightConfig>
  ToFlexItemHighlightConfig(protocol::Overlay::FlexItemHighlightConfig*);
  static std::unique_ptr<InspectorIsolationModeHighlightConfig>
  ToIsolationModeHighlightConfig(
      protocol::Overlay::IsolationModeHighlightConfig*,
      int highlight_index);
  static std::optional<LineStyle> ToLineStyle(protocol::Overlay::LineStyle*);
  static std::optional<BoxStyle> ToBoxStyle(protocol::Overlay::BoxStyle*);
  static std::unique_ptr<InspectorHighlightConfig> ToHighlightConfig(
      protocol::Overlay::HighlightConfig*);
  InspectorOverlayAgent(WebLocalFrameImpl*,
                        InspectedFrames*,
                        v8_inspector::V8InspectorSession*,
                        InspectorDOMAgent*);
  InspectorOverlayAgent(const InspectorOverlayAgent&) = delete;
  InspectorOverlayAgent& operator=(const InspectorOverlayAgent&) = delete;
  ~InspectorOverlayAgent() override;
  void Trace(Visitor*) const override;

  // protocol::Dispatcher::OverlayCommandHandler implementation.
  protocol::Response enable() override;
  protocol::Response disable() override;
  protocol::Response setShowAdHighlights(bool) override;
  protocol::Response setShowPaintRects(bool) override;
  protocol::Response setShowLayoutShiftRegions(bool) override;
  protocol::Response setShowDebugBorders(bool) override;
  protocol::Response setShowFPSCounter(bool) override;
  protocol::Response setShowScrollBottleneckRects(bool) override;
  protocol::Response setShowHitTestBorders(bool) override;
  protocol::Response setShowWebVitals(bool) override;
  protocol::Response setShowViewportSizeOnResize(bool) override;
  protocol::Response setPausedInDebuggerMessage(std::optional<String>) override;
  protocol::Response setInspectMode(
      const String& mode,
      std::unique_ptr<protocol::Overlay::HighlightConfig>) override;
  protocol::Response highlightRect(
      int x,
      int y,
      int width,
      int height,
      std::unique_ptr<protocol::DOM::RGBA> color,
      std::unique_ptr<protocol::DOM::RGBA> outline_color) override;
  protocol::Response highlightQuad(
      std::unique_ptr<protocol::Array<double>> quad,
      std::unique_ptr<protocol::DOM::RGBA> color,
      std::unique_ptr<protocol::DOM::RGBA> outline_color) override;
  protocol::Response highlightNode(
      std::unique_ptr<protocol::Overlay::HighlightConfig>,
      std::optional<int> node_id,
      std::optional<int> backend_node_id,
      std::optional<String> object_id,
      std::optional<String> selector_list) override;
  protocol::Response highlightSourceOrder(
      std::unique_ptr<protocol::Overlay::SourceOrderConfig>,
      std::optional<int> node_id,
      std::optional<int> backend_node_id,
      std::optional<String> object_id) override;
  protocol::Response hideHighlight() override;
  protocol::Response highlightFrame(
      const String& frame_id,
      std::unique_ptr<protocol::DOM::RGBA> content_color,
      std::unique_ptr<protocol::DOM::RGBA> content_outline_color) override;
  protocol::Response getHighlightObjectForTest(
      int node_id,
      std::optional<bool> include_distance,
      std::optional<bool> include_style,
      std::optional<String> color_format,
      std::optional<bool> show_accessibility_info,
      std::unique_ptr<protocol::DictionaryValue>* highlight) override;
  protocol::Response getGridHighlightObjectsForTest(
      std::unique_ptr<protocol::Array<int>> node_ids,
      std::unique_ptr<protocol::DictionaryValue>* highlights) override;
  protocol::Response getSourceOrderHighlightObjectForTest(
      int node_id,
      std::unique_ptr<protocol::DictionaryValue>* highlights) override;
  protocol::Response setShowHinge(
      std::unique_ptr<protocol::Overlay::HingeConfig> hinge_config) override;
  protocol::Response setShowWindowControlsOverlay(
      std::unique_ptr<protocol::Overlay::WindowControlsOverlayConfig>
          wco_config) override;
  protocol::Response setShowGridOverlays(
      std::unique_ptr<
          protocol::Array<protocol::Overlay::GridNodeHighlightConfig>>
          grid_node_highlight_configs) override;
  protocol::Response setShowFlexOverlays(
      std::unique_ptr<
          protocol::Array<protocol::Overlay::FlexNodeHighlightConfig>>
          flex_node_highlight_configs) override;
  protocol::Response setShowScrollSnapOverlays(
      std::unique_ptr<
          protocol::Array<protocol::Overlay::ScrollSnapHighlightConfig>>
          scroll_snap_highlight_configs) override;
  protocol::Response setShowContainerQueryOverlays(
      std::unique_ptr<
          protocol::Array<protocol::Overlay::ContainerQueryHighlightConfig>>
          container_query_highlight_configs) override;
  protocol::Response setShowIsolatedElements(
      std::unique_ptr<
          protocol::Array<protocol::Overlay::IsolatedElementHighlightConfig>>
          isolated_element_highlight_configs) override;

  // InspectorBaseAgent overrides.
  void Restore() override;
  void Dispose() override;

  void Inspect(Node*);
  bool HasAXContext(Node*);
  void EnsureAXContext(Node*);
  void EnsureAXContext(Document&);
  void DispatchBufferedTouchEvents();
  void SetPageIsScrolling(bool is_scrolling);
  WebInputEventResult HandleInputEvent(const WebInputEvent&);
  WebInputEventResult HandleInputEventInOverlay(const WebInputEvent&);
  void PageLayoutInvalidated(bool resized);
  void EvaluateInOverlay(const String& method, const String& argument);
  void EvaluateInOverlay(const String& method,
                         std::unique_ptr<protocol::Value> argument);
  String EvaluateInOverlayForTest(const String&);

  void UpdatePrePaint();
  void PaintOverlay(GraphicsContext&);

  bool IsInspectorLayer(const cc::Layer*) const;

  LocalFrame* GetFrame() const;
  float WindowToViewportScale() const;
  void ScheduleUpdate();

  float EmulationScaleFactor() const;

  void DidInitializeFrameWidget();

 private:
  class InspectorOverlayChromeClient;
  class InspectorPageOverlayDelegate;

  // InspectorOverlayHost::Delegate implementation.
  void Dispatch(const ScriptValue& message,
                ExceptionState& exception_state) override;

  bool IsEmpty();
  bool FrameWidgetInitialized() const;

  LocalFrame* OverlayMainFrame();
  void Reset(const gfx::Size& viewport_size,
             const gfx::SizeF& visual_viewport_size);
  void OnResizeTimer(TimerBase*);
  void PaintOverlayPage();

  protocol::Response CompositingEnabled();

  bool IsVisible() const { return inspect_tool_ || hinge_; }
  bool InSomeInspectMode();
  void SetNeedsUnbufferedInput(bool unbuffered);
  void PickTheRightTool();
  // Set or clear a mode tool, or add a highlight tool
  protocol::Response SetInspectTool(InspectTool* inspect_tool);
  void ClearInspectTool();
  void LoadOverlayPageResource();
  void EnsureEnableFrameOverlay();
  void DisableFrameOverlay();
  InspectorSourceOrderConfig SourceOrderConfigFromInspectorObject(
      std::unique_ptr<protocol::Overlay::SourceOrderConfig>
          source_order_inspector_object);
  protocol::Response HighlightConfigFromInspectorObject(
      std::unique_ptr<protocol::Overlay::HighlightConfig>
          highlight_inspector_object,
      std::unique_ptr<InspectorHighlightConfig>*);
  Member<WebLocalFrameImpl> frame_impl_;
  Member<InspectedFrames> inspected_frames_;
  Member<Page> overlay_page_;
  Member<InspectorOverlayChromeClient> overlay_chrome_client_;
  Member<InspectorOverlayHost> overlay_host_;
  bool resize_timer_active_;
  HeapTaskRunnerTimer<InspectorOverlayAgent> resize_timer_;
  bool disposed_;
  v8_inspector::V8InspectorSession* v8_session_;
  Member<InspectorDOMAgent> dom_agent_;
  Member<FrameOverlay> frame_overlay_;
  Member<InspectTool> inspect_tool_;
  Member<PersistentTool> persistent_tool_;
  Member<Hinge> hinge_;
  // The agent needs to keep AXContext because it enables caching of
  // a11y attributes shown in the inspector overlay.
  HeapHashMap<WeakMember<Document>, std::unique_ptr<AXContext>>
      document_to_ax_context_;
  bool swallow_next_mouse_up_;
  bool is_page_scrolling_ = false;
  std::unique_ptr<cc::LayerTreeDebugState> original_layer_tree_debug_state_;

  DOMNodeId backend_node_id_to_inspect_;
  InspectorAgentState::Boolean enabled_;
  InspectorAgentState::Boolean show_ad_highlights_;
  InspectorAgentState::Boolean show_debug_borders_;
  InspectorAgentState::Boolean show_fps_counter_;
  InspectorAgentState::Boolean show_paint_rects_;
  InspectorAgentState::Boolean show_layout_shift_regions_;
  InspectorAgentState::Boolean show_scroll_bottleneck_rects_;
  InspectorAgentState::Boolean show_hit_test_borders_;
  InspectorAgentState::Boolean show_web_vitals_;
  InspectorAgentState::Boolean show_size_on_resize_;
  InspectorAgentState::String paused_in_debugger_message_;
  InspectorAgentState::String inspect_mode_;
  InspectorAgentState::Bytes inspect_mode_protocol_config_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_OVERLAY_AGENT_H_
