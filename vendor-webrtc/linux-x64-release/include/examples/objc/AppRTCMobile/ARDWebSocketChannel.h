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

#import "ARDSignalingChannel.h"

// Wraps a WebSocket connection to the AppRTC WebSocket server.
@interface ARDWebSocketChannel : NSObject <ARDSignalingChannel>

- (instancetype)initWithURL:(NSURL *)url
                    restURL:(NSURL *)restURL
                   delegate:(id<ARDSignalingChannelDelegate>)delegate;

// Registers with the WebSocket server for the given room and client id once
// the web socket connection is open.
- (void)registerForRoomId:(NSString *)roomId clientId:(NSString *)clientId;

// Sends message over the WebSocket connection if registered, otherwise POSTs to
// the web socket server instead.
- (void)sendMessage:(ARDSignalingMessage *)message;

@end

// Loopback mode is used to cause the client to connect to itself for testing.
// A second web socket connection is established simulating the other client.
// Any messages received are sent back to the WebSocket server after modifying
// them as appropriate.
@interface ARDLoopbackWebSocketChannel : ARDWebSocketChannel

- (instancetype)initWithURL:(NSURL *)url restURL:(NSURL *)restURL;

@end
