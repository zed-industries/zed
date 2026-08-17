/*
 * Copyright (C) 2006 Apple Computer, Inc.  All rights reserved.
 * Copyright (C) Research In Motion Limited 2009-2010. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_H_

#include <array>
#include <memory>
#include <optional>

#include "base/check_op.h"
#include "base/containers/heap_array.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/numerics/checked_math.h"
#include "base/numerics/safe_conversions.h"
#include "base/strings/strcat.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/graphics/color_behavior.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation_enum.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_image.h"
#include "third_party/blink/renderer/platform/image-decoders/image_animation.h"
#include "third_party/blink/renderer/platform/image-decoders/image_frame.h"
#include "third_party/blink/renderer/platform/image-decoders/segment_reader.h"
#include "third_party/blink/renderer/platform/instrumentation/histogram.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/skia/include/core/SkImageInfo.h"
#include "third_party/skia/modules/skcms/skcms.h"

class SkColorSpace;
class SkData;

namespace gfx {
struct HDRMetadata;
}  // namespace gfx

namespace blink {

#if SK_B32_SHIFT
inline skcms_PixelFormat XformColorFormat() {
  return skcms_PixelFormat_RGBA_8888;
}
#else
inline skcms_PixelFormat XformColorFormat() {
  return skcms_PixelFormat_BGRA_8888;
}
#endif

// ImagePlanes can be used to decode color components into provided buffers
// instead of using an ImageFrame.
class PLATFORM_EXPORT ImagePlanes final {
  USING_FAST_MALLOC(ImagePlanes);

 public:
  ImagePlanes();
  ImagePlanes(const ImagePlanes&) = delete;
  ImagePlanes& operator=(const ImagePlanes&) = delete;

  // |color_type| is kGray_8_SkColorType if GetYUVBitDepth() == 8 and either
  // kA16_float_SkColorType or kA16_unorm_SkColorType if GetYUVBitDepth() > 8.
  //
  // TODO(crbug/910276): To support YUVA, ImagePlanes needs to support a
  // variable number of planes.
  ImagePlanes(base::span<void*, cc::kNumYUVPlanes> planes,
              base::span<const wtf_size_t, cc::kNumYUVPlanes> row_bytes,
              SkColorType color_type);

  void* Plane(cc::YUVIndex);
  wtf_size_t RowBytes(cc::YUVIndex) const;
  SkColorType color_type() const { return color_type_; }
  void SetHasCompleteScan() { has_complete_scan_ = true; }
  bool HasCompleteScan() const { return has_complete_scan_; }

 private:
  std::array<void*, cc::kNumYUVPlanes> planes_;
  std::array<wtf_size_t, cc::kNumYUVPlanes> row_bytes_;
  SkColorType color_type_ = kUnknown_SkColorType;
  bool has_complete_scan_ = false;
};

class PLATFORM_EXPORT ColorProfile final {
  USING_FAST_MALLOC(ColorProfile);

 public:
  ColorProfile(const skcms_ICCProfile&,
               base::HeapArray<uint8_t> = base::HeapArray<uint8_t>());
  ColorProfile(const ColorProfile&) = delete;
  ColorProfile& operator=(const ColorProfile&) = delete;
  static std::unique_ptr<ColorProfile> Create(base::span<const uint8_t> buffer);
  ~ColorProfile();

  const skcms_ICCProfile* GetProfile() const { return &profile_; }

 private:
  skcms_ICCProfile profile_;
  base::HeapArray<uint8_t> buffer_;
};

class PLATFORM_EXPORT ColorProfileTransform final {
  USING_FAST_MALLOC(ColorProfileTransform);

 public:
  ColorProfileTransform(const skcms_ICCProfile* src_profile,
                        const skcms_ICCProfile* dst_profile);
  ColorProfileTransform(const ColorProfileTransform&) = delete;
  ColorProfileTransform& operator=(const ColorProfileTransform&) = delete;
  ~ColorProfileTransform();

  const skcms_ICCProfile* SrcProfile() const;
  const skcms_ICCProfile* DstProfile() const;

 private:
  raw_ptr<const skcms_ICCProfile> src_profile_;
  skcms_ICCProfile dst_profile_;
};

// ImageDecoder is a base for all format-specific decoders
// (e.g. JPEGImageDecoder). This base manages the ImageFrame cache.
//
class PLATFORM_EXPORT ImageDecoder {
  USING_FAST_MALLOC(ImageDecoder);

 public:
  static constexpr wtf_size_t kNoDecodedImageByteLimit =
      static_cast<wtf_size_t>(-1);

  enum AlphaOption { kAlphaPremultiplied, kAlphaNotPremultiplied };
  enum HighBitDepthDecodingOption {
    // Decode everything to uint8 pixel format (kN32 channel order).
    kDefaultBitDepth,
    // Decode high bit depth images to half float pixel format.
    kHighBitDepthToHalfFloat
  };

  // The first three values are as defined in webp/decode.h, the last value
  // specifies WebP animation formats.
  enum CompressionFormat {
    kUndefinedFormat = 0,
    kLossyFormat = 1,
    kLosslessFormat = 2,
    kWebPAnimationFormat = 3,
    kMaxValue = kWebPAnimationFormat,
  };

  // For images which contain both animations and still images, indicates which
  // is preferred. When unspecified the decoder will use hints from the data
  // stream to make a decision.
  //
  // Note: |animation_option| is unused by formats like GIF or APNG since they
  // do not have distinct still and animated tracks. I.e., there is either only
  // an animation or only a still image. If a caller only wants a still image
  // from a GIF or APNG animation, they can choose to only decode the first
  // frame. With a format like AVIF where there are distinct still and animation
  // tracks, callers need a mechanism to choose.
  enum class AnimationOption {
    kUnspecified,
    kPreferAnimation,
    kPreferStillImage,
  };

  ImageDecoder(const ImageDecoder&) = delete;
  ImageDecoder& operator=(const ImageDecoder&) = delete;
  virtual ~ImageDecoder();

  // Returns a caller-owned decoder of the appropriate type.  Returns nullptr if
  // we can't sniff a supported type from the provided data (possibly
  // because there isn't enough data yet).
  // Sets |max_decoded_bytes_| to Platform::MaxImageDecodedBytes().
  static std::unique_ptr<ImageDecoder> Create(
      scoped_refptr<SegmentReader> data,
      bool data_complete,
      AlphaOption,
      HighBitDepthDecodingOption,
      ColorBehavior,
      cc::AuxImage aux_image,
      const size_t platform_max_decoded_bytes,
      const SkISize& desired_size = SkISize::MakeEmpty(),
      AnimationOption animation_option = AnimationOption::kUnspecified);
  static std::unique_ptr<ImageDecoder> Create(
      scoped_refptr<SharedBuffer> data,
      bool data_complete,
      AlphaOption alpha_option,
      HighBitDepthDecodingOption high_bit_depth_decoding_option,
      ColorBehavior color_behavior,
      cc::AuxImage aux_image,
      size_t platform_max_decoded_bytes,
      const SkISize& desired_size = SkISize::MakeEmpty(),
      AnimationOption animation_option = AnimationOption::kUnspecified) {
    return Create(SegmentReader::CreateFromSharedBuffer(std::move(data)),
                  data_complete, alpha_option, high_bit_depth_decoding_option,
                  color_behavior, aux_image, platform_max_decoded_bytes,
                  desired_size, animation_option);
  }

  // Similar to above, but does not allow mime sniffing. Creates explicitly
  // based on the |mime_type| value.
  static std::unique_ptr<ImageDecoder> CreateByMimeType(
      String mime_type,
      scoped_refptr<SegmentReader> data,
      bool data_complete,
      AlphaOption alpha_option,
      HighBitDepthDecodingOption high_bit_depth_decoding_option,
      ColorBehavior color_behavior,
      cc::AuxImage aux_image,
      size_t platform_max_decoded_bytes,
      const SkISize& desired_size = SkISize::MakeEmpty(),
      AnimationOption animation_option = AnimationOption::kUnspecified);

  virtual String FilenameExtension() const = 0;
  virtual const AtomicString& MimeType() const = 0;

  bool IsAllDataReceived() const;

  // Returns true if the decoder supports decoding to high bit depth. The
  // decoded output will be high bit depth (half float backed bitmap) iff
  // encoded image is high bit depth and high_bit_depth_decoding_option_ is set
  // to kHighBitDepthToHalfFloat.
  virtual bool ImageIsHighBitDepth();

  // Returns true if the buffer holds enough data to instantiate a decoder.
  // This is useful for callers to determine whether a decoder instantiation
  // failure is due to insufficient or bad data.
  static bool HasSufficientDataToSniffMimeType(const SharedBuffer&);

  // Looks at the image data to determine and return the image MIME type.
  static String SniffMimeType(scoped_refptr<SharedBuffer> image_data);

  // Returns the image data's compression format.
  static CompressionFormat GetCompressionFormat(
      scoped_refptr<SharedBuffer> image_data,
      String mime_type);

  // Chooses one of the Blink.DecodedImage.<type>Density.Count.* histograms
  // based on the image area, and adds 1 to the bucket for the bits per pixel
  // in the histogram.
  template <const char type[]>
  static void UpdateBppHistogram(gfx::Size size, size_t image_size_bytes);

  void SetData(scoped_refptr<SegmentReader> data, bool all_data_received) {
    if (failed_) {
      return;
    }
    data_ = std::move(data);
    is_all_data_received_ = all_data_received;
    OnSetData(data_);
  }

  void SetData(scoped_refptr<SharedBuffer> data, bool all_data_received) {
    SetData(SegmentReader::CreateFromSharedBuffer(std::move(data)),
            all_data_received);
  }

  virtual void OnSetData(scoped_refptr<SegmentReader> data) {}

  bool IsSizeAvailable();

  bool IsDecodedSizeAvailable() const { return !failed_ && size_available_; }

  virtual gfx::Size Size() const;
  virtual Vector<SkISize> GetSupportedDecodeSizes() const;

  // Check for the existence of a gainmap image. If one exists, extract the
  // SkGainmapInfo rendering parameters, and a SegmentReader for the embedded
  // gainmap image's encoded data, and return true.
  virtual bool GetGainmapInfoAndData(
      SkGainmapInfo& out_gainmap_info,
      scoped_refptr<SegmentReader>& out_gainmap_data) const;

  // Decoders which downsample images should override this method to
  // return the actual decoded size.
  virtual gfx::Size DecodedSize() const;

  // The YUV subsampling of the image.
  virtual cc::YUVSubsampling GetYUVSubsampling() const;

  // Image decoders that support YUV decoding must override this to
  // provide the size of each component.
  virtual gfx::Size DecodedYUVSize(cc::YUVIndex) const;

  // Image decoders that support YUV decoding must override this to
  // return the width of each row of the memory allocation.
  virtual wtf_size_t DecodedYUVWidthBytes(cc::YUVIndex) const;

  // Image decoders that support YUV decoding must override this to
  // return the SkYUVColorSpace that is used to convert from YUV
  // to RGB.
  virtual SkYUVColorSpace GetYUVColorSpace() const;

  // Image decoders that support high bit depth YUV decoding can override this.
  //
  // Note: If an implementation advertises a bit depth > 8 it must support both
  // kA16_unorm_SkColorType and kA16_float_SkColorType ImagePlanes.
  virtual uint8_t GetYUVBitDepth() const;

  // Image decoders that support HDR metadata can override this.
  virtual std::optional<gfx::HDRMetadata> GetHDRMetadata() const;

  // Image decoders that support C2PA manifest embedding can override this.
  virtual bool HasC2PAManifest() const;

  // Returns the information required to decide whether or not hardware
  // acceleration can be used to decode this image. Callers of this function
  // must ensure the header was successfully parsed prior to calling this
  // method, i.e., IsDecodedSizeAvailable() must return true.
  virtual cc::ImageHeaderMetadata MakeMetadataForDecodeAcceleration() const;

  // This will only differ from Size() for ICO (where each frame is a
  // different icon) or other formats where different frames are different
  // sizes. This does NOT differ from Size() for GIF or WebP, since
  // decoding GIF or WebP composites any smaller frames against previous
  // frames to create full-size frames.
  virtual gfx::Size FrameSizeAtIndex(wtf_size_t) const;

  // Returns whether the size is legal (i.e. not going to result in
  // overflow elsewhere).  If not, marks decoding as failed.
  virtual bool SetSize(unsigned width, unsigned height);

  // Calls DecodeFrameCount() to get the current frame count (if possible),
  // without decoding the individual frames.  Resizes |frame_buffer_cache_| to
  // the new size and returns that size.
  //
  // Note: FrameCount() returns the return value of DecodeFrameCount(). For more
  // information on the return value, see the comment for DecodeFrameCount().
  wtf_size_t FrameCount();

  virtual int RepetitionCount() const;

  // Decodes as much of the requested frame as possible, and returns an
  // ImageDecoder-owned pointer.
  ImageFrame* DecodeFrameBufferAtIndex(wtf_size_t);

  // Whether the requested frame has alpha.
  virtual bool FrameHasAlphaAtIndex(wtf_size_t) const;

  // Whether or not the frame is fully received.
  virtual bool FrameIsReceivedAtIndex(wtf_size_t) const;

  // Returns true if a cached complete decode is available.
  bool FrameIsDecodedAtIndex(wtf_size_t) const;

  // Timestamp for displaying a frame. This method is only used by animated
  // images. Only formats with timestamps (like AVIF) should implement this.
  virtual std::optional<base::TimeDelta> FrameTimestampAtIndex(
      wtf_size_t) const;

  // Duration for displaying a frame. This method is only used by animated
  // images.
  virtual base::TimeDelta FrameDurationAtIndex(wtf_size_t) const;

  // Number of bytes in the decoded frame. Returns 0 if the decoder doesn't
  // have this frame cached (either because it hasn't been decoded, or because
  // it has been cleared).
  virtual wtf_size_t FrameBytesAtIndex(wtf_size_t) const;

  ImageOrientationEnum Orientation() const { return orientation_; }
  gfx::Size DensityCorrectedSize() const { return density_corrected_size_; }

  // Updates orientation, pixel density etc based on the Exif metadata stored in
  // |exif_data|.
  void ApplyExifMetadata(const SkData* exif_data,
                         const gfx::Size& physical_size);

  bool IgnoresColorSpace() const {
    return color_behavior_ == ColorBehavior::kIgnore;
  }
  ColorBehavior GetColorBehavior() const { return color_behavior_; }

  // This returns the color space that will be included in the SkImageInfo of
  // SkImages created from this decoder. This will be nullptr unless the
  // decoder was created with the option ColorSpaceTagged.
  sk_sp<SkColorSpace> ColorSpaceForSkImages();

  // This returns whether or not the image included a not-ignored embedded
  // color profile. This is independent of whether or not that profile's
  // transform has been baked into the pixel values.
  bool HasEmbeddedColorProfile() const { return embedded_color_profile_.get(); }

  void SetEmbeddedColorProfile(std::unique_ptr<ColorProfile> profile);

  // Transformation from embedded color space to target color space.
  ColorProfileTransform* ColorTransform();

  AlphaOption GetAlphaOption() const {
    return premultiply_alpha_ ? kAlphaPremultiplied : kAlphaNotPremultiplied;
  }

  cc::AuxImage GetAuxImage() const { return aux_image_; }

  wtf_size_t GetMaxDecodedBytes() const { return max_decoded_bytes_; }

  // Sets the "decode failure" flag.  For caller convenience (since so
  // many callers want to return false after calling this), returns false
  // to enable easy tailcalling.  Subclasses may override this to also
  // clean up any local data.
  virtual bool SetFailed();

  bool Failed() const { return failed_; }

  // Clears decoded pixel data from all frames except the provided frame. If
  // subsequent frames depend on this frame's required previous frame, then that
  // frame is also kept in cache to prevent re-decoding from the beginning.
  // Callers may pass WTF::kNotFound to clear all frames.
  // Note: If |frame_buffer_cache_| contains only one frame, it won't be
  // cleared. Returns the number of bytes of frame data actually cleared.
  virtual wtf_size_t ClearCacheExceptFrame(wtf_size_t);

  // If the image has a cursor hot-spot, stores it in the argument
  // and returns true. Otherwise returns false.
  virtual bool HotSpot(gfx::Point&) const;

  virtual void SetMemoryAllocator(SkBitmap::Allocator* allocator);

  bool CanDecodeToYUV() const { return allow_decode_to_yuv_; }
  // Should only be called if CanDecodeToYuv() returns true, in which case
  // the subclass of ImageDecoder must override this method.
  virtual void DecodeToYUV();
  void SetImagePlanes(std::unique_ptr<ImagePlanes> image_planes) {
    image_planes_ = std::move(image_planes);
  }
  bool HasDisplayableYUVData() const {
    return image_planes_ && image_planes_->HasCompleteScan();
  }

  // Indicates if the data contains both an animation and still image.
  virtual bool ImageHasBothStillAndAnimatedSubImages() const;

 protected:
  ImageDecoder(AlphaOption alpha_option,
               HighBitDepthDecodingOption high_bit_depth_decoding_option,
               ColorBehavior color_behavior,
               cc::AuxImage aux_image,
               wtf_size_t max_decoded_bytes);

  // Calculates the most recent frame whose image data may be needed in
  // order to decode frame |frame_index|, based on frame disposal methods
  // and |frame_rect_is_opaque|, where |frame_rect_is_opaque| signifies whether
  // the rectangle of frame at |frame_index| is known to be opaque.
  // If no previous frame's data is required, returns WTF::kNotFound.
  //
  // This function requires that the previous frame's
  // |required_previous_frame_index_| member has been set correctly. The
  // easiest way to ensure this is for subclasses to call this method and
  // store the result on the frame via SetRequiredPreviousFrameIndex()
  // as soon as the frame has been created and parsed sufficiently to
  // determine the disposal method; assuming this happens for all frames
  // in order, the required invariant will hold.
  //
  // Image formats which do not use more than one frame do not need to
  // worry about this; see comments on
  // ImageFrame::required_previous_frame_index_.
  wtf_size_t FindRequiredPreviousFrame(wtf_size_t frame_index,
                                       bool frame_rect_is_opaque);

  // This is called by ClearCacheExceptFrame() if that method decides it wants
  // to preserve another frame, to avoid unnecessary redecoding.
  wtf_size_t ClearCacheExceptTwoFrames(wtf_size_t, wtf_size_t);
  virtual void ClearFrameBuffer(wtf_size_t frame_index);

  // Decodes the image sufficiently to determine the image size.
  virtual void DecodeSize() = 0;

  // Decodes the image sufficiently to determine the number of frames and
  // returns that number.
  //
  // If an image format supports images with multiple frames, the decoder must
  // override this method. FrameCount() calls this method and resizes
  // |frame_buffer_cache_| to the return value of this method. Therefore, on
  // failure this method should return |frame_buffer_cache_.size()| (the
  // existing number of frames) instead of 0 to leave |frame_buffer_cache_|
  // unchanged.
  //
  // This method may return an increasing frame count as frames are received and
  // parsed. Alternatively, if the total frame count is available in the image
  // header, this method may return the total frame count without checking how
  // many frames are received.
  virtual wtf_size_t DecodeFrameCount();

  // Called to initialize the frame buffer with the given index, based on the
  // provided and previous frame's characteristics. Returns true on success.
  // Before calling this method, the caller must verify that the frame exists.
  // On failure, the client should call SetFailed. This method does not call
  // SetFailed itself because that might delete the object directly making this
  // call.
  bool InitFrameBuffer(wtf_size_t);

  // Performs any decoder-specific setup of the requested frame after it has
  // been newly created, e.g. setting the frame's duration or disposal method.
  virtual void InitializeNewFrame(wtf_size_t) {}

  // Decodes the requested frame.
  virtual void Decode(wtf_size_t) = 0;

  // This method is only required for animated images. It returns a vector with
  // all frame indices that need to be decoded in order to succesfully decode
  // the provided frame.  The indices are returned in reverse order, so the
  // last frame needs to be decoded first.  Before calling this method, the
  // caller must verify that the frame exists.
  Vector<wtf_size_t> FindFramesToDecode(wtf_size_t) const;

  // This is called by Decode() after decoding a frame in an animated image.
  // Before calling this method, the caller must verify that the frame exists.
  // @return true  if the frame was fully decoded,
  //         false otherwise.
  bool PostDecodeProcessing(wtf_size_t);

  // The GIF and PNG decoders set the default alpha setting of the ImageFrame to
  // true. When the frame rect does not contain any (semi-) transparent pixels,
  // this may need to be changed to false. This depends on whether the required
  // previous frame adds transparency to the image, outside of the frame rect.
  // This methods corrects the alpha setting of the frame buffer to false when
  // the whole frame is opaque.
  //
  // This method should be called by the GIF and PNG decoder when the pixels in
  // the frame rect do *not* contain any transparent pixels. Before calling
  // this method, the caller must verify that the frame exists.
  void CorrectAlphaWhenFrameBufferSawNoAlpha(wtf_size_t);

  scoped_refptr<SegmentReader> data_;  // The encoded data.
  Vector<ImageFrame, 1> frame_buffer_cache_;
  const bool premultiply_alpha_;
  const HighBitDepthDecodingOption high_bit_depth_decoding_option_;
  const ColorBehavior color_behavior_;
  const cc::AuxImage aux_image_;
  ImageOrientationEnum orientation_ = ImageOrientationEnum::kDefault;
  gfx::Size density_corrected_size_;

  // The maximum amount of memory a decoded image should require. Ideally,
  // image decoders should downsample large images to fit under this limit
  // (and then return the downsampled size from DecodedSize()). Ignoring
  // this limit can cause excessive memory use or even crashes on low-
  // memory devices.
  const wtf_size_t max_decoded_bytes_;

  // While decoding, we may learn that there are so many animation frames that
  // we would go beyond our cache budget.
  // If that happens, purge_aggressively_ is set to true. This signals
  // future decodes to purge old frames as it goes.
  void UpdateAggressivePurging(wtf_size_t index);

  // The method is only relevant for multi-frame images.
  //
  // This method indicates whether the provided frame has enough data to decode
  // successive frames that depend on it. It is used by ClearCacheExceptFrame
  // to determine which frame to keep in cache when the indicated frame is not
  // yet sufficiently decoded.
  //
  // The default condition is that the frame status needs to be FramePartial or
  // FrameComplete, since the data of previous frames is copied in
  // InitFrameBuffer() before setting the status to FramePartial. For WebP,
  // however, the status needs to be FrameComplete since the complete buffer is
  // used to do alpha blending in WEBPImageDecoder::ApplyPostProcessing().
  //
  // Before calling this, verify that frame |index| exists by checking that
  // |index| is smaller than |frame_buffer_cache_|.size().
  virtual bool FrameStatusSufficientForSuccessors(wtf_size_t index);

  // Note that |allow_decode_to_yuv_| being true merely means that the
  // ImageDecoder supports decoding to YUV. Other layers higher in the
  // stack (the PaintImageGenerator, ImageFrameGenerator, or cache) may
  // decline to go down the YUV path.
  bool allow_decode_to_yuv_;
  std::unique_ptr<ImagePlanes> image_planes_;

 private:
  // Some code paths compute the size of the image as "width * height * 4 or 8"
  // and return it as a (signed) int.  Avoid overflow.
  inline bool SizeCalculationMayOverflow(unsigned width,
                                         unsigned height,
                                         unsigned decoded_bytes_per_pixel) {
    base::CheckedNumeric<int32_t> total_size = width;
    total_size *= height;
    total_size *= decoded_bytes_per_pixel;
    return !total_size.IsValid();
  }

  bool purge_aggressively_;

  // Update `sk_image_color_space_` and `embedded_to_sk_image_transform_`, if
  // needed.
  void UpdateSkImageColorSpaceAndTransform();

  // This methods gets called at the end of InitFrameBuffer. Subclasses can do
  // format specific initialization, for e.g. alpha settings, here.
  virtual void OnInitFrameBuffer(wtf_size_t) {}

  // Called by InitFrameBuffer to determine if it can take the bitmap of the
  // previous frame. This condition is different for GIF and WEBP.
  virtual bool CanReusePreviousFrameBuffer(wtf_size_t) const;

  gfx::Size size_;
  bool size_available_ = false;
  bool is_all_data_received_ = false;
  bool failed_ = false;

  // The precise color profile of the image.
  std::unique_ptr<ColorProfile> embedded_color_profile_;

  // The color space for the SkImage that will be produced.  If
  // `color_behavior_` is tag, then this is the SkColorSpace representation of
  // `embedded_color_profile_`. If `color_behavior_` is convert to sRGB, then
  // this is sRGB.
  sk_sp<SkColorSpace> sk_image_color_space_;

  // Transforms `embedded_color_profile_` to `sk_image_color_space_`. This
  // is needed if `sk_image_color_space_` is not an exact representation of
  // `embedded_color_profile_`.
  std::unique_ptr<ColorProfileTransform> embedded_to_sk_image_transform_;
};

// static
template <const char type[]>
void ImageDecoder::UpdateBppHistogram(gfx::Size size, size_t image_size_bytes) {
#define DEFINE_BPP_HISTOGRAM(var, suffix)                                    \
  DEFINE_THREAD_SAFE_STATIC_LOCAL(                                           \
      CustomCountHistogram, var,                                             \
      (base::StrCat({"Blink.DecodedImage.", type, "Density.Count.", suffix}) \
           .c_str(),                                                         \
       1, 1000, 100))

  // From 1 pixel to 1 MP, we have one histogram per 0.1 MP.
  // From 2 MP to 13 MP, we have one histogram per 1 MP.
  // Finally, we have one histogram for > 13 MP.
  DEFINE_BPP_HISTOGRAM(density_point_1_mp_histogram, "0.1MP");
  DEFINE_BPP_HISTOGRAM(density_point_2_mp_histogram, "0.2MP");
  DEFINE_BPP_HISTOGRAM(density_point_3_mp_histogram, "0.3MP");
  DEFINE_BPP_HISTOGRAM(density_point_4_mp_histogram, "0.4MP");
  DEFINE_BPP_HISTOGRAM(density_point_5_mp_histogram, "0.5MP");
  DEFINE_BPP_HISTOGRAM(density_point_6_mp_histogram, "0.6MP");
  DEFINE_BPP_HISTOGRAM(density_point_7_mp_histogram, "0.7MP");
  DEFINE_BPP_HISTOGRAM(density_point_8_mp_histogram, "0.8MP");
  DEFINE_BPP_HISTOGRAM(density_point_9_mp_histogram, "0.9MP");
  static std::array<CustomCountHistogram* const, 9> density_histogram_small = {
      &density_point_1_mp_histogram, &density_point_2_mp_histogram,
      &density_point_3_mp_histogram, &density_point_4_mp_histogram,
      &density_point_5_mp_histogram, &density_point_6_mp_histogram,
      &density_point_7_mp_histogram, &density_point_8_mp_histogram,
      &density_point_9_mp_histogram};

  DEFINE_BPP_HISTOGRAM(density_1_mp_histogram, "01MP");
  DEFINE_BPP_HISTOGRAM(density_2_mp_histogram, "02MP");
  DEFINE_BPP_HISTOGRAM(density_3_mp_histogram, "03MP");
  DEFINE_BPP_HISTOGRAM(density_4_mp_histogram, "04MP");
  DEFINE_BPP_HISTOGRAM(density_5_mp_histogram, "05MP");
  DEFINE_BPP_HISTOGRAM(density_6_mp_histogram, "06MP");
  DEFINE_BPP_HISTOGRAM(density_7_mp_histogram, "07MP");
  DEFINE_BPP_HISTOGRAM(density_8_mp_histogram, "08MP");
  DEFINE_BPP_HISTOGRAM(density_9_mp_histogram, "09MP");
  DEFINE_BPP_HISTOGRAM(density_10_mp_histogram, "10MP");
  DEFINE_BPP_HISTOGRAM(density_11_mp_histogram, "11MP");
  DEFINE_BPP_HISTOGRAM(density_12_mp_histogram, "12MP");
  DEFINE_BPP_HISTOGRAM(density_13_mp_histogram, "13MP");
  static std::array<CustomCountHistogram* const, 13> density_histogram_big = {
      &density_1_mp_histogram,  &density_2_mp_histogram,
      &density_3_mp_histogram,  &density_4_mp_histogram,
      &density_5_mp_histogram,  &density_6_mp_histogram,
      &density_7_mp_histogram,  &density_8_mp_histogram,
      &density_9_mp_histogram,  &density_10_mp_histogram,
      &density_11_mp_histogram, &density_12_mp_histogram,
      &density_13_mp_histogram};

  DEFINE_BPP_HISTOGRAM(density_14plus_mp_histogram, "14+MP");

#undef DEFINE_BPP_HISTOGRAM

  uint64_t image_area = size.Area64();
  CHECK_NE(image_area, 0u);
  // The calculation of density_centi_bpp cannot overflow. SetSize() ensures
  // that image_area won't overflow int32_t. And image_size_bytes must be much
  // smaller than UINT64_MAX / (100 * 8), which is roughly 2^54, or 16 peta
  // bytes.
  base::CheckedNumeric<uint64_t> checked_image_size_bytes = image_size_bytes;
  base::CheckedNumeric<uint64_t> density_centi_bpp =
      (checked_image_size_bytes * 100 * 8 + image_area / 2) / image_area;

  CustomCountHistogram* density_histogram;
  if (image_area <= 900000) {
    // One histogram per 0.1 MP.
    int n = (static_cast<int>(image_area) + (100000 - 1)) / 100000;
    density_histogram = density_histogram_small[n - 1];
  } else if (image_area <= 13000000) {
    // One histogram per 1 MP.
    int n = (static_cast<int>(image_area) + (1000000 - 1)) / 1000000;
    density_histogram = density_histogram_big[n - 1];
  } else {
    density_histogram = &density_14plus_mp_histogram;
  }

  density_histogram->Count(base::saturated_cast<base::Histogram::Sample32>(
      density_centi_bpp.ValueOrDie()));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_H_
