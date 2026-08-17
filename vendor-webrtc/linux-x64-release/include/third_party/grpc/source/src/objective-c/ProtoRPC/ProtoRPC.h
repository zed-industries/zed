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

// import legacy header for compatibility with users using the ProtoRPC interface
#import "ProtoRPCLegacy.h"

#import "ProtoMethod.h"

NS_ASSUME_NONNULL_BEGIN

@class GRPCRequestOptions;
@class GRPCCallOptions;
@class GPBMessage;

/** An object can implement this protocol to receive responses from server from a call. */
@protocol GRPCProtoResponseHandler <NSObject>

@required

/**
 * All the responses must be issued to a user-provided dispatch queue. This property specifies the
 * dispatch queue to be used for issuing the notifications.
 */
@property(atomic, readonly) dispatch_queue_t dispatchQueue;

@optional

/**
 * Issued when initial metadata is received from the server.
 */
- (void)didReceiveInitialMetadata:(nullable NSDictionary *)initialMetadata;

/**
 * Issued when a message is received from the server. The message is the deserialized proto object.
 */
- (void)didReceiveProtoMessage:(nullable GPBMessage *)message;

/**
 * Issued when a call finished. If the call finished successfully, \p error is nil and \p
 * trailingMetadata consists any trailing metadata received from the server. Otherwise, \p error
 * is non-nil and contains the corresponding error information, including gRPC error codes and
 * error descriptions.
 */
- (void)didCloseWithTrailingMetadata:(nullable NSDictionary *)trailingMetadata
                               error:(nullable NSError *)error;

/**
 * Issued when flow control is enabled for the call and a message (written with writeMessage: method
 * of GRPCStreamingProtoCall or the initializer of GRPCUnaryProtoCall) is passed to gRPC core with
 * SEND_MESSAGE operation.
 */
- (void)didWriteMessage;

@end

/**
 * A convenience class of objects that act as response handlers of calls. Issues
 * response to a single handler when the response is completed.
 *
 * The object is stateful and should not be reused for multiple calls. If multiple calls share the
 * same response handling logic, create separate GRPCUnaryResponseHandler objects for each call.
 */
@interface GRPCUnaryResponseHandler<ResponseType> : NSObject <GRPCProtoResponseHandler>

/**
 * Creates a responsehandler object with a unary call handler.
 *
 * responseHandler: The unary handler to be called when the call is completed.
 * responseDispatchQueue: the dispatch queue on which the response handler
 * should be issued. If it's nil, the handler will use the main queue.
 */
- (nullable instancetype)initWithResponseHandler:(void (^)(ResponseType, NSError *))handler
                           responseDispatchQueue:(nullable dispatch_queue_t)dispatchQueue;

/** Response headers received during the call. */
@property(readonly, nullable) NSDictionary *responseHeaders;

/** Response trailers received during the call. */
@property(readonly, nullable) NSDictionary *responseTrailers;

@end

/** A unary-request RPC call with Protobuf. */
@interface GRPCUnaryProtoCall : NSObject

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Users should not use this initializer directly. Call objects will be created, initialized, and
 * returned to users by methods of the generated service.
 */
- (nullable instancetype)initWithRequestOptions:(GRPCRequestOptions *)requestOptions
                                        message:(GPBMessage *)message
                                responseHandler:(id<GRPCProtoResponseHandler>)handler
                                    callOptions:(nullable GRPCCallOptions *)callOptions
                                  responseClass:(Class)responseClass NS_DESIGNATED_INITIALIZER;

/**
 * Start the call. This function must only be called once for each instance.
 */
- (void)start;

/**
 * Cancel the request of this call at best effort. It attempts to notify the server that the RPC
 * should be cancelled, and issue didCloseWithTrailingMetadata:error: callback with error code
 * CANCELED if no other error code has already been issued.
 */
- (void)cancel;

@end

/** A client-streaming RPC call with Protobuf. */
@interface GRPCStreamingProtoCall : NSObject

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Users should not use this initializer directly. Call objects will be created, initialized, and
 * returned to users by methods of the generated service.
 */
- (nullable instancetype)initWithRequestOptions:(GRPCRequestOptions *)requestOptions
                                responseHandler:(id<GRPCProtoResponseHandler>)handler
                                    callOptions:(nullable GRPCCallOptions *)callOptions
                                  responseClass:(Class)responseClass NS_DESIGNATED_INITIALIZER;

/**
 * Start the call. This function must only be called once for each instance.
 */
- (void)start;

/**
 * Cancel the request of this call at best effort. It attempts to notify the server that the RPC
 * should be cancelled, and issue didCloseWithTrailingMetadata:error: callback with error code
 * CANCELED if no other error code has already been issued.
 */
- (void)cancel;

/**
 * Send a message to the server. The message should be a Protobuf message which will be serialized
 * internally.
 */
- (void)writeMessage:(GPBMessage *)message;

/**
 * Finish the RPC request and half-close the call. The server may still send messages and/or
 * trailers to the client.
 */
- (void)finish;

/**
 * Tell gRPC to receive another message.
 *
 * This method should only be used when flow control is enabled. If flow control is enabled, gRPC
 * will only receive additional messages after the user indicates so by using either
 * receiveNextMessage: or receiveNextMessages: methods. If flow control is not enabled, messages
 * will be automatically received after the previous one is delivered.
 */
- (void)receiveNextMessage;

/**
 * Tell gRPC to receive another N message.
 *
 * This method should only be used when flow control is enabled. If flow control is enabled, the
 * messages received from the server are buffered in gRPC until the user want to receive the next
 * message. If flow control is not enabled, messages will be automatically received after the
 * previous one is delivered.
 */
- (void)receiveNextMessages:(NSUInteger)numberOfMessages;

@end

NS_ASSUME_NONNULL_END
