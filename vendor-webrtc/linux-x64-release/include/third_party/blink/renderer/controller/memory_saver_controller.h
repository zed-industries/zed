// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_MEMORY_SAVER_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_MEMORY_SAVER_CONTROLLER_H_

#include "base/timer/timer.h"
#include "third_party/blink/renderer/controller/controller_export.h"

namespace blink {

// Tracks available memory to toggle memory saver mode on v8 isolates.
class CONTROLLER_EXPORT MemorySaverController {
 public:
  static void Initialize();

 private:
  MemorySaverController();

  void Sample();
  void SetMemorySaverModeForAllIsolates(bool memory_saver_mode_enabled);

  base::RepeatingTimer sample_timer_;
  bool memory_saver_enabled_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_MEMORY_SAVER_CONTROLLER_H_
