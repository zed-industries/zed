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
#if TARGET_OS_IPHONE
#import <UIKit/UIKit.h>
#else
#import <AppKit/AppKit.h>
#endif

#import "base/RTCVideoFrame.h"

NS_ASSUME_NONNULL_BEGIN
/**
 * Protocol defining ability to render RTCVideoFrame in Metal enabled views.
 */
@protocol RTC_OBJC_TYPE(RTCMTLRenderer) <NSObject>

/**
 * Method to be implemented to perform actual rendering of the provided frame.
 *
 * @param frame The frame to be rendered.
 */
- (void)drawFrame:(RTC_OBJC_TYPE(RTCVideoFrame) *)frame;

/**
 * Sets the provided view as rendering destination if possible.
 *
 * If not possible method returns NO and callers of the method are responisble
 * for performing cleanups.
 */

#if TARGET_OS_IOS || TARGET_OS_TV
- (BOOL)addRenderingDestination:(__kindof UIView *)view;
#else
- (BOOL)addRenderingDestination:(__kindof NSView *)view;
#endif

@end

/**
 * Implementation of RTCMTLRenderer protocol.
 */
NS_AVAILABLE(10_11, 9_0)
@interface RTC_OBJC_TYPE(RTCMTLRenderer) : NSObject <RTC_OBJC_TYPE(RTCMTLRenderer)>

/** @abstract   A wrapped RTCVideoRotation, or nil.
    @discussion When not nil, the rotation of the actual frame is ignored when
   rendering.
 */
@property(atomic, nullable) NSValue *rotationOverride;

@end

NS_ASSUME_NONNULL_END
