/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "RTCVideoCapturer.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Error passing block.
 */
typedef void (^RTCFileVideoCapturerErrorBlock)(NSError *error);

/**
 * Captures buffers from bundled video file.
 *
 * See @c RTCVideoCapturer for more info on capturers.
 */
RTC_OBJC_EXPORT

NS_CLASS_AVAILABLE_IOS(10)
@interface RTC_OBJC_TYPE (RTCFileVideoCapturer) : RTC_OBJC_TYPE(RTCVideoCapturer)

/**
 * Starts asynchronous capture of frames from video file.
 *
 * Capturing is not started if error occurs. Underlying error will be
 * relayed in the errorBlock if one is provided.
 * Successfully captured video frames will be passed to the delegate.
 *
 * @param nameOfFile The name of the bundled video file to be read.
 * @errorBlock block to be executed upon error.
 */
- (void)startCapturingFromFileNamed:(NSString *)nameOfFile
                            onError:(__nullable RTCFileVideoCapturerErrorBlock)errorBlock;

/**
 * Immediately stops capture.
 */
- (void)stopCapture;
@end

NS_ASSUME_NONNULL_END
