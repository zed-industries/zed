/*
 *  Copyright 2015 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>

#import "sdk/objc/base/RTCMacros.h"

@class AVCaptureSession;

/** RTCCameraPreviewView is a view that renders local video from an
 *  AVCaptureSession.
 */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCCameraPreviewView) : UIView

/** The capture session being rendered in the view. Capture session
 *  is assigned to AVCaptureVideoPreviewLayer async in the same
 *  queue that the AVCaptureSession is started/stopped.
 */
@property(nonatomic, strong) AVCaptureSession* captureSession;

@end
