/*
 *
 * Copyright 2019 gRPC authors.
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

#import <GRPCClient/GRPCInterceptor.h>
#import <GRPCClient/GRPCTransport.h>

NS_ASSUME_NONNULL_BEGIN

/**
 * Private interfaces of the transport registry.
 */
@interface GRPCTransportRegistry (Private)

/**
 * Get a transport implementation's factory by its transport id. If the transport id was not
 * registered with the registry, the default transport factory (core + secure) is returned. If the
 * default transport does not exist, an exception is thrown.
 */
- (id<GRPCTransportFactory>)getTransportFactoryWithID:(GRPCTransportID)transportID;

@end

/**
 * GRPCTransportManager is a helper class to forward messages between the last interceptor and the
 * transport instance.
 *
 * All methods except the initializer of the class can only be called on the manager's dispatch
 * queue. Since the manager's dispatch queue is the same as the transport's dispatch queue, it is
 * also safe to call the manager's methods in the corresponding transport instance's methods that
 * implement GRPCInterceptorInterface.
 *
 * When a transport instance is shutting down, it must call -shutDown method of its associated
 * transport manager for proper clean-up.
 */
@interface GRPCTransportManager : NSObject <GRPCInterceptorInterface>

- (instancetype)initWithTransportID:(GRPCTransportID)transportID
                previousInterceptor:(id<GRPCResponseHandler>)previousInterceptor;

/**
 * Notify the manager that the transport has shut down and the manager should release references to
 * its response handler and stop forwarding requests/responses.
 */
- (void)shutDown;

/** Forward initial metadata to the previous interceptor in the interceptor chain */
- (void)forwardPreviousInterceptorWithInitialMetadata:(nullable NSDictionary *)initialMetadata;

/** Forward a received message to the previous interceptor in the interceptor chain */
- (void)forwardPreviousInterceptorWithData:(nullable id)data;

/** Forward call close and trailing metadata to the previous interceptor in the interceptor chain */
- (void)forwardPreviousInterceptorCloseWithTrailingMetadata:
            (nullable NSDictionary *)trailingMetadata
                                                      error:(nullable NSError *)error;

/** Forward write completion to the previous interceptor in the interceptor chain */
- (void)forwardPreviousInterceptorDidWriteData;

@end

NS_ASSUME_NONNULL_END
