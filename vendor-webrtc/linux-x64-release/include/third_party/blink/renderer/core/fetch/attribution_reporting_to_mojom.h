// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_ATTRIBUTION_REPORTING_TO_MOJOM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_ATTRIBUTION_REPORTING_TO_MOJOM_H_

#include "services/network/public/mojom/attribution.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class AttributionReportingRequestOptions;
class ExceptionState;
class ExecutionContext;

// Converts an IDL AttributionReportingRequestOptions to its Mojo counterpart.
// Throws an exception if the execution context doesn't allow Attribution
// Reporting in its permission policy.
CORE_EXPORT
network::mojom::AttributionReportingEligibility
ConvertAttributionReportingRequestOptionsToMojom(
    const AttributionReportingRequestOptions&,
    const ExecutionContext&,
    ExceptionState&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_ATTRIBUTION_REPORTING_TO_MOJOM_H_
