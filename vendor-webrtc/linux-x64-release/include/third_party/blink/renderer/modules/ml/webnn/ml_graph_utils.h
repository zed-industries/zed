// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_UTILS_H_

#include "base/types/expected.h"
#include "services/webnn/public/cpp/graph_validation_utils.h"
#include "services/webnn/public/cpp/operand_descriptor.h"
#include "services/webnn/public/mojom/webnn_graph.mojom-blink.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_conv_2d_filter_operand_layout.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_conv_transpose_2d_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_input_operand_layout.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_operand_descriptor.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer_view.h"
#include "third_party/blink/renderer/modules/ml/webnn/ml_graph_builder.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ScriptState;

MODULES_EXPORT DOMArrayBufferView::ViewType GetArrayBufferViewType(
    webnn::OperandDataType data_type);

// Create a default permutation vector [rank - 1, ..., 0].
Vector<uint32_t> CreateDefaultPermutation(const wtf_size_t rank);

// Create a axes vector [0, ..., rank - 1].
Vector<uint32_t> CreateAllAxes(const wtf_size_t rank);

// Create a default axes vector [1, ... , rank - 1] when rank > 1 and an empty
// vector when rank <= 1 for layer normalization specified in
// https://www.w3.org/TR/webnn/#api-mlgraphbuilder-layernorm.
Vector<uint32_t> CreateLayerNormalizationDefaultAxes(const wtf_size_t rank);

// Create a default strides vector [1, ..., 1] for slice.
Vector<uint32_t> CreateSliceDefaultStrides(wtf_size_t rank);

// Helper to validate filer layout for Nhwc input layout.
base::expected<void, String> ValidateFilterLayout(
    bool depthwise,
    V8MLInputOperandLayout input_layout,
    V8MLConv2dFilterOperandLayout filter_layout);

// Helper to get output sizes for convolution transpose 2d Node.
webnn::Size2d<uint32_t> CalculateConvTransposeOutputSize2D(
    const blink::MLConvTranspose2dOptions* options,
    uint32_t input_height,
    uint32_t input_width,
    uint32_t filter_height,
    uint32_t filter_width,
    uint32_t stride_height,
    uint32_t stride_width,
    uint32_t dilation_height,
    uint32_t dilation_width,
    uint32_t output_padding_height,
    uint32_t output_padding_width);

V8MLOperandDataType ToBlinkDataType(webnn::OperandDataType data_type);
webnn::OperandDataType FromBlinkDataType(V8MLOperandDataType::Enum data_type);

MODULES_EXPORT bool IsLogicalBinaryOperator(
    webnn::mojom::blink::ElementWiseBinary::Kind kind);

MODULES_EXPORT void LogConsoleWarning(
    ScriptState* script_state,
    const String& message,
    mojom::blink::ConsoleMessageSource message_source =
        mojom::blink::ConsoleMessageSource::kJavaScript);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_UTILS_H_
