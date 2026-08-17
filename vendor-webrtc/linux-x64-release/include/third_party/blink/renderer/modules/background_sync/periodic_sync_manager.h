// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_PERIODIC_SYNC_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_PERIODIC_SYNC_MANAGER_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/public/mojom/background_sync/background_sync.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class BackgroundSyncOptions;
class ExceptionState;
class ScriptState;
class ServiceWorkerRegistration;

class PeriodicSyncManager final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PeriodicSyncManager(ServiceWorkerRegistration* registration,
                      scoped_refptr<base::SequencedTaskRunner> task_runner);

  // IDL exposed interface
  ScriptPromise<IDLUndefined> registerPeriodicSync(
      ScriptState* script_state,
      const String& tag,
      const BackgroundSyncOptions* options,
      ExceptionState& exception_state);
  ScriptPromise<IDLSequence<IDLString>> getTags(ScriptState* script_state);
  ScriptPromise<IDLUndefined> unregister(ScriptState* script_state,
                                         const String& tag);

  void Trace(Visitor* visitor) const override;

 private:
  // Returns an initialized
  // mojo::Remote<mojom::blink::PeriodicBackgroundSyncService>. A connection
  // with the the browser's BackgroundSyncService is created the first time this
  // method is called.
  mojom::blink::PeriodicBackgroundSyncService* GetBackgroundSyncServiceRemote();

  // Callbacks
  void RegisterCallback(ScriptPromiseResolver<IDLUndefined>* resolver,
                        mojom::blink::BackgroundSyncError error,
                        mojom::blink::SyncRegistrationOptionsPtr options);
  void GetRegistrationsCallback(
      ScriptPromiseResolver<IDLSequence<IDLString>>* resolver,
      mojom::blink::BackgroundSyncError error,
      WTF::Vector<mojom::blink::SyncRegistrationOptionsPtr> registrations);
  void UnregisterCallback(ScriptPromiseResolver<IDLUndefined>* resolver,
                          mojom::blink::BackgroundSyncError error);

  Member<ServiceWorkerRegistration> registration_;
  scoped_refptr<base::SequencedTaskRunner> task_runner_;
  HeapMojoRemote<mojom::blink::PeriodicBackgroundSyncService>
      background_sync_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_PERIODIC_SYNC_MANAGER_H_
