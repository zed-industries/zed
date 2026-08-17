/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCVideoCodecInfo.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Model class for user defined settings.
 *
 * Handles storing the settings and provides default values if setting is not
 * set. Also provides list of available options for different settings. Stores
 * for example video codec, video resolution and maximum bitrate.
 */
@interface ARDSettingsModel : NSObject

/**
 * Returns array of available capture resoultions.
 *
 * The capture resolutions are represented as strings in the following format
 * [width]x[height]
 */
- (NSArray<NSString *> *)availableVideoResolutions;

/**
 * Returns current video resolution string.
 * If no resolution is in store, default value of 640x480 is returned.
 * When defaulting to value, the default is saved in store for consistency
 * reasons.
 */
- (NSString *)currentVideoResolutionSettingFromStore;
- (int)currentVideoResolutionWidthFromStore;
- (int)currentVideoResolutionHeightFromStore;

/**
 * Stores the provided video resolution string into the store.
 *
 * If the provided resolution is no part of the available video resolutions
 * the store operation will not be executed and NO will be returned.
 * @param resolution the string to be stored.
 * @return YES/NO depending on success.
 */
- (BOOL)storeVideoResolutionSetting:(NSString *)resolution;

/**
 * Returns array of available video codecs.
 */
- (NSArray<RTC_OBJC_TYPE(RTCVideoCodecInfo) *> *)availableVideoCodecs;

/**
 * Returns current video codec setting from store if present or default (H264)
 * otherwise.
 */
- (RTC_OBJC_TYPE(RTCVideoCodecInfo) *)currentVideoCodecSettingFromStore;

/**
 * Stores the provided video codec setting into the store.
 *
 * If the provided video codec is not part of the available video codecs
 * the store operation will not be executed and NO will be returned.
 * @param video codec settings the string to be stored.
 * @return YES/NO depending on success.
 */
- (BOOL)storeVideoCodecSetting:(RTC_OBJC_TYPE(RTCVideoCodecInfo) *)videoCodec;

/**
 * Returns current max bitrate setting from store if present.
 */
- (nullable NSNumber *)currentMaxBitrateSettingFromStore;

/**
 * Stores the provided bitrate value into the store.
 *
 * @param bitrate NSNumber representation of the max bitrate value.
 */
- (void)storeMaxBitrateSetting:(nullable NSNumber *)bitrate;

/**
 * Returns current audio only setting from store if present or default (NO)
 * otherwise.
 */
- (BOOL)currentAudioOnlySettingFromStore;

/**
 * Stores the provided audio only setting into the store.
 *
 * @param setting the boolean value to be stored.
 */
- (void)storeAudioOnlySetting:(BOOL)audioOnly;

/**
 * Returns current create AecDump setting from store if present or default (NO)
 * otherwise.
 */
- (BOOL)currentCreateAecDumpSettingFromStore;

/**
 * Stores the provided create AecDump setting into the store.
 *
 * @param setting the boolean value to be stored.
 */
- (void)storeCreateAecDumpSetting:(BOOL)createAecDump;

/**
 * Returns current setting whether to use manual audio config from store if
 * present or default (YES) otherwise.
 */
- (BOOL)currentUseManualAudioConfigSettingFromStore;

/**
 * Stores the provided use manual audio config setting into the store.
 *
 * @param setting the boolean value to be stored.
 */
- (void)storeUseManualAudioConfigSetting:(BOOL)useManualAudioConfig;

@end
NS_ASSUME_NONNULL_END
