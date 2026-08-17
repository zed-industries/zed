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

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCRtpCodecCapability) : NSObject

/** The preferred RTP payload type. */
@property(nonatomic, readonly, nullable) NSNumber *preferredPayloadType;

/**
 * The codec MIME subtype. Valid types are listed in:
 * http://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-2
 *
 * Several supported types are represented by the constants above.
 */
@property(nonatomic, readonly) NSString *name;

/**
 * The media type of this codec. Equivalent to MIME top-level type.
 *
 * Valid values are kRTCMediaStreamTrackKindAudio and
 * kRTCMediaStreamTrackKindVideo.
 */
@property(nonatomic, readonly) NSString *kind;

/** The codec clock rate expressed in Hertz. */
@property(nonatomic, readonly, nullable) NSNumber *clockRate;

/**
 * The number of audio channels (mono=1, stereo=2).
 * Set to null for video codecs.
 **/
@property(nonatomic, readonly, nullable) NSNumber *numChannels;

/** The "format specific parameters" field from the "a=fmtp" line in the SDP */
@property(nonatomic, readonly) NSDictionary<NSString *, NSString *> *parameters;

/** The MIME type of the codec. */
@property(nonatomic, readonly) NSString *mimeType;

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
