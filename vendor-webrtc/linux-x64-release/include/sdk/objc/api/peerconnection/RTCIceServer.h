/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

typedef NS_ENUM(NSUInteger, RTC_OBJC_TYPE(RTCTlsCertPolicy)) {
  RTC_OBJC_TYPE(RTCTlsCertPolicySecure),
  RTC_OBJC_TYPE(RTCTlsCertPolicyInsecureNoCheck)
};

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCIceServer) : NSObject

/** URI(s) for this server represented as NSStrings. */
@property(nonatomic, readonly) NSArray<NSString *> *urlStrings;

/** Username to use if this RTCIceServer object is a TURN server. */
@property(nonatomic, readonly, nullable) NSString *username;

/** Credential to use if this RTCIceServer object is a TURN server. */
@property(nonatomic, readonly, nullable) NSString *credential;

/**
 * TLS certificate policy to use if this RTCIceServer object is a TURN server.
 */
@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCTlsCertPolicy) tlsCertPolicy;

/**
  If the URIs in `urls` only contain IP addresses, this field can be used
  to indicate the hostname, which may be necessary for TLS (using the SNI
  extension). If `urls` itself contains the hostname, this isn't necessary.
 */
@property(nonatomic, readonly, nullable) NSString *hostname;

/** List of protocols to be used in the TLS ALPN extension. */
@property(nonatomic, readonly) NSArray<NSString *> *tlsAlpnProtocols;

/**
  List elliptic curves to be used in the TLS elliptic curves extension.
  Only curve names supported by OpenSSL should be used (eg. "P-256","X25519").
  */
@property(nonatomic, readonly) NSArray<NSString *> *tlsEllipticCurves;

- (nonnull instancetype)init NS_UNAVAILABLE;

/** Convenience initializer for a server with no authentication (e.g. STUN). */
- (instancetype)initWithURLStrings:(NSArray<NSString *> *)urlStrings;

/**
 * Initialize an RTCIceServer with its associated URLs, optional username,
 * optional credential, and credentialType.
 */
- (instancetype)initWithURLStrings:(NSArray<NSString *> *)urlStrings
                          username:(nullable NSString *)username
                        credential:(nullable NSString *)credential;

/**
 * Initialize an RTCIceServer with its associated URLs, optional username,
 * optional credential, and TLS cert policy.
 */
- (instancetype)initWithURLStrings:(NSArray<NSString *> *)urlStrings
                          username:(nullable NSString *)username
                        credential:(nullable NSString *)credential
                     tlsCertPolicy:(RTC_OBJC_TYPE(RTCTlsCertPolicy))tlsCertPolicy;

/**
 * Initialize an RTCIceServer with its associated URLs, optional username,
 * optional credential, TLS cert policy and hostname.
 */
- (instancetype)initWithURLStrings:(NSArray<NSString *> *)urlStrings
                          username:(nullable NSString *)username
                        credential:(nullable NSString *)credential
                     tlsCertPolicy:(RTC_OBJC_TYPE(RTCTlsCertPolicy))tlsCertPolicy
                          hostname:(nullable NSString *)hostname;

/**
 * Initialize an RTCIceServer with its associated URLs, optional username,
 * optional credential, TLS cert policy, hostname and ALPN protocols.
 */
- (instancetype)initWithURLStrings:(NSArray<NSString *> *)urlStrings
                          username:(nullable NSString *)username
                        credential:(nullable NSString *)credential
                     tlsCertPolicy:(RTC_OBJC_TYPE(RTCTlsCertPolicy))tlsCertPolicy
                          hostname:(nullable NSString *)hostname
                  tlsAlpnProtocols:(NSArray<NSString *> *)tlsAlpnProtocols;

/**
 * Initialize an RTCIceServer with its associated URLs, optional username,
 * optional credential, TLS cert policy, hostname, ALPN protocols and
 * elliptic curves.
 */
- (instancetype)initWithURLStrings:(NSArray<NSString *> *)urlStrings
                          username:(nullable NSString *)username
                        credential:(nullable NSString *)credential
                     tlsCertPolicy:(RTC_OBJC_TYPE(RTCTlsCertPolicy))tlsCertPolicy
                          hostname:(nullable NSString *)hostname
                  tlsAlpnProtocols:(nullable NSArray<NSString *> *)tlsAlpnProtocols
                 tlsEllipticCurves:(nullable NSArray<NSString *> *)tlsEllipticCurves
    NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
