// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_TYPE_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_TYPE_CONVERTER_H_

#include <optional>

#include "mojo/public/cpp/bindings/type_converter.h"
#include "services/webnn/public/mojom/webnn_context_provider.mojom-blink.h"
#include "services/webnn/public/mojom/webnn_graph.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_ml_operand_data_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

class MLOperand;
class MLOperator;

// Returns the next operand ID to use when adding entries to graph_info's
// id_to_operand_map.
uint64_t NextOperandId(const webnn::mojom::blink::GraphInfo& graph_info);

std::optional<String> SerializeMojoOperation(
    const HeapHashMap<Member<const MLOperand>, uint64_t>& operand_to_id_map,
    const webnn::ContextProperties& context_properties,
    const MLOperator* op,
    webnn::mojom::blink::GraphInfo* graph_info);

}  // namespace blink

namespace mojo {

template <>
struct TypeConverter<webnn::mojom::blink::OperandPtr, blink::MLOperand*> {
  static webnn::mojom::blink::OperandPtr Convert(
      const blink::MLOperand* ml_operand);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_GRAPH_TYPE_CONVERTER_H_
