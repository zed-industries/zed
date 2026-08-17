// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_CONTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_CONTENT_H_

#include <cstddef>

#include "base/auto_reset.h"
#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource_observer.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "third_party/blink/renderer/platform/graphics/image_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_counted_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"
#include "third_party/blink/renderer/platform/loader/fetch/media_timing.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_status.h"
#include "ui/gfx/geometry/size.h"

namespace base {
class TimeTicks;
}

namespace blink {

class FetchParameters;
class ImageResourceInfo;
class KURL;
class ResourceError;
class ResourceFetcher;
class ResourceResponse;
class UseCounter;
enum RespectImageOrientationEnum : uint8_t;
struct ResourcePriority;

// ImageResourceContent is a container that holds fetch result of
// an ImageResource in a decoded form.
// Classes that use the fetched images
// should hold onto this class and/or inherit ImageResourceObserver,
// instead of holding onto ImageResource or inheriting ResourceClient.
// https://docs.google.com/document/d/1O-fB83mrE0B_V8gzXNqHgmRLCvstTB4MMi3RnVLr8bE/edit?usp=sharing
// TODO(hiroshige): Make ImageResourceContent ResourceClient and remove the
// word 'observer' from ImageResource.
class CORE_EXPORT ImageResourceContent final
    : public GarbageCollected<ImageResourceContent>,
      public ImageObserver,
      public MediaTiming {
 public:
  // Used for loading.
  // Returned content will be associated immediately later with ImageResource.
  static ImageResourceContent* CreateNotStarted() {
    return MakeGarbageCollected<ImageResourceContent>(nullptr);
  }

  // Creates ImageResourceContent from an already loaded image.
  static ImageResourceContent* CreateLoaded(scoped_refptr<blink::Image>);

  static ImageResourceContent* Fetch(FetchParameters&, ResourceFetcher*);

  explicit ImageResourceContent(scoped_refptr<blink::Image> = nullptr);

  // Returns the NullImage() if the image is not available yet.
  blink::Image* GetImage() const;
  bool HasImage() const { return image_.get(); }

  // Returns true if enough of the image has been decoded to allow its size to
  // be determined. If this returns true, so will HasImage().
  bool IsSizeAvailable() const {
    return size_available_ != Image::kSizeUnavailable;
  }

  // Returns the intrinsic width and height of the image, or 0x0 if no image
  // exists. IsSizeAvailable() can be used to determine if the value returned is
  // reliable. If the image is a BitmapImage, then this corresponds to the
  // physical pixel dimensions of the image. If the image is an SVGImage, this
  // does not quite return the intrinsic width/height, but rather a concrete
  // object size resolved using a default object size of 300x150.
  // TODO(fs): Make SVGImages return proper intrinsic width/height.
  gfx::Size IntrinsicSize(
      RespectImageOrientationEnum should_respect_image_orientation) const;

  void AddObserver(ImageResourceObserver*);
  void RemoveObserver(ImageResourceObserver*);
  void DidRemoveObserver();

  // The device pixel ratio we got from the server for this image, or 1.0.
  float DevicePixelRatioHeaderValue() const;
  bool HasDevicePixelRatioHeaderValue() const;

  // Correct the image orientation preference for potentially cross-origin
  // content.
  RespectImageOrientationEnum ForceOrientationIfNecessary(
      RespectImageOrientationEnum default_orientation) const;

  void Trace(Visitor*) const override;

  // Content status and deriving predicates.
  // https://docs.google.com/document/d/1O-fB83mrE0B_V8gzXNqHgmRLCvstTB4MMi3RnVLr8bE/edit#heading=h.6cyqmir0f30h
  // Normal transitions:
  //   kNotStarted -> kPending -> kCached|kLoadError|kDecodeError.
  // Additional transitions in multipart images:
  //   kCached -> kLoadError|kDecodeError.
  // Transitions due to revalidation:
  //   kCached -> kPending.
  // Transitions due to reload:
  //   kCached|kLoadError|kDecodeError -> kPending.
  //
  // ImageResourceContent::GetContentStatus() can be different from
  // ImageResource::GetStatus(). Use ImageResourceContent::GetContentStatus().
  ResourceStatus GetContentStatus() const;
  void SetIsSufficientContentLoadedForPaint() override;
  bool IsSufficientContentLoadedForPaint() const override;
  bool IsLoaded() const;
  bool IsLoading() const;
  bool ErrorOccurred() const;
  bool LoadFailedOrCanceled() const;
  void SetIsBroken();
  bool IsBroken() const override;
  bool IsAnimatedImage() const override;
  bool IsPaintedFirstFrame() const override;
  bool TimingAllowPassed() const override;
  base::TimeTicks GetFirstVideoFrameTime() const override {
    // This returns a null time, which is currently used to signal that this is
    // an animated image, rather than a video, and we should use the
    // ImagePaintTimingDetector to set the first frame time in the ImageRecord
    // instead.
    // TODO(iclelland): Find a better way to set this from IPTD and use it, to
    // use this for images as well as videos.
    return base::TimeTicks();
  }

  // Redirecting methods to Resource.
  const KURL& Url() const override;
  bool IsDataUrl() const override;
  base::TimeTicks LoadResponseEnd() const;
  base::TimeTicks DiscoveryTime() const override;
  base::TimeTicks LoadStart() const override;
  base::TimeTicks LoadEnd() const override;
  AtomicString MediaType() const override;
  bool IsAccessAllowed() const;
  const ResourceResponse& GetResponse() const;
  std::optional<ResourceError> GetResourceError() const;
  // DEPRECATED: ImageResourceContents consumers shouldn't need to worry about
  // whether the underlying Resource is being revalidated.
  bool IsCacheValidator() const;

  // For FrameSerializer.
  bool HasCacheControlNoStoreHeader() const;

  void EmulateLoadStartedForInspector(ResourceFetcher*,
                                      const AtomicString& initiator_name);

  // The following public methods should be called from ImageResource only.

  // UpdateImage() is the single control point of image content modification
  // from ImageResource that all image updates should call.
  // We clear and/or update images in this single method
  // (controlled by UpdateImageOption) rather than providing separate methods,
  // in order to centralize state changes and
  // not to expose the state in between to ImageResource.
  enum UpdateImageOption {
    // Updates the image (including placeholder and decode error handling
    // and notifying observers) if needed.
    kUpdateImage,

    // Clears the image and then updates the image if needed.
    kClearAndUpdateImage,

    // Clears the image and always notifies observers (without updating).
    kClearImageAndNotifyObservers,
  };
  enum class UpdateImageResult {
    kNoDecodeError,

    // Decode error occurred. Observers are not notified.
    // Only occurs when UpdateImage or ClearAndUpdateImage is specified.
    kShouldDecodeError,
  };
  [[nodiscard]] UpdateImageResult UpdateImage(scoped_refptr<SharedBuffer>,
                                              ResourceStatus,
                                              UpdateImageOption,
                                              bool all_data_received,
                                              bool is_multipart);

  void NotifyStartLoad();
  void DestroyDecodedData();
  void DoResetAnimation();

  void SetImageResourceInfo(ImageResourceInfo*);

  void UpdateResourceInfoFromObservers();
  gfx::Size MaxSize() const { return cached_info_.max_size_; }
  InterpolationQuality MaxInterpolationQuality() const {
    return cached_info_.max_interpolation_quality_;
  }

  // Returns priority information to be used for setting the Resource's
  // priority. This is NOT the current Resource's priority.
  std::pair<ResourcePriority, ResourcePriority> PriorityFromObservers() const;
  // Returns the current Resource's priority used by MediaTiming.
  std::optional<WebURLRequest::Priority> RequestPriority() const override;
  scoped_refptr<const SharedBuffer> ResourceBuffer() const;
  bool ShouldUpdateImageImmediately() const;
  bool HasObservers() const {
    return !observers_.empty() || !finished_observers_.empty();
  }
  bool CanBeSpeculativelyDecoded() const;
  bool HasNonDegenerateSizeForDecode() const {
    // If an observer has 0x0 size, we will not consider it for speculative
    // decode.
    return !cached_info_.max_size_.IsZero();
  }
  ImageDecoder::CompressionFormat GetCompressionFormat() const;

  // Returns the number of bytes of image data which should be used for entropy
  // calculations. Ideally this should exclude metadata from within the image
  // file, but currently just returns the complete file size.
  // TODO(iclelland): Eventually switch this, and related calculations, to bits
  // rather than bytes.
  uint64_t ContentSizeForEntropy() const override;

  void LoadDeferredImage(ResourceFetcher* fetcher);

  // Returns whether the resource request has been tagged as an ad.
  bool IsAdResource() const;

  // Records the decoded image type in a UseCounter if the image is a
  // BitmapImage. |use_counter| may be a null pointer.
  void RecordDecodedImageType(UseCounter* use_counter);

 private:
  using CanDeferInvalidation = ImageResourceObserver::CanDeferInvalidation;

  // ImageObserver
  void DecodedSizeChangedTo(const blink::Image*, size_t new_size) override;
  bool ShouldPauseAnimation(const blink::Image*) override;
  void Changed(const blink::Image*) override;
  void AsyncLoadCompleted(const blink::Image*) override;

  scoped_refptr<Image> CreateImage(bool is_multipart);
  void ClearImage();

  enum NotifyFinishOption { kShouldNotifyFinish, kDoNotNotifyFinish };

  void NotifyObservers(NotifyFinishOption, CanDeferInvalidation);
  void HandleObserverFinished(ImageResourceObserver*);
  void UpdateToLoadedContentStatus(ResourceStatus);
  void UpdateImageAnimationPolicy();

  class ProhibitAddRemoveObserverInScope : public base::AutoReset<bool> {
   public:
    ProhibitAddRemoveObserverInScope(const ImageResourceContent* content)
        : AutoReset(&content->is_add_remove_observer_prohibited_, true) {}
  };

  Member<ImageResourceInfo> info_;

  float device_pixel_ratio_header_value_ = 1.0;

  scoped_refptr<blink::Image> image_;

  base::TimeTicks discovery_time_;

  HeapHashCountedSet<WeakMember<ImageResourceObserver>> observers_;
  HeapHashCountedSet<WeakMember<ImageResourceObserver>> finished_observers_;

  // This is updated during ResourceFetcher::UpdateResourceInfoFromObservers
  // when layout is clean and cached for use when layout may not be clean.
  struct {
    ResourcePriority priority_;
    ResourcePriority priority_excluding_image_loader_;
    gfx::Size max_size_;
    InterpolationQuality max_interpolation_quality_ = kInterpolationNone;
  } cached_info_;

  // Keep one-byte members together to avoid wasting space on padding.

  ResourceStatus content_status_ = ResourceStatus::kNotStarted;

  mutable bool is_add_remove_observer_prohibited_ = false;

  Image::SizeAvailability size_available_ = Image::kSizeUnavailable;

  bool has_device_pixel_ratio_header_value_ = false;

  bool is_broken_ = false;

#if DCHECK_IS_ON()
  bool is_update_image_being_called_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_CONTENT_H_
