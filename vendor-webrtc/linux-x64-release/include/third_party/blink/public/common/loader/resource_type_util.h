// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_RESOURCE_TYPE_UTIL_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_RESOURCE_TYPE_UTIL_H_

#include "services/network/public/mojom/fetch_api.mojom-shared.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/loader/resource_load_info.mojom-shared.h"

namespace blink {

BLINK_COMMON_EXPORT bool IsRequestDestinationFrame(
    network::mojom::RequestDestination destination);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_RESOURCE_TYPE_UTIL_H_
