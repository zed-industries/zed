// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_BUILDER_H_

#include "base/types/expected.h"
#include "services/webnn/public/mojom/webnn_graph_builder.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_operand_data_type.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer_view.h"
#include "third_party/blink/renderer/modules/ml/webnn/allow_shared_buffer_source_util.h"
#include "third_party/blink/renderer/modules/ml/webnn/ml_graph.h"
#include "third_party/blink/renderer/modules/ml/webnn/ml_operator.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"

namespace blink {

class ExceptionState;
class MLArgMinMaxOptions;
class MLBatchNormalizationOptions;
class MLContext;
class MLClampOptions;
class MLConv2dOptions;
class MLConvTranspose2dOptions;
class MLCumulativeSumOptions;
class MLEluOptions;
class MLGatherOptions;
class MLGemmOptions;
class MLGruOptions;
class MLGruCellOptions;
class MLGraph;
class MLHardSigmoidOptions;
class MLInstanceNormalizationOptions;
class MLLayerNormalizationOptions;
class MLLeakyReluOptions;
class MLLinearOptions;
class MLLstmOptions;
class MLLstmCellOptions;
class MLOperatorOptions;
class MLPadOptions;
class MLPool2dOptions;
class MLReduceOptions;
class MLResample2dOptions;
class MLReverseOptions;
class MLScatterOptions;
class MLSliceOptions;
class MLSplitOptions;
class MLTransposeOptions;
class MLTriangularOptions;
class MLOperand;
class MLOperandDescriptor;
class ScriptState;

typedef HeapVector<std::pair<String, Member<MLOperand>>> MLNamedOperands;

class MODULES_EXPORT MLGraphBuilder final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MLGraphBuilder* Create(ScriptState* script_state,
                                MLContext* context,
                                ExceptionState& exception_state);

  explicit MLGraphBuilder(
      ExecutionContext* execution_context,
      MLContext* context,
      mojo::PendingAssociatedRemote<webnn::mojom::blink::WebNNGraphBuilder>
          pending_remote);

  MLGraphBuilder(const MLGraphBuilder&) = delete;
  MLGraphBuilder& operator=(const MLGraphBuilder&) = delete;

  ~MLGraphBuilder() override;

  void Trace(Visitor* visitor) const override;

  MLContext* GetContext() const;

  struct Size2D {
    uint32_t height;
    uint32_t width;
  };

  // ml_graph_builder.idl
  MLOperand* input(ScriptState* script_state,
                   String name,
                   const MLOperandDescriptor* desc,
                   ExceptionState& exception_state);
  MLOperand* constant(ScriptState* script_state,
                      const MLOperandDescriptor* desc,
                      AllowSharedBufferSource* buffer,
                      ExceptionState& exception_state);

  // The order of operations declaration is the same as spec.
  MLOperand* argMin(MLOperand* input,
                    const uint32_t axis,
                    const MLArgMinMaxOptions* options,
                    ExceptionState& exception_state);
  MLOperand* argMax(MLOperand* input,
                    const uint32_t axis,
                    const MLArgMinMaxOptions* options,
                    ExceptionState& exception_state);

  MLOperand* batchNormalization(MLOperand* input,
                                MLOperand* mean,
                                MLOperand* variance,
                                const MLBatchNormalizationOptions* options,
                                ExceptionState& exception_state);

  MLOperand* clamp(MLOperand* input,
                   const MLClampOptions* options,
                   ExceptionState& exception_state);

  MLOperand* concat(const HeapVector<Member<MLOperand>>& inputs,
                    const uint32_t axis,
                    const MLOperatorOptions* options,
                    ExceptionState& exception_state);

  MLOperand* conv2d(MLOperand* input,
                    MLOperand* filter,
                    const MLConv2dOptions* options,
                    ExceptionState& exception_state);

  MLOperand* convTranspose2d(MLOperand* input,
                             MLOperand* filter,
                             const MLConvTranspose2dOptions* options,
                             ExceptionState& exception_state);

  MLOperand* cumulativeSum(MLOperand* input,
                           const uint32_t axis,
                           const MLCumulativeSumOptions* options,
                           ExceptionState& exception_state);

  // Element-wise binary operations
  MLOperand* add(MLOperand* a,
                 MLOperand* b,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* sub(MLOperand* a,
                 MLOperand* b,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* mul(MLOperand* a,
                 MLOperand* b,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* div(MLOperand* a,
                 MLOperand* b,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* max(MLOperand* a,
                 MLOperand* b,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* min(MLOperand* a,
                 MLOperand* b,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* pow(MLOperand* a,
                 MLOperand* b,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* equal(MLOperand* a,
                   MLOperand* b,
                   const MLOperatorOptions* options,
                   ExceptionState& exception_state);
  MLOperand* greater(MLOperand* a,
                     MLOperand* b,
                     const MLOperatorOptions* options,
                     ExceptionState& exception_state);
  MLOperand* greaterOrEqual(MLOperand* a,
                            MLOperand* b,
                            const MLOperatorOptions* options,
                            ExceptionState& exception_state);
  MLOperand* lesser(MLOperand* a,
                    MLOperand* b,
                    const MLOperatorOptions* options,
                    ExceptionState& exception_state);
  MLOperand* lesserOrEqual(MLOperand* a,
                           MLOperand* b,
                           const MLOperatorOptions* options,
                           ExceptionState& exception_state);
  MLOperand* notEqual(MLOperand* a,
                      MLOperand* b,
                      const MLOperatorOptions* options,
                      ExceptionState& exception_state);
  MLOperand* logicalAnd(MLOperand* a,
                        MLOperand* b,
                        const MLOperatorOptions* options,
                        ExceptionState& exception_state);
  MLOperand* logicalOr(MLOperand* a,
                       MLOperand* b,
                       const MLOperatorOptions* options,
                       ExceptionState& exception_state);
  MLOperand* logicalXor(MLOperand* a,
                        MLOperand* b,
                        const MLOperatorOptions* options,
                        ExceptionState& exception_state);

  // Element-wise unary operations
  MLOperand* abs(MLOperand* input,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* ceil(MLOperand* input,
                  const MLOperatorOptions* options,
                  ExceptionState& exception_state);
  MLOperand* cos(MLOperand* input,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* exp(MLOperand* input,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* floor(MLOperand* input,
                   const MLOperatorOptions* options,
                   ExceptionState& exception_state);
  MLOperand* log(MLOperand* input,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* neg(MLOperand* input,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* sign(MLOperand* input,
                  const MLOperatorOptions* options,
                  ExceptionState& exception_state);
  MLOperand* sin(MLOperand* input,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* tan(MLOperand* input,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* erf(MLOperand* input,
                 const MLOperatorOptions* options,
                 ExceptionState& exception_state);
  MLOperand* identity(MLOperand* input,
                      const MLOperatorOptions* options,
                      ExceptionState& exception_state);
  MLOperand* logicalNot(MLOperand* input,
                        const MLOperatorOptions* options,
                        ExceptionState& exception_state);
  MLOperand* reciprocal(MLOperand* input,
                        const MLOperatorOptions* options,
                        ExceptionState& exception_state);
  MLOperand* sqrt(MLOperand* input,
                  const MLOperatorOptions* options,
                  ExceptionState& exception_state);

  MLOperand* cast(MLOperand* input,
                  const V8MLOperandDataType output_data_type,
                  const MLOperatorOptions* options,
                  ExceptionState& exception_state);

  MLOperand* dequantizeLinear(MLOperand* input,
                              MLOperand* scale,
                              MLOperand* zeroPoint,
                              const MLOperatorOptions* options,
                              ExceptionState& exception_state);

  MLOperand* elu(MLOperand* input,
                 const MLEluOptions* options,
                 ExceptionState& exception_state);

  MLOperand* expand(MLOperand* input,
                    const Vector<uint32_t>& new_shape,
                    const MLOperatorOptions* options,
                    ExceptionState& exception_state);

  MLOperand* gather(MLOperand* input,
                    MLOperand* indices,
                    const MLGatherOptions* options,
                    ExceptionState& exception_state);

  MLOperand* gatherElements(MLOperand* input,
                            MLOperand* indices,
                            const MLGatherOptions* options,
                            ExceptionState& exception_state);

  MLOperand* gatherND(MLOperand* input,
                      MLOperand* indices,
                      const MLOperatorOptions* options,
                      ExceptionState& exception_state);

  MLOperand* gelu(MLOperand* input,
                  const MLOperatorOptions* options,
                  ExceptionState& exception_state);

  MLOperand* gemm(MLOperand* a,
                  MLOperand* b,
                  const MLGemmOptions* options,
                  ExceptionState& exception_state);

  HeapVector<Member<MLOperand>> gru(MLOperand* input,
                                    MLOperand* weight,
                                    MLOperand* recurrent_weight,
                                    const uint32_t steps,
                                    const uint32_t hidden_size,
                                    MLGruOptions* options,
                                    ExceptionState& exception_state);

  MLOperand* gruCell(MLOperand* input,
                     MLOperand* weight,
                     MLOperand* recurrent_weight,
                     MLOperand* hidden_state,
                     const uint32_t hidden_size,
                     MLGruCellOptions* options,
                     ExceptionState& exception_state);

  MLOperand* hardSigmoid(MLOperand* input,
                         const MLHardSigmoidOptions* options,
                         ExceptionState& exception_state);

  MLOperand* hardSwish(MLOperand* input,
                       const MLOperatorOptions* options,
                       ExceptionState& exception_state);

  MLOperand* instanceNormalization(
      MLOperand* input,
      const MLInstanceNormalizationOptions* options,
      ExceptionState& exception_state);

  MLOperand* layerNormalization(MLOperand* input,
                                const MLLayerNormalizationOptions* options,
                                ExceptionState& exception_state);

  MLOperand* leakyRelu(MLOperand* input,
                       const MLLeakyReluOptions* options,
                       ExceptionState& exception_state);

  MLOperand* linear(MLOperand* input,
                    const MLLinearOptions* options,
                    ExceptionState& exception_state);

  HeapVector<Member<MLOperand>> lstm(MLOperand* input,
                                     MLOperand* weight,
                                     MLOperand* recurrent_weight,
                                     const uint32_t steps,
                                     const uint32_t hidden_size,
                                     MLLstmOptions* options,
                                     ExceptionState& exception_state);

  HeapVector<Member<MLOperand>> lstmCell(MLOperand* input,
                                         MLOperand* weight,
                                         MLOperand* recurrent_weight,
                                         MLOperand* hidden_state,
                                         MLOperand* cell_state,
                                         uint32_t hidden_size,
                                         MLLstmCellOptions* options,
                                         ExceptionState& exception_state);

  MLOperand* matmul(MLOperand* a,
                    MLOperand* b,
                    const MLOperatorOptions* options,
                    ExceptionState& exception_state);

  MLOperand* pad(ScriptState* script_state,
                 MLOperand* input,
                 const Vector<uint32_t>& beginningPadding,
                 const Vector<uint32_t>& endingPadding,
                 const MLPadOptions* options,
                 ExceptionState& exception_state);

  // Pooling operations
  MLOperand* averagePool2d(MLOperand* input,
                           const MLPool2dOptions* options,
                           ExceptionState& exception_state);
  MLOperand* l2Pool2d(MLOperand* input,
                      const MLPool2dOptions* options,
                      ExceptionState& exception_state);
  MLOperand* maxPool2d(MLOperand* input,
                       const MLPool2dOptions* options,
                       ExceptionState& exception_state);

  MLOperand* prelu(MLOperand* input,
                   MLOperand* slope,
                   const MLOperatorOptions* options,
                   ExceptionState& exception_state);

  MLOperand* quantizeLinear(MLOperand* input,
                            MLOperand* scale,
                            MLOperand* zeroPoint,
                            const MLOperatorOptions* options,
                            ExceptionState& exception_state);

  // Reduction operations
  MLOperand* reduceL1(MLOperand* input,
                      const MLReduceOptions* options,
                      ExceptionState& exception_state);
  MLOperand* reduceL2(MLOperand* input,
                      const MLReduceOptions* options,
                      ExceptionState& exception_state);
  MLOperand* reduceLogSum(MLOperand* input,
                          const MLReduceOptions* options,
                          ExceptionState& exception_state);
  MLOperand* reduceLogSumExp(MLOperand* input,
                             const MLReduceOptions* options,
                             ExceptionState& exception_state);
  MLOperand* reduceMax(MLOperand* input,
                       const MLReduceOptions* options,
                       ExceptionState& exception_state);
  MLOperand* reduceMean(MLOperand* input,
                        const MLReduceOptions* options,
                        ExceptionState& exception_state);
  MLOperand* reduceMin(MLOperand* input,
                       const MLReduceOptions* options,
                       ExceptionState& exception_state);
  MLOperand* reduceProduct(MLOperand* input,
                           const MLReduceOptions* options,
                           ExceptionState& exception_state);
  MLOperand* reduceSum(MLOperand* input,
                       const MLReduceOptions* options,
                       ExceptionState& exception_state);
  MLOperand* reduceSumSquare(MLOperand* input,
                             const MLReduceOptions* options,
                             ExceptionState& exception_state);

  MLOperand* relu(MLOperand* input,
                  const MLOperatorOptions* options,
                  ExceptionState& exception_state);

  MLOperand* reshape(MLOperand* input,
                     const Vector<uint32_t>& new_shape,
                     const MLOperatorOptions* options,
                     ExceptionState& exception_state);

  MLOperand* reverse(MLOperand* input,
                     const MLReverseOptions* options,
                     ExceptionState& exception_state);

  MLOperand* resample2d(ScriptState* script_state,
                        MLOperand* input,
                        const MLResample2dOptions* options,
                        ExceptionState& exception_state);

  MLOperand* scatterElements(MLOperand* input,
                             MLOperand* indices,
                             MLOperand* updates,
                             const MLScatterOptions* options,
                             ExceptionState& exception_state);

  MLOperand* scatterND(MLOperand* input,
                       MLOperand* indices,
                       MLOperand* updates,
                       const MLOperatorOptions* options,
                       ExceptionState& exception_state);

  MLOperand* sigmoid(MLOperand* input,
                     const MLOperatorOptions* options,
                     ExceptionState& exception_state);

  MLOperand* slice(MLOperand* input,
                   const Vector<uint32_t>& starts,
                   const Vector<uint32_t>& sizes,
                   const MLSliceOptions* options,
                   ExceptionState& exception_state);

  MLOperand* softmax(MLOperand* input,
                     uint32_t axis,
                     const MLOperatorOptions* options,
                     ExceptionState& exception_state);
  MLOperand* softmax(MLOperand* input,
                     const MLOperatorOptions* options,
                     ExceptionState& exception_state);

  MLOperand* softplus(MLOperand* input,
                      const MLOperatorOptions* options,
                      ExceptionState& exception_state);

  MLOperand* softsign(MLOperand* input,
                      const MLOperatorOptions* options,
                      ExceptionState& exception_state);

  HeapVector<Member<MLOperand>> split(MLOperand* input,
                                      const uint32_t splits,
                                      const MLSplitOptions* options,
                                      ExceptionState& exception_state);
  HeapVector<Member<MLOperand>> split(MLOperand* input,
                                      const Vector<uint32_t>& splits,
                                      const MLSplitOptions* options,
                                      ExceptionState& exception_state);

  MLOperand* tanh(MLOperand* input,
                  const MLOperatorOptions* options,
                  ExceptionState& exception_state);

  MLOperand* tile(MLOperand* input,
                  const Vector<uint32_t>& repetitions,
                  const MLOperatorOptions* options,
                  ExceptionState& exception_state);

  MLOperand* transpose(MLOperand* input,
                       const MLTransposeOptions* options,
                       ExceptionState& exception_state);

  MLOperand* triangular(MLOperand* input,
                        const MLTriangularOptions* options,
                        ExceptionState& exception_state);

  MLOperand* where(MLOperand* condition,
                   MLOperand* true_value,
                   MLOperand* false_value,
                   const MLOperatorOptions* options,
                   ExceptionState& exception_state);

  ScriptPromise<MLGraph> build(ScriptState* script_state,
                               const MLNamedOperands& outputs,
                               ExceptionState& exception_state);

  void OnConnectionError();

 private:
  void DidCreateWebNNGraph(
      ScriptPromiseResolver<blink::MLGraph>* resolver,
      std::pair<MLGraph::NamedOperandDescriptors,
                MLGraph::NamedOperandDescriptors> input_and_output_constraints,
      webnn::mojom::blink::CreateGraphResultPtr result);

  // Check whether the graph builder is in an invalid state when the
  // `has_built_` is true or the `remote_` is unbound due to context lost. It
  // must be run for each method of the graph builder.
  [[nodiscard]] base::expected<void, String> ValidateGraphBuilderState() const;

  // Performs platform-agnostic and operand-agnostic validation checks which
  // must be run for each built operand. Returns an error message which may be
  // used to throw a TypeError if `input` is not valid to use with this builder.
  [[nodiscard]] base::expected<void, String> ValidateInput(
      const MLOperand* input);
  // Convenience method to validate several inputs at once.
  [[nodiscard]] base::expected<void, String> ValidateInputs(
      const HeapVector<Member<MLOperand>>& inputs);

  Member<MLContext> ml_context_;

  HeapMojoAssociatedRemote<webnn::mojom::blink::WebNNGraphBuilder> remote_;

  // Tracks whether `build()` has been called (with valid inputs). If so, `this`
  // is effectively invalid and all methods should reject.
  bool has_built_ = false;

  // Keep the unresolved `ScriptPromiseResolver` which will be rejected when the
  // Mojo pipe is unexpectedly disconnected.
  Member<ScriptPromiseResolver<MLGraph>> pending_resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_BUILDER_H_
