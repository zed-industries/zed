/*
 * Copyright (C) 2008, 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_IMAGE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_IMAGE_DATA_H_

#include "base/numerics/checked_math.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_image_data_settings.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/imagebitmap/image_bitmap_source.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/predefined_color_space.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/skia/include/core/SkPixmap.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class ExceptionState;
class ImageBitmapOptions;
class V8ImageDataPixelFormat;
class V8PredefinedColorSpace;

class CORE_EXPORT ImageData final : public ScriptWrappable,
                                    public ImageBitmapSource {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructors that take width, height, and an optional ImageDataSettings.
  static ImageData* Create(unsigned width,
                           unsigned height,
                           ExceptionState& exception_state) {
    return ValidateAndCreate(width, height, std::nullopt, /*settings=*/nullptr,
                             ValidateAndCreateParams(), exception_state);
  }
  static ImageData* Create(unsigned width,
                           unsigned height,
                           const ImageDataSettings* settings,
                           ExceptionState& exception_state) {
    return ValidateAndCreate(width, height, std::nullopt, settings,
                             ValidateAndCreateParams(), exception_state);
  }

  // Constructors that take Uint8ClampedArray, width, optional height, and
  // optional ImageDataSettings.
  static ImageData* Create(NotShared<DOMUint8ClampedArray> data,
                           unsigned width,
                           ExceptionState& exception_state) {
    return ValidateAndCreate(width, std::nullopt, data, nullptr,
                             ValidateAndCreateParams(), exception_state);
  }
  static ImageData* Create(NotShared<DOMUint8ClampedArray> data,
                           unsigned width,
                           unsigned height,
                           ExceptionState& exception_state) {
    return ValidateAndCreate(width, height, data, /*settings=*/nullptr,
                             ValidateAndCreateParams(), exception_state);
  }
  static ImageData* Create(NotShared<DOMUint8ClampedArray> data,
                           unsigned width,
                           unsigned height,
                           const ImageDataSettings* settings,
                           ExceptionState& exception_state) {
    return ValidateAndCreate(width, height, data, settings,
                             ValidateAndCreateParams(), exception_state);
  }

  // Constructor that takes DOMFloat16Array, width, optional height, and
  // optional ImageDataSettings.
  static ImageData* Create(NotShared<DOMFloat16Array> data,
                           unsigned width,
                           ExceptionState& exception_state) {
    ValidateAndCreateParams params;
    params.require_canvas_floating_point = true;
    return ValidateAndCreate(width, std::nullopt, data, nullptr, params,
                             exception_state);
  }
  static ImageData* Create(NotShared<DOMFloat16Array> data,
                           unsigned width,
                           unsigned height,
                           const ImageDataSettings* settings,
                           ExceptionState& exception_state) {
    ValidateAndCreateParams params;
    params.require_canvas_floating_point = true;
    return ValidateAndCreate(width, height, data, settings, params,
                             exception_state);
  }

  // Constructor that takes DOMFloat32Array, width, optional height, and
  // optional ImageDataSettings.
  static ImageData* Create(NotShared<DOMFloat32Array> data,
                           unsigned width,
                           ExceptionState& exception_state) {
    ValidateAndCreateParams params;
    params.require_canvas_floating_point = true;
    return ValidateAndCreate(width, std::nullopt, data, nullptr, params,
                             exception_state);
  }
  static ImageData* Create(NotShared<DOMFloat32Array> data,
                           unsigned width,
                           unsigned height,
                           const ImageDataSettings* settings,
                           ExceptionState& exception_state) {
    ValidateAndCreateParams params;
    params.require_canvas_floating_point = true;
    return ValidateAndCreate(width, height, data, settings, params,
                             exception_state);
  }

  // ValidateAndCreate is the common path that all ImageData creation code
  // should call directly. The other Create functions are to be called only by
  // generated code.
  struct ValidateAndCreateParams {
    // When a too-large ImageData is created using a constructor, it has
    // historically thrown an IndexSizeError. When created through a 2D
    // canvas, it has historically thrown a RangeError. This flag will
    // trigger the RangeError path.
    bool context_2d_error_mode = false;
    // Constructors in IDL files cannot specify RuntimeEnabled restrictions.
    // This argument is passed by Create functions that should require that the
    // CanvasFloatingPoint feature be enabled.
    bool require_canvas_floating_point = false;
    // If the caller is guaranteed to write over the result in its entirety,
    // then this flag may be used to skip initialization of the result's
    // data.
    bool zero_initialize = true;
    // If no color space is specified, then use this value for the resulting
    // ImageData.
    PredefinedColorSpace default_color_space = PredefinedColorSpace::kSRGB;
  };
  static ImageData* ValidateAndCreate(
      unsigned width,
      std::optional<unsigned> height,
      std::optional<NotShared<DOMArrayBufferView>> data,
      const ImageDataSettings* settings,
      ValidateAndCreateParams params,
      ExceptionState& exception_state);
  // TODO(https://crbug.com/1198606): Remove this.
  ImageDataSettings* getSettings() { return settings_.Get(); }

  static ImageData* CreateForTest(const gfx::Size&);
  static ImageData* CreateForTest(const gfx::Size&,
                                  NotShared<DOMArrayBufferView>,
                                  PredefinedColorSpace,
                                  SkColorType);

  ImageData(const gfx::Size&,
            NotShared<DOMArrayBufferView>,
            PredefinedColorSpace,
            SkColorType);

  gfx::Size Size() const { return size_; }
  int width() const { return size_.width(); }
  int height() const { return size_.height(); }
  V8PredefinedColorSpace colorSpace() const;
  V8ImageDataPixelFormat pixelFormat() const;

  // TODO(https://crbug.com/1198606): Remove this.
  ImageDataSettings* getSettings() const;

  const V8ImageDataArray* data() const { return data_.Get(); }

  bool IsBufferBaseDetached() const;
  PredefinedColorSpace GetPredefinedColorSpace() const { return color_space_; }
  SkColorType GetSkColorType() const { return color_type_; }

  // Returns a span to the raw bytes of the underlying data. Requires that the
  // buffer is attached.
  base::span<uint8_t> RawByteSpan() const;

  // Return an SkPixmap that references this data directly.
  SkPixmap GetSkPixmap() const;

  // ImageBitmapSource implementation
  ImageBitmapSourceStatus CheckUsability() const override { return base::ok(); }
  ScriptPromise<ImageBitmap> CreateImageBitmap(
      ScriptState*,
      std::optional<gfx::Rect> crop_rect,
      const ImageBitmapOptions*,
      ExceptionState&) override;

  void Trace(Visitor*) const override;

  [[nodiscard]] v8::Local<v8::Object> AssociateWithWrapper(
      v8::Isolate* isolate,
      const WrapperTypeInfo* wrapper_type_info,
      v8::Local<v8::Object> wrapper) override;

 private:
  gfx::Size size_;
  // TODO(https://crbug.com/1198606): Remove this.
  Member<ImageDataSettings> settings_;
  Member<V8ImageDataArray> data_;
  NotShared<DOMUint8ClampedArray> data_u8_;
  NotShared<DOMFloat16Array> data_f16_;
  NotShared<DOMFloat32Array> data_f32_;
  PredefinedColorSpace color_space_ = PredefinedColorSpace::kSRGB;
  SkColorType color_type_ = kRGBA_8888_SkColorType;

  static NotShared<DOMArrayBufferView> AllocateAndValidateDataArray(
      const unsigned&,
      SkColorType,
      bool initialize,
      ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_IMAGE_DATA_H_
