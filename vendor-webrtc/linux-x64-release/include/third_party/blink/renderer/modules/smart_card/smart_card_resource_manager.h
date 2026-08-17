// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_RESOURCE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_RESOURCE_MANAGER_H_

#include <optional>

#include "third_party/blink/public/mojom/smart_card/smart_card.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace blink {

class NavigatorBase;
class SmartCardContext;

class MODULES_EXPORT SmartCardResourceManager final
    : public ScriptWrappable,
      public Supplement<NavigatorBase>,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Getter for navigator.smartCard
  static SmartCardResourceManager* smartCard(NavigatorBase&);

  explicit SmartCardResourceManager(NavigatorBase&);

  // ExecutionContextLifecycleObserver overrides.
  void ContextDestroyed() override;

  // ScriptWrappable overrides
  void Trace(Visitor*) const override;

  // SmartCardResourceManager idl
  ScriptPromise<SmartCardContext> establishContext(
      ScriptState* script_state,
      ExceptionState& exception_state);

 private:
  void EnsureServiceConnection();
  void CloseServiceConnection();

  void OnCreateContextDone(
      ScriptPromiseResolver<SmartCardContext>*,
      device::mojom::blink::SmartCardCreateContextResultPtr);

  HeapMojoRemote<mojom::blink::SmartCardService> service_;
  HeapHashSet<Member<ScriptPromiseResolver<SmartCardContext>>>
      create_context_promises_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_RESOURCE_MANAGER_H_
