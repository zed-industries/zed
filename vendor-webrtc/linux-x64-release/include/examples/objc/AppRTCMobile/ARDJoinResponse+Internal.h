/*
 *  Copyright 2014 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "ARDJoinResponse.h"

@interface ARDJoinResponse ()

@property(nonatomic, assign) ARDJoinResultType result;
@property(nonatomic, assign) BOOL isInitiator;
@property(nonatomic, strong) NSString* roomId;
@property(nonatomic, strong) NSString* clientId;
@property(nonatomic, strong) NSArray* messages;
@property(nonatomic, strong) NSURL* webSocketURL;
@property(nonatomic, strong) NSURL* webSocketRestURL;

@end
