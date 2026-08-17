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

#import "RTCDtmfSender.h"
#import "RTCMediaStreamTrack.h"
#import "RTCRtpParameters.h"
#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCRtpSender)<NSObject>

    /** A unique identifier for this sender. */
    @property(nonatomic, readonly) NSString *senderId;

/** The currently active RTCRtpParameters, as defined in
 *  https://www.w3.org/TR/webrtc/#idl-def-RTCRtpParameters.
 */
@property(nonatomic, copy) RTC_OBJC_TYPE(RTCRtpParameters) * parameters;

/** The RTCMediaStreamTrack associated with the sender.
 *  Note: reading this property returns a new instance of
 *  RTCMediaStreamTrack. Use isEqual: instead of == to compare
 *  RTCMediaStreamTrack instances.
 */
@property(nonatomic, copy, nullable) RTC_OBJC_TYPE(RTCMediaStreamTrack) * track;

/** IDs of streams associated with the RTP sender */
@property(nonatomic, copy) NSArray<NSString *> *streamIds;

/** The RTCDtmfSender accociated with the RTP sender. */
@property(nonatomic, readonly, nullable) id<RTC_OBJC_TYPE(RTCDtmfSender)>
    dtmfSender;

@end

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCRtpSender) : NSObject <RTC_OBJC_TYPE(RTCRtpSender)>

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
