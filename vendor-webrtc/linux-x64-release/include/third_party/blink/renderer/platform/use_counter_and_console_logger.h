// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_USE_COUNTER_AND_CONSOLE_LOGGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_USE_COUNTER_AND_CONSOLE_LOGGER_H_

#include "third_party/blink/renderer/platform/instrumentation/use_counter.h"
#include "third_party/blink/renderer/platform/loader/fetch/console_logger.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// An abstract class that wraps `UseCounter` and `ConsoleLogger`.
class PLATFORM_EXPORT UseCounterAndConsoleLogger : public UseCounter,
                                                   public ConsoleLogger {
 public:
  ~UseCounterAndConsoleLogger() override = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_USE_COUNTER_AND_CONSOLE_LOGGER_H_
