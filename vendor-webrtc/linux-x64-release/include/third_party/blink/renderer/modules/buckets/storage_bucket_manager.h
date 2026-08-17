// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BUCKETS_STORAGE_BUCKET_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BUCKETS_STORAGE_BUCKET_MANAGER_H_

#include "third_party/blink/public/mojom/buckets/bucket_manager_host.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExceptionState;
class NavigatorBase;
class StorageBucketOptions;
class StorageBucket;

class MODULES_EXPORT StorageBucketManager final
    : public ScriptWrappable,
      public Supplement<NavigatorBase>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Web-exposed as navigator.storageBuckets
  static StorageBucketManager* storageBuckets(NavigatorBase& navigator);

  explicit StorageBucketManager(NavigatorBase& navigator);
  ~StorageBucketManager() override = default;

  ScriptPromise<StorageBucket> open(ScriptState* script_state,
                                    const String& name,
                                    const StorageBucketOptions* options,
                                    ExceptionState& exception_state);
  ScriptPromise<IDLSequence<IDLString>> keys(ScriptState* script_state,
                                             ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> Delete(ScriptState* script_state,
                                     const String& name,
                                     ExceptionState& exception_state);

  // GarbageCollected
  void Trace(Visitor*) const override;

  // These are not exposed to the web applications and only used by DevTools.
  StorageBucket* GetBucketForDevtools(ScriptState* script_state,
                                      const String& name);

 private:
  mojom::blink::BucketManagerHost* GetBucketManager(ScriptState* script_state);

  void DidOpen(ScriptPromiseResolver<StorageBucket>* resolver,
               const String& name,
               mojo::PendingRemote<mojom::blink::BucketHost> bucket_remote,
               mojom::blink::BucketError error);
  void DidGetKeys(ScriptPromiseResolver<IDLSequence<IDLString>>* resolver,
                  const Vector<String>& keys,
                  bool success);
  void DidDelete(ScriptPromiseResolver<IDLUndefined>*, bool success);

  HeapMojoRemote<mojom::blink::BucketManagerHost> manager_remote_;

  Member<NavigatorBase> navigator_base_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BUCKETS_STORAGE_BUCKET_MANAGER_H_
