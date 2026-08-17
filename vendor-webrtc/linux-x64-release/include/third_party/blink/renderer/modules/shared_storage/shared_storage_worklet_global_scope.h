// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WORKLET_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WORKLET_GLOBAL_SCOPE_H_

#include <stdint.h>

#include "base/check.h"
#include "base/functional/callback_forward.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "mojo/public/cpp/bindings/associated_remote.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/network/public/mojom/url_loader_factory.mojom-blink-forward.h"
#include "third_party/abseil-cpp/absl/numeric/int128.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/private_aggregation/private_aggregation_host.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/shared_storage/shared_storage_worklet_service.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/workers/worklet_global_scope.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/shared_storage/private_aggregation.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

struct GlobalScopeCreationParams;
class CodeCacheFetcher;
class ModuleScriptDownloader;
class SharedStorageOperationDefinition;
class V8NoArgumentConstructor;
class SharedStorage;
class ScriptCachedMetadataHandler;
class SharedStorageWorkletNavigator;
class PrivateAggregation;
class Crypto;
class StorageInterestGroup;

// mojom::blink::SharedStorageWorkletService implementation. Responsible for
// handling worklet operations. This object lives on the worklet thread.
class MODULES_EXPORT SharedStorageWorkletGlobalScope final
    : public WorkletGlobalScope,
      public Supplementable<SharedStorageWorkletGlobalScope>,
      public mojom::blink::SharedStorageWorkletService {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SharedStorageWorkletGlobalScope(
      std::unique_ptr<GlobalScopeCreationParams> creation_params,
      WorkerThread* thread);
  ~SharedStorageWorkletGlobalScope() override;

  void BindSharedStorageWorkletService(
      mojo::PendingReceiver<mojom::blink::SharedStorageWorkletService> receiver,
      base::OnceClosure disconnect_handler);

  // SharedStorageWorkletGlobalScope IDL
  void Register(const String& name,
                V8NoArgumentConstructor* operation_ctor,
                ExceptionState&);

  // WorkletGlobalScope implementation:
  void OnConsoleApiMessage(mojom::ConsoleMessageLevel level,
                           const String& message,
                           SourceLocation* location) override;

  bool IsSharedStorageWorkletGlobalScope() const override { return true; }

  // `SharedStorageWorkletGlobalScope` is populated with a null SecurityContext,
  // and any attempt to use the default `ExecutionContext::IsSecureContext()`
  // check would fail. Override with `true` to give us the desired status (i.e.
  // the worklet must have originated from a secure Window context, so it must
  // also be a secure context).
  //
  // TODO(crbug.com/1414951): Remove this and undo marking
  // `ExecutionContext::IsSecureContext()` as virtual once
  // `SharedStorageWorkletGlobalScope` is populated with a real SecurityContext.
  bool IsSecureContext() const override { return true; }

  WorkletToken GetWorkletToken() const override { return token_; }
  ExecutionContextToken GetExecutionContextToken() const override {
    return token_;
  }

  // Do the finalization jobs (e.g. flush the private aggregation reports). At
  // the end of this call, `client_` and `private_aggregation_host_` will be
  // reset, thus we cannot rely on observer method like `Dispose()`.
  void NotifyContextDestroyed() override;

  void Trace(Visitor*) const override;

  // mojom::blink::SharedStorageWorkletService implementation:
  void Initialize(mojo::PendingAssociatedRemote<
                      mojom::blink::SharedStorageWorkletServiceClient> client,
                  mojom::blink::SharedStorageWorkletPermissionsPolicyStatePtr
                      permissions_policy_state,
                  const String& embedder_context) override;
  void AddModule(mojo::PendingRemote<network::mojom::blink::URLLoaderFactory>
                     pending_url_loader_factory,
                 const KURL& script_source_url,
                 AddModuleCallback callback) override;
  void RunURLSelectionOperation(
      const String& name,
      const Vector<KURL>& urls,
      BlinkCloneableMessage serialized_data,
      mojom::blink::PrivateAggregationOperationDetailsPtr pa_operation_details,
      RunURLSelectionOperationCallback callback) override;
  void RunOperation(
      const String& name,
      BlinkCloneableMessage serialized_data,
      mojom::blink::PrivateAggregationOperationDetailsPtr pa_operation_details,
      RunOperationCallback callback) override;

  // SharedStorageWorkletGlobalScope IDL
  SharedStorage* sharedStorage(ScriptState*, ExceptionState&);
  PrivateAggregation* privateAggregation(ScriptState*, ExceptionState&);
  Crypto* crypto(ScriptState*, ExceptionState&);
  ScriptPromise<IDLSequence<StorageInterestGroup>> interestGroups(
      ScriptState*,
      ExceptionState&);
  SharedStorageWorkletNavigator* Navigator(ScriptState*, ExceptionState&);

  // Returns the unique ID for the currently running operation.
  int64_t GetCurrentOperationId();

  mojom::blink::SharedStorageWorkletServiceClient*
  GetSharedStorageWorkletServiceClient() {
    return client_.get();
  }

  const String& embedder_context() const { return embedder_context_; }

  const mojom::blink::SharedStorageWorkletPermissionsPolicyStatePtr&
  permissions_policy_state() const {
    return permissions_policy_state_;
  }

  bool add_module_finished() const { return add_module_finished_; }

 private:
  void OnModuleScriptDownloaded(
      const KURL& script_source_url,
      mojom::blink::SharedStorageWorkletService::AddModuleCallback callback,
      std::unique_ptr<std::string> response_body,
      std::string error_message,
      network::mojom::URLResponseHeadPtr response_head);

  void DidReceiveCachedCode();

  void RecordAddModuleFinished();

  // Performs preliminary checks that are common to `RunURLSelectionOperation`
  // `RunOperation`. On success, sets `operation_definition` to the registered
  // operation. On failure, sets `error_message` to the error to be returned.
  bool PerformCommonOperationChecks(
      const String& operation_name,
      String& error_message,
      SharedStorageOperationDefinition*& operation_definition);

  network::mojom::RequestDestination GetDestination() const override {
    // Once we migrate to the blink-worklet's script loading infra, this needs
    // to return a valid destination defined in the Fetch standard:
    // https://fetch.spec.whatwg.org/#concept-request-destination
    //
    // Not called as the current implementation uses the custom module script
    // loader.
    NOTREACHED();
  }

  // Sets continuation-preserved embedder data to allow us to identify this
  // particular operation invocation later, even after asynchronous operations.
  // Returns a callback that should be run when the operation finishes.
  base::OnceCallback<void(PrivateAggregation::TerminationStatus)>
  StartOperation(
      mojom::blink::PrivateAggregationOperationDetailsPtr pa_operation_details);

  // Notifies the `private_aggregation_` that the operation with the given ID
  // has finished and whether it finished due to an uncaught error.
  void FinishOperation(
      int64_t operation_id,
      PrivateAggregation::TerminationStatus termination_status);

  PrivateAggregation* GetOrCreatePrivateAggregation();

  bool IsPrivateAggregationEnabled();

  bool add_module_finished_ = false;

  int64_t operation_counter_ = 0;

  base::OnceClosure handle_script_download_response_after_code_cache_response_;

  // `receiver_`'s disconnect handler explicitly deletes the worklet thread
  // object that owns this service, thus deleting `this` upon disconnect. To
  // ensure that the worklet thread object and this service are not leaked,
  // `receiver_` must be cut off from the remote side when the worklet is
  // supposed to be destroyed.
  HeapMojoReceiver<mojom::blink::SharedStorageWorkletService,
                   SharedStorageWorkletGlobalScope>
      receiver_{this, this};

  // If this worklet is inside a fenced frame or a URN iframe,
  // `embedder_context_` represents any contextual information written to the
  // frame's `blink::FencedFrameConfig` by the embedder before navigation to the
  // config. `embedder_context_` is passed to the worklet upon initialization.
  String embedder_context_;

  // The per-global-scope shared storage object. Created on the first access of
  // `sharedStorage`.
  Member<SharedStorage> shared_storage_;

  // The per-global-scope private aggregation object. Created on the first
  // access of `privateAggregation`.
  Member<PrivateAggregation> private_aggregation_;

  // The per-global-scope crypto object. Created on the first access of
  // `crypto`.
  Member<Crypto> crypto_;

  // The per-global-scope navigator object. Created on the first access of
  // `navigator`.
  Member<SharedStorageWorkletNavigator> navigator_;

  // The map from the registered operation names to their definition.
  HeapHashMap<String, Member<SharedStorageOperationDefinition>>
      operation_definition_map_;

  std::unique_ptr<ModuleScriptDownloader> module_script_downloader_;

  scoped_refptr<CodeCacheFetcher> code_cache_fetcher_;

  // This is associated because on the client side (i.e. worklet host), we want
  // the call-in methods (e.g. storage access) and the callback methods
  // (e.g. finish of a run-operation) to preserve their invocation order. This
  // guarantee is desirable, as the client may shut down the service immediately
  // after it gets the callback and sees no more outstanding operations, thus we
  // want it to be more likely for the worklet to finish its intended work.
  //
  // In contrast, the `receiver_` doesn't need to be associated. This is a
  // standalone service, so the starting of a worklet operation doesn't have to
  // depend on / preserve the order with messages of other types.
  HeapMojoAssociatedRemote<mojom::blink::SharedStorageWorkletServiceClient>
      client_{this};

  // The state of the permissions policy features applicable to the shared
  // storage worklet.
  mojom::blink::SharedStorageWorkletPermissionsPolicyStatePtr
      permissions_policy_state_;

  const SharedStorageWorkletToken token_;
};

template <>
struct DowncastTraits<SharedStorageWorkletGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsSharedStorageWorkletGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WORKLET_GLOBAL_SCOPE_H_
