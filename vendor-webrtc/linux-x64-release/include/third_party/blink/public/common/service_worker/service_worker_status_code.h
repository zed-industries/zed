// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_STATUS_CODE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_STATUS_CODE_H_

#include "third_party/blink/public/common/common_export.h"

namespace blink {

// Generic service worker operation statuses.
// This enum is used in UMA histograms. Append-only.
enum class ServiceWorkerStatusCode {
  // Operation succeeded.
  kOk = 0,

  // Generic operation error (more specific error code should be used in
  // general).
  kErrorFailed = 1,

  // Operation was aborted (e.g. due to context or child process shutdown).
  kErrorAbort = 2,

  // Starting a new service worker script context failed.
  kErrorStartWorkerFailed = 3,

  // Could not find a renderer process to run a service worker.
  kErrorProcessNotFound = 4,

  // Generic error code to indicate the specified item is not found.
  kErrorNotFound = 5,

  // Generic error code to indicate the specified item already exists.
  kErrorExists = 6,

  // Install event handling failed.
  kErrorInstallWorkerFailed = 7,

  // Activate event handling failed.
  kErrorActivateWorkerFailed = 8,

  // Sending an IPC to the worker failed (often due to child process is
  // terminated).
  kErrorIpcFailed = 9,

  // Operation is failed by network issue.
  kErrorNetwork = 10,

  // Operation is failed by security issue.
  kErrorSecurity = 11,

  // Event handling failed (event.waitUntil Promise rejected).
  kErrorEventWaitUntilRejected = 12,

  // An error triggered by invalid worker state.
  kErrorState = 13,

  // The Service Worker took too long to finish a task.
  kErrorTimeout = 14,

  // An error occurred during initial script evaluation.
  kErrorScriptEvaluateFailed = 15,

  // Generic error to indicate failure to read/write the disk cache.
  kErrorDiskCache = 16,

  // The worker is in Redundant state.
  kErrorRedundant = 17,

  // The worker was disallowed (by ContentClient: e.g., due to
  // browser settings).
  kErrorDisallowed = 18,

  // Obsolete.
  // kErrorDisabledWorker = 19,

  // The arguments to call the API were invalid.
  kErrorInvalidArguments = 20,

  // The storage operation failed due to a Mojo connection error with the
  // Storage Service. This typically means the process hosting the Storage
  // Service has crashed.
  kErrorStorageDisconnected = 21,

  // The storage data is corrupted.
  kErrorStorageDataCorrupted = 22,

  // Add new status codes here and update kMaxValue and enums.xml. The next new
  // status code should be 23.

  // Note: kMaxValue is needed only for histograms.
  kMaxValue = kErrorStorageDataCorrupted,
};

BLINK_COMMON_EXPORT const char* ServiceWorkerStatusToString(
    ServiceWorkerStatusCode code);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_STATUS_CODE_H_
