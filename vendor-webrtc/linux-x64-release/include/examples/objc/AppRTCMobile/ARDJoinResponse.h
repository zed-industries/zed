/*
 *  Copyright 2014 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

typedef NS_ENUM(NSInteger, ARDJoinResultType) {
  kARDJoinResultTypeUnknown,
  kARDJoinResultTypeSuccess,
  kARDJoinResultTypeFull
};

// Result of joining a room on the room server.
@interface ARDJoinResponse : NSObject

@property(nonatomic, readonly) ARDJoinResultType result;
@property(nonatomic, readonly) BOOL isInitiator;
@property(nonatomic, readonly) NSString *roomId;
@property(nonatomic, readonly) NSString *clientId;
@property(nonatomic, readonly) NSArray *messages;
@property(nonatomic, readonly) NSURL *webSocketURL;
@property(nonatomic, readonly) NSURL *webSocketRestURL;

+ (ARDJoinResponse *)responseFromJSONData:(NSData *)data;

@end
