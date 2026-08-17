// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_OPERAND_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_OPERAND_H_

#include <optional>

#include "base/types/expected.h"
#include "services/webnn/public/cpp/operand_descriptor.h"
#include "services/webnn/public/mojom/webnn_graph.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_operand_descriptor.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class MLConstantOperand;
class MLGraphBuilder;
class MLOperator;

class MODULES_EXPORT MLOperand : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Validate and create different kinds of operand if there are no errors.
  // Otherwise return an error message which may be used
  // to throw a TypeError if the inputs are not valid.
  static base::expected<MLOperand*, String> ValidateAndCreateInput(
      const webnn::ContextProperties& context_properties,
      MLGraphBuilder* builder,
      V8MLOperandDataType::Enum v8_data_type,
      Vector<uint32_t> shape,
      String name);
  // Similar to the methods above, but since we're passed `descriptor` we can
  // skip the validation.
  static MLOperand* CreateOutput(MLGraphBuilder* builder,
                                 webnn::OperandDescriptor descriptor,
                                 const MLOperator* ml_operator);

  // The constructor shouldn't be called directly. The callers should use
  // Create* methods instead.
  MLOperand(MLGraphBuilder* builder,
            webnn::mojom::blink::Operand::Kind kind,
            webnn::OperandDescriptor descriptor);

  MLOperand(const MLOperand&) = delete;
  MLOperand& operator=(const MLOperand&) = delete;

  ~MLOperand() override;

  void Trace(Visitor* visitor) const override;

  MLGraphBuilder* Builder() const;
  webnn::mojom::blink::Operand::Kind Kind() const;
  const String& Name() const;
  const MLOperator* Operator() const;
  const HeapHashSet<Member<const MLOperator>>& DependentOperators() const;

  // Convenience methods for accessing native types, which avoid a copy
  // compared to using the corresponding methods which return blink types.
  const webnn::OperandDescriptor& Descriptor() const;
  webnn::OperandDataType DataType() const;
  const std::vector<uint32_t>& Shape() const;

  // The total number of elements in the operand. Its value is the product of
  // all values of the shape. For scalar operand, the number of elements is 1.
  size_t NumberOfElements() const;

  // The byte length of the oprand. It is defined by WebNN spec as:
  // https://www.w3.org/TR/webnn/#mloperanddescriptor-byte-length
  size_t ByteLength() const;

  wtf_size_t Rank() const;

  // IDL interface:
  V8MLOperandDataType dataType() const;
  Vector<uint32_t> shape() const;

  MLConstantOperand const* AsConstantOperand() const;

  void AddDependentOperator(const MLOperator* ml_operator);

 protected:
  Member<MLGraphBuilder> builder_;

  const webnn::mojom::blink::Operand::Kind kind_;

  // Represents a valid MLOperandDescriptor.
  // https://www.w3.org/TR/webnn/#dictdef-mloperanddescriptor
  const webnn::OperandDescriptor descriptor_;

  // The name of input operand. According to
  // https://www.w3.org/TR/webnn/#dom-mlgraphbuilder-input, only input operand
  // is created with a name.
  String name_;
  // The operator that produces the output operand. Only output operand has an
  // operator that produces the operand by an operator build method of
  // MLGraphBuilder interface.
  Member<const MLOperator> operator_;

  // Operators that use this operand as an input.
  HeapHashSet<Member<const MLOperator>> dependent_operators_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_OPERAND_H_
