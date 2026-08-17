// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_CONNECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_CONNECTION_H_

#include "services/device/public/mojom/smart_card.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_piece.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {
class DOMArrayBuffer;
class SmartCardConnectionStatus;
class SmartCardContext;
class SmartCardTransactionOptions;
class SmartCardTransmitOptions;
class V8SmartCardDisposition;
class V8SmartCardTransactionCallback;

class SmartCardConnection final : public ScriptWrappable,
                                  public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SmartCardConnection(
      mojo::PendingRemote<device::mojom::blink::SmartCardConnection>,
      device::mojom::blink::SmartCardProtocol active_protocol,
      SmartCardContext* smart_card_context,
      ExecutionContext*);

  // SmartCardConnection idl
  ScriptPromise<IDLUndefined> disconnect(ScriptState* script_state,
                                         ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> disconnect(
      ScriptState* script_state,
      const V8SmartCardDisposition& disposition,
      ExceptionState& exception_state);
  ScriptPromise<DOMArrayBuffer> transmit(ScriptState* script_state,
                                         const DOMArrayPiece& send_buffer,
                                         SmartCardTransmitOptions* options,
                                         ExceptionState& exception_state);
  ScriptPromise<SmartCardConnectionStatus> status(
      ScriptState* script_state,
      ExceptionState& exception_state);
  ScriptPromise<DOMArrayBuffer> control(ScriptState* script_state,
                                        uint32_t control_code,
                                        const DOMArrayPiece& data,
                                        ExceptionState& exception_state);
  ScriptPromise<DOMArrayBuffer> getAttribute(ScriptState* script_state,
                                             uint32_t tag,
                                             ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> setAttribute(ScriptState* script_state,
                                           uint32_t tag,
                                           const DOMArrayPiece& data,
                                           ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> startTransaction(
      ScriptState* script_state,
      V8SmartCardTransactionCallback* transaction,
      SmartCardTransactionOptions* options,
      ExceptionState& exception_state);
  // Called by SmartCardContext
  void OnOperationInProgressCleared();

  void OnTransactionCallbackDone(
      device::mojom::blink::SmartCardDisposition disposition);
  void OnTransactionCallbackFailed(const ScriptValue& exception);

  // ScriptWrappable overrides
  void Trace(Visitor*) const override;

 private:
  void SetOperationInProgress(ScriptPromiseResolverBase*);
  void ClearOperationInProgress(ScriptPromiseResolverBase*);
  bool EnsureConnection(ExceptionState& exception_state) const;
  void OnDisconnectDone(ScriptPromiseResolver<IDLUndefined>* resolver,
                        device::mojom::blink::SmartCardResultPtr result);
  void OnPlainResult(ScriptPromiseResolver<IDLUndefined>* resolver,
                     device::mojom::blink::SmartCardResultPtr result);
  void OnDataResult(ScriptPromiseResolver<DOMArrayBuffer>* resolver,
                    device::mojom::blink::SmartCardDataResultPtr result);
  void OnStatusDone(ScriptPromiseResolver<SmartCardConnectionStatus>*,
                    device::mojom::blink::SmartCardStatusResultPtr result);
  void OnBeginTransactionDone(
      ScriptPromiseResolver<IDLUndefined>* resolver,
      V8SmartCardTransactionCallback* transaction_callback,
      AbortSignal* signal,
      AbortSignal::AlgorithmHandle* abort_handle,
      device::mojom::blink::SmartCardTransactionResultPtr result);
  void OnEndTransactionDone(device::mojom::blink::SmartCardResultPtr result);
  void CloseMojoConnection();
  void EndTransaction(device::mojom::blink::SmartCardDisposition);

  Member<ScriptPromiseResolverBase> ongoing_request_;
  HeapMojoRemote<device::mojom::blink::SmartCardConnection> connection_;
  device::mojom::blink::SmartCardProtocol active_protocol_;
  Member<SmartCardContext> smart_card_context_;

  class TransactionState;
  Member<TransactionState> transaction_state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_CONNECTION_H_
