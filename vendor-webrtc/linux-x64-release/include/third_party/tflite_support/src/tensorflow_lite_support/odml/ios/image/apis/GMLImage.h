// Copyright 2021 Google LLC. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#import <CoreMedia/CoreMedia.h>
#import <CoreVideo/CoreVideo.h>
#import <UIKit/UIKit.h>

NS_ASSUME_NONNULL_BEGIN

/** Types of image sources. */
typedef NSInteger GMLImageSourceType NS_TYPED_ENUM NS_SWIFT_NAME(MLImageSourceType);
/** Image source is a `UIImage`. */
static const GMLImageSourceType GMLImageSourceTypeImage = 0;
/** Image source is a `CVPixelBuffer`. */
static const GMLImageSourceType GMLImageSourceTypePixelBuffer = 1;
/** Image source is a `CMSampleBuffer`. */
static const GMLImageSourceType GMLImageSourceTypeSampleBuffer = 2;

/** An image used in on-device machine learning. */
NS_SWIFT_NAME(MLImage)
@interface GMLImage : NSObject

/** Width of the image in pixels. */
@property(nonatomic, readonly) CGFloat width;

/** Height of the image in pixels. */
@property(nonatomic, readonly) CGFloat height;

/**
 * The display orientation of the image. If `imageSourceType` is `.image`, the default value is
 * `image.imageOrientation`; otherwise the default value is `.up`.
 */
@property(nonatomic) UIImageOrientation orientation;

/** The type of the image source. */
@property(nonatomic, readonly) GMLImageSourceType imageSourceType;

/** The source image. `nil` if `imageSourceType` is not `.image`. */
@property(nonatomic, readonly, nullable) UIImage *image;

/** The source pixel buffer. `nil` if `imageSourceType` is not `.pixelBuffer`. */
@property(nonatomic, readonly, nullable) CVPixelBufferRef pixelBuffer;

/** The source sample buffer. `nil` if `imageSourceType` is not `.sampleBuffer`. */
@property(nonatomic, readonly, nullable) CMSampleBufferRef sampleBuffer;

/**
 * Initializes an `MLImage` object with the given image.
 *
 * @param image The image to use as the source. Its `CGImage` property must not be `NULL`.
 * @return A new `MLImage` instance with the given image as the source. `nil` if the given `image`
 *     is `nil` or invalid.
 */
- (nullable instancetype)initWithImage:(UIImage *)image NS_DESIGNATED_INITIALIZER;

/**
 * Initializes an `MLImage` object with the given pixel buffer.
 *
 * @param pixelBuffer The pixel buffer to use as the source. It will be retained by the new
 *     `MLImage` instance for the duration of its lifecycle.
 * @return A new `MLImage` instance with the given pixel buffer as the source. `nil` if the given
 *     pixel buffer is `nil` or invalid.
 */
- (nullable instancetype)initWithPixelBuffer:(CVPixelBufferRef)pixelBuffer
    NS_DESIGNATED_INITIALIZER;

/**
 * Initializes an `MLImage` object with the given sample buffer.
 *
 * @param sampleBuffer The sample buffer to use as the source. It will be retained by the new
 *     `MLImage` instance for the duration of its lifecycle. The sample buffer must be based on a
 *     pixel buffer (not compressed data). In practice, it should be the video output of the camera
 *     on an iOS device, not other arbitrary types of `CMSampleBuffer`s.
 * @return A new `MLImage` instance with the given sample buffer as the source. `nil` if the given
 *     sample buffer is `nil` or invalid.
 */
- (nullable instancetype)initWithSampleBuffer:(CMSampleBufferRef)sampleBuffer
    NS_DESIGNATED_INITIALIZER;

/** Unavailable. */
- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
