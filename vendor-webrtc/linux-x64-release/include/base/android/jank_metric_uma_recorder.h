// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_JANK_METRIC_UMA_RECORDER_H_
#define BASE_ANDROID_JANK_METRIC_UMA_RECORDER_H_

#include "base/android/jni_android.h"
#include "base/base_export.h"
#include "base/feature_list.h"

namespace base::android {

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class FrameJankStatus {
  kJanky = 0,
  kNonJanky = 1,
  kMaxValue = kNonJanky,
};

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class JankScenario {
  PERIODIC_REPORTING = 1,
  OMNIBOX_FOCUS = 2,
  NEW_TAB_PAGE = 3,
  STARTUP = 4,
  TAB_SWITCHER = 5,
  OPEN_LINK_IN_NEW_TAB = 6,
  START_SURFACE_HOMEPAGE = 7,
  START_SURFACE_TAB_SWITCHER = 8,
  FEED_SCROLLING = 9,
  WEBVIEW_SCROLLING = 10,
  COMBINED_WEBVIEW_SCROLLING = 11,
  // This value should always be last and is not persisted to logs, exposed only
  // for testing.
  MAX_VALUE = COMBINED_WEBVIEW_SCROLLING + 1
};

// Resolves the above name to a histogram value.
BASE_EXPORT const char* GetAndroidFrameTimelineJankHistogramName(
    JankScenario scenario);
// Resolves the above name to a histogram value.
BASE_EXPORT const char* GetAndroidFrameTimelineDurationHistogramName(
    JankScenario scenario);

BASE_EXPORT void RecordJankMetrics(
    JNIEnv* env,
    const base::android::JavaParamRef<jlongArray>& java_durations_ns,
    const base::android::JavaParamRef<jintArray>& java_missed_vsyncs,
    jlong java_reporting_interval_start_time,
    jlong java_reporting_interval_duration,
    jint java_scenario_enum);
}  // namespace base::android
#endif  // BASE_ANDROID_JANK_METRIC_UMA_RECORDER_H_
