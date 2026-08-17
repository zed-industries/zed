/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_TOOLS_VIDEO_FILE_WRITER_H_
#define RTC_TOOLS_VIDEO_FILE_WRITER_H_

#include <string>

#include "api/scoped_refptr.h"
#include "rtc_tools/video_file_reader.h"

namespace webrtc {
namespace test {

// Writes video to file, determining YUV or Y4M format from the file extension.
void WriteVideoToFile(const scoped_refptr<Video>& video,
                      const std::string& file_name,
                      int fps);

// Writes Y4M video to file.
void WriteY4mVideoToFile(const scoped_refptr<Video>& video,
                         const std::string& file_name,
                         int fps);

// Writes YUV video to file.
void WriteYuvVideoToFile(const scoped_refptr<Video>& video,
                         const std::string& file_name,
                         int fps);

}  // namespace test
}  // namespace webrtc

#endif  // RTC_TOOLS_VIDEO_FILE_WRITER_H_
