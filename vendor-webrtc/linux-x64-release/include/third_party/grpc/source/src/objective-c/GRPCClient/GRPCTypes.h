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

#import <Foundation/Foundation.h>

/**
 * gRPC error codes.
 * Note that a few of these are never produced by the gRPC libraries, but are of
 * general utility for server applications to produce.
 */
typedef NS_ENUM(NSUInteger, GRPCErrorCode) {
  /** The operation was cancelled (typically by the caller). */
  GRPCErrorCodeCancelled = 1,

  /**
   * Unknown error. Errors raised by APIs that do not return enough error
   * information may be converted to this error.
   */
  GRPCErrorCodeUnknown = 2,

  /**
   * The client specified an invalid argument. Note that this differs from
   * FAILED_PRECONDITION. INVALID_ARGUMENT indicates arguments that are
   * problematic regardless of the state of the server (e.g., a malformed file
   * name).
   */
  GRPCErrorCodeInvalidArgument = 3,

  /**
   * Deadline expired before operation could complete. For operations that
   * change the state of the server, this error may be returned even if the
   * operation has completed successfully. For example, a successful response
   * from the server could have been delayed long enough for the deadline to
   * expire.
   */
  GRPCErrorCodeDeadlineExceeded = 4,

  /** Some requested entity (e.g., file or directory) was not found. */
  GRPCErrorCodeNotFound = 5,

  /** Some entity that we attempted to create (e.g., file or directory) already
     exists. */
  GRPCErrorCodeAlreadyExists = 6,

  /**
   * The caller does not have permission to execute the specified operation.
   * PERMISSION_DENIED isn't used for rejections caused by exhausting some
   * resource (RESOURCE_EXHAUSTED is used instead for those errors).
   * PERMISSION_DENIED doesn't indicate a failure to identify the caller
   * (UNAUTHENTICATED is used instead for those errors).
   */
  GRPCErrorCodePermissionDenied = 7,

  /**
   * The request does not have valid authentication credentials for the
   * operation (e.g. the caller's identity can't be verified).
   */
  GRPCErrorCodeUnauthenticated = 16,

  /** Some resource has been exhausted, perhaps a per-user quota. */
  GRPCErrorCodeResourceExhausted = 8,

  /**
   * The RPC was rejected because the server is not in a state required for the
   * procedure's execution. For example, a directory to be deleted may be
   * non-empty, etc. The client should not retry until the server state has been
   * explicitly fixed (e.g. by performing another RPC). The details depend on
   * the service being called, and should be found in the NSError's userInfo.
   */
  GRPCErrorCodeFailedPrecondition = 9,

  /**
   * The RPC was aborted, typically due to a concurrency issue like sequencer
   * check failures, transaction aborts, etc. The client should retry at a
   * higher-level (e.g., restarting a read- modify-write sequence).
   */
  GRPCErrorCodeAborted = 10,

  /**
   * The RPC was attempted past the valid range. E.g., enumerating past the end
   * of a list. Unlike INVALID_ARGUMENT, this error indicates a problem that may
   * be fixed if the system state changes. For example, an RPC to get elements
   * of a list will generate INVALID_ARGUMENT if asked to return the element at
   * a negative index, but it will generate OUT_OF_RANGE if asked to return the
   * element at an index past the current size of the list.
   */
  GRPCErrorCodeOutOfRange = 11,

  /** The procedure is not implemented or not supported/enabled in this server.
   */
  GRPCErrorCodeUnimplemented = 12,

  /**
   * Internal error. Means some invariant expected by the server application or
   * the gRPC library has been broken.
   */
  GRPCErrorCodeInternal = 13,

  /**
   * The server is currently unavailable. This is most likely a transient
   * condition and may be corrected by retrying with a backoff. Note that it is
   * not always safe to retry non-idempotent operations.
   */
  GRPCErrorCodeUnavailable = 14,

  /** Unrecoverable data loss or corruption. */
  GRPCErrorCodeDataLoss = 15,
};

/**
 * Safety remark of a gRPC method as defined in RFC 2616 Section 9.1
 */
typedef NS_ENUM(NSUInteger, GRPCCallSafety) {
  /**
   * Signal that there is no guarantees on how the call affects the server
   * state.
   */
  GRPCCallSafetyDefault = 0,
};

/**
 * Compression algorithm to be used by a gRPC call.
 *
 * <b>This enumeration and corresponding call option GRPCCallOptions.transportType are deprecated by
 * the call option GRPCCallOptions.transport. </b>
 */
typedef NS_ENUM(NSUInteger, GRPCCompressionAlgorithm) {
  GRPCCompressNone = 0,
  GRPCCompressDeflate,
  GRPCCompressGzip,
  GRPCStreamCompressGzip,
};

/** GRPCCompressAlgorithm is deprecated. */
typedef GRPCCompressionAlgorithm GRPCCompressAlgorithm;

/** The transport to be used by a gRPC call */
typedef NS_ENUM(NSUInteger, GRPCTransportType) {
  GRPCTransportTypeDefault = 0,
  /** gRPC internal HTTP/2 stack with BoringSSL */
  GRPCTransportTypeChttp2BoringSSL = 0,
  /** Cronet stack */
  GRPCTransportTypeCronet,
  /** Insecure channel. FOR TEST ONLY! */
  GRPCTransportTypeInsecure,
};

/** Domain of NSError objects produced by gRPC. */
extern NSString* _Nonnull const kGRPCErrorDomain;

/**
 * Keys used in |NSError|'s |userInfo| dictionary to store the response headers
 * and trailers sent by the server.
 */
extern NSString* _Nonnull const kGRPCHeadersKey;
extern NSString* _Nonnull const kGRPCTrailersKey;

/** The id of a transport implementation. */
typedef const char* _Nonnull GRPCTransportID;

/**
 * Implement this protocol to provide a token to gRPC when a call is initiated.
 */
@protocol GRPCAuthorizationProtocol

/**
 * This method is called when gRPC is about to start the call. When OAuth token is acquired,
 * \a handler is expected to be called with \a token being the new token to be used for this call.
 */
- (void)getTokenWithHandler:(void (^_Nonnull)(NSString* _Nullable token))handler;

@end

/** gRPC metadata dictionary typedef */
typedef NSDictionary<NSString*, id> GRPCMetadataDictionary;
