/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_WEB_PLUGIN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_WEB_PLUGIN_H_

#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_plugin.h"

namespace cc {
class PaintCanvas;
}

namespace blink {

class WebCoalescedInputEvent;
class WebDragData;
class WebPluginContainer;
class WebURLResponse;
struct WebPluginParams;

class FakeWebPlugin : public WebPlugin {
 public:
  explicit FakeWebPlugin(const WebPluginParams&);

  // WebPlugin methods:
  bool Initialize(WebPluginContainer*) override;
  void Destroy() override;
  bool CanProcessDrag() const override { return false; }
  void UpdateAllLifecyclePhases(blink::DocumentUpdateReason) override {}
  void Paint(cc::PaintCanvas*, const gfx::Rect&) override {}
  void UpdateGeometry(const gfx::Rect& client_rect,
                      const gfx::Rect& clip_rect,
                      const gfx::Rect& window_clip_rect,
                      bool is_visible) override {}
  void UpdateFocus(bool, mojom::blink::FocusType) override {}
  void UpdateVisibility(bool) override {}
  WebInputEventResult HandleInputEvent(const WebCoalescedInputEvent&,
                                       ui::Cursor*) override {
    return WebInputEventResult::kNotHandled;
  }
  bool HandleDragStatusUpdate(WebDragStatus,
                              const WebDragData&,
                              DragOperationsMask,
                              const gfx::PointF& position,
                              const gfx::PointF& screen_position) override {
    return false;
  }
  void DidReceiveResponse(const WebURLResponse&) override {}
  void DidReceiveData(base::span<const char> data) override {}
  void DidFinishLoading() override {}
  void DidFailLoading(const WebURLError&) override {}

 protected:
  ~FakeWebPlugin() override;

  WebPluginContainer* Container() const override { return container_; }

 private:
  WebPluginContainer* container_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FAKE_WEB_PLUGIN_H_
