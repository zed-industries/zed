// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_CONSTANT_OPERAND_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_CONSTANT_OPERAND_H_

#include "services/webnn/public/cpp/operand_descriptor.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/modules/ml/webnn/ml_operand.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class MLGraphBuilder;

// Represents an `MLOperand` created from the `MLGraphBuilder.constant()`
// method. See https://www.w3.org/TR/webnn/#api-mlgraphbuilder-constant.
class MODULES_EXPORT MLConstantOperand final : public MLOperand {
 public:
  // Creates a constant operand on `builder`. After creating an instance of this
  // class, the caller may create a "pending" constant operand in the WebNN
  // Service with the generated `handle_`, which identifies the weight data
  // associated with this operand.
  MLConstantOperand(MLGraphBuilder* builder,
                    webnn::OperandDescriptor descriptor);

  MLConstantOperand(const MLConstantOperand&) = delete;
  MLConstantOperand& operator=(const MLConstantOperand&) = delete;

  ~MLConstantOperand() override;

  void Trace(Visitor* visitor) const override;

  const WebNNPendingConstantToken& handle() const { return handle_; }

 private:
  // Identifies this constant operand in the WebNN service.
  const WebNNPendingConstantToken handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_CONSTANT_OPERAND_H_
