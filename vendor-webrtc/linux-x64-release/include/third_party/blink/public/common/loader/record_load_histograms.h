// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_RECORD_LOAD_HISTOGRAMS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_RECORD_LOAD_HISTOGRAMS_H_

#include "services/network/public/mojom/fetch_api.mojom-shared.h"
#include "third_party/blink/public/common/common_export.h"
#include "url/origin.h"

namespace blink {

// Logs histograms when a resource destined for a renderer (One with a
// network::mojom::RequestDestination) finishes loading, or when a load is
// aborted. Not used for internal network requests initiated by the browser
// itself.
BLINK_COMMON_EXPORT void RecordLoadHistograms(
    const url::Origin& origin,
    network::mojom::RequestDestination destination,
    int net_error);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_RECORD_LOAD_HISTOGRAMS_H_
