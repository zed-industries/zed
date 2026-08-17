// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CSS_LAYOUT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CSS_LAYOUT_DEFINITION_H_

#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/bindings/scoped_persistent.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "v8/include/v8.h"

namespace blink {

class BlockNode;
class ConstraintSpace;
class CustomLayoutScope;
class FragmentResultOptions;
class IntrinsicSizesResultOptions;
class ScriptState;
class SerializedScriptValue;
class V8IntrinsicSizesCallback;
class V8LayoutCallback;
class V8NoArgumentConstructor;
struct BoxStrut;
struct LogicalSize;

// Represents a javascript class registered on the LayoutWorkletGlobalScope by
// the author.
// https://drafts.css-houdini.org/css-layout-api/#layout-definition
class CSSLayoutDefinition final : public GarbageCollected<CSSLayoutDefinition>,
                                  public NameClient {
 public:
  CSSLayoutDefinition(
      ScriptState*,
      V8NoArgumentConstructor* constructor,
      V8IntrinsicSizesCallback* intrinsic_sizes,
      V8LayoutCallback* layout,
      const Vector<CSSPropertyID>& native_invalidation_properties,
      const Vector<AtomicString>& custom_invalidation_properties,
      const Vector<CSSPropertyID>& child_native_invalidation_properties,
      const Vector<AtomicString>& child_custom_invalidation_properties);
  ~CSSLayoutDefinition() final;

  // This class represents an instance of the layout class defined by the
  // CSSLayoutDefinition.
  class Instance final : public GarbageCollected<Instance> {
   public:
    Instance(CSSLayoutDefinition*, v8::Local<v8::Value> instance);

    // Runs the web developer defined layout, returns true if everything
    // succeeded. It populates the FragmentResultOptions dictionary, and
    // fragment_result_data.
    bool Layout(const ConstraintSpace&,
                const Document&,
                const BlockNode&,
                const LogicalSize& border_box_size,
                const BoxStrut& border_scrollbar_padding,
                CustomLayoutScope*,
                FragmentResultOptions*&,
                scoped_refptr<SerializedScriptValue>* fragment_result_data);

    // Runs the web developer defined intrinsicSizes, returns true if everything
    // succeeded. It populates the IntrinsicSizesResultOptions dictionary.
    bool IntrinsicSizes(const ConstraintSpace&,
                        const Document&,
                        const BlockNode&,
                        const LogicalSize& border_box_size,
                        const BoxStrut& border_scrollbar_padding,
                        const LayoutUnit child_available_block_size,
                        CustomLayoutScope*,
                        IntrinsicSizesResultOptions**,
                        bool* child_depends_on_block_constraints);

    void Trace(Visitor*) const;

   private:
    Member<CSSLayoutDefinition> definition_;
    TraceWrapperV8Reference<v8::Value> instance_;
  };

  // Creates an instance of the web developer defined class. May return a
  // nullptr if constructing the instance failed.
  Instance* CreateInstance();

  const Vector<CSSPropertyID>& NativeInvalidationProperties() const {
    return native_invalidation_properties_;
  }
  const Vector<AtomicString>& CustomInvalidationProperties() const {
    return custom_invalidation_properties_;
  }
  const Vector<CSSPropertyID>& ChildNativeInvalidationProperties() const {
    return child_native_invalidation_properties_;
  }
  const Vector<AtomicString>& ChildCustomInvalidationProperties() const {
    return child_custom_invalidation_properties_;
  }

  ScriptState* GetScriptState() const { return script_state_.Get(); }

  void Trace(Visitor* visitor) const;

  const char* NameInHeapSnapshot() const override {
    return "CSSLayoutDefinition";
  }

 private:
  Member<ScriptState> script_state_;

  // This object keeps the class instances, constructor function, intrinsic
  // sizes function, and layout function alive. It participates in wrapper
  // tracing as it holds onto V8 wrappers.
  Member<V8NoArgumentConstructor> constructor_;
  Member<V8IntrinsicSizesCallback> intrinsic_sizes_;
  Member<V8LayoutCallback> layout_;

  // If a constructor call ever fails, we'll refuse to create any more
  // instances of the web developer provided class.
  bool constructor_has_failed_ = false;

  Vector<CSSPropertyID> native_invalidation_properties_;
  Vector<AtomicString> custom_invalidation_properties_;
  Vector<CSSPropertyID> child_native_invalidation_properties_;
  Vector<AtomicString> child_custom_invalidation_properties_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CSS_LAYOUT_DEFINITION_H_
