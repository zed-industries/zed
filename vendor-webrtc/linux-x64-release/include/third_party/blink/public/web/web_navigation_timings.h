// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_NAVIGATION_TIMINGS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_NAVIGATION_TIMINGS_H_

#include "base/time/time.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-shared.h"
#include "third_party/blink/public/mojom/navigation/system_entropy.mojom-shared.h"

namespace blink {

struct WebNavigationTimings {
  base::TimeTicks input_start;
  base::TimeTicks navigation_start;
  base::TimeTicks redirect_start;
  base::TimeTicks redirect_end;
  base::TimeTicks fetch_start;
  blink::mojom::ParentResourceTimingAccess parent_resource_timing_access =
      blink::mojom::ParentResourceTimingAccess::kDoNotReport;
  blink::mojom::SystemEntropy system_entropy_at_navigation_start =
      blink::mojom::SystemEntropy::kEmpty;
  base::TimeTicks critical_ch_restart;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_NAVIGATION_TIMINGS_H_
