// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_APP_BANNER_BEFORE_INSTALL_PROMPT_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_APP_BANNER_BEFORE_INSTALL_PROMPT_EVENT_H_

#include <utility>
#include "third_party/blink/public/mojom/app_banner/app_banner.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_app_banner_prompt_result.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class BeforeInstallPromptEvent;
class BeforeInstallPromptEventInit;
class ExceptionState;

using UserChoiceProperty =
    ScriptPromiseProperty<AppBannerPromptResult, IDLUndefined>;

class BeforeInstallPromptEvent final
    : public Event,
      public mojom::blink::AppBannerEvent,
      public ActiveScriptWrappable<BeforeInstallPromptEvent>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  BeforeInstallPromptEvent(const AtomicString& name,
                           ExecutionContext&,
                           mojo::PendingRemote<mojom::blink::AppBannerService>,
                           mojo::PendingReceiver<mojom::blink::AppBannerEvent>,
                           const Vector<String>& platforms);
  BeforeInstallPromptEvent(ExecutionContext*,
                           const AtomicString& name,
                           const BeforeInstallPromptEventInit*);
  ~BeforeInstallPromptEvent() override;

  static BeforeInstallPromptEvent* Create(
      const AtomicString& name,
      ExecutionContext& frame,
      mojo::PendingRemote<mojom::blink::AppBannerService> service_remote,
      mojo::PendingReceiver<mojom::blink::AppBannerEvent> event_receiver,
      const Vector<String>& platforms) {
    return MakeGarbageCollected<BeforeInstallPromptEvent>(
        name, frame, std::move(service_remote), std::move(event_receiver),
        platforms);
  }

  static BeforeInstallPromptEvent* Create(
      ExecutionContext* execution_context,
      const AtomicString& name,
      const BeforeInstallPromptEventInit* init) {
    return MakeGarbageCollected<BeforeInstallPromptEvent>(execution_context,
                                                          name, init);
  }

  Vector<String> platforms() const;
  ScriptPromise<AppBannerPromptResult> userChoice(ScriptState*,
                                                  ExceptionState&);
  ScriptPromise<AppBannerPromptResult> prompt(ScriptState*, ExceptionState&);

  const AtomicString& InterfaceName() const override;
  void preventDefault() override;

  // ScriptWrappable
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

 private:
  // mojom::blink::AppBannerEvent methods:
  void BannerAccepted(const String& platform) override;
  void BannerDismissed() override;

  HeapMojoRemote<mojom::blink::AppBannerService> banner_service_remote_;
  HeapMojoReceiver<mojom::blink::AppBannerEvent, BeforeInstallPromptEvent>
      receiver_;
  Vector<String> platforms_;
  Member<UserChoiceProperty> user_choice_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_APP_BANNER_BEFORE_INSTALL_PROMPT_EVENT_H_
