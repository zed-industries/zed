/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <ReplayKit/ReplayKit.h>

#import "sdk/objc/base/RTCLogging.h"

#import "ARDAppClient.h"

@protocol ARDExternalSampleDelegate;

API_AVAILABLE(ios(10.0))
@interface ARDBroadcastSampleHandler
    : RPBroadcastSampleHandler <ARDAppClientDelegate>

@property(nonatomic, strong) id<ARDExternalSampleDelegate> capturer;

@end
