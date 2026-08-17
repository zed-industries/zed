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

NS_ASSUME_NONNULL_BEGIN

RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCRtxCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCRedCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCUlpfecCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCFlexfecCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCOpusCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCIsacCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCL16CodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCG722CodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCIlbcCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCPcmuCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCPcmaCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCDtmfCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCComfortNoiseCodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCVp8CodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCVp9CodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCH264CodecName);
RTC_EXTERN const NSString *const RTC_CONSTANT_TYPE(RTCAv1CodecName);

/** Defined in https://www.w3.org/TR/webrtc/#idl-def-rtcrtpcodecparameters */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCRtpCodecParameters) : NSObject

/** The RTP payload type. */
@property(nonatomic, assign) int payloadType;

/**
 * The codec MIME subtype. Valid types are listed in:
 * http://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-2
 *
 * Several supported types are represented by the constants above.
 */
@property(nonatomic, readonly, nonnull) NSString *name;

/**
 * The media type of this codec. Equivalent to MIME top-level type.
 *
 * Valid values are kRTCMediaStreamTrackKindAudio and
 * kRTCMediaStreamTrackKindVideo.
 */
@property(nonatomic, readonly, nonnull) NSString *kind;

/** The codec clock rate expressed in Hertz. */
@property(nonatomic, readonly, nullable) NSNumber *clockRate;

/**
 * The number of channels (mono=1, stereo=2).
 * Set to null for video codecs.
 **/
@property(nonatomic, readonly, nullable) NSNumber *numChannels;

/** The "format specific parameters" field from the "a=fmtp" line in the SDP */
@property(nonatomic, readonly, nonnull) NSDictionary *parameters;

- (instancetype)init;

@end

NS_ASSUME_NONNULL_END
