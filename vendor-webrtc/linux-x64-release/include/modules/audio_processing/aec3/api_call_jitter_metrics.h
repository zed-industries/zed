/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_API_CALL_JITTER_METRICS_H_
#define MODULES_AUDIO_PROCESSING_AEC3_API_CALL_JITTER_METRICS_H_

namespace webrtc {

// Stores data for reporting metrics on the API call jitter.
class ApiCallJitterMetrics {
 public:
  class Jitter {
   public:
    Jitter();
    void Update(int num_api_calls_in_a_row);
    void Reset();

    int min() const { return min_; }
    int max() const { return max_; }

   private:
    int max_;
    int min_;
  };

  ApiCallJitterMetrics() { Reset(); }

  // Update metrics for render API call.
  void ReportRenderCall();

  // Update and periodically report metrics for capture API call.
  void ReportCaptureCall();

  // Methods used only for testing.
  const Jitter& render_jitter() const { return render_jitter_; }
  const Jitter& capture_jitter() const { return capture_jitter_; }
  bool WillReportMetricsAtNextCapture() const;

 private:
  void Reset();

  Jitter render_jitter_;
  Jitter capture_jitter_;

  int num_api_calls_in_a_row_ = 0;
  int frames_since_last_report_ = 0;
  bool last_call_was_render_ = false;
  bool proper_call_observed_ = false;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_API_CALL_JITTER_METRICS_H_
