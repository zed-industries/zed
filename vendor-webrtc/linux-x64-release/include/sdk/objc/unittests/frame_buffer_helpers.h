/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <AVFoundation/AVFoundation.h>

#include "api/video/i420_buffer.h"

void DrawGradientInRGBPixelBuffer(CVPixelBufferRef pixelBuffer);

webrtc::scoped_refptr<webrtc::I420Buffer> CreateI420Gradient(int width,
                                                             int height);

void CopyI420BufferToCVPixelBuffer(
    webrtc::scoped_refptr<webrtc::I420Buffer> i420Buffer,
    CVPixelBufferRef pixelBuffer);
