// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_TENSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_TENSOR_H_

#include "base/timer/elapsed_timer.h"
#include "base/types/expected.h"
#include "base/types/pass_key.h"
#include "services/webnn/public/cpp/ml_tensor_usage.h"
#include "services/webnn/public/cpp/operand_descriptor.h"
#include "services/webnn/public/cpp/webnn_trace.h"
#include "services/webnn/public/mojom/webnn_context_provider.mojom-blink.h"
#include "services/webnn/public/mojom/webnn_tensor.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_operand_data_type.h"
#include "third_party/blink/renderer/modules/ml/webnn/allow_shared_buffer_source_util.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class MLTensorDescriptor;
class MLContext;

class MODULES_EXPORT MLTensor : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Instances should only be constructed via `MLContext.createTensor()`.
  // This method is public as required by the `MakeGarbageCollected` helper.
  //
  // `descriptor` describes the tensor data type and shape.
  // `create_tensor_success` contains the resulting handles to the created
  // tensor. which may be used to execute a context operation with respective
  // tensor.
  MLTensor(ExecutionContext* execution_context,
           MLContext* context,
           webnn::OperandDescriptor descriptor,
           webnn::MLTensorUsage usage,
           webnn::mojom::blink::CreateTensorSuccessPtr create_tensor_success,
           base::PassKey<MLContext> pass_key);
  MLTensor(const MLTensor&) = delete;
  MLTensor& operator=(const MLTensor&) = delete;

  ~MLTensor() override;

  void Trace(Visitor* visitor) const override;

  // ml_tensor.idl
  V8MLOperandDataType dataType() const;
  Vector<uint32_t> shape() const;
  bool importableToWebGPU() const;
  bool readable() const;
  bool writable() const;

  void destroy();

  // Convenience methods for accessing native types, which avoid a copy
  // compared to using the corresponding methods which return blink types.
  const webnn::OperandDescriptor& Descriptor() const;
  webnn::OperandDataType DataType() const;
  const std::vector<uint32_t>& Shape() const;
  const webnn::MLTensorUsage& Usage() const;

  uint64_t PackedByteLength() const;

  const blink::WebNNTensorToken& handle() const { return webnn_handle_; }

  const MLContext* context() const { return ml_context_.Get(); }

  bool IsValid() const { return remote_tensor_.is_bound(); }

  // Read data from the MLTensor. The resolver should be resolved with a copy of
  // the tensor data. Otherwise, the resolver should be rejected accordingly.
  ScriptPromise<DOMArrayBuffer> ReadTensorImpl(webnn::ScopedTrace scoped_trace,
                                               ScriptState* script_state,
                                               ExceptionState& exception_state);

  ScriptPromise<IDLUndefined> ReadTensorImpl(webnn::ScopedTrace scoped_trace,
                                             ScriptState* script_state,
                                             AllowSharedBufferSource* dst_data,
                                             ExceptionState& exception_state);

  // Write data to the MLTensor. If write was successful, the data will be
  // stored in the MLTensor.
  void WriteTensorImpl(base::span<const uint8_t> src_data,
                       ExceptionState& exception_state);

 private:
  // The callback of reading from `WebNNTensor` by calling hardware accelerated
  // OS machine learning APIs.
  void OnDidReadTensor(webnn::ScopedTrace scoped_trace,
                       ScriptPromiseResolver<DOMArrayBuffer>* resolver,
                       base::ElapsedTimer read_tensor_timer,
                       webnn::mojom::blink::ReadTensorResultPtr result);
  void OnDidReadTensorByob(webnn::ScopedTrace scoped_trace,
                           ScriptPromiseResolver<IDLUndefined>* resolver,
                           AllowSharedBufferSource* dst_data,
                           base::ElapsedTimer read_tensor_timer,
                           webnn::mojom::blink::ReadTensorResultPtr result);

  void OnConnectionError();

  Member<MLContext> ml_context_;

  // Represents a valid MLTensorDescriptor.
  const webnn::OperandDescriptor descriptor_;

  // Represents usage flags for the MLTensor.
  const webnn::MLTensorUsage usage_;

  // Identifies this `WebNNTensor` mojo instance in the service process.
  const blink::WebNNTensorToken webnn_handle_;

  // The `WebNNTensor` is a tensor that can be used by the hardware
  // accelerated OS machine learning API.
  HeapMojoAssociatedRemote<webnn::mojom::blink::WebNNTensor> remote_tensor_;

  // Keep a set of unresolved `ScriptPromiseResolver`s which will be
  // rejected when the Mojo pipe is unexpectedly disconnected.
  HeapHashSet<Member<ScriptPromiseResolver<DOMArrayBuffer>>> pending_resolvers_;
  HeapHashSet<Member<ScriptPromiseResolver<IDLUndefined>>>
      pending_byob_resolvers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_TENSOR_H_
