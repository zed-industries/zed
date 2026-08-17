// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_WEB_LAUNCH_SERVICE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_WEB_LAUNCH_SERVICE_IMPL_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_directory_handle.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/web_launch/web_launch.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class LocalFrame;
class LocalDOMWindow;

// Implementation of WebLaunchService, to allow the renderer to receive launch
// parameters from the browser process.
class MODULES_EXPORT WebLaunchServiceImpl final
    : public GarbageCollected<WebLaunchServiceImpl>,
      public mojom::blink::WebLaunchService,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];
  static WebLaunchServiceImpl* From(LocalDOMWindow&);
  static void BindReceiver(
      LocalFrame* frame,
      mojo::PendingAssociatedReceiver<mojom::blink::WebLaunchService>);
  explicit WebLaunchServiceImpl(base::PassKey<WebLaunchServiceImpl>,
                                LocalDOMWindow& window);
  ~WebLaunchServiceImpl() override;

  // Not copyable or movable
  WebLaunchServiceImpl(const WebLaunchServiceImpl&) = delete;
  WebLaunchServiceImpl& operator=(const WebLaunchServiceImpl&) = delete;

  void Bind(mojo::PendingAssociatedReceiver<mojom::blink::WebLaunchService>);
  void Trace(Visitor* visitor) const override;

  // blink::mojom::WebLaunchService:
  void SetLaunchFiles(
      WTF::Vector<mojom::blink::FileSystemAccessEntryPtr>) override;
  void EnqueueLaunchParams(const KURL& launch_url) override;

 private:
  HeapMojoAssociatedReceiver<mojom::blink::WebLaunchService,
                             WebLaunchServiceImpl,
                             HeapMojoWrapperMode::kForceWithoutContextObserver>
      receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_WEB_LAUNCH_SERVICE_IMPL_H_
