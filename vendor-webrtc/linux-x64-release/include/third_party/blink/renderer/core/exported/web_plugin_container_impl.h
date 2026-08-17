/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 * Copyright (C) 2014 Opera Software ASA. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_PLUGIN_CONTAINER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_PLUGIN_CONTAINER_IMPL_H_

#include "base/containers/span.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/common/input/web_coalesced_input_event.h"
#include "third_party/blink/public/common/input/web_touch_event.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/input/pointer_lock_result.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_plugin_container.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/embedded_content_view.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace cc {
class Layer;
}

namespace blink {

class Event;
class GestureEvent;
class HTMLFrameOwnerElement;
class HTMLPlugInElement;
class KeyboardEvent;
class LocalFrameView;
class MouseEvent;
class ResourceError;
class ResourceResponse;
class TouchEvent;
class WebKeyboardEvent;
class WebPlugin;
class WheelEvent;
struct WebPrintParams;
struct WebPrintPresetOptions;

class CORE_EXPORT WebPluginContainerImpl final
    : public GarbageCollected<WebPluginContainerImpl>,
      public EmbeddedContentView,
      public WebPluginContainer {
  USING_PRE_FINALIZER(WebPluginContainerImpl, PreFinalize);

 public:
  // Check if plugins support a given command |name|.
  static bool SupportsCommand(const WebString& name);

  WebPluginContainerImpl(HTMLPlugInElement&, WebPlugin*);
  ~WebPluginContainerImpl() override;

  // EmbeddedContentView methods
  bool IsPluginView() const override { return true; }
  LocalFrameView* ParentFrameView() const override;
  LayoutEmbeddedContent* GetLayoutEmbeddedContent() const override;
  void AttachToLayout() override;
  void DetachFromLayout() override;
  // |paint_offset| is used to to paint the contents at the correct location.
  // It should be issued as a transform operation before painting the contents.
  void Paint(GraphicsContext&,
             PaintFlags,
             const CullRect&,
             const gfx::Vector2d& paint_offset) const override;
  void UpdateGeometry() override;
  void Show() override;
  void Hide() override;

  cc::Layer* CcLayer() const;
  v8::Local<v8::Object> ScriptableObject(v8::Isolate*);
  bool SupportsKeyboardFocus() const;
  bool SupportsInputMethod() const;
  bool CanProcessDrag() const;
  bool WantsWheelEvents() const;
  void UpdateAllLifecyclePhases();
  void SetFocused(bool, mojom::blink::FocusType);
  void HandleEvent(Event&);
  bool IsErrorplaceholder();
  void EventListenersRemoved();

  // WebPluginContainer methods
  WebElement GetElement() override;
  WebDocument GetDocument() override;
  void DispatchProgressEvent(const WebString& type,
                             bool length_computable,
                             uint64_t loaded,
                             uint64_t total,
                             const WebString& url) override;
  void EnqueueMessageEvent(const WebDOMMessageEvent&) override;
  void Invalidate() override;
  void ScheduleAnimation() override;
  void ReportGeometry() override;
  v8::Local<v8::Object> V8ObjectForElement() override;
  void LoadFrameRequest(const WebURLRequest&, const WebString& target) override;
  bool IsRectTopmost(const gfx::Rect&) override;
  void RequestTouchEventType(TouchEventRequestType) override;
  void SetWantsWheelEvents(bool) override;
  gfx::Point RootFrameToLocalPoint(const gfx::Point&) override;
  gfx::Point LocalToRootFramePoint(const gfx::Point&) override;
  bool WasTargetForLastMouseEvent() override;
  // Non-Oilpan, this cannot be null. With Oilpan, it will be
  // null when in a disposed state, pending finalization during the next GC.
  WebPlugin* Plugin() override { return web_plugin_; }
  void SetPlugin(WebPlugin*) override;
  void UsePluginAsFindHandler() override;
  void ReportFindInPageMatchCount(int identifier,
                                  int total,
                                  bool final_update) override;
  void ReportFindInPageSelection(int identifier,
                                 int index,
                                 bool final_update) override;
  float PageScaleFactor() override;
  float LayoutZoomFactor() override;
  void SetCcLayer(cc::Layer*) override;
  void RequestFullscreen() override;
  bool IsFullscreenElement() const override;
  void CancelFullscreen() override;
  bool IsMouseLocked() override;
  bool LockMouse(bool request_unadjusted_movement) override;
  void UnlockMouse() override;

  // Printing interface. The plugin can support custom printing
  // (which means it controls the layout, number of pages etc).
  // Whether the plugin supports its own paginated print. The other print
  // interface methods are called only if this method returns true.
  bool SupportsPaginatedPrint() const;
  // Returns true on success and sets the out parameter to the print preset
  // options for the document.
  bool GetPrintPresetOptionsFromDocument(WebPrintPresetOptions*) const;
  // Sets up printing at the specified WebPrintParams. Returns the number of
  // pages to be printed at these settings.
  int PrintBegin(const WebPrintParams& print_params) const;
  // Prints the page specified by `page_index`  into the supplied canvas.
  void PrintPage(int page_index, GraphicsContext& gc);
  // Ends the print operation.
  void PrintEnd();

  // Copy the selected text.
  void Copy();

  // Pass the edit command to the plugin.
  bool ExecuteEditCommand(const WebString& name);
  bool ExecuteEditCommand(const WebString& name, const WebString& value);

  // Resource load events for the plugin's source data:
  void DidReceiveResponse(const ResourceResponse&);
  void DidReceiveData(base::span<const char> data);
  void DidFinishLoading();
  void DidFailLoading(const ResourceError&);

  void Trace(Visitor*) const override;
  // USING_PRE_FINALIZER does not allow for virtual dispatch from the finalizer
  // method. Here we call Dispose() which does the correct virtual dispatch.
  void PreFinalize() { Dispose(); }
  void Dispose() override;
  void SetFrameRect(const gfx::Rect&) override;
  void PropagateFrameRects() override { ReportGeometry(); }

  void MaybeLostMouseLock();

 protected:
  void ParentVisibleChanged() override;

 private:
  // Sets |windowRect| to the content rect of the plugin in screen space.
  // Sets |clippedAbsoluteRect| to the visible rect for the plugin, clipped to
  // the visible screen of the root frame, in local space of the plugin.
  // Sets |unclippedAbsoluteRect| to the visible rect for the plugin (but
  // without also clipping to the screen), in local space of the plugin.
  void ComputeClipRectsForPlugin(
      const HTMLFrameOwnerElement* plugin_owner_element,
      gfx::Rect& window_rect,
      gfx::Rect& clipped_local_rect,
      gfx::Rect& unclipped_int_local_rect) const;

  WebTouchEvent TransformTouchEvent(const WebInputEvent&);
  WebCoalescedInputEvent TransformCoalescedTouchEvent(
      const WebCoalescedInputEvent&);

  void HandleMouseEvent(MouseEvent&);
  void HandleDragEvent(MouseEvent&);
  void HandleWheelEvent(WheelEvent&);
  void HandleKeyboardEvent(KeyboardEvent&);
  bool HandleCutCopyPasteKeyboardEvent(const WebKeyboardEvent&);
  void HandleTouchEvent(TouchEvent&);
  void HandleGestureEvent(GestureEvent&);

  void HandleLockMouseResult(mojom::blink::PointerLockResult result);

  void SynthesizeMouseEventIfPossible(TouchEvent&);

  void FocusPlugin();

  void CalculateGeometry(gfx::Rect& window_rect,
                         gfx::Rect& clip_rect,
                         gfx::Rect& unobscured_rect);

  friend class WebPluginContainerTest;
  class MouseLockLostListener;

  Member<HTMLPlugInElement> element_;
  Member<MouseLockLostListener> mouse_lock_lost_listener_;
  WebPlugin* web_plugin_;
  cc::Layer* layer_ = nullptr;
  TouchEventRequestType touch_event_request_type_ = kTouchEventRequestTypeNone;
  bool wants_wheel_events_ = false;
};

template <>
struct DowncastTraits<WebPluginContainerImpl> {
  static bool AllowFrom(const EmbeddedContentView& embedded_content_view) {
    return embedded_content_view.IsPluginView();
  }
  // Unlike EmbeddedContentView, we need not worry about object type for
  // container. WebPluginContainerImpl is the only subclass of
  // WebPluginContainer.
  static bool AllowFrom(const WebPluginContainer& container) { return true; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_PLUGIN_CONTAINER_IMPL_H_
