/*
 *  Copyright 2014 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "ARDSignalingMessage.h"

typedef NS_ENUM(NSInteger, ARDSignalingChannelState) {
  // State when disconnected.
  kARDSignalingChannelStateClosed,
  // State when connection is established but not ready for use.
  kARDSignalingChannelStateOpen,
  // State when connection is established and registered.
  kARDSignalingChannelStateRegistered,
  // State when connection encounters a fatal error.
  kARDSignalingChannelStateError
};

@protocol ARDSignalingChannel;
@protocol ARDSignalingChannelDelegate <NSObject>

- (void)channel:(id<ARDSignalingChannel>)channel
    didChangeState:(ARDSignalingChannelState)state;

- (void)channel:(id<ARDSignalingChannel>)channel
    didReceiveMessage:(ARDSignalingMessage *)message;

@end

@protocol ARDSignalingChannel <NSObject>

@property(nonatomic, readonly) NSString *roomId;
@property(nonatomic, readonly) NSString *clientId;
@property(nonatomic, readonly) ARDSignalingChannelState state;
@property(nonatomic, weak) id<ARDSignalingChannelDelegate> delegate;

// Registers the channel for the given room and client id.
- (void)registerForRoomId:(NSString *)roomId clientId:(NSString *)clientId;

// Sends signaling message over the channel.
- (void)sendMessage:(ARDSignalingMessage *)message;

@end
