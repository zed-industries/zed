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

#import "sdk/objc/base/RTCMacros.h"

@class RTC_OBJC_TYPE(RTCFileVideoCapturer);

/**
 * Controls a file capturer.
 */
NS_CLASS_AVAILABLE_IOS(10)
@interface ARDFileCaptureController : NSObject

/**
 * Creates instance of the controller.
 *
 * @param capturer The capturer to be controlled.
 */
- (instancetype)initWithCapturer:
    (RTC_OBJC_TYPE(RTCFileVideoCapturer) *)capturer;

/**
 * Starts the file capturer.
 *
 * Possible errors produced by the capturer will be logged.
 */
- (void)startCapture;

/**
 * Immediately stops capturer.
 */
- (void)stopCapture;

@end
