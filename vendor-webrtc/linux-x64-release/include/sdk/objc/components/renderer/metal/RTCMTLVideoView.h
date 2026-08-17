/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#if TARGET_OS_OSX
#import <AppKit/AppKit.h>
#endif

#import "RTCMacros.h"
#import "RTCVideoFrame.h"
#import "RTCVideoRenderer.h"
#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * RTCMTLVideoView is thin wrapper around MTKView.
 *
 * It has id<RTCVideoRenderer> property that renders video frames in the view's
 * bounds using Metal.
 */
#if TARGET_OS_IPHONE
NS_CLASS_AVAILABLE_IOS(9)
#elif TARGET_OS_OSX
NS_AVAILABLE_MAC(10.11)
#endif

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCMTLVideoView) :

#if TARGET_OS_IPHONE
  UIView<RTC_OBJC_TYPE(RTCVideoRenderer)>
#elif TARGET_OS_OSX
  NSView<RTC_OBJC_TYPE(RTCVideoRenderer)>
#endif

@property(nonatomic, weak) id<RTC_OBJC_TYPE(RTCVideoViewDelegate)> delegate;

#if TARGET_OS_IPHONE
@property(nonatomic) UIViewContentMode videoContentMode;
#endif

/** @abstract Enables/disables rendering.
 */
@property(nonatomic, getter=isEnabled) BOOL enabled;

/** @abstract Wrapped RTCVideoRotation, or nil.
 */
@property(nonatomic, nullable) NSValue* rotationOverride;

+ (BOOL)isMetalAvailable;

@end

NS_ASSUME_NONNULL_END
