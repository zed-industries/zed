/*
 *
 * Copyright 2015 gRPC authors.
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

#import <Foundation/Foundation.h>

#include <grpc/grpc.h>

@protocol GRPCChannelFactory;

@class GRPCCompletionQueue;
@class GRPCCallOptions;
@class GRPCChannelConfiguration;
struct grpc_channel_credentials;

NS_ASSUME_NONNULL_BEGIN

/**
 * Signature for the channel. If two channel's signatures are the same and connect to the same
 * remote, they share the same underlying \a GRPCChannel object.
 */
@interface GRPCChannelConfiguration : NSObject <NSCopying>

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

/** The host that this channel is connected to. */
@property(copy, readonly) NSString *host;

/**
 * Options of the corresponding call. Note that only the channel-related options are of interest to
 * this class.
 */
@property(readonly) GRPCCallOptions *callOptions;

/** Acquire the factory to generate a new channel with current configurations. */
@property(readonly) id<GRPCChannelFactory> channelFactory;

/** Acquire the dictionary of channel args with current configurations. */
@property(copy, readonly) NSDictionary *channelArgs;

- (nullable instancetype)initWithHost:(NSString *)host
                          callOptions:(GRPCCallOptions *)callOptions NS_DESIGNATED_INITIALIZER;

@end

/**
 * Each separate instance of this class represents at least one TCP connection to the provided host.
 */
@interface GRPCChannel : NSObject

- (nullable instancetype)init NS_UNAVAILABLE;

+ (nullable instancetype)new NS_UNAVAILABLE;

/**
 * Create a channel with remote \a host and signature \a channelConfigurations.
 */
- (nullable instancetype)initWithChannelConfiguration:
    (GRPCChannelConfiguration *)channelConfiguration NS_DESIGNATED_INITIALIZER;

/**
 * Create a grpc core call object (grpc_call) from this channel. If no call is created, NULL is
 * returned.
 */
- (nullable grpc_call *)unmanagedCallWithPath:(NSString *)path
                              completionQueue:(GRPCCompletionQueue *)queue
                                  callOptions:(GRPCCallOptions *)callOptions;

@end

NS_ASSUME_NONNULL_END
