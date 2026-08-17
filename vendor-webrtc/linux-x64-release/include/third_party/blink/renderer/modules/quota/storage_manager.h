// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_STORAGE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_STORAGE_MANAGER_H_

#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/public/mojom/quota/quota_manager_host.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class ExecutionContext;
class ScriptState;
class StorageEstimate;

class StorageManager final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit StorageManager(ExecutionContext*);
  ~StorageManager() override;

  ScriptPromise<IDLBoolean> persisted(ScriptState*, ExceptionState&);
  ScriptPromise<IDLBoolean> persist(ScriptState*, ExceptionState&);

  ScriptPromise<StorageEstimate> estimate(ScriptState*, ExceptionState&);

  // ScriptWrappable:
  void Trace(Visitor* visitor) const override;

 private:
  mojom::blink::PermissionService* GetPermissionService(ExecutionContext*);

  void PermissionServiceConnectionError();
  void PermissionRequestComplete(ScriptPromiseResolver<IDLBoolean>*,
                                 mojom::blink::PermissionStatus);

  // Binds the interface (if not already bound) with the given interface
  // provider, and returns it,
  mojom::blink::QuotaManagerHost* GetQuotaHost(ExecutionContext*);

  HeapMojoRemote<mojom::blink::PermissionService> permission_service_;
  HeapMojoRemote<mojom::blink::QuotaManagerHost> quota_host_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_STORAGE_MANAGER_H_
