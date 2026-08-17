// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_EXTENDED_SERVICE_WORKER_STATUS_CODE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_EXTENDED_SERVICE_WORKER_STATUS_CODE_H_

namespace blink {

// Generic service worker operation statuses.
// This enum is used in UMA histograms. Append-only.
enum class ExtendedServiceWorkerStatusCode {

  // A placeholder value for when an extended status code cannot be determined.
  kUnknown = 0,

  // TODO(crbug.com/346732739): Implement the remaining extended status codes.

  kMaxValue = kUnknown,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_EXTENDED_SERVICE_WORKER_STATUS_CODE_H_
