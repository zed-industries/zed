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

@class ARDJoinResponse;
@class ARDMessageResponse;
@class ARDSignalingMessage;

@protocol ARDRoomServerClient <NSObject>

- (void)joinRoomWithRoomId:(NSString *)roomId
                isLoopback:(BOOL)isLoopback
         completionHandler:(void (^)(ARDJoinResponse *response,
                                     NSError *error))completionHandler;

- (void)sendMessage:(ARDSignalingMessage *)message
            forRoomId:(NSString *)roomId
             clientId:(NSString *)clientId
    completionHandler:(void (^)(ARDMessageResponse *response,
                                NSError *error))completionHandler;

- (void)leaveRoomWithRoomId:(NSString *)roomId
                   clientId:(NSString *)clientId
          completionHandler:(void (^)(NSError *error))completionHandler;

@end
