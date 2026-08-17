// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_ML_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_ML_H_

#include "services/webnn/public/mojom/webnn_context_provider.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/ml/ml_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class GPUDevice;
class MLContextOptions;
class ScriptState;

// This class represents the "Machine Learning" object "navigator.ml" used by
// the WebNN API. See https://www.w3.org/TR/webnn/#api-ml.
class MODULES_EXPORT ML final : public ScriptWrappable,
                                public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ML(ExecutionContext* execution_context);

  ML(const ML&) = delete;
  ML& operator=(const ML&) = delete;

  void Trace(blink::Visitor*) const override;

  // IDL interface:
  ScriptPromise<MLContext> createContext(ScriptState* state,
                                         MLContextOptions* option,
                                         ExceptionState& exception_state);
  ScriptPromise<MLContext> createContext(ScriptState* state,
                                         GPUDevice* gpu_device,
                                         ExceptionState& exception_state);

 private:
  // Reset the remote of `WebNNContextProvider` if the remote is cut off from
  // its receiver.
  void OnWebNNServiceConnectionError();

  // There is only one WebNN service running out of renderer process to access
  // the hardware accelerated OS machine learning API. Every `navigator.ml`
  // object has one `WebNNContextProvider` message pipe to create `WebNNContext`
  // mojo interface.
  void EnsureWebNNServiceConnection();

  // WebNN support multiple types of neural network inference hardware
  // acceleration such as CPU, GPU and ML specialized accelerator, the context
  // of webnn in service is used to map different device and represent a state
  // of graph execution processes.
  HeapMojoRemote<webnn::mojom::blink::WebNNContextProvider>
      webnn_context_provider_;

  // Keep a set of unresolved `ScriptPromiseResolver`s which will be
  // rejected when the Mojo pipe is unexpectedly disconnected.
  HeapHashSet<Member<ScriptPromiseResolver<MLContext>>> pending_resolvers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_ML_H_
