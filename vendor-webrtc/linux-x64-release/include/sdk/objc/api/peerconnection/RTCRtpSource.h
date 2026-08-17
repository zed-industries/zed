/*
 *  Copyright 2024 The WebRTC project authors. All Rights Reserved.
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

/** Represents the source type of received media. */
typedef NS_ENUM(NSInteger, RTCRtpSourceType) {
  RTCRtpSourceTypeSSRC,
  RTCRtpSourceTypeCSRC,
};

@class RTC_OBJC_TYPE(RTCRtpSource);

RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCRtpSource)<NSObject>

    /**
    A positive integer value specifying the CSRC identifier of the contributing
    source or SSRC identifier of the synchronization source. This uniquely
    identifies the source of the particular stream RTP packets. */
    @property(nonatomic, readonly) uint32_t sourceId;

@property(nonatomic, readonly) RTCRtpSourceType sourceType;

/**
A floating-point value between 0.0 and 1.0 specifying the audio level contained
in the last RTP packet played from the contributing source.
*/
@property(nonatomic, readonly, nullable) NSNumber *audioLevel;

/**
A timestamp indicating the most recent time at which a frame originating from
this source was delivered to the receiver's track
*/
@property(nonatomic, readonly) CFTimeInterval timestampUs;

/**
The RTP timestamp of the media. This source-generated timestamp indicates the
time at which the media in this packet, scheduled for play out at the time
indicated by timestamp, was initially sampled or generated. It may be useful for
sequencing and synchronization purposes.
*/
@property(nonatomic, readonly) uint32_t rtpTimestamp;

@end

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCRtpSource) : NSObject <RTC_OBJC_TYPE(RTCRtpSource)>

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
