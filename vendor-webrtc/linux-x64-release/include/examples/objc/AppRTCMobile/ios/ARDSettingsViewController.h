/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <UIKit/UIKit.h>

@class ARDSettingsModel;

NS_ASSUME_NONNULL_BEGIN
/**
 * Displays settings options.
 */
@interface ARDSettingsViewController : UITableViewController

/**
 * Creates new instance.
 *
 * @param style the table view style that should be used
 * @param settingsModel model class for the user settings.
 */
- (instancetype)initWithStyle:(UITableViewStyle)style
                settingsModel:(ARDSettingsModel *)settingsModel;

#pragma mark - Unavailable

- (instancetype)initWithStyle:(UITableViewStyle)style NS_UNAVAILABLE;
- (instancetype)init NS_UNAVAILABLE;
+ (instancetype)new NS_UNAVAILABLE;

@end
NS_ASSUME_NONNULL_END
