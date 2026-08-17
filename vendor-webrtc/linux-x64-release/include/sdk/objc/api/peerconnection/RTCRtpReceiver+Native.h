/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCRtpReceiver.h"

#include "api/crypto/frame_decryptor_interface.h"
#include "api/scoped_refptr.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * This class extension exposes methods that work directly with injectable C++
 * components.
 */
@interface RTC_OBJC_TYPE (RTCRtpReceiver)
()

    /** Sets a user defined frame decryptor that will decrypt the entire frame.
     * This will decrypt the entire frame using the user provided decryption
     * mechanism regardless of whether SRTP is enabled or not.
     */
    - (void)setFrameDecryptor
    : (webrtc::scoped_refptr<webrtc::FrameDecryptorInterface>)frameDecryptor;

@end

NS_ASSUME_NONNULL_END
