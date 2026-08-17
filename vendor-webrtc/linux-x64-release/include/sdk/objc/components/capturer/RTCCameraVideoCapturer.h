/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>

#import "RTCVideoCapturer.h"
#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

// Camera capture that implements RTCVideoCapturer. Delivers frames to a
// RTCVideoCapturerDelegate (usually RTCVideoSource).
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCCameraVideoCapturer) : RTC_OBJC_TYPE(RTCVideoCapturer)

// Capture session that is used for capturing. Valid from initialization to dealloc.
@property(readonly, nonatomic) AVCaptureSession *captureSession;

// Returns list of available capture devices that support video capture.
+ (NSArray<AVCaptureDevice *> *)captureDevices;

// Returns list of formats that are supported by this class for this device.
+ (NSArray<AVCaptureDeviceFormat *> *)supportedFormatsForDevice:
    (AVCaptureDevice *)device;

#if !TARGET_OS_VISION
+ (CGFloat)defaultZoomFactorForDeviceType:(AVCaptureDeviceType)deviceType;
#endif

- (instancetype)initWithDelegate:
    (nullable __weak id<RTC_OBJC_TYPE(RTCVideoCapturerDelegate)>)delegate;

- (instancetype)initWithDelegate:
                    (nullable __weak id<RTC_OBJC_TYPE(RTCVideoCapturerDelegate)>)delegate
                  captureSession:(AVCaptureSession *)captureSession;

// Returns the most efficient supported output pixel format for this capturer.
- (FourCharCode)preferredOutputPixelFormat;

// Starts the capture session asynchronously and notifies callback on
// completion. The device will capture video in the format given in the `format`
// parameter. If the pixel format in `format` is supported by the WebRTC
// pipeline, the same pixel format will be used for the output. Otherwise, the
// format returned by `preferredOutputPixelFormat` will be used.
- (void)startCaptureWithDevice:(AVCaptureDevice *)device
                        format:(AVCaptureDeviceFormat *)format
                           fps:(NSInteger)fps
             completionHandler:
                 (nullable void (^)(NSError *_Nullable))completionHandler;
// Stops the capture session asynchronously and notifies callback on completion.
- (void)stopCaptureWithCompletionHandler:
    (nullable void (^)(void))completionHandler;

// Starts the capture session asynchronously.
- (void)startCaptureWithDevice:(AVCaptureDevice *)device
                        format:(AVCaptureDeviceFormat *)format
                           fps:(NSInteger)fps;
// Stops the capture session asynchronously.
- (void)stopCapture;

@end

NS_ASSUME_NONNULL_END
