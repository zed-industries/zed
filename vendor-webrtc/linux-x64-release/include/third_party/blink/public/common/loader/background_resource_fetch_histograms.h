// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_BACKGROUND_RESOURCE_FETCH_HISTOGRAMS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_BACKGROUND_RESOURCE_FETCH_HISTOGRAMS_H_

namespace blink {

// Following histogram name and the values are in the blink/public because
// they are used in the browser tests under content/.
static constexpr char kBackgroundResourceFetchSupportStatusHistogramName[] =
    "Blink.ResourceRequest.BackgroundResourceFetchSupportStatus";

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class BackgroundResourceFetchSupportStatus {
  kSupported = 0,
  kUnsupportedSyncRequest = 1,
  kUnsupportedNonGetRequest = 2,
  kUnsupportedNonHttpUrlRequest = 3,
  kUnsupportedKeepAliveRequest = 4,
  kUnsupportedPrefetchOnlyDocument = 5,
  kMaxValue = kUnsupportedPrefetchOnlyDocument,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_BACKGROUND_RESOURCE_FETCH_HISTOGRAMS_H_
