// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_LIBFUZZER_PROTO_SKIA_IMAGE_FILTER_PROTO_CONVERTER_H_
#define TESTING_LIBFUZZER_PROTO_SKIA_IMAGE_FILTER_PROTO_CONVERTER_H_

#include <random>
#include <set>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

#include "third_party/skia/include/core/SkPoint.h"

#include "testing/libfuzzer/proto/skia_image_filter.pb.h"

using google::protobuf::FieldDescriptor;
using google::protobuf::Message;
using google::protobuf::Reflection;

typedef std::unordered_map<std::string, std::string> string_map_t;

namespace skia_image_filter_proto_converter {

// Takes an Input proto as input and converts it to a string that will usually
// be deserialized as a skia image filter.
class Converter {
 public:
  Converter();
  Converter(const Converter&);
  ~Converter();

  // Provides the public interface for this class's functionality by converting
  // Input to a string representing a serialized image filter.
  std::string Convert(const Input&);

 private:
  // These constexprs are copied from skia.
  static constexpr uint8_t kICC_Flag = 1 << 1;
  static constexpr size_t kICCTagTableEntrySize = 12;
  static constexpr uint32_t kMatrix_Flag = 1 << 0;
  static constexpr uint8_t k0_Version = 0;
  static constexpr uint8_t kTransferFn_Flag = 1 << 3;
  static constexpr size_t kLut8InputSize = 48;
  static constexpr size_t kOneChannelGammasSize = 256;
  static constexpr size_t kMaxLut16GammaEntries = 4096;
  static constexpr uint8_t kLut8Precision = 1;
  static const uint32_t kPictEofTag;
  static const uint32_t kProfileLookupTable[];
  static const uint32_t kInputColorSpaceLookupTable[];
  static const uint32_t kPCSLookupTable[];
  static const uint32_t kTagLookupTable[];
  static const char kPictureMagicString[];
  static const char kSkPictReaderTag[];
  static const uint8_t kCountNibBits[];

  // The size of kColorTableBuffer.
  static const int kColorTableBufferLength;

  // Used to bound flattenable_depth_.
  static const int kFlattenableDepthLimit;

  // Used to bound numeric fields.
  static const int kNumBound;

  // Used by ColorTableToArray to store a ColorTable Message as an array.
  static uint8_t kColorTableBuffer[];

  // There will be a 1/kMutateEnumDenominator chance that WriteEnum
  // writes an invalid enum value instead of the one given to us by LPM.
  // This must be greater than 1.
  static const uint8_t kMutateEnumDenominator;

  // Mapping of field names to types.
  static const string_map_t kFieldToFlattenableName;

  // Used by IsBlacklisted to determine which skia flattenable should not be
  // serialized.
  static const std::set<std::string> kMisbehavedFlattenableBlacklist;

  // Probably the most important attribute, a char vector that contains
  // serialized skia flattenable written by the Visit functions. The contents of
  // output_ is returned by Convert().
  std::vector<char> output_;

  // Stores the size of output_ when a skia flattenable is being written (since
  // they need to store their size).
  std::vector<size_t> start_sizes_;

  // Keep a record of whether we used kStrokeAndFill_Style or kStroke_Style
  // already since using them multiple times increases the risk of OOMs and
  // timeouts.
  bool stroke_style_used_;

  // Used to keep track of how nested are the skia flattenables we are writing.
  // We use this to limit nesting to avoid OOMs and timeouts.
  int flattenable_depth_;

  // Nesting PairPathEffects is particularly likely to cause OOMs and timeouts
  // so limit this even more than other skia flattenables.
  int pair_path_effect_depth_;

  // Don't allow ComposeColorFilters to contain themselves or else LPM will go
  // crazy and nest them to the point that it causes OOMs and timeouts (these
  // filters are more likely than other skia flattenables to cause these
  // problems).
  bool in_compose_color_filter_;

  // Used to generate random numbers (for replacing NaNs, and mutating enum
  // values).
  std::mt19937 rand_gen_;

  // A distribution from 2-kMutateEnumDenominator that will be used
  // to generate a random value that we will use to decide if an enum value
  // should be written as-is or if it should be mutated.
  // The reason why there is a 2/kMutateEnumDenominator chance rather than a
  // 1/kMutateEnumDenominator chance is because we treat making an enum value
  // too small as a separate case from making it too big.
  std::uniform_int_distribution<> enum_mutator_chance_distribution_;

  // Prevents WriteEnum from writing an invalid enum value instead of the one it
  // was given.
  bool dont_mutate_enum_;

  // BoundNum and BoundFloat will only return positive numbers when this is
  // true.
  bool bound_positive_;

// In production we don't need attributes used by ICC code since it is not
// built for production code.
#ifdef DEVELOPMENT
  uint32_t icc_base_;
  int tag_offset_;
#endif  // DEVELOPMENT

  // Reset any state used by the flattenable so that is can be used to convert
  // another proto input.
  void Reset();

  void Visit(const PictureImageFilter&);
  void Visit(const Picture&);
  void Visit(const PictureTagChild&);

  // The generic Visit function. The compiler will allow this to be called on
  // any proto message, though this won't result in a correct conversion.
  // However some very simple messages have contents that can pretty much be
  // written as they are defined by the fields of msg (eg "DistantLight"). This
  // method is intended for those messages and will properly convert
  // those. Note that this method is viral in that any fields on msg that
  // contain other messages will only be written using this method and
  // WriteFields. For example, "DistantLight" has a field "direction" that
  // contains a "Point3" message. It is OK to call this on "DistantLight"
  // messages because it is OK to call this on "Point3". In essence, it
  // is only correct to call this method on msg if it is correct to call this
  // method on any fields (or fields of fields etc.) of msg that are also
  // Messages. See the file comment on skia_image_filter.proto for an
  // explanation of how this and WriteFields are used for autovisit.
  void Visit(const Message& msg);

  void Visit(const PictureData&);
  void Visit(const RecordingData&);
  void Visit(const LightParent&);
  void Visit(const ImageFilterChild&);
  void Visit(const ImageFilterParent&, const int num_inputs_required);
  void Visit(const MatrixImageFilter&);
  void Visit(const Matrix&, bool is_local = false);
  void Visit(const SpecularLightingImageFilter&);
  void Visit(const PaintImageFilter&);
  void Visit(const Paint&);
  void Visit(const PaintEffects&);
  void Visit(const PathEffectChild&);
  void Visit(const LooperChild&);
  void Visit(const LayerDrawLooper&);
  void Visit(const LayerInfo&);
  void Visit(const ColorFilterChild&);
  void Visit(const ComposeColorFilter&);
  void Visit(const OverdrawColorFilter&);
  void Visit(const ToSRGBColorFilter&);
  void Visit(const ColorFilterMatrix&);
  void Visit(const ColorMatrixFilterRowMajor255&);
  void Visit(const MergeImageFilter&);
  void Visit(const XfermodeImageFilter&);
  void Visit(const DiffuseLightingImageFilter&);
  void Visit(const XfermodeImageFilter_Base&);
  void Visit(const TileImageFilter&);
  void Visit(const OffsetImageFilter&);
  void Visit(const ErodeImageFilter&);
  void Visit(const DilateImageFilter&);
  void Visit(const DiscretePathEffect&);
  void Visit(const MatrixConvolutionImageFilter&);
  void Visit(const MagnifierImageFilter&);
  void Visit(const LocalMatrixImageFilter&);
  void Visit(const ImageSource&);
  void Visit(const Path&);
  void Visit(const PathRef&);
  void Visit(const DropShadowImageFilter&);
  void Visit(const DisplacementMapEffect&);
  void Visit(const ComposeImageFilter&);
  void Visit(const ColorFilterImageFilter&);
  void Visit(const BlurImageFilterImpl&);
  void Visit(const AlphaThresholdFilterImpl&);
  void Visit(const Region&);
  void Visit(const Path1DPathEffect&);

  // Writes the correct PairPathEffect skia flattenable (SkSumPathEffect or
  // SkComposePathEffect) depending on pair.type(). Note that it writes the
  // entire skia flattenable (including name and size) unlike most Visit
  // functions and thus should not be be preceded by a call to
  // PreVisitFlattenable, nor should it be followed by a call to
  // PostVisitFlattenable.
  void Visit(const PairPathEffect&);

  void Visit(const ShaderChild&);
  void Visit(const Color4Shader&);
  void Visit(const GradientDescriptor&);
  void Visit(const GradientParent&);
  void Visit(const TwoPointConicalGradient&);
  void Visit(const LinearGradient&);
  void Visit(const SweepGradient&);
  void Visit(const RadialGradient&);
  void Visit(const PictureShader&);
  void Visit(const LocalMatrixShader&);
  void Visit(const ComposeShader&);
  void Visit(const ColorFilterShader&);
  void Visit(const ImageShader&);
  void Visit(const Color4f&);
  void Visit(const Image&);
  void Visit(const ImageData&);
  void Visit(const ColorSpaceChild&);
  void Visit(const ColorSpace_XYZ&);
  void Visit(const ColorSpaceNamed&);
  void Visit(const TransferFn&);
  void Visit(const MaskFilterChild&);
  void Visit(const Table_ColorFilter&);
  void Visit(const EmbossMaskFilter&);
  void Visit(const EmbossMaskFilterLight&);
  void Visit(const DashImpl&);
  void Visit(const Path2DPathEffect&);
  void Visit(const ArithmeticImageFilter&);
  void Visit(const LightChild&);
  void Visit(const CropRectangle&);
  void Visit(const Rectangle&);
  void Visit(const PictureInfo&);
  void Visit(const BlurMaskFilter&);
  void Visit(const HighContrast_Filter&);
  void Visit(const ReaderPictureTag&);
  void Visit(const Vertices&);
  void Visit(const TextBlob&);
  template <class T>
  void Visit(const google::protobuf::RepeatedPtrField<T>& repeated_field);
  void VisitPictureTag(const PathPictureTag& path_picture_tag, uint32_t tag);
  void VisitPictureTag(const PaintPictureTag& paint_picture_tag, uint32_t tag);
  template <class T>
  void VisitPictureTag(const T& picture_tag_child, uint32_t tag);

  // Returns false if there is too much nesting (determined by
  // kFlattenableDepthLimit and flattenable_depth_).  Writes name and reserves a
  // space to write the size of the flattenable. Also increments
  // flattenable_depth_.
  bool PreVisitFlattenable(const std::string& name);

  // Writes the size of the flattenable to the reserved space, ensures that
  // output_ is four byte aligned and then decrements flattenable_depth_.
  void PostVisitFlattenable();

  std::tuple<int32_t, int32_t, int32_t, int32_t> WriteNonEmptyIRect(
      const IRect& irect);

  void WriteColorSpaceVersion();
  // Write a string in the proper serialized format, padding if necessary.
  void WriteString(std::string str);

  // Get the size of a skia flattenable that was just written and insert it at
  // the proper location. Every call to this method should have a corresponding
  // call to RecordSize.
  void WriteBytesWritten();

  // Reserves space to write the size of what we are serializing and records
  // info so that WriteBytesWritten can determine the size. Every call to this
  // method should have a corresponding call to WriteBytesWritten that it is
  // followed by.
  void RecordSize();

  // Write size to position in output_.
  void InsertSize(const size_t size, const uint32_t position);

  // Pops off the end of start_sizes_.
  size_t PopStartSize();

  // Pad the write_size bytes that were written with zeroes so that the
  // write_size + number of padding bytes is divisible by four.
  void Pad(const size_t write_size);

  // Write size elements of RepeatedField of uint32_ts repeated_field as an
  // array.
  void WriteArray(
      const google::protobuf::RepeatedField<uint32_t>& repeated_field,
      const size_t size);

  // Write size bytes of arr as an array and pad if necessary.
  void WriteArray(const char* arr, const size_t size);

  void WriteBool(const bool bool_val);

  // Write the fields of msg starting with the field whose number is start and
  // ending with the field whose number is end. If end is 0 then all fields
  // until the last one will be written. start defaults to 1 and end defaults to
  // 0. Note that not all possible (eg repeated bools) fields are supported,
  // consult the code to determine if the field is supported (an error will be
  // thrown if a field is unsupported). Note that WriteFields is viral in that
  // if msg contains a field containing another Message, let's say msg2, then
  // WriteFields(msg2) will be called (assuming that fields is in the range of
  // msgs we are writing). If there is a method defined Visit(const Message2&
  // msg2), it will not be called because there is no simple way to determine
  // the type of msg2 using protobuf's reflection API. WriteFields will bound
  // any numeric fields before writing them to avoid OOMs and timeouts. See the
  // file comment on skia_image_filter.proto for an explanation of how this and
  // the generic Visit function are used for autovisit. Note that this may write
  // invalid enum values instead of the ones provided if dont_mutate_enum_ is
  // true. Note that this method may not work if the max field of msg has number
  // that is different than the number of fields. For example, if msg does not
  // have a field with field number 1, but has fields with field numbers 2 and
  // 3, calling WriteFields(msg, 2) will not write field 3.
  void WriteFields(const Message& msg,
                   const unsigned start = 1,
                   const unsigned end = 0);

  // Given the name of a proto field, field_name returns the name of the
  // flattenable skia flattenable object it represents.
  std::string FieldToFlattenableName(const std::string& field_name) const;

  void CheckAlignment() const;
  // Append our proto Message proto_point to sk_points as an SkPoint.
  void AppendAsSkPoint(std::vector<SkPoint>& sk_points,
                       const Point& proto_point) const;

  template <typename T>
  void WriteUInt8(T num);
  void WriteUInt16(uint16_t num);
  void WriteNum(const char (&num_arr)[4]);
  // Write num as a number. Assumes num is four bytes or less.
  template <class T>
  void WriteNum(T num);

  // Write the enum value described by field descriptor and stored on message,
  // or write an invalid enum value with some probability if dont_mutate_enums_
  // is false.
  void WriteEnum(const Message& msg,
                 const Reflection* reflection,
                 const FieldDescriptor* field_descriptor);

  // Bound a num using num_bound so that it won't cause OOMs or timeouts.
  template <typename T>
  T BoundNum(T num, int upper_bound) const;

  // kNumBound cant be a default parameter to BoundNum(T num, int upper_bound)
  // so this function exists instead.
  template <typename T>
  T BoundNum(T num);

  // kNumBound cant be a default parameter to BoundFloat(T num, int upper_bound)
  // so this function exists instead.
  float BoundFloat(float num);

  // Bound a float num using kNumBound so that it won't cause OOMs or
  // timeouts.
  float BoundFloat(float num, const float num_bound);

  // Convert input_num to a uint8_t.
  template <typename T>
  uint8_t ToUInt8(const T input_num) const;

  float GetRandomFloat(std::mt19937* gen_ptr);
  float GetRandomFloat(float seed, float min, float max);

  // Write a sane value of field_value, which should be the value of a field of
  // a matrix.
  void WriteMatrixField(float field_value);

  // Saturating wrapper for stdlib.h's abs, which has undefined behavior when
  // given INT_MIN. This returns abs(INT_MIN+1) if val is INT_MIN.
  int Abs(const int val) const;

  // Writes the representation of a rectangle returned by GetValidRectangle.
  template <typename T>
  void WriteRectangle(std::tuple<T, T, T, T> rectangle);

  // Bound the points making up a rectangle so that the returned tuple is a
  // valid rectangle.
  std::tuple<float, float, float, float> GetValidRectangle(float left,
                                                           float top,
                                                           float right,
                                                           float bottom);

  std::tuple<int32_t, int32_t, int32_t, int32_t> GetValidIRect(int32_t left,
                                                               int32_t top,
                                                               int32_t right,
                                                               int32_t bottom);

  bool IsFinite(float num) const;
  bool IsBlacklisted(const std::string& field_name) const;

  // Converts color_table from our proto Message format to a 256-byte array.
  // Note that this function modifies kColorTableBuffer.
  const uint8_t* ColorTableToArray(const ColorTable& color_table);

#ifdef DEVELOPMENT
  // ICC related functions
  void Visit(const ICC&);
  void Visit(const ICCColorSpace&);
  void Visit(const ICCXYZ&);
  void Visit(const ICCGray&);
  void Visit(const ICCA2B0&);
  void Visit(const ICCA2B0AToB&);
  void Visit(const ICCA2B0Lut8&);
  void Visit(const ICCA2B0Lut16&);
  uint8_t GetClutGridPoints(const ICCA2B0Lut8&);
  uint32_t GetCurrentICCOffset();
  uint32_t GetLut8Size(const ICCA2B0Lut8&);
  uint32_t GetLut16Size(const ICCA2B0Lut16&);
  void WriteLut8(const ICCA2B0Lut8&);
  void WriteA2B0TagCommon();
  void WriteTagSize(const char (&tag)[4], const size_t size);
  void WriteBigEndian(uint32_t num);
  void WriteTagHeader(const uint32_t tag, const uint32_t len);

  // Write num_fields zeroes to fill the space used by ignored or reserved
  // fields in an ICC ColorSpace.
  void WriteIgnoredFields(const int num_fields);

  // Bound illuminant using num to avoid OOMs and timeouts.
  int32_t BoundIlluminant(float illuminant, const float num) const;

  // ImageInfo related functions
  void Visit(const ImageInfo&, const int32_t width, const int32_t height);
  std::tuple<int32_t, int32_t, int32_t>
  GetNumPixelBytes(const ImageInfo& image_info, int32_t width, int32_t height);

  size_t ComputeMinByteSize(int32_t width,
                            int32_t height,
                            ImageInfo::AlphaType alpha_type) const;
#endif  // DEVELOPMENT
};
}  // namespace skia_image_filter_proto_converter
#endif  // TESTING_LIBFUZZER_PROTO_SKIA_IMAGE_FILTER_PROTO_CONVERTER_H_
