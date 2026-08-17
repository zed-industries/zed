/*
 *
 * Copyright 2018 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#import <GRPCClient/GRPCCallOptions.h>

NS_ASSUME_NONNULL_BEGIN

@protocol GRPCChannel;
@class GRPCChannel;
@class GRPCChannelPool;
@class GRPCCompletionQueue;
@class GRPCChannelConfiguration;
@class GRPCWrappedCall;

/**
 * A proxied channel object that can be retained and used to create GRPCWrappedCall object
 * regardless of the current connection status. If a connection is not established when a
 * GRPCWrappedCall object is requested, it issues a connection/reconnection. This behavior is to
 * follow that of gRPC core's channel object.
 */
@interface GRPCPooledChannel : NSObject

- (nullable instancetype)init NS_UNAVAILABLE;

+ (nullable instancetype)new NS_UNAVAILABLE;

/**
 * Initialize with an actual channel object \a channel and a reference to the channel pool.
 */
- (nullable instancetype)initWithChannelConfiguration:
    (GRPCChannelConfiguration *)channelConfiguration;

/**
 * Create a GRPCWrappedCall object (grpc_call) from this channel. If channel is disconnected, get a
 * new channel object from the channel pool.
 */
- (nullable GRPCWrappedCall *)wrappedCallWithPath:(NSString *)path
                                  completionQueue:(GRPCCompletionQueue *)queue
                                      callOptions:(GRPCCallOptions *)callOptions;

/**
 * Notify the pooled channel that a wrapped call object is no longer referenced and will be
 * dealloc'ed.
 */
- (void)notifyWrappedCallDealloc:(GRPCWrappedCall *)wrappedCall;

/**
 * Force the channel to disconnect immediately. GRPCWrappedCall objects previously created with
 * \a wrappedCallWithPath are failed if not already finished. Subsequent calls to
 * unmanagedCallWithPath: will attempt to reconnect to the remote channel.
 */
- (void)disconnect;

@end

/**
 * Manage the pool of connected channels. When a channel is no longer referenced by any call,
 * destroy the channel after a certain period of time elapsed.
 */
@interface GRPCChannelPool : NSObject

- (nullable instancetype)init NS_UNAVAILABLE;

+ (nullable instancetype)new NS_UNAVAILABLE;

/**
 * Get the global channel pool.
 */
+ (nullable instancetype)sharedInstance;

/**
 * Return a channel with a particular configuration. The channel may be a cached channel.
 */
- (nullable GRPCPooledChannel *)channelWithHost:(NSString *)host
                                    callOptions:(GRPCCallOptions *)callOptions;

/**
 * Disconnect all channels in this pool.
 */
- (void)disconnectAllChannels;

@end

NS_ASSUME_NONNULL_END
