// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PERMISSIONS_PERMISSIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PERMISSIONS_PERMISSIONS_H_

#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/permissions/permission_status_listener.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace blink {

class ExecutionContext;
class NavigatorBase;
class PermissionStatus;
class ScriptState;
class ScriptValue;
enum class PermissionType;

class Permissions final : public ScriptWrappable,
                          public Supplement<NavigatorBase>,
                          public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Getter for navigator.permissions
  static Permissions* permissions(NavigatorBase&);

  explicit Permissions(NavigatorBase&);

  ScriptPromise<PermissionStatus> query(ScriptState*,
                                        const ScriptValue&,
                                        ExceptionState&);
  ScriptPromise<PermissionStatus> request(ScriptState*,
                                          const ScriptValue&,
                                          ExceptionState&);
  ScriptPromise<PermissionStatus> revoke(ScriptState*,
                                         const ScriptValue&,
                                         ExceptionState&);
  ScriptPromise<IDLSequence<PermissionStatus>>
  requestAll(ScriptState*, const HeapVector<ScriptObject>&, ExceptionState&);

  // ExecutionContextLifecycleStateObserver:
  void ContextDestroyed() override;

  void PermissionStatusObjectCreated() { ++created_permission_status_objects_; }

  void Trace(Visitor*) const override;

 private:
  mojom::blink::PermissionService* GetService(ExecutionContext*);
  void ServiceConnectionError();

  void QueryTaskComplete(ScriptPromiseResolver<PermissionStatus>* resolver,
                         mojom::blink::PermissionDescriptorPtr descriptor,
                         base::TimeTicks query_start_time,
                         mojom::blink::PermissionStatus result);

  void TaskComplete(ScriptPromiseResolver<PermissionStatus>* resolver,
                    mojom::blink::PermissionDescriptorPtr descriptor,
                    mojom::blink::PermissionStatus result);

  void VerifyPermissionAndReturnStatus(
      ScriptPromiseResolverBase* resolver,
      mojom::blink::PermissionDescriptorPtr descriptor,
      mojom::blink::PermissionStatus result);
  void VerifyPermissionsAndReturnStatus(
      ScriptPromiseResolverBase* resolver,
      Vector<mojom::blink::PermissionDescriptorPtr> descriptors,
      Vector<int> caller_index_to_internal_index,
      int last_verified_permission_index,
      bool is_bulk_request,
      const Vector<mojom::blink::PermissionStatus>& results);

  void PermissionVerificationComplete(
      ScriptPromiseResolverBase* resolver,
      Vector<mojom::blink::PermissionDescriptorPtr> descriptors,
      Vector<int> caller_index_to_internal_index,
      const Vector<mojom::blink::PermissionStatus>& results,
      mojom::blink::PermissionDescriptorPtr verification_descriptor,
      int internal_index_to_verify,
      bool is_bulk_request,
      mojom::blink::PermissionStatus verification_result);

  PermissionStatusListener* GetOrCreatePermissionStatusListener(
      mojom::blink::PermissionStatus status,
      mojom::blink::PermissionDescriptorPtr descriptor);
  std::optional<PermissionType> GetPermissionType(
      const mojom::blink::PermissionDescriptor& descriptor);
  mojom::blink::PermissionDescriptorPtr CreatePermissionVerificationDescriptor(
      PermissionType descriptor_type);

  int created_permission_status_objects_ = 0;

  HeapHashMap<PermissionType, Member<PermissionStatusListener>> listeners_;

  HeapMojoRemote<mojom::blink::PermissionService> service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PERMISSIONS_PERMISSIONS_H_
