// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_OPERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_OPERATOR_H_

#include <variant>

#include "services/webnn/public/mojom/webnn_graph.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/dictionary_base.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class MLArgMinMaxOptions;
class MLCumulativeSumOptions;
class MLGraphBuilder;
class MLOperand;
class MLOperatorOptions;
class MLGruCellOptions;
class MLLstmOptions;
class MLLstmCellOptions;
class MLPadOptions;
class MLReverseOptions;
class MLSliceOptions;
class MLSplitOptions;

class MODULES_EXPORT MLOperator : public GarbageCollected<MLOperator> {
 public:
  using OperationSubKind =
      std::variant<webnn::mojom::blink::ArgMinMax::Kind,
                   webnn::mojom::blink::Conv2d::Kind,
                   webnn::mojom::blink::ElementWiseBinary::Kind,
                   webnn::mojom::blink::ElementWiseUnary::Kind,
                   webnn::mojom::blink::Pool2d::Kind,
                   webnn::mojom::blink::Reduce::Kind,
                   std::monostate>;

  static String OperatorKindToString(
      webnn::mojom::blink::Operation::Tag kind,
      OperationSubKind sub_kind = std::monostate{});

  // It is safe for a caller, usually a MLGraphBuidler operation build method,
  // that passes the reference of the options dictionary argument received from
  // Blink to MLOperator constructor and stores it in this object. This is
  // because that WebIDL spec (https://webidl.spec.whatwg.org/#idl-dictionaries)
  // mentiones that "an operation that accepts a dictionary as an argument will
  // perform a one-time conversion from the given ECMAScript value into the
  // dictionary, based on the current properties of the ECMAScript object.
  // Modifications to the dictionary will not be reflected in the corresponding
  // ECMAScript object, and vice-versa". Blink code generator follows the spec
  // and does a deep-copy of the members of an options dictionary, e.g.,
  // MLConv2dOptions::FillMembersFromV8Object, before passing it to a
  // MLGraphBuilder operation build method.
  MLOperator(MLGraphBuilder* builder,
             webnn::mojom::blink::Operation::Tag kind,
             const MLOperatorOptions* options,
             OperationSubKind sub_kind = std::monostate{});

  MLOperator(const MLOperator&) = delete;
  MLOperator& operator=(const MLOperator&) = delete;

  virtual ~MLOperator();

  void Trace(Visitor* visitor) const;

  webnn::mojom::blink::Operation::Tag Kind() const;
  OperationSubKind SubKind() const;
  template <typename MojomKind>
  MojomKind SubKind() const {
    return std::get<MojomKind>(SubKind());
  }

  const MLOperatorOptions* Options() const;
  const HeapVector<Member<MLOperand>>& Inputs() const;
  const HeapVector<Member<MLOperand>>& Outputs() const;
  MLGraphBuilder const* Builder() const { return builder_.Get(); }

  // According to WebNN programming model
  // https://www.w3.org/TR/webnn/#programming-model, neural networks are
  // represented as computational graphs of mathematical operators (nodes)
  // connected by operands (edges). This method connects the operator with its
  // input and output operands during a computational graph building session. An
  // operator is only allowed to be connected once.
  void Connect(HeapVector<Member<MLOperand>> inputs,
               HeapVector<Member<MLOperand>> outputs);

 private:
  Member<MLGraphBuilder> builder_;
  webnn::mojom::blink::Operation::Tag kind_;

  // The correct type of options_ depends on OperatorKind. For example, if the
  // OperatorKind is kClamp, options_ could static_cast to MLClampOptions.
  Member<const MLOperatorOptions> options_;
  OperationSubKind sub_kind_;

  HeapVector<Member<MLOperand>> inputs_;
  HeapVector<Member<MLOperand>> outputs_;
};

// TODO: crbug.com/325612086 - Remove all these subclasses. This information
// should all be contained within the respective mojo Operation struct.

class MODULES_EXPORT MLArgMinMaxOperator : public MLOperator {
 public:
  MLArgMinMaxOperator(MLGraphBuilder* builder,
                      OperationSubKind sub_kind,
                      const uint32_t axis,
                      const MLArgMinMaxOptions* options);

  MLArgMinMaxOperator(const MLArgMinMaxOperator&) = delete;
  MLArgMinMaxOperator& operator=(const MLArgMinMaxOperator&) = delete;

  ~MLArgMinMaxOperator() override;

  uint32_t Axis() const { return axis_; }

 private:
  uint32_t axis_;
};

class MODULES_EXPORT MLConcatOperator : public MLOperator {
 public:
  MLConcatOperator(MLGraphBuilder* builder,
                   const uint32_t axis,
                   const MLOperatorOptions* options);

  MLConcatOperator(const MLConcatOperator&) = delete;
  MLConcatOperator& operator=(const MLConcatOperator&) = delete;

  ~MLConcatOperator() override;

  uint32_t Axis() const;

 private:
  uint32_t axis_;
};

class MODULES_EXPORT MLCumulativeSumOperator : public MLOperator {
 public:
  MLCumulativeSumOperator(MLGraphBuilder* builder,
                          const uint32_t axis,
                          const MLCumulativeSumOptions* options);

  MLCumulativeSumOperator(const MLCumulativeSumOperator&) = delete;
  MLCumulativeSumOperator& operator=(const MLCumulativeSumOperator&) = delete;

  ~MLCumulativeSumOperator() override;

  uint32_t Axis() const { return axis_; }

 private:
  const uint32_t axis_;
};

class MODULES_EXPORT MLLstmOperator : public MLOperator {
 public:
  MLLstmOperator(MLGraphBuilder* builder,
                 uint32_t steps,
                 uint32_t hidden_size,
                 const MLLstmOptions* options);

  MLLstmOperator(const MLLstmOperator&) = delete;
  MLLstmOperator& operator=(const MLLstmOperator&) = delete;

  ~MLLstmOperator() override;

  uint32_t steps() const;
  uint32_t hidden_size() const;

 private:
  uint32_t steps_;
  uint32_t hidden_size_;
};

class MODULES_EXPORT MLLstmCellOperator : public MLOperator {
 public:
  MLLstmCellOperator(MLGraphBuilder* builder,
                     uint32_t hidden_size,
                     const MLLstmCellOptions* options);

  MLLstmCellOperator(const MLLstmCellOperator&) = delete;
  MLLstmCellOperator& operator=(const MLLstmCellOperator&) = delete;

  ~MLLstmCellOperator() override;

  uint32_t hidden_size() const;

 private:
  const uint32_t hidden_size_;
};

class MODULES_EXPORT MLGruOperator : public MLOperator {
 public:
  MLGruOperator(MLGraphBuilder* builder,
                uint32_t steps,
                uint32_t hidden_size,
                const MLOperatorOptions* options);

  MLGruOperator(const MLGruOperator&) = delete;
  MLGruOperator& operator=(const MLGruOperator&) = delete;

  ~MLGruOperator() override;

  uint32_t steps() const { return steps_; }
  uint32_t hidden_size() const { return hidden_size_; }

 private:
  const uint32_t steps_;
  const uint32_t hidden_size_;
};

class MODULES_EXPORT MLGruCellOperator : public MLOperator {
 public:
  MLGruCellOperator(MLGraphBuilder* builder,
                    uint32_t hidden_size,
                    const MLGruCellOptions* options);

  MLGruCellOperator(const MLGruCellOperator&) = delete;
  MLGruCellOperator& operator=(const MLGruCellOperator&) = delete;

  ~MLGruCellOperator() override;

  uint32_t hidden_size() const { return hidden_size_; }

 private:
  const uint32_t hidden_size_;
};

class MODULES_EXPORT MLPadOperator : public MLOperator {
 public:
  MLPadOperator(MLGraphBuilder* builder,
                const Vector<uint32_t>& beginning_padding,
                const Vector<uint32_t>& ending_padding,
                const MLPadOptions* options);

  MLPadOperator(const MLPadOperator&) = delete;
  MLPadOperator& operator=(const MLPadOperator&) = delete;

  ~MLPadOperator() override;

  const Vector<uint32_t>& BeginningPadding() const;
  const Vector<uint32_t>& EndingPadding() const;

 private:
  Vector<uint32_t> beginning_padding_;
  Vector<uint32_t> ending_padding_;
};

class MODULES_EXPORT MLReverseOperator : public MLOperator {
 public:
  MLReverseOperator(MLGraphBuilder* builder,
                    Vector<uint32_t> axes,
                    const MLReverseOptions* options);

  MLReverseOperator(const MLReverseOperator&) = delete;
  MLReverseOperator& operator=(const MLReverseOperator&) = delete;

  ~MLReverseOperator() override;

  const Vector<uint32_t>& Axes() const;

 private:
  Vector<uint32_t> axes_;
};

class MODULES_EXPORT MLSliceOperator : public MLOperator {
 public:
  MLSliceOperator(MLGraphBuilder* builder,
                  const Vector<uint32_t>& starts,
                  const Vector<uint32_t>& sizes,
                  const Vector<uint32_t>& strides,
                  const MLSliceOptions* options);

  MLSliceOperator(const MLSliceOperator&) = delete;
  MLSliceOperator& operator=(const MLSliceOperator&) = delete;

  ~MLSliceOperator() override;

  const Vector<uint32_t>& Starts() const;
  const Vector<uint32_t>& Sizes() const;
  const Vector<uint32_t>& Strides() const;

 private:
  Vector<uint32_t> starts_;
  Vector<uint32_t> sizes_;
  Vector<uint32_t> strides_;
};

class MODULES_EXPORT MLSoftmaxOperator : public MLOperator {
 public:
  MLSoftmaxOperator(MLGraphBuilder* builder,
                    const uint32_t axis,
                    const MLOperatorOptions* options);

  MLSoftmaxOperator(const MLSoftmaxOperator&) = delete;
  MLSoftmaxOperator& operator=(const MLSoftmaxOperator&) = delete;

  ~MLSoftmaxOperator() override;

  uint32_t Axis() const { return axis_; }

 private:
  const uint32_t axis_;
};

class MODULES_EXPORT MLSplitOperator : public MLOperator {
 public:
  MLSplitOperator(MLGraphBuilder* builder,
                  const uint32_t splits,
                  const MLSplitOptions* options);
  MLSplitOperator(MLGraphBuilder* builder,
                  const Vector<uint32_t>& splits,
                  const MLSplitOptions* options);

  MLSplitOperator(const MLSplitOperator&) = delete;
  MLSplitOperator& operator=(const MLSplitOperator&) = delete;

  ~MLSplitOperator() override;

  bool IsEvenSplit() const;
  uint32_t SplitNumber() const;
  const Vector<uint32_t>& SplitSizes() const;

 private:
  bool is_even_split_;
  uint32_t split_number_;
  Vector<uint32_t> split_sizes_;
};

class MODULES_EXPORT MLTileOperator : public MLOperator {
 public:
  MLTileOperator(MLGraphBuilder* builder,
                 const Vector<uint32_t>& repetitons,
                 const MLOperatorOptions* options);

  MLTileOperator(const MLTileOperator&) = delete;
  MLTileOperator& operator=(const MLTileOperator&) = delete;

  ~MLTileOperator() override;

  const Vector<uint32_t>& Repetitions() const;

 private:
  Vector<uint32_t> repetitions_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_OPERATOR_H_
