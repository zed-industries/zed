/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <AvailabilityMacros.h>
#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCDataChannelConfiguration) : NSObject

/** Set to YES if ordered delivery is required. */
@property(nonatomic, assign) BOOL isOrdered;

/** Deprecated. Use maxPacketLifeTime. */
@property(nonatomic, assign) NSInteger maxRetransmitTimeMs DEPRECATED_ATTRIBUTE;

/**
 * Max period in milliseconds in which retransmissions will be sent. After this
 * time, no more retransmissions will be sent. -1 if unset.
 */
@property(nonatomic, assign) int maxPacketLifeTime;

/** The max number of retransmissions. -1 if unset. */
@property(nonatomic, assign) int maxRetransmits;

/** Set to YES if the channel has been externally negotiated and we do not send
 * an in-band signalling in the form of an "open" message.
 */
@property(nonatomic, assign) BOOL isNegotiated;

/** Deprecated. Use channelId. */
@property(nonatomic, assign) int streamId DEPRECATED_ATTRIBUTE;

/** The id of the data channel. */
@property(nonatomic, assign) int channelId;

/** Set by the application and opaque to the WebRTC implementation. */
@property(nonatomic) NSString* protocol;

@end

NS_ASSUME_NONNULL_END
