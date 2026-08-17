/*
 * Copyright 2025 LiveKit
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
 */

#import <Foundation/Foundation.h>

#import "RTCMacros.h"

#import "RTCFrameCryptor.h"

NS_ASSUME_NONNULL_BEGIN

@class RTC_OBJC_TYPE(RTCFrameCryptorKeyProvider);

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCEncryptedPacket) : NSObject

@property(nonatomic, readonly) NSData *data;

@property(nonatomic, readonly) NSData *iv;

@property(nonatomic, readonly) uint32_t keyIndex;

- (instancetype)initWithData:(NSData *)data iv:(NSData *)iv keyIndex:(uint32_t)keyIndex;

@end

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCDataPacketCryptor) : NSObject

- (nullable instancetype)initWithAlgorithm:(RTC_OBJC_TYPE(RTCCryptorAlgorithm))algorithm
                               keyProvider:(RTC_OBJC_TYPE(RTCFrameCryptorKeyProvider) *)keyProvider;

- (nullable RTC_OBJC_TYPE(RTCEncryptedPacket) *)encrypt:(NSString*)participantId keyIndex:(uint32_t)keyIndex data:(NSData *)data;

- (nullable NSData *)decrypt:(NSString*)participantId encryptedPacket:(RTC_OBJC_TYPE(RTCEncryptedPacket) *)packet;

@end

NS_ASSUME_NONNULL_END
