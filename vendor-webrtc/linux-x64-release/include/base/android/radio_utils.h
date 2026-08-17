// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_RADIO_UTILS_H_
#define BASE_ANDROID_RADIO_UTILS_H_

#include <optional>

#include "base/android/jni_android.h"

namespace base {
namespace android {

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused. Keep in sync with RadioSignalLevel
// in //tools/metrics/histograms/enums.xml.
enum class RadioSignalLevel {
  kNoneOrUnknown = 0,
  kPoor = 1,
  kModerate = 2,
  kGood = 3,
  kGreat = 4,
  kMaxValue = kGreat,
};

enum class RadioDataActivity {
  kNone = 0,
  kIn = 1,
  kOut = 2,
  kInOut = 3,
  kDormant = 4,
};

enum class RadioConnectionType {
  kUnknown = 0,
  kWifi = 1,
  kCell = 2,
};

class BASE_EXPORT RadioUtils {
 public:
  class OverrideForTesting {
   public:
    OverrideForTesting();
    ~OverrideForTesting();

    void SetConnectionTypeForTesting(RadioConnectionType connection_type) {
      connection_type_ = connection_type;
    }

    RadioConnectionType GetConnectionType() { return connection_type_; }

   private:
    RadioConnectionType connection_type_;
  };
  static bool IsSupported();
  static RadioConnectionType GetConnectionType();
  static std::optional<RadioSignalLevel> GetCellSignalLevel();
  static std::optional<RadioDataActivity> GetCellDataActivity();
};

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_RADIO_UTILS_H_
