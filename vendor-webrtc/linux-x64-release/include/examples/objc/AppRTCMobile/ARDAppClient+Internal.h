/*
 *  Copyright 2014 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "ARDAppClient.h"

#import "sdk/objc/api/peerconnection/RTCPeerConnection.h"

#import "ARDRoomServerClient.h"
#import "ARDSignalingChannel.h"
#import "ARDTURNClient.h"

@class RTC_OBJC_TYPE(RTCPeerConnectionFactory);

@interface ARDAppClient () <ARDSignalingChannelDelegate,
                            RTC_OBJC_TYPE (RTCPeerConnectionDelegate)>

// All properties should only be mutated from the main queue.
@property(nonatomic, strong) id<ARDRoomServerClient> roomServerClient;
@property(nonatomic, strong) id<ARDSignalingChannel> channel;
@property(nonatomic, strong) id<ARDSignalingChannel> loopbackChannel;
@property(nonatomic, strong) id<ARDTURNClient> turnClient;

@property(nonatomic, strong) RTC_OBJC_TYPE(RTCPeerConnection) * peerConnection;
@property(nonatomic, strong) RTC_OBJC_TYPE(RTCPeerConnectionFactory) * factory;
@property(nonatomic, strong) NSMutableArray *messageQueue;

@property(nonatomic, assign) BOOL isTurnComplete;
@property(nonatomic, assign) BOOL hasReceivedSdp;
@property(nonatomic, readonly) BOOL hasJoinedRoomServerRoom;

@property(nonatomic, strong) NSString *roomId;
@property(nonatomic, strong) NSString *clientId;
@property(nonatomic, assign) BOOL isInitiator;
@property(nonatomic, strong) NSMutableArray *iceServers;
@property(nonatomic, strong) NSURL *webSocketURL;
@property(nonatomic, strong) NSURL *webSocketRestURL;
@property(nonatomic, readonly) BOOL isLoopback;

@property(nonatomic, strong) RTC_OBJC_TYPE(RTCMediaConstraints) *
    defaultPeerConnectionConstraints;

- (instancetype)initWithRoomServerClient:(id<ARDRoomServerClient>)rsClient
                        signalingChannel:(id<ARDSignalingChannel>)channel
                              turnClient:(id<ARDTURNClient>)turnClient
                                delegate:(id<ARDAppClientDelegate>)delegate;

@end
