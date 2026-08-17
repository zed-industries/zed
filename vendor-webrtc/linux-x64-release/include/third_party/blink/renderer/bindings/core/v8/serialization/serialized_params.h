// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZED_PARAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZED_PARAMS_H_

#include "third_party/blink/renderer/core/html/canvas/image_data.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"

namespace blink {

// This enumeration specifies the extra tags used to specify the color settings
// of the serialized/deserialized ImageData and ImageBitmap objects.
enum class ImageSerializationTag : uint32_t {
  kEndTag = 0,
  // followed by a SerializedPredefinedColorSpace enum. This is used for
  // ImageData, and only for backwards compatibility for ImageBitmap. For
  // ImageBitmap, the tag kParametricColorSpaceTag should be used (and
  // overrides kPredefinedColorSpaceTag if both are are specified, although
  // both should not be specified).
  kPredefinedColorSpaceTag = 1,
  // followed by a SerializedPixelFormat enum, used only for ImageBitmap.
  kCanvasPixelFormatTag = 2,
  // followed by a SerializedImageDataPixelFormat enum, used only for
  // ImageData.
  kImageDataPixelFormatTag = 3,
  // followed by 1 if the image is origin clean and zero otherwise.
  kOriginCleanTag = 4,
  // followed by 1 if the image is premultiplied and zero otherwise.
  kIsPremultipliedTag = 5,
  // followed by 1 if the image is known to be opaque (alpha = 1 everywhere)
  kCanvasOpacityModeTag = 6,
  // followed by 16 doubles, for a parametric color space definition. The first
  // 7 doubles are the parameters g,a,b,c,d,e,f, in that order, for a transfer
  // function to convert encoded to linear space as
  //   f(x) = c*x + f         : 0 <= x < d
  //        = (a*x + b)^g + e : d <= x
  // The PQ transfer function is indicated by g=-2 and the HLG transfer
  // function is indicated by g=-3. The next 9 doubles are a 3x3 matrix in
  // row-major order that converts an r,g,b triple in the linear color space to
  // x,y,z in the XYZ D50 color space.
  kParametricColorSpaceTag = 7,
  // followed by a SerializedImageOrientation enum, used only for ImageBitmap.
  kImageOrientationTag = 8,

  kLast = kImageOrientationTag,
};

// This enumeration specifies the values used to serialize PredefinedColorSpace.
enum class SerializedPredefinedColorSpace : uint32_t {
  // Legacy sRGB color space is deprecated as of M65. Objects in legacy color
  // space will be serialized as sRGB since the legacy behavior is now merged
  // with sRGB color space. Deserialized objects with legacy color space also
  // will be interpreted as sRGB.
  kLegacyObsolete = 0,
  kSRGB = 1,
  kRec2020 = 2,
  kP3 = 3,
  kRec2100HLG = 4,
  kRec2100PQ = 5,
  kSRGBLinear = 6,
  kLast = kSRGBLinear,
};

// This enumeration specifies the values used to serialize CanvasPixelFormat.
enum class SerializedPixelFormat : uint32_t {
  // This is to preserve legacy object when Native was a possible enum state
  // this will be resolved as either a RGB or BGR pixel format for
  // canvas_color_params
  kNative8_LegacyObsolete = 0,
  kF16 = 1,
  kRGBA8 = 2,
  kBGRA8 = 3,
  kRGBX8 = 4,
  kLast = kRGBX8,
};

// This enumeration specifies the values used to serialize
// ImageDataPixelFormat.
enum class SerializedImageDataPixelFormat : uint32_t {
  kRgbaUnorm8 = 0,
  kRgbaFloat16 = 1,
  kRgbaFloat32 = 2,
  kLast = kRgbaFloat32,
};

enum class SerializedOpacityMode : uint32_t {
  kNonOpaque = 0,
  kOpaque = 1,
  kLast = kOpaque,
};

enum class SerializedImageOrientation : uint32_t {
  kTopLeft = 0,
  kTopRight = 1,
  kBottomRight = 2,
  kBottomLeft = 3,
  kLeftTop = 4,
  kRightTop = 5,
  kRightBottom = 6,
  kLeftBottom = 7,
  kLast = kLeftBottom,
};

enum class SerializedTextDirection : uint32_t {
  kLtr = 0,
  kRtl = 1,
  kLast = kRtl,
};

class SerializedImageDataSettings {
 public:
  SerializedImageDataSettings(PredefinedColorSpace, V8ImageDataPixelFormat);
  SerializedImageDataSettings(SerializedPredefinedColorSpace,
                              SerializedImageDataPixelFormat);

  ImageDataSettings* GetImageDataSettings() const;

  SerializedPredefinedColorSpace GetSerializedPredefinedColorSpace() const {
    return color_space_;
  }
  SerializedImageDataPixelFormat GetSerializedImageDataPixelFormat() const {
    return pixel_format_;
  }

 private:
  SerializedPredefinedColorSpace color_space_ =
      SerializedPredefinedColorSpace::kSRGB;
  SerializedImageDataPixelFormat pixel_format_ =
      SerializedImageDataPixelFormat::kRgbaUnorm8;
};

inline constexpr uint32_t kSerializedParametricColorSpaceLength = 16;
inline constexpr float kSerializedPQConstant = -2.f;
inline constexpr float kSerializedHLGConstant = -3.f;

class SerializedImageBitmapSettings {
 public:
  SerializedImageBitmapSettings();
  explicit SerializedImageBitmapSettings(SkImageInfo, ImageOrientationEnum);
  SerializedImageBitmapSettings(SerializedPredefinedColorSpace,
                                const Vector<double>& sk_color_space,
                                SerializedPixelFormat,
                                SerializedOpacityMode,
                                uint32_t is_premultiplied,
                                SerializedImageOrientation);

  SkImageInfo GetSkImageInfo(uint32_t width, uint32_t height) const;
  ImageOrientationEnum GetImageOrientation() const;

  const Vector<double>& GetSerializedSkColorSpace() { return sk_color_space_; }
  SerializedPixelFormat GetSerializedPixelFormat() const {
    return pixel_format_;
  }
  SerializedOpacityMode GetSerializedOpacityMode() const {
    return opacity_mode_;
  }
  uint32_t IsPremultiplied() const { return is_premultiplied_; }
  SerializedImageOrientation GetSerializedImageOrientation() const {
    return image_orientation_;
  }

 private:
  SerializedPredefinedColorSpace color_space_ =
      SerializedPredefinedColorSpace::kSRGB;
  Vector<double> sk_color_space_;
  SerializedPixelFormat pixel_format_ = SerializedPixelFormat::kRGBA8;
  SerializedOpacityMode opacity_mode_ = SerializedOpacityMode::kNonOpaque;
  bool is_premultiplied_ = true;
  SerializedImageOrientation image_orientation_ =
      SerializedImageOrientation::kTopLeft;
};

class SerializedTextDirectionSettings {
 public:
  SerializedTextDirectionSettings(TextDirection);
  SerializedTextDirectionSettings(SerializedTextDirection);

  TextDirection GetTextDirection() const;

  SerializedTextDirection GetSerializedTextDirection() const {
    return text_direction_;
  }

 private:
  SerializedTextDirection text_direction_ = SerializedTextDirection::kLtr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_SERIALIZED_PARAMS_H_
