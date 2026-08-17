/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCCertificate) : NSObject <NSCopying>

/** Private key in PEM. */
@property(nonatomic, readonly, copy) NSString *private_key;

/** Public key in an x509 cert encoded in PEM. */
@property(nonatomic, readonly, copy) NSString *certificate;

/**
 * Initialize an RTCCertificate with PEM strings for private_key and
 * certificate.
 */
- (instancetype)initWithPrivateKey:(NSString *)private_key
                       certificate:(NSString *)certificate
    NS_DESIGNATED_INITIALIZER;

- (instancetype)init NS_UNAVAILABLE;

/** Generate a new certificate for 're' use.
 *
 *  Optional dictionary of parameters. Defaults to KeyType ECDSA if none are
 *  provided.
 *  - name: "ECDSA" or "RSASSA-PKCS1-v1_5"
 */
+ (nullable RTC_OBJC_TYPE(RTCCertificate) *)generateCertificateWithParams:
    (NSDictionary *)params;

@end

NS_ASSUME_NONNULL_END
