// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_CSS_PAINT_IMAGE_GENERATOR_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_CSS_PAINT_IMAGE_GENERATOR_IMPL_H_

#include "third_party/blink/renderer/core/css/css_paint_image_generator.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/scoped_persistent.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/gfx/geometry/size_f.h"
#include "v8/include/v8.h"

namespace blink {

class CSSSyntaxDefinition;
class Document;
class DocumentPaintDefinition;
class Image;
class PaintWorklet;

class MODULES_EXPORT CSSPaintImageGeneratorImpl final
    : public CSSPaintImageGenerator {
 public:
  static CSSPaintImageGenerator* Create(const String& name,
                                        const Document&,
                                        Observer*);

  CSSPaintImageGeneratorImpl(Observer*, PaintWorklet*, const String&);
  CSSPaintImageGeneratorImpl(PaintWorklet*, const String&);
  ~CSSPaintImageGeneratorImpl() override;

  // The |container_size| is without subpixel snapping.
  scoped_refptr<Image> Paint(const ImageResourceObserver&,
                             const gfx::SizeF& container_size,
                             const GCedCSSStyleValueVector*) final;
  const Vector<CSSPropertyID>& NativeInvalidationProperties() const final;
  const Vector<AtomicString>& CustomInvalidationProperties() const final;
  bool HasAlpha() const final;
  const Vector<CSSSyntaxDefinition>& InputArgumentTypes() const final;
  bool IsImageGeneratorReady() const final;
  int WorkletId() const final;

  // Should be called from the PaintWorkletGlobalScope when a javascript class
  // is registered with the same name.
  void NotifyGeneratorReady();

  bool GetValidDocumentDefinitionForTesting(
      DocumentPaintDefinition*& definition) const {
    return GetValidDocumentDefinition(definition);
  }
  unsigned GetRegisteredDefinitionCountForTesting() const;

  void Trace(Visitor*) const override;

 private:
  // Used for main-thread CSS Paint.
  bool HasDocumentDefinition() const;

  // Used for main-thread CSS Paint.
  // This function first checks whether the document definition with |name_|
  // exists or not. If it does exist, the function fetches the document
  // definition and checks if it is valid. The function returns true when the
  // document definition exists and is valid.
  bool GetValidDocumentDefinition(DocumentPaintDefinition*&) const;

  Member<Observer> observer_;
  Member<PaintWorklet> paint_worklet_;
  const String name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_CSS_PAINT_IMAGE_GENERATOR_IMPL_H_
