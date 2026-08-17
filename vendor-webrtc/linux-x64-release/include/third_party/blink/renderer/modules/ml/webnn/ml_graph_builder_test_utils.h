// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_BUILDER_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_BUILDER_TEST_UTILS_H_

#include <optional>

#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_context_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_operand_data_type.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer_view.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"

namespace blink {

class MLGraphBuilder;
class MLOperand;

MLOperand* BuildInput(ScriptState* script_state,
                      MLGraphBuilder* builder,
                      const String& name,
                      const Vector<uint32_t>& dimensions,
                      V8MLOperandDataType::Enum data_type,
                      ExceptionState& exception_state);

MaybeShared<DOMArrayBufferView> CreateDOMArrayBufferView(
    size_t size,
    V8MLOperandDataType::Enum data_type);

MLOperand* BuildConstant(
    ScriptState* script_state,
    MLGraphBuilder* builder,
    const Vector<uint32_t>& dimensions,
    V8MLOperandDataType::Enum data_type,
    ExceptionState& exception_state,
    std::optional<MaybeShared<DOMArrayBufferView>> buffer_view = std::nullopt);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_BUILDER_TEST_UTILS_H_
