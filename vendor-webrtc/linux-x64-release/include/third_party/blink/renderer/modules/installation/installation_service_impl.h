// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INSTALLATION_INSTALLATION_SERVICE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INSTALLATION_INSTALLATION_SERVICE_IMPL_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/installation/installation.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class LocalDOMWindow;
class LocalFrame;

class MODULES_EXPORT InstallationServiceImpl final
    : public GarbageCollected<InstallationServiceImpl>,
      public mojom::blink::InstallationService,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];
  static InstallationServiceImpl* From(LocalDOMWindow&);
  static void BindReceiver(
      LocalFrame*,
      mojo::PendingReceiver<mojom::blink::InstallationService> receiver);

  explicit InstallationServiceImpl(base::PassKey<InstallationServiceImpl>,
                                   LocalFrame&);

  // Not copyable or movable
  InstallationServiceImpl(const InstallationServiceImpl&) = delete;
  InstallationServiceImpl& operator=(const InstallationServiceImpl&) = delete;

  void Bind(mojo::PendingReceiver<mojom::blink::InstallationService> receiver);

  void Trace(Visitor* visitor) const override;

  void OnInstall() override;

 private:
  HeapMojoReceiverSet<mojom::blink::InstallationService,
                      InstallationServiceImpl>
      receivers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INSTALLATION_INSTALLATION_SERVICE_IMPL_H_
