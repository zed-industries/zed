// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_CONTEXT_H_

#include "services/device/public/mojom/smart_card.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_smart_card_access_mode.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_smart_card_protocol.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"

namespace blink {

class SmartCardReaderStateIn;

class SmartCardGetStatusChangeOptions;
class SmartCardReaderStateOut;
class SmartCardConnection;
class SmartCardConnectOptions;
class SmartCardConnectResult;

class SmartCardContext final : public ScriptWrappable,
                               public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SmartCardContext(mojo::PendingRemote<device::mojom::blink::SmartCardContext>,
                   ExecutionContext*);

  // SmartCardContext idl
  ScriptPromise<IDLSequence<IDLString>> listReaders(
      ScriptState* script_state,
      ExceptionState& exception_state);

  ScriptPromise<IDLSequence<SmartCardReaderStateOut>> getStatusChange(
      ScriptState* script_state,
      const HeapVector<Member<SmartCardReaderStateIn>>& reader_states,
      SmartCardGetStatusChangeOptions* options,
      ExceptionState& exception_state);

  ScriptPromise<SmartCardConnectResult> connect(
      ScriptState* script_state,
      const String& reader_name,
      V8SmartCardAccessMode access_mode,
      SmartCardConnectOptions* options,
      ExceptionState& exception_state);

  // ScriptWrappable overrides
  void Trace(Visitor*) const override;

  // Called by SmartCardCancelAlgorithm
  void Cancel();

  ////
  // Called also by SmartCardConnection instances in this context.

  bool EnsureNoOperationInProgress(ExceptionState& exception_state) const;

  void SetConnectionOperationInProgress(ScriptPromiseResolverBase*);
  void ClearConnectionOperationInProgress(ScriptPromiseResolverBase*);

  bool IsOperationInProgress() const;

 private:
  // Sets the PC/SC operation that is in progress in this context.
  // CHECKs that there was no operation in progress.
  void SetOperationInProgress(ScriptPromiseResolverBase*);

  // Clears the operation in progress.
  // CHECKs that the given operation matches the one set to be in progress.
  void ClearOperationInProgress(ScriptPromiseResolverBase*);

  void CloseMojoConnection();
  bool EnsureMojoConnection(ExceptionState& exception_state) const;
  void OnListReadersDone(
      ScriptPromiseResolver<IDLSequence<IDLString>>* resolver,
      device::mojom::blink::SmartCardListReadersResultPtr);
  void OnGetStatusChangeDone(
      ScriptPromiseResolver<IDLSequence<SmartCardReaderStateOut>>* resolver,
      AbortSignal* signal,
      AbortSignal::AlgorithmHandle* abort_handle,
      device::mojom::blink::SmartCardStatusChangeResultPtr result);
  void OnCancelDone(device::mojom::blink::SmartCardResultPtr result);
  void OnConnectDone(ScriptPromiseResolver<SmartCardConnectResult>* resolver,
                     device::mojom::blink::SmartCardConnectResultPtr result);

  HeapMojoRemote<device::mojom::blink::SmartCardContext> scard_context_;
  // The currently ongoing request, if any.
  Member<ScriptPromiseResolverBase> request_;

  // Whether request_ comes from a blink::SmartCardConnection.
  bool is_connection_request_ = false;

  // The connections created by this context.
  HeapHashSet<WeakMember<SmartCardConnection>> connections_;

  FrameOrWorkerScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_CONTEXT_H_
