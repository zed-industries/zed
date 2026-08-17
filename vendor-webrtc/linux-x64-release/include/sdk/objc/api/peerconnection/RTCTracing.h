/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

RTC_EXTERN void RTC_OBJC_TYPE(RTCSetupInternalTracer)(void);
/** Starts capture to specified file. Must be a valid writable path.
 *  Returns YES if capture starts.
 */
RTC_EXTERN BOOL RTC_OBJC_TYPE(RTCStartInternalCapture)(NSString* filePath);
RTC_EXTERN void RTC_OBJC_TYPE(RTCStopInternalCapture)(void);
RTC_EXTERN void RTC_OBJC_TYPE(RTCShutdownInternalTracer)(void);
