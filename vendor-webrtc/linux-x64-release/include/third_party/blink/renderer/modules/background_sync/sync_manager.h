// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SYNC_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SYNC_MANAGER_H_

#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/public/mojom/background_sync/background_sync.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ScriptState;
class ServiceWorkerRegistration;

class SyncManager final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SyncManager(ServiceWorkerRegistration*,
              scoped_refptr<base::SequencedTaskRunner>);

  ScriptPromise<IDLUndefined> registerFunction(ScriptState*,
                                               const String& tag,
                                               ExceptionState& exception_state);
  ScriptPromise<IDLSequence<IDLString>> getTags(ScriptState*);

  void Trace(Visitor*) const override;

  enum { kUnregisteredSyncID = -1 };

 private:
  // Callbacks
  void RegisterCallback(ScriptPromiseResolver<IDLUndefined>*,
                        mojom::blink::BackgroundSyncError,
                        mojom::blink::SyncRegistrationOptionsPtr options);
  static void GetRegistrationsCallback(
      ScriptPromiseResolver<IDLSequence<IDLString>>*,
      mojom::blink::BackgroundSyncError,
      WTF::Vector<mojom::blink::SyncRegistrationOptionsPtr> registrations);

  Member<ServiceWorkerRegistration> registration_;
  HeapMojoRemote<mojom::blink::OneShotBackgroundSyncService>
      background_sync_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SYNC_MANAGER_H_
