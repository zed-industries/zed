// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_FILTER_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/style/filter_operations.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/forward.h"  // IWYU pragma: keep (blink::Visitor)

// IWYU pragma: no_include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class Element;
class ExceptionState;
class ExecutionContext;
class Font;
class ScriptState;

// This class stores an unresolved filter on CanvasRenderingContext2DState that
// has been created from the CanvasFilter javascript object. It will be parsed
// into FilterOperations by the CanvasFilterOperationResolver upon creation.
class MODULES_EXPORT CanvasFilter final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CanvasFilter* Create(ScriptState* script_state,
                              const V8CanvasFilterInput* init,
                              ExceptionState& exception_state);

  // The `style_resolution_host` is an optional argument that is only used when
  // the filter is defined with a CSS String. In case `filter_input` is a string
  // and `style_resolution_host` is nullptr, the filter operations will be
  // computed without using style resolution.
  static FilterOperations CreateFilterOperations(
      const V8CanvasFilterInput& filter_input,
      const Font* font,
      Element* style_resolution_host,
      ExecutionContext& execution_context,
      ExceptionState& exception_state);

  explicit CanvasFilter(FilterOperations filter_operations);

  const FilterOperations& Operations() const { return filter_operations_; }

  void Trace(Visitor* visitor) const override;

 private:
  FilterOperations filter_operations_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_FILTER_H_
