/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
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

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCRtcpParameters) : NSObject

/** The Canonical Name used by RTCP. */
@property(nonatomic, readonly, copy) NSString *cname;

/** Whether reduced size RTCP is configured or compound RTCP. */
@property(nonatomic, assign) BOOL isReducedSize;

- (instancetype)init;

@end

NS_ASSUME_NONNULL_END
