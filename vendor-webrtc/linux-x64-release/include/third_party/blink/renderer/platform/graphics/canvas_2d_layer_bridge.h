/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_2D_LAYER_BRIDGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_2D_LAYER_BRIDGE_H_

#include "third_party/blink/renderer/platform/graphics/canvas_hibernation_handler.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class CanvasResourceHost;

// Canvas2DLayerBridge historically served as the means of creating resources
// for 2D canvas contexts. It has since had almost all functionality migrated
// out of it and is merely a wrapper of CanvasHibernationHandler. However,
// HTMLCanvasElement and CanvasRenderingContext2D still continue to base logic
// on the dynamic presence/absence of a bridge instance for legacy reasons.
// Until we migrate all of that legacy logic, we are leaving Canvas2DLayerBridge
// in place.
// DO NOT ADD ANY CODE OR FUNCTIONALITY HERE.
// TODO(crbug.com/40280152): Eliminate this class once it is actually used
// strictly as a wrapper for CanvasHibernationHandler's functionality.
class PLATFORM_EXPORT Canvas2DLayerBridge {
 public:
  explicit Canvas2DLayerBridge(CanvasResourceHost& resource_host);
  Canvas2DLayerBridge(const Canvas2DLayerBridge&) = delete;
  Canvas2DLayerBridge& operator=(const Canvas2DLayerBridge&) = delete;

  virtual ~Canvas2DLayerBridge();

  CanvasHibernationHandler& GetHibernationHandler() {
    return hibernation_handler_;
  }

 private:
  CanvasHibernationHandler hibernation_handler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_2D_LAYER_BRIDGE_H_
