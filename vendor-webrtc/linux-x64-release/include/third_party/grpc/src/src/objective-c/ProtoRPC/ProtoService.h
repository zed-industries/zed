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

#import <GRPCClient/GRPCCallOptions.h>
#import "ProtoRPC.h"

@protocol GRXWriteable;
@class GRXWriter;
@class GRPCCallOptions;

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wnullability-completeness"

__attribute__((deprecated("Please use GRPCProtoService.")))
@interface ProtoService : NSObject

- (nullable instancetype)initWithHost:(nonnull NSString *)host
                          packageName:(nonnull NSString *)packageName
                          serviceName:(nonnull NSString *)serviceName
                          callOptions:(nullable GRPCCallOptions *)callOptions
    NS_DESIGNATED_INITIALIZER;

- (nullable GRPCUnaryProtoCall *)RPCToMethod:(nonnull NSString *)method
                                     message:(nonnull id)message
                             responseHandler:(nonnull id<GRPCProtoResponseHandler>)handler
                                 callOptions:(nullable GRPCCallOptions *)callOptions
                               responseClass:(nonnull Class)responseClass;

- (nullable GRPCStreamingProtoCall *)RPCToMethod:(nonnull NSString *)method
                                 responseHandler:(nonnull id<GRPCProtoResponseHandler>)handler
                                     callOptions:(nullable GRPCCallOptions *)callOptions
                                   responseClass:(nonnull Class)responseClass;

@end

@interface ProtoService (Legacy)

- (instancetype)initWithHost:(NSString *)host
                 packageName:(NSString *)packageName
                 serviceName:(NSString *)serviceName;

- (GRPCProtoCall *)RPCToMethod:(NSString *)method
                requestsWriter:(GRXWriter *)requestsWriter
                 responseClass:(Class)responseClass
            responsesWriteable:(id<GRXWriteable>)responsesWriteable;

@end

#pragma clang diagnostic pop

/**
 * This subclass is empty now. Eventually we'll remove ProtoService class
 * to avoid potential naming conflict
 */
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
@interface GRPCProtoService : ProtoService
#pragma clang diagnostic pop

@end
