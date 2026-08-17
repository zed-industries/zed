// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_FILTER_OPERATION_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_FILTER_OPERATION_RESOLVER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/style/filter_operations.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Element;
class ExceptionState;
class ExecutionContext;
class Font;

// Similar to
// third_party/blink/renderer/core/css/resolver/filter_operation_resolver.h but
// the input is a from a canvas filter object instead of a CSSValue.
// CanvasFilters are created in javascript by passing in dictionaries like so:
//  ctx.filter = new CanvasFilter({filter: "gaussianBlur", stdDeviation: 5});
// This class resolves these inputs into FilterOperations that can be used by
// CanvasRenderingContext2DState's GetFilter() functions.
class MODULES_EXPORT CanvasFilterOperationResolver {
  STATIC_ONLY(CanvasFilterOperationResolver);

 public:
  static FilterOperations CreateFilterOperationsFromList(
      const HeapVector<ScriptObject>& filters,
      ExecutionContext& execution_context,
      ExceptionState& exception_state);

  // Can be used with or without style resolution. If a `style_resolution_host`
  // is passed, its `StyleResolver` will be used to compute the filter
  // operations. If its a nullptr, the `FilterOperationResolver` will be used
  // directly.
  static FilterOperations CreateFilterOperationsFromCSSFilter(
      const String& filter_string,
      const ExecutionContext& execution_context,
      Element* style_resolution_host,
      const Font* font);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_FILTER_OPERATION_RESOLVER_H_
