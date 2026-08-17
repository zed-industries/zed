// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_APP_BANNER_APP_BANNER_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_APP_BANNER_APP_BANNER_CONTROLLER_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/app_banner/app_banner.mojom-blink.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class LocalDOMWindow;
class LocalFrame;

class AppBannerController final : public GarbageCollected<AppBannerController>,
                                  public Supplement<LocalDOMWindow>,
                                  public mojom::blink::AppBannerController {
 public:
  static const char kSupplementName[];
  static AppBannerController* From(LocalDOMWindow&);
  static void BindReceiver(
      LocalFrame*,
      mojo::PendingReceiver<mojom::blink::AppBannerController>);

  explicit AppBannerController(base::PassKey<AppBannerController>,
                               LocalDOMWindow&);

  // Not copyable or movable
  AppBannerController(const AppBannerController&) = delete;
  AppBannerController& operator=(const AppBannerController&) = delete;

  void Trace(Visitor* visitor) const override;

  // mojom::blink::AppBannerController overrides:
  void BannerPromptRequest(mojo::PendingRemote<mojom::blink::AppBannerService>,
                           mojo::PendingReceiver<mojom::blink::AppBannerEvent>,
                           const Vector<String>& platforms,
                           BannerPromptRequestCallback) override;

 private:
  void Bind(mojo::PendingReceiver<mojom::blink::AppBannerController> receiver);

  HeapMojoReceiver<mojom::blink::AppBannerController, AppBannerController>
      receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_APP_BANNER_APP_BANNER_CONTROLLER_H_
