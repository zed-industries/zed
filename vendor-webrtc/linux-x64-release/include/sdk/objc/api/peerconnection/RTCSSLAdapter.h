/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

/**
 * Initialize and clean up the SSL library. Failure is fatal. These call the
 * corresponding functions in webrtc/rtc_base/ssladapter.h.
 */
RTC_EXTERN BOOL RTC_OBJC_TYPE(RTCInitializeSSL)(void);
RTC_EXTERN BOOL RTC_OBJC_TYPE(RTCCleanupSSL)(void);
