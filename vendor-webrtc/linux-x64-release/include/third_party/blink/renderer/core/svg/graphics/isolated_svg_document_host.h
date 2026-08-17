/*
 * Copyright (C) 2006 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2008, 2009 Apple Inc. All rights reserved.
 * Copyright (C) Research In Motion Limited 2011. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_ISOLATED_SVG_DOCUMENT_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_ISOLATED_SVG_DOCUMENT_HOST_H_

#include "base/functional/callback.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"

namespace blink {

class AgentGroupScheduler;
struct ColorProviderColorMaps;
class IsolatedSVGChromeClient;
class LocalFrame;
class Page;
class SVGSVGElement;
class Settings;

// Encapsulation of an (SVG)Document that is isolated/independent from other
// documents. Does not run scripts. Used by SVGImage.
class IsolatedSVGDocumentHost final
    : public GarbageCollected<IsolatedSVGDocumentHost> {
 public:
  // https://svgwg.org/svg2-draft/conform.html#processing-modes
  enum class ProcessingMode {
    kStatic,    // Corresponds to "secure static mode".
    kAnimated,  // Corresponds to "secure animated mode".
  };
  IsolatedSVGDocumentHost(IsolatedSVGChromeClient&,
                          AgentGroupScheduler&,
                          scoped_refptr<const SharedBuffer>,
                          base::OnceClosure async_load_callback,
                          const Settings* inherited_settings,
                          const ColorProviderColorMaps* inherited_color_maps,
                          ProcessingMode);
  ~IsolatedSVGDocumentHost();

  void Shutdown();

  LocalFrame* GetFrame();
  SVGSVGElement* RootElement();

  bool IsLoaded() const { return load_state_ == kCompleted; }

  void Trace(Visitor* visitor) const;

 private:
  static void CopySettingsFrom(Settings& settings,
                               const Settings& inherited_settings);
  void LoadCompleted();
  void AsyncLoadCompleted();

  class LocalFrameClient;

  Member<Page> page_;
  Member<LocalFrameClient> frame_client_;
  base::OnceClosure async_load_callback_;
  TaskHandle async_load_task_handle_;

  enum LoadState {
    kNotStarted,
    kPending,
    kWaitingForAsyncLoadCompletion,
    kCompleted,
  };
  LoadState load_state_ = kNotStarted;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_ISOLATED_SVG_DOCUMENT_HOST_H_
