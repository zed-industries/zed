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

#import <grpc/impl/compression_types.h>

#import "GRPCChannelFactory.h"

#import <GRPCClient/GRPCCallOptions.h>

NS_ASSUME_NONNULL_BEGIN

@class GRPCCompletionQueue;
struct grpc_call;
struct grpc_channel_credentials;

@interface GRPCHost : NSObject

+ (void)resetAllHostSettings;

@property(nonatomic, readonly) NSString *address;
@property(nonatomic, copy, nullable) NSString *userAgentPrefix;
@property(nonatomic, copy, nullable) NSString *userAgentSuffix;
@property(nonatomic) grpc_compression_algorithm compressAlgorithm;
@property(nonatomic) int keepaliveInterval;
@property(nonatomic) int keepaliveTimeout;
@property(nonatomic) id logContext;
@property(nonatomic) BOOL retryEnabled;

@property(nonatomic) unsigned int minConnectTimeout;
@property(nonatomic) unsigned int initialConnectBackoff;
@property(nonatomic) unsigned int maxConnectBackoff;

@property(nonatomic) id<GRPCChannelFactory> channelFactory;

/** The following properties should only be modified for testing: */

@property(nonatomic, copy, nullable) NSString *hostNameOverride;

/** The default response size limit is 4MB. Set this to override that default. */
@property(nonatomic) NSUInteger responseSizeLimitOverride;

- (nullable instancetype)init NS_UNAVAILABLE;
/** Host objects initialized with the same address are the same. */
+ (nullable instancetype)hostWithAddress:(NSString *)address;
- (nullable instancetype)initWithAddress:(NSString *)address NS_DESIGNATED_INITIALIZER;
- (BOOL)setTLSPEMRootCerts:(nullable NSString *)pemRootCerts
            withPrivateKey:(nullable NSString *)pemPrivateKey
             withCertChain:(nullable NSString *)pemCertChain
                     error:(NSError **)errorPtr;

@property(atomic) GRPCTransportType transportType;

+ (GRPCCallOptions *)callOptionsForHost:(NSString *)host;

@end

NS_ASSUME_NONNULL_END
