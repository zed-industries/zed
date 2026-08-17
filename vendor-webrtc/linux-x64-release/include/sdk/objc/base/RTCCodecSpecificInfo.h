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

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

/** Implement this protocol to pass codec specific info from the encoder.
 *  Corresponds to webrtc::CodecSpecificInfo.
 */
RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCCodecSpecificInfo)<NSObject> @end

NS_ASSUME_NONNULL_END
