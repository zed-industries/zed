// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_CAPTURE_TASK_HISTOGRAM_REPORTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_CAPTURE_TASK_HISTOGRAM_REPORTER_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/instrumentation/histogram.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace blink {

// This class collects and reports metric data for the ContentCaptureTask.
class CORE_EXPORT ContentCaptureTaskHistogramReporter
    : public RefCounted<ContentCaptureTaskHistogramReporter> {
 public:
  // Visible for testing.
  static constexpr char kCaptureContentTime[] =
      "ContentCapture.CaptureContentTime2";
  static constexpr char kSentContentCount[] =
      "ContentCapture.SentContentCount2";

  ContentCaptureTaskHistogramReporter();
  ~ContentCaptureTaskHistogramReporter();

  // Invoked on a capturing session starts, a session begins with
  // OnCaptureContentStarted(), ends with OnAllCaptureContentSent().
  void OnCaptureContentStarted();
  // Invoked on the on-screen content captured and ready to be sent out.
  void OnCaptureContentEnded(size_t captured_content_count);

  void RecordsSentContentCountPerDocument(int sent_content_count);

 private:
  // The time to start capturing content.
  base::TimeTicks capture_content_start_time_;

  // Records time to capture the content, its range is from 0 to 50,000
  // microseconds.
  CustomCountHistogram capture_content_time_histogram_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CONTENT_CAPTURE_CONTENT_CAPTURE_TASK_HISTOGRAM_REPORTER_H_
