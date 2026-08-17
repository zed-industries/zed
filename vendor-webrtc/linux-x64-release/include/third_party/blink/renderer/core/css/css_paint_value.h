// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PAINT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PAINT_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_custom_ident_value.h"
#include "third_party/blink/renderer/core/css/css_image_generator_value.h"
#include "third_party/blink/renderer/core/css/css_paint_image_generator.h"
#include "third_party/blink/renderer/core/css/css_variable_data.h"
#include "third_party/blink/renderer/core/css/cssom/cross_thread_style_value.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CORE_EXPORT CSSPaintValue : public CSSImageGeneratorValue {
 public:
  explicit CSSPaintValue(CSSCustomIdentValue* name);
  CSSPaintValue(CSSCustomIdentValue* name, bool threaded_compositing_enabled);
  CSSPaintValue(CSSCustomIdentValue* name,
                HeapVector<Member<CSSVariableData>>&&);
  ~CSSPaintValue();

  String CustomCSSText() const;

  String GetName() const;

  // The |target_size| is container size with subpixel snapping when used
  // in the context of paint images.
  scoped_refptr<Image> GetImage(const ImageResourceObserver&,
                                const Document&,
                                const ComputedStyle&,
                                const gfx::SizeF& target_size);

  bool KnownToBeOpaque(const Document&, const ComputedStyle&) const;

  bool Equals(const CSSPaintValue&) const;

  const Vector<CSSPropertyID>* NativeInvalidationProperties(
      const Document&) const;
  const Vector<AtomicString>* CustomInvalidationProperties(
      const Document&) const;

  const GCedCSSStyleValueVector* GetParsedInputArgumentsForTesting() {
    return parsed_input_arguments_.Get();
  }
  void BuildInputArgumentValuesForTesting(
      Vector<std::unique_ptr<CrossThreadStyleValue>>& style_value) {
    BuildInputArgumentValues(style_value);
  }

  bool IsUsingCustomProperty(const AtomicString& custom_property_name,
                             const Document&) const;

  void CreateGeneratorForTesting(const Document& document) {
    EnsureGenerator(document);
  }
  unsigned NumberOfGeneratorsForTesting() const { return generators_.size(); }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  class Observer final : public CSSPaintImageGenerator::Observer {
   public:
    explicit Observer(CSSPaintValue* owner_value) : owner_value_(owner_value) {}
    Observer(const Observer&) = delete;
    Observer& operator=(const Observer&) = delete;

    ~Observer() override = default;
    void Trace(Visitor* visitor) const override {
      visitor->Trace(owner_value_);
      CSSPaintImageGenerator::Observer::Trace(visitor);
    }

    void PaintImageGeneratorReady() final;

   private:
    Member<CSSPaintValue> owner_value_;
  };

  CSSPaintImageGenerator& EnsureGenerator(const Document&);
  void PaintImageGeneratorReady();

  bool ParseInputArguments(const Document&);

  void BuildInputArgumentValues(
      Vector<std::unique_ptr<CrossThreadStyleValue>>&);

  bool input_arguments_invalid_ = false;

  Member<CSSCustomIdentValue> name_;
  // CSSValues may be shared between Documents. This map stores the
  // CSSPaintImageGenerator for each Document using this CSSPaintValue. We use a
  // WeakMember to ensure that entries are removed when Documents are destroyed
  // (since the CSSValue may outlive any given Document).
  HeapHashMap<WeakMember<const Document>, Member<CSSPaintImageGenerator>>
      generators_;
  Member<Observer> paint_image_generator_observer_;
  Member<GCedCSSStyleValueVector> parsed_input_arguments_;
  HeapVector<Member<CSSVariableData>> argument_variable_data_;
  enum class OffThreadPaintState { kUnknown, kOffThread, kMainThread };
  // Indicates whether this paint worklet is composited or not. kUnknown
  // indicates that it has not been decided yet.
  // TODO(crbug.com/987974): Make this variable reset when there is a style
  // change.
  OffThreadPaintState off_thread_paint_state_;
};

template <>
struct DowncastTraits<CSSPaintValue> {
  static bool AllowFrom(const CSSValue& value) { return value.IsPaintValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PAINT_VALUE_H_
