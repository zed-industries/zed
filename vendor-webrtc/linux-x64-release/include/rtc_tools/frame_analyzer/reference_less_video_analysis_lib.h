/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_FRAME_ANALYZER_REFERENCE_LESS_VIDEO_ANALYSIS_LIB_H_
#define RTC_TOOLS_FRAME_ANALYZER_REFERENCE_LESS_VIDEO_ANALYSIS_LIB_H_

#include <stddef.h>

#include <string>
#include <vector>

#include "api/scoped_refptr.h"
#include "rtc_tools/video_file_reader.h"

// Returns true if the frame is frozen based on psnr and ssim freezing
// threshold values.
bool frozen_frame(std::vector<double> psnr_per_frame,
                  std::vector<double> ssim_per_frame,
                  size_t frame);

// Returns the vector of identical cluster of frames that are frozen
// and appears continuously.
std::vector<int> find_frame_clusters(const std::vector<double>& psnr_per_frame,
                                     const std::vector<double>& ssim_per_frame);

// Prints various freezing metrics like identical frames,
// total unique frames etc.
void print_freezing_metrics(const std::vector<double>& psnr_per_frame,
                            const std::vector<double>& ssim_per_frame);

// Compute the metrics like freezing score based on PSNR and SSIM values for a
// given video file.
void compute_metrics(const webrtc::scoped_refptr<webrtc::test::Video>& video,
                     std::vector<double>* psnr_per_frame,
                     std::vector<double>* ssim_per_frame);

// Compute freezing score metrics and prints the metrics
// for a list of video files.
int run_analysis(const std::string& video_file);

#endif  // RTC_TOOLS_FRAME_ANALYZER_REFERENCE_LESS_VIDEO_ANALYSIS_LIB_H_
