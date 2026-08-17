/*
 * Copyright (C) 2007 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEV_TOOLS_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEV_TOOLS_HOST_H_

#include "base/values.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_accelerator.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_show_context_menu_item.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Document;
class FrontendMenuProvider;
class InspectorFrontendClient;
class LocalFrame;

class CORE_EXPORT DevToolsHost final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  DevToolsHost(InspectorFrontendClient*, LocalFrame* frontend_frame);
  ~DevToolsHost() override;

  void Trace(Visitor*) const override;
  void DisconnectClient();

  float zoomFactor();

  float ConvertLengthForEmbedder(float length);

  void copyText(const String& text);
  String platform() const;

  void showContextMenuAtPoint(v8::Isolate*,
                              float x,
                              float y,
                              const HeapVector<Member<ShowContextMenuItem>>&,
                              Document* = nullptr);
  void sendMessageToEmbedder(base::Value::Dict message);
  void sendMessageToEmbedder(const String& message);

  bool isHostedMode();

  LocalFrame* FrontendFrame() { return frontend_frame_.Get(); }

  void ClearMenuProvider() { menu_provider_ = nullptr; }

  // kMaxContextMenuAction means no action (e.g. separator), all
  // actual actions should be less. This needs to be 1000 because the range
  // of custom context menu actions is restricted to the difference between
  // IDC_CONTENT_CONTEXT_CUSTOM_FIRST and IDC_CONTENT_CONTEXT_CUSTOM_LAST, which
  // is 1000.
  static const uint32_t kMaxContextMenuAction = 1000;

 private:
  friend class FrontendMenuProvider;

  void EvaluateScript(const String&);

  Member<InspectorFrontendClient> client_;
  Member<LocalFrame> frontend_frame_;
  Member<FrontendMenuProvider> menu_provider_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEV_TOOLS_HOST_H_
