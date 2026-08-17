/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
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
@interface RTC_OBJC_TYPE (RTCIceCandidateErrorEvent) : NSObject

/** The local IP address used to communicate with the STUN or TURN server. */
@property(nonatomic, readonly) NSString *address;

/** The port used to communicate with the STUN or TURN server. */
@property(nonatomic, readonly) int port;

/** The STUN or TURN URL that identifies the STUN or TURN server for which the
 * failure occurred. */
@property(nonatomic, readonly) NSString *url;

/** The numeric STUN error code returned by the STUN or TURN server. If no host
 * candidate can reach the server, errorCode will be set to the value 701 which
 * is outside the STUN error code range. This error is only fired once per
 * server URL while in the RTCIceGatheringState of "gathering". */
@property(nonatomic, readonly) int errorCode;

/** The STUN reason text returned by the STUN or TURN server. If the server
 * could not be reached, errorText will be set to an implementation-specific
 * value providing details about the error. */
@property(nonatomic, readonly) NSString *errorText;

- (instancetype)init NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
