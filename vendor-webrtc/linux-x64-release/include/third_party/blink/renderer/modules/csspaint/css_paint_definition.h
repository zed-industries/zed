// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_CSS_PAINT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_CSS_PAINT_DEFINITION_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_paint_rendering_context_2d_settings.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_syntax_definition.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/modules/csspaint/paint_definition.h"
#include "third_party/blink/renderer/modules/csspaint/paint_worklet_global_scope.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "ui/gfx/geometry/size_f.h"
#include "v8/include/v8.h"

namespace blink {

class PaintWorkletStylePropertyMap;
class ScriptState;
class StylePropertyMapReadOnly;
class V8NoArgumentConstructor;
class V8PaintCallback;

// Represents a javascript class registered on the PaintWorkletGlobalScope by
// the author. It will store the properties for invalidation and input argument
// types as well.
class MODULES_EXPORT CSSPaintDefinition final
    : public GarbageCollected<CSSPaintDefinition>,
      public NameClient,
      public PaintDefinition {
 public:
  CSSPaintDefinition(
      ScriptState*,
      V8NoArgumentConstructor* constructor,
      V8PaintCallback* paint,
      const Vector<CSSPropertyID>& native_invalidation_properties,
      const Vector<AtomicString>& custom_invalidation_properties,
      const Vector<CSSSyntaxDefinition>& input_argument_types,
      const PaintRenderingContext2DSettings*,
      PaintWorkletGlobalScope*);
  ~CSSPaintDefinition() override;

  // PaintDefinition override
  PaintRecord Paint(
      const CompositorPaintWorkletInput*,
      const CompositorPaintWorkletJob::AnimatedPropertyValues&) override;

  // Invokes the javascript 'paint' callback on an instance of the javascript
  // class. The size given will be the size of the PaintRenderingContext2D
  // given to the callback.
  //
  // This may return an empty PaintRecord (representing an invalid image) if
  // javascript throws an error.
  //
  // The |container_size| is without subpixel snapping.
  PaintRecord Paint(const gfx::SizeF& container_size,
                    float zoom,
                    StylePropertyMapReadOnly*,
                    const GCedCSSStyleValueVector*);
  const Vector<CSSPropertyID>& NativeInvalidationProperties() const {
    return native_invalidation_properties_;
  }
  const Vector<AtomicString>& CustomInvalidationProperties() const {
    return custom_invalidation_properties_;
  }
  const Vector<CSSSyntaxDefinition>& InputArgumentTypes() const {
    return input_argument_types_;
  }
  const PaintRenderingContext2DSettings* GetPaintRenderingContext2DSettings()
      const {
    return context_settings_.Get();
  }

  ScriptState* GetScriptState() const { return script_state_.Get(); }

  void Trace(Visitor* visitor) const override;
  const char* NameInHeapSnapshot() const override {
    return "CSSPaintDefinition";
  }

 private:
  void MaybeCreatePaintInstance();
  void ApplyAnimatedPropertyOverrides(
      PaintWorkletStylePropertyMap* style_map,
      const CompositorPaintWorkletJob::AnimatedPropertyValues&
          animated_property_values);

  Member<ScriptState> script_state_;

  // This object keeps the class instance object, constructor function and
  // paint function alive. It participates in wrapper tracing as it holds onto
  // V8 wrappers.
  Member<V8NoArgumentConstructor> constructor_;
  Member<V8PaintCallback> paint_;

  // At the moment there is only ever one instance of a paint class per type.
  TraceWrapperV8Reference<v8::Value> instance_;

  bool did_call_constructor_;

  Vector<CSSPropertyID> native_invalidation_properties_;
  Vector<AtomicString> custom_invalidation_properties_;
  // Input argument types, if applicable.
  Vector<CSSSyntaxDefinition> input_argument_types_;
  Member<const PaintRenderingContext2DSettings> context_settings_;
  WeakMember<PaintWorkletGlobalScope> global_scope_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_CSS_PAINT_DEFINITION_H_
