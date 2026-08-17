/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_DEV_TOOLS_FRONTEND_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_DEV_TOOLS_FRONTEND_IMPL_H_

#include "base/values.h"
#include "third_party/blink/public/mojom/devtools/devtools_frontend.mojom-blink.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/core/inspector/inspector_frontend_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DevToolsHost;
class LocalFrame;

// This class lives as long as a frame (being a supplement), or until
// it's host (mojom.DevToolsFrontendHost) is destroyed.
class DevToolsFrontendImpl final
    : public GarbageCollected<DevToolsFrontendImpl>,
      public Supplement<LocalFrame>,
      public mojom::blink::DevToolsFrontend,
      public InspectorFrontendClient,
      public LocalFrame::WidgetCreationObserver {
 public:
  static const char kSupplementName[];

  static void BindMojoRequest(
      LocalFrame*,
      mojo::PendingAssociatedReceiver<mojom::blink::DevToolsFrontend>);
  static DevToolsFrontendImpl* From(LocalFrame*);

  DevToolsFrontendImpl(
      LocalFrame&,
      mojo::PendingAssociatedReceiver<mojom::blink::DevToolsFrontend>);

  DevToolsFrontendImpl(const DevToolsFrontendImpl&) = delete;
  DevToolsFrontendImpl& operator=(const DevToolsFrontendImpl&) = delete;

  ~DevToolsFrontendImpl() override;
  void DidClearWindowObject();
  void Trace(Visitor*) const override;

  // LocalFrame::WidgetCreationObserver implementation.
  void OnLocalRootWidgetCreated() override;

 private:
  void DestroyOnHostGone();

  // mojom::blink::DevToolsFrontend implementation.
  void SetupDevToolsFrontend(
      const String& api_script,
      mojo::PendingAssociatedRemote<mojom::blink::DevToolsFrontendHost>)
      override;
  void SetupDevToolsExtensionAPI(const String& extension_api) override;

  // InspectorFrontendClient implementation.
  void SendMessageToEmbedder(base::Value::Dict) override;

  Member<DevToolsHost> devtools_host_;
  String api_script_;
  // The host_ must outlive the ExecutionContext of LocalFrame, so it should not
  // be associated with the ExecutionContext of LocalFrame.
  HeapMojoAssociatedRemote<mojom::blink::DevToolsFrontendHost> host_{nullptr};
  // The receiver_ must outlive the ExecutionContext of LocalFrame, so it should
  // not be associated with the ExecutionContext of LocalFrame.
  HeapMojoAssociatedReceiver<mojom::blink::DevToolsFrontend,
                             DevToolsFrontendImpl>
      receiver_{this, nullptr};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_DEV_TOOLS_FRONTEND_IMPL_H_
