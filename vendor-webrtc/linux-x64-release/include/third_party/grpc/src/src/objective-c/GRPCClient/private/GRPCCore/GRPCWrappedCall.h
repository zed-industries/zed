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

#import "GRPCRequestHeaders.h"

@interface GRPCOperation : NSObject
@property(nonatomic, readonly) grpc_op op;
/** Guaranteed to be called when the operation has finished. */
- (void)finish;
@end

@interface GRPCOpSendMetadata : GRPCOperation

- (instancetype)initWithMetadata:(NSDictionary *)metadata handler:(void (^)(void))handler;

- (instancetype)initWithMetadata:(NSDictionary *)metadata
                           flags:(uint32_t)flags
                         handler:(void (^)(void))handler NS_DESIGNATED_INITIALIZER;

@end

@interface GRPCOpSendMessage : GRPCOperation

- (instancetype)initWithMessage:(NSData *)message
                        handler:(void (^)(void))handler NS_DESIGNATED_INITIALIZER;

@end

@interface GRPCOpSendClose : GRPCOperation

- (instancetype)initWithHandler:(void (^)(void))handler NS_DESIGNATED_INITIALIZER;

@end

@interface GRPCOpRecvMetadata : GRPCOperation

- (instancetype)initWithHandler:(void (^)(NSDictionary *))handler NS_DESIGNATED_INITIALIZER;

@end

@interface GRPCOpRecvMessage : GRPCOperation

- (instancetype)initWithHandler:(void (^)(grpc_byte_buffer *))handler NS_DESIGNATED_INITIALIZER;

@end

@interface GRPCOpRecvStatus : GRPCOperation

- (instancetype)initWithHandler:(void (^)(NSError *, NSDictionary *))handler
    NS_DESIGNATED_INITIALIZER;

@end

#pragma mark GRPCWrappedCall

@class GRPCPooledChannel;

@interface GRPCWrappedCall : NSObject

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

- (instancetype)initWithUnmanagedCall:(grpc_call *)unmanagedCall
                        pooledChannel:(GRPCPooledChannel *)pooledChannel NS_DESIGNATED_INITIALIZER;

- (void)startBatchWithOperations:(NSArray *)ops errorHandler:(void (^)(void))errorHandler;

- (void)startBatchWithOperations:(NSArray *)ops;

- (void)cancel;

- (void)channelDisconnected;

@end
