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

#import <RxLibrary/GRXWriter.h>
#import "GRPCTypes.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wnullability-completeness"

/**
 * This is the legacy interface of this gRPC library. This API is deprecated and users should use
 * GRPCCall2 in GRPCCall.h. This API exists solely for the purpose of backwards compatibility.
 * Represents a single gRPC remote call.
 */
@interface GRPCCall : GRXWriter

- (instancetype)init NS_UNAVAILABLE;

/**
 * The container of the request headers of an RPC conforms to this protocol, which is a subset of
 * NSMutableDictionary's interface. It will become a NSMutableDictionary later on.
 * The keys of this container are the header names, which per the HTTP standard are case-
 * insensitive. They are stored in lowercase (which is how HTTP/2 mandates them on the wire), and
 * can only consist of ASCII characters.
 * A header value is a NSString object (with only ASCII characters), unless the header name has the
 * suffix "-bin", in which case the value has to be a NSData object.
 */
/**
 * These HTTP headers will be passed to the server as part of this call. Each HTTP header is a
 * name-value pair with string names and either string or binary values.
 *
 * The passed dictionary has to use NSString keys, corresponding to the header names. The value
 * associated to each can be a NSString object or a NSData object. E.g.:
 *
 * call.requestHeaders = @{@"authorization": @"Bearer ..."};
 *
 * call.requestHeaders[@"my-header-bin"] = someData;
 *
 * After the call is started, trying to modify this property is an error.
 *
 * The property is initialized to an empty NSMutableDictionary.
 */
@property(atomic, readonly) NSMutableDictionary *requestHeaders;

/**
 * This dictionary is populated with the HTTP headers received from the server. This happens before
 * any response message is received from the server. It has the same structure as the request
 * headers dictionary: Keys are NSString header names; names ending with the suffix "-bin" have a
 * NSData value; the others have a NSString value.
 *
 * The value of this property is nil until all response headers are received, and will change before
 * any of -writeValue: or -writesFinishedWithError: are sent to the writeable.
 */
@property(atomic, copy, readonly) NSDictionary *responseHeaders;

/**
 * Same as responseHeaders, but populated with the HTTP trailers received from the server before the
 * call finishes.
 *
 * The value of this property is nil until all response trailers are received, and will change
 * before -writesFinishedWithError: is sent to the writeable.
 */
@property(atomic, copy, readonly) NSDictionary *responseTrailers;

/**
 * The request writer has to write NSData objects into the provided Writeable. The server will
 * receive each of those separately and in order as distinct messages.
 * A gRPC call might not complete until the request writer finishes. On the other hand, the request
 * finishing doesn't necessarily make the call to finish, as the server might continue sending
 * messages to the response side of the call indefinitely (depending on the semantics of the
 * specific remote method called).
 * To finish a call right away, invoke cancel.
 * host parameter should not contain the scheme (http:// or https://), only the name or IP addr
 * and the port number, for example @"localhost:5050".
 */
- (instancetype)initWithHost:(NSString *)host
                        path:(NSString *)path
              requestsWriter:(GRXWriter *)requestWriter;

/**
 * Finishes the request side of this call, notifies the server that the RPC should be cancelled, and
 * finishes the response side of the call with an error of code CANCELED.
 */
- (void)cancel;

/**
 * The following methods are deprecated.
 */
+ (void)setCallSafety:(GRPCCallSafety)callSafety host:(NSString *)host path:(NSString *)path;
@property(atomic, copy, readwrite) NSString *serverName;
@property NSTimeInterval timeout;
- (void)setResponseDispatchQueue:(dispatch_queue_t)queue;

@end

#pragma mark Backwards compatibiity

/** This protocol is kept for backwards compatibility with existing code. */
DEPRECATED_MSG_ATTRIBUTE("Use NSDictionary or NSMutableDictionary instead.")
@protocol GRPCRequestHeaders <NSObject>
@property(nonatomic, readonly) NSUInteger count;

- (id)objectForKeyedSubscript:(id)key;
- (void)setObject:(id)obj forKeyedSubscript:(id)key;

- (void)removeAllObjects;
- (void)removeObjectForKey:(id)key;
@end

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated"
/** This is only needed for backwards-compatibility. */
@interface NSMutableDictionary (GRPCRequestHeaders) <GRPCRequestHeaders>
@end
#pragma clang diagnostic pop
#pragma clang diagnostic pop
