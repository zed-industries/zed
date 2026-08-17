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

@class ARDMainView;

@protocol ARDMainViewDelegate <NSObject>

- (void)mainView:(ARDMainView *)mainView
    didInputRoom:(NSString *)room
      isLoopback:(BOOL)isLoopback;
- (void)mainViewDidToggleAudioLoop:(ARDMainView *)mainView;

@end

// The main view of AppRTCMobile. It contains an input field for entering a room
// name on apprtc to connect to.
@interface ARDMainView : UIView

@property(nonatomic, weak) id<ARDMainViewDelegate> delegate;
// Updates the audio loop button as needed.
@property(nonatomic, assign) BOOL isAudioLoopPlaying;

@end
