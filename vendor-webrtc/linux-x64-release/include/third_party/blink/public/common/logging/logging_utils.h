// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOGGING_LOGGING_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOGGING_LOGGING_UTILS_H_

#include "base/logging.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-forward.h"

namespace blink {

logging::LogSeverity BLINK_COMMON_EXPORT
ConsoleMessageLevelToLogSeverity(blink::mojom::ConsoleMessageLevel level);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOGGING_LOGGING_UTILS_H_
