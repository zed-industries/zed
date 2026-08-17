// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_INSTALL_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_INSTALL_EVENT_H_

#include "third_party/blink/public/mojom/service_worker/service_worker.mojom-blink.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_fetch_handler_type.mojom-blink.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"

namespace blink {

class ExceptionState;
class ScriptState;
class V8UnionRouterRuleOrRouterRuleSequence;

class MODULES_EXPORT InstallEvent : public ExtendableEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static InstallEvent* Create(const AtomicString& type,
                              const ExtendableEventInit*);
  static InstallEvent* Create(const AtomicString& type,
                              const ExtendableEventInit*,
                              int event_id,
                              WaitUntilObserver*);

  InstallEvent(const AtomicString& type, const ExtendableEventInit*);
  InstallEvent(const AtomicString& type,
               const ExtendableEventInit*,
               int event_id,
               WaitUntilObserver*);
  ~InstallEvent() override;

  const AtomicString& InterfaceName() const override;

  ScriptPromise<IDLUndefined> addRoutes(
      ScriptState*,
      const V8UnionRouterRuleOrRouterRuleSequence*,
      ExceptionState&);

 protected:
  const int event_id_;

 private:
  void ConvertServiceWorkerRouterRules(
      ScriptState* script_state,
      const V8UnionRouterRuleOrRouterRuleSequence* v8_rules,
      ExceptionState& exception_state,
      const KURL& base_url,
      mojom::blink::ServiceWorkerFetchHandlerType fetch_handler_type,
      blink::ServiceWorkerRouterRules& rules);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_INSTALL_EVENT_H_
