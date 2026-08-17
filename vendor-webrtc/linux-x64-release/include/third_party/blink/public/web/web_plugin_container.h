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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_CONTAINER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_CONTAINER_H_

#include "third_party/blink/public/platform/web_common.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/rect.h"
#include "v8/include/v8-forward.h"

namespace cc {
class Layer;
}

namespace blink {

class WebDocument;
class WebElement;
class WebPlugin;
class WebString;
class WebURLRequest;
class WebDOMMessageEvent;

class WebPluginContainer {
 public:
  enum TouchEventRequestType {
    kTouchEventRequestTypeNone,
    kTouchEventRequestTypeRaw,
    kTouchEventRequestTypeRawLowLatency,
    kTouchEventRequestTypeSynthesizedMouse,
  };

  // Returns the element containing this plugin.
  virtual WebElement GetElement() = 0;

  // Returns the owning document for the plugin.
  virtual WebDocument GetDocument() = 0;

  // Synchronously dispatches the progress event.
  virtual void DispatchProgressEvent(const WebString& type,
                                     bool length_computable,
                                     uint64_t loaded,
                                     uint64_t total,
                                     const WebString& url) = 0;

  // Enqueue's a task to dispatch the event.
  // TODO(esprehn): Why are progress events sync and message events async!?
  virtual void EnqueueMessageEvent(const WebDOMMessageEvent&) = 0;

  virtual void Invalidate() = 0;

  // Schedules an animation of the WebView that contains the plugin, as well as
  // the plugin.
  virtual void ScheduleAnimation() = 0;

  // Causes the container to report its current geometry via
  // WebPlugin::updateGeometry.
  virtual void ReportGeometry() = 0;

  // Returns the scriptable object associated with the DOM element
  // containing the plugin as a native v8 object.
  virtual v8::Local<v8::Object> V8ObjectForElement() = 0;

  // Loads an URL in the specified frame (or the frame containing this
  // plugin if target is empty).  If notifyNeeded is true, then upon
  // completion, WebPlugin::didFinishLoadingFrameRequest is called if the
  // load was successful or WebPlugin::didFailLoadingFrameRequest is
  // called if the load failed.  The given notifyData is passed along to
  // the callback.
  virtual void LoadFrameRequest(const WebURLRequest&,
                                const WebString& target) = 0;

  // Determines whether the given rectangle in this plugin is above all other
  // content. The rectangle is in the plugin's coordinate system.
  virtual bool IsRectTopmost(const gfx::Rect&) = 0;

  // Notifies when the plugin changes the kind of touch-events it accepts.
  virtual void RequestTouchEventType(TouchEventRequestType) = 0;

  // Notifies when the plugin starts/stops accepting wheel events. Without
  // calling the function with true, the container might not always able to
  // receive wheel events in some cases (such as when threaded compositing
  // is in use but a scroll bar is not in use).
  virtual void SetWantsWheelEvents(bool) = 0;

  // Converts root frame's coordinates to plugin's local coordinates.
  virtual gfx::Point RootFrameToLocalPoint(const gfx::Point&) = 0;

  // Converts plugin's local coordinate to root frame's coordinates.
  virtual gfx::Point LocalToRootFramePoint(const gfx::Point&) = 0;

  // Returns the plugin this container owns. This plugin will be
  // automatically destroyed when the container is destroyed.
  virtual WebPlugin* Plugin() = 0;

  // Sets the plugin owned by this container. If the container already owned
  // a different plugin before this call, that old plugin is now unowned.
  // The caller is then responsible for destroying the old plugin.
  virtual void SetPlugin(WebPlugin*) = 0;

  // Sets |this| as find handler for the associated frame.
  virtual void UsePluginAsFindHandler() = 0;

  virtual void ReportFindInPageMatchCount(int identifier,
                                          int total,
                                          bool final_update) = 0;
  virtual void ReportFindInPageSelection(int identifier,
                                         int index,
                                         bool final_update) = 0;

  // PageScaleFactor represents visual viewport pinch zoom.
  virtual float PageScaleFactor() = 0;
  // LayoutZoomFactor represents hardware device pixel ratio, browser zoom and,
  // the effect of the CSS "zoom" property applied to ancestors.
  virtual float LayoutZoomFactor() = 0;

  // Sets the layer representing the plugin for compositing. The
  // WebPluginContainer does *not* take ownership.
  virtual void SetCcLayer(cc::Layer*) = 0;

  virtual void RequestFullscreen() = 0;
  virtual bool IsFullscreenElement() const = 0;
  virtual void CancelFullscreen() = 0;

  // Returns true if this container was the target for the last mouse event.
  virtual bool WasTargetForLastMouseEvent() = 0;

  // Whether this plugin current has the mouse lock or not.
  virtual bool IsMouseLocked() = 0;

  // Request to lock the mouse. A subsequent callback on
  // WebPlugin::DidReceiveMouseLockResult will be called.
  virtual bool LockMouse(bool request_unadjusted_movement) = 0;

  // Request to unlock a current mouse lock.
  virtual void UnlockMouse() = 0;

 protected:
  ~WebPluginContainer() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_CONTAINER_H_
