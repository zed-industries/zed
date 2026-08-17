// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_EMBEDDED_WORKER_STATUS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_EMBEDDED_WORKER_STATUS_H_

namespace blink {

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class EmbeddedWorkerStatus {
  kStopped = 0,
  kStarting = 1,
  kRunning = 2,
  kStopping = 3,
  kMaxValue = kStopping,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_EMBEDDED_WORKER_STATUS_H_
