/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
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
@interface RTC_OBJC_TYPE (RTCPeerConnectionFactoryOptions) : NSObject

@property(nonatomic, assign) BOOL disableEncryption;

@property(nonatomic, assign) BOOL disableNetworkMonitor;

@property(nonatomic, assign) BOOL ignoreLoopbackNetworkAdapter;

@property(nonatomic, assign) BOOL ignoreVPNNetworkAdapter;

@property(nonatomic, assign) BOOL ignoreCellularNetworkAdapter;

@property(nonatomic, assign) BOOL ignoreWiFiNetworkAdapter;

@property(nonatomic, assign) BOOL ignoreEthernetNetworkAdapter;

- (instancetype)init NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
