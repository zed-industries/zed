// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_PAINT_WORKLET_STYLE_PROPERTY_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_PAINT_WORKLET_STYLE_PROPERTY_MAP_H_

#include <memory>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/cross_thread_style_value.h"
#include "third_party/blink/renderer/core/css/cssom/style_property_map_read_only.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/graphics/platform_paint_worklet_layer_painter.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ComputedStyle;

// This class is designed for CSS Paint such that it can be safely passed cross
// threads.
//
// Here is a typical usage.
// At CSSPaintValue::GetImage which is on the main thread, call the
// BuildCrossThreadData and give the data to the Blink::PaintWorkletInput.
// The PaintWorkletInput is passed to the worklet thread, and we build an
// instance of PaintWorkletStylePropertyMap from the data, and the instance is
// eventually pass to the JS paint callback.
class CORE_EXPORT PaintWorkletStylePropertyMap
    : public StylePropertyMapReadOnly {
 public:
  using StylePropertyMapEntry = std::pair<String, CSSStyleValueVector>;
  using CrossThreadData =
      HashMap<String, std::unique_ptr<CrossThreadStyleValue>>;
  // Build the data that will be passed to the worklet thread to construct a
  // style map. Should be called on the main thread only.
  // TODO(xidachen): consider making the input_property_ids as part of the
  // return value. Or make both CrossThreadData and input_property_ids as
  // params and return a bool.
  static std::optional<CrossThreadData> BuildCrossThreadData(
      const Document&,
      UniqueObjectId unique_object_id,
      const ComputedStyle&,
      const Vector<CSSPropertyID>& native_properties,
      const Vector<AtomicString>& custom_properties,
      CompositorPaintWorkletInput::PropertyKeys& input_property_keys);

  static CrossThreadData CopyCrossThreadData(const CrossThreadData& data);

  // This constructor should be called on the worklet-thread only.
  explicit PaintWorkletStylePropertyMap(CrossThreadData data);
  PaintWorkletStylePropertyMap(const PaintWorkletStylePropertyMap&) = delete;
  PaintWorkletStylePropertyMap& operator=(const PaintWorkletStylePropertyMap&) =
      delete;

  CSSStyleValue* get(const ExecutionContext*,
                     const String& property_name,
                     ExceptionState&) const override;

  CSSStyleValueVector getAll(const ExecutionContext*,
                             const String& property_name,
                             ExceptionState&) const override;

  bool has(const ExecutionContext*,
           const String& property_name,
           ExceptionState&) const override;

  unsigned int size() const override;

  void Trace(Visitor*) const override;

  const CrossThreadData& StyleMapDataForTest() const { return data_; }

  CrossThreadData& StyleMapData() { return data_; }

 private:
  IterationSource* CreateIterationSource(ScriptState*,
                                         ExceptionState&) override;

  CrossThreadData data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_PAINT_WORKLET_STYLE_PROPERTY_MAP_H_
