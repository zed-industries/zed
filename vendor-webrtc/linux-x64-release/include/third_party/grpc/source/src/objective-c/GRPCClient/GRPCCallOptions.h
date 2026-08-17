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

#import <Foundation/Foundation.h>

#import "GRPCTypes.h"

NS_ASSUME_NONNULL_BEGIN

@protocol GRPCInterceptorFactory;

/**
 * Immutable user configurable options for a gRPC call.
 * Caller can obtain a mutable copy of type \b GRPCMutableCallOptions by calling [option
 * mutableCopy]
 */
@interface GRPCCallOptions : NSObject <NSCopying, NSMutableCopying>

// Call parameters
/**
 * The authority for the RPC. If nil, the default authority will be used.
 *
 * Note: This property does not have effect on Cronet transport and will be ignored.
 * Note: This property cannot be used to validate a self-signed server certificate. It control the
 *       :authority header field of the call and performs an extra check that server's certificate
 *       matches the :authority header.
 */
@property(nonatomic, copy, readonly, nullable) NSString *serverAuthority;

/**
 * The timeout for the RPC call in seconds. If set to 0, the call will not timeout. If set to
 * positive, the gRPC call returns with status GRPCErrorCodeDeadlineExceeded if it is not completed
 * within \a timeout seconds. A negative value is not allowed.
 */
@property(nonatomic, readonly) NSTimeInterval timeout;

/**
 * Enable flow control of a gRPC call. The option is default to NO. If set to YES, writeData: method
 * should only be called at most once before a didWriteData callback is issued, and
 * receiveNextMessage: must be called each time before gRPC call issues a didReceiveMessage
 * callback.
 */
@property(nonatomic, readonly) BOOL flowControlEnabled;

/**
 * An array of interceptor factories. When a call starts, interceptors are created
 * by these factories and chained together with the same order as the factories in
 * this array. This parameter should not be modified by any interceptor and will
 * not take effect if done so.
 */
@property(nonatomic, copy, readonly) NSArray<id<GRPCInterceptorFactory>> *interceptorFactories;

// OAuth2 parameters. Users of gRPC may specify one of the following two parameters.

/**
 * The OAuth2 access token string. The string is prefixed with "Bearer " then used as value of the
 * request's "authorization" header field. This parameter should not be used simultaneously with
 * \a authTokenProvider.
 */
@property(nonatomic, copy, readonly, nullable) NSString *oauth2AccessToken;

/**
 * The interface to get the OAuth2 access token string. gRPC will attempt to acquire token when
 * initiating the call. This parameter should not be used simultaneously with \a oauth2AccessToken.
 */
@property(nonatomic, readonly, nullable) id<GRPCAuthorizationProtocol> authTokenProvider;

/**
 * Initial metadata key-value pairs that should be included in the request.
 * Dictionary key is of type NSString, value should be either NSString or NSData containing binary
 * bytes data.
 */
@property(nonatomic, copy, readonly, nullable) GRPCMetadataDictionary *initialMetadata;

// Channel parameters; take into account of channel signature.

/**
 * Custom string that is prefixed to a request's user-agent header field before gRPC's internal
 * user-agent string.
 */
@property(nonatomic, copy, readonly, nullable) NSString *userAgentPrefix;

/**
 * Custom string that is suffixed to a request's user-agent header field after gRPC's internal
 * user-agent string.
 */
@property(nonatomic, copy, readonly, nullable) NSString *userAgentSuffix;

/**
 * The size limit for the response received from server. If it is exceeded, an error with status
 * code GRPCErrorCodeResourceExhausted is returned.
 */
@property(nonatomic, readonly) NSUInteger responseSizeLimit;

/**
 * The compression algorithm to be used by the gRPC call. For more details refer to
 * https://github.com/grpc/grpc/blob/master/doc/compression.md
 */
@property(nonatomic, readonly) GRPCCompressionAlgorithm compressionAlgorithm;

/**
 * Enable/Disable gRPC call's retry feature. The default is enabled. For details of this feature
 * refer to
 * https://github.com/grpc/proposal/blob/master/A6-client-retries.md
 */
@property(nonatomic, readonly) BOOL retryEnabled;

/**
 * Maximum interval in seconds between two consecutive retries.
 * Internal-only property used for GTMSessionFetcher transport retry policy.
 */
@property(nonatomic, readonly) NSTimeInterval maxRetryInterval;

/**
 * Minimum interval in seconds between two consecutive retries.
 * Internal-only property used for GTMSessionFetcher transport retry policy.
 */
@property(nonatomic, readonly) NSTimeInterval minRetryInterval;

/**
 * Multiplier used to increase the interval between retries.
 * Internal-only property used for GTMSessionFetcher transport retry policy.
 */
@property(readonly) double retryFactor;

// HTTP/2 keep-alive feature. The parameter \a keepaliveInterval specifies the interval between two
// PING frames. The parameter \a keepaliveTimeout specifies the length of the period for which the
// call should wait for PING ACK. If PING ACK is not received after this period, the call fails.
// Negative values are not allowed.
@property(nonatomic, readonly) NSTimeInterval keepaliveInterval;
@property(nonatomic, readonly) NSTimeInterval keepaliveTimeout;

// Parameters for connection backoff. Negative values are not allowed.
// For details of gRPC's backoff behavior, refer to
// https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md
@property(nonatomic, readonly) NSTimeInterval connectMinTimeout;
@property(nonatomic, readonly) NSTimeInterval connectInitialBackoff;
@property(nonatomic, readonly) NSTimeInterval connectMaxBackoff;

/**
 * Specify channel args to be used for this call. For a list of channel args available, see
 * grpc/grpc_types.h
 */
@property(nonatomic, copy, readonly, nullable) GRPCMetadataDictionary *additionalChannelArgs;

// Parameters for SSL authentication.

/**
 * PEM format root certifications that is trusted. If set to nil, gRPC uses a list of default
 * root certificates.
 */
@property(nonatomic, copy, readonly, nullable) NSString *PEMRootCertificates;

/**
 * PEM format private key for client authentication, if required by the server.
 */
@property(nonatomic, copy, readonly, nullable) NSString *PEMPrivateKey;

/**
 * PEM format certificate chain for client authentication, if required by the server.
 */
@property(nonatomic, copy, readonly, nullable) NSString *PEMCertificateChain;

/**
 * Deprecated: this option is deprecated. Please use the property \a transport
 * instead.
 *
 * Select the transport type to be used for this call.
 */
@property(nonatomic, readonly) GRPCTransportType transportType;

/**
 * The transport to be used for this call. Users may choose a native transport
 * identifier defined in \a GRPCTransport or provided by a non-native transport
 * implementation. If the option is left to be NULL, gRPC will use its default
 * transport.
 *
 * This is currently an experimental option.
 */
@property(nonatomic, readonly) GRPCTransportID transport;

/**
 * Override the hostname during the TLS hostname validation process.
 */
@property(nonatomic, copy, readonly, nullable) NSString *hostNameOverride;

/**
 * A string that specify the domain where channel is being cached. Channels with different domains
 * will not get cached to the same connection.
 */
@property(nonatomic, copy, readonly, nullable) NSString *channelPoolDomain;

/**
 * Channel id allows control of channel caching within a channelPoolDomain. A call with a unique
 * channelID will create a new channel (connection) instead of reusing an existing one. Multiple
 * calls in the same channelPoolDomain using identical channelID are allowed to share connection
 * if other channel options are also the same.
 */
@property(nonatomic, readonly) NSUInteger channelID;

/**
 * Hash for channel options.
 */
@property(nonatomic, readonly) NSUInteger channelOptionsHash;

/**
 * Return if the channel options are equal to another object.
 */
- (BOOL)hasChannelOptionsEqualTo:(GRPCCallOptions *)callOptions;

@end

/**
 * Mutable user configurable options for a gRPC call.
 * Caller can obtain an immutable copy of type \b GRPCCallOptions by calling [option copy]
 */
@interface GRPCMutableCallOptions : GRPCCallOptions <NSCopying, NSMutableCopying>

// Call parameters
/**
 * The authority for the RPC. If nil, the default authority will be used.
 *
 * Note: This property does not have effect on Cronet transport and will be ignored.
 * Note: This property cannot be used to validate a self-signed server certificate. It control the
 *       :authority header field of the call and performs an extra check that server's certificate
 *       matches the :authority header.
 */
@property(nonatomic, copy, readwrite, nullable) NSString *serverAuthority;

/**
 * The timeout for the RPC call in seconds. If set to 0, the call will not timeout. If set to
 * positive, the gRPC call returns with status GRPCErrorCodeDeadlineExceeded if it is not completed
 * within \a timeout seconds. Negative value is invalid; setting the parameter to negative value
 * will reset the parameter to 0.
 */
@property(nonatomic, readwrite) NSTimeInterval timeout;

/**
 * Enable flow control of a gRPC call. The option is default to NO. If set to YES, writeData: method
 * should only be called at most once before a didWriteData callback is issued, and
 * receiveNextMessage: must be called each time before gRPC call can issue a didReceiveMessage
 * callback.
 *
 * If writeData: method is called more than once before issuance of a didWriteData callback, gRPC
 * will continue to queue the message and write them to gRPC core in order. However, the user
 * assumes their own responsibility of flow control by keeping tracking of the pending writes in
 * the call.
 */
@property(nonatomic, readwrite) BOOL flowControlEnabled;

/**
 * An array of interceptor factories. When a call starts, interceptors are created
 * by these factories and chained together with the same order as the factories in
 * this array. This parameter should not be modified by any interceptor and will
 * not take effect if done so.
 */
@property(nonatomic, copy, readwrite) NSArray<id<GRPCInterceptorFactory>> *interceptorFactories;

// OAuth2 parameters. Users of gRPC may specify one of the following two parameters.

/**
 * The OAuth2 access token string. The string is prefixed with "Bearer " then used as value of the
 * request's "authorization" header field. This parameter should not be used simultaneously with
 * \a authTokenProvider.
 */
@property(nonatomic, copy, readwrite, nullable) NSString *oauth2AccessToken;

/**
 * The interface to get the OAuth2 access token string. gRPC will attempt to acquire token when
 * initiating the call. This parameter should not be used simultaneously with \a oauth2AccessToken.
 */
@property(nonatomic, readwrite, nullable) id<GRPCAuthorizationProtocol> authTokenProvider;

/**
 * Initial metadata key-value pairs that should be included in the request.
 * Dictionary key is of type NSString, value should be either NSString or NSData containing binary
 * bytes data.
 */
@property(nonatomic, nonatomic, copy, readwrite, nullable) GRPCMetadataDictionary *initialMetadata;

// Channel parameters; take into account of channel signature.

/**
 * Custom string that is prefixed to a request's user-agent header field before gRPC's internal
 * user-agent string.
 */
@property(nonatomic, copy, readwrite, nullable) NSString *userAgentPrefix;

/**
 * Custom string that is suffixed to a request's user-agent header field after gRPC's internal
 * user-agent string.
 */
@property(nonatomic, copy, readwrite, nullable) NSString *userAgentSuffix;

/**
 * The size limit for the response received from server. If it is exceeded, an error with status
 * code GRPCErrorCodeResourceExhausted is returned.
 */
@property(nonatomic, readwrite) NSUInteger responseSizeLimit;

/**
 * The compression algorithm to be used by the gRPC call. For more details refer to
 * https://github.com/grpc/grpc/blob/master/doc/compression.md
 */
@property(nonatomic, readwrite) GRPCCompressionAlgorithm compressionAlgorithm;

/**
 * Enable/Disable gRPC call's retry feature. The default is enabled. For details of this feature
 * refer to
 * https://github.com/grpc/proposal/blob/master/A6-client-retries.md
 */
@property(nonatomic, readwrite) BOOL retryEnabled;

/**
 * Maximum interval in seconds between two consecutive retries. Pass 0 to use default.
 * Internal-only property used for GTMSessionFetcher transport retry policy.
 */
@property(nonatomic, readwrite) NSTimeInterval maxRetryInterval;

/**
 * Minimum interval in seconds between two consecutive retries. Pass 0 to use default.
 * Internal-only property used for GTMSessionFetcher transport retry policy.
 */
@property(nonatomic, readwrite) NSTimeInterval minRetryInterval;

/**
 * Multiplier used to increase the interval between retries. Pass 0 to use default.
 * Internal-only property used for GTMSessionFetcher transport retry policy.
 */
@property(nonatomic, readwrite) double retryFactor;

// HTTP/2 keep-alive feature. The parameter \a keepaliveInterval specifies the interval between two
// PING frames. The parameter \a keepaliveTimeout specifies the length of the period for which the
// call should wait for PING ACK. If PING ACK is not received after this period, the call fails.
// Negative values are invalid; setting these parameters to negative value will reset the
// corresponding parameter to 0.
@property(nonatomic, readwrite) NSTimeInterval keepaliveInterval;
@property(nonatomic, readwrite) NSTimeInterval keepaliveTimeout;

// Parameters for connection backoff. Negative value is invalid; setting the parameters to negative
// value will reset corresponding parameter to 0.
// For details of gRPC's backoff behavior, refer to
// https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md
@property(nonatomic, readwrite) NSTimeInterval connectMinTimeout;
@property(nonatomic, readwrite) NSTimeInterval connectInitialBackoff;
@property(nonatomic, readwrite) NSTimeInterval connectMaxBackoff;

/**
 * Specify channel args to be used for this call. For a list of channel args available, see
 * grpc/grpc_types.h
 */
@property(nonatomic, copy, readwrite, nullable) GRPCMetadataDictionary *additionalChannelArgs;

// Parameters for SSL authentication.

/**
 * PEM format root certifications that is trusted. If set to nil, gRPC uses a list of default
 * root certificates.
 */
@property(nonatomic, copy, readwrite, nullable) NSString *PEMRootCertificates;

/**
 * PEM format private key for client authentication, if required by the server.
 */
@property(nonatomic, copy, readwrite, nullable) NSString *PEMPrivateKey;

/**
 * PEM format certificate chain for client authentication, if required by the server.
 */
@property(nonatomic, copy, readwrite, nullable) NSString *PEMCertificateChain;

/**
 * Deprecated: this option is deprecated. Please use the property \a transport
 * instead.
 *
 * Select the transport type to be used for this call.
 */
@property(nonatomic, readwrite) GRPCTransportType transportType;

/**
 * The transport to be used for this call. Users may choose a native transport
 * identifier defined in \a GRPCTransport or provided by a non-native ttransport
 * implementation. If the option is left to be NULL, gRPC will use its default
 * transport.
 *
 * An interceptor must not change the value of this option.
 */
@property(nonatomic, readwrite) GRPCTransportID transport;

/**
 * Override the hostname during the TLS hostname validation process.
 */
@property(nonatomic, copy, readwrite, nullable) NSString *hostNameOverride;

/**
 * A string that specify the domain where channel is being cached. Channels with different domains
 * will not get cached to the same channel. For example, a gRPC example app may use the channel pool
 * domain 'io.grpc.example' so that its calls do not reuse the channel created by other modules in
 * the same process.
 */
@property(nonatomic, copy, readwrite, nullable) NSString *channelPoolDomain;

/**
 * Channel id allows a call to force creating a new channel (connection) rather than using a cached
 * channel. Calls using distinct channelID's will not get cached to the same channel.
 */
@property(nonatomic, readwrite) NSUInteger channelID;

@end

NS_ASSUME_NONNULL_END
