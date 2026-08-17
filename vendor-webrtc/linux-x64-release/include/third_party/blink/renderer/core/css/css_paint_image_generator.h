// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PAINT_IMAGE_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PAINT_IMAGE_GENERATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class CSSSyntaxDefinition;
class Document;
class Image;
class ImageResourceObserver;

// Produces a PaintGeneratedImage from a CSS Paint API callback.
// https://drafts.css-houdini.org/css-paint-api/
class CORE_EXPORT CSSPaintImageGenerator
    : public GarbageCollected<CSSPaintImageGenerator> {
 public:
  // This observer is used if the paint worklet doesn't have a javascript
  // class registered with the correct name yet.
  // paintImageGeneratorReady is called when the javascript class is
  // registered and ready to use.
  class Observer : public GarbageCollected<Observer> {
   public:
    virtual ~Observer() = default;

    virtual void PaintImageGeneratorReady() = 0;
    virtual void Trace(Visitor* visitor) const {}
  };

  static CSSPaintImageGenerator* Create(const String& name,
                                        const Document&,
                                        Observer*);
  virtual ~CSSPaintImageGenerator();

  typedef CSSPaintImageGenerator* (*CSSPaintImageGeneratorCreateFunction)(
      const String&,
      const Document&,
      Observer*);
  static void Init(CSSPaintImageGeneratorCreateFunction);
  static CSSPaintImageGeneratorCreateFunction* GetCreateFunctionForTesting();

  // Invokes the CSS Paint API 'paint' callback. May return a nullptr
  // representing an invalid image if an error occurred.
  // The |container_size| is the container size with subpixel snapping.
  virtual scoped_refptr<Image> Paint(const ImageResourceObserver&,
                                     const gfx::SizeF& container_size,
                                     const GCedCSSStyleValueVector*) = 0;

  virtual const Vector<CSSPropertyID>& NativeInvalidationProperties() const = 0;
  virtual const Vector<AtomicString>& CustomInvalidationProperties() const = 0;
  virtual bool HasAlpha() const = 0;
  virtual const Vector<CSSSyntaxDefinition>& InputArgumentTypes() const = 0;
  virtual bool IsImageGeneratorReady() const = 0;
  virtual int WorkletId() const = 0;

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PAINT_IMAGE_GENERATOR_H_
