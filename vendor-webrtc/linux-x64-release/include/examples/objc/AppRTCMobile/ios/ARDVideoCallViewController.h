/*
 *  Copyright 2015 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <UIKit/UIKit.h>

@class ARDVideoCallViewController;
@protocol ARDVideoCallViewControllerDelegate <NSObject>

- (void)viewControllerDidFinish:(ARDVideoCallViewController *)viewController;

@end

@interface ARDVideoCallViewController : UIViewController

@property(nonatomic, weak) id<ARDVideoCallViewControllerDelegate> delegate;

- (instancetype)initForRoom:(NSString *)room
                 isLoopback:(BOOL)isLoopback
                   delegate:(id<ARDVideoCallViewControllerDelegate>)delegate;

@end
