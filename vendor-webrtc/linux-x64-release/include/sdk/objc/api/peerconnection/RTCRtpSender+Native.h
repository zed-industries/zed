/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCRtpSender.h"

#include "api/crypto/frame_encryptor_interface.h"
#include "api/scoped_refptr.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * This class extension exposes methods that work directly with injectable C++
 * components.
 */
@interface RTC_OBJC_TYPE (RTCRtpSender)
()

    /** Sets a defined frame encryptor that will encrypt the entire frame
     * before it is sent across the network. This will encrypt the entire frame
     * using the user provided encryption mechanism regardless of whether SRTP
     * is enabled or not.
     */
    - (void)setFrameEncryptor
    : (webrtc::scoped_refptr<webrtc::FrameEncryptorInterface>)frameEncryptor;

@end

NS_ASSUME_NONNULL_END
