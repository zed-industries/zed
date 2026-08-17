// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_IDENTIFIABILITY_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_IDENTIFIABILITY_METRICS_H_

#include "third_party/blink/public/common/privacy_budget/identifiable_surface.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token_builder.h"

namespace blink {

class ExecutionContext;
class MediaStreamConstraints;

IdentifiableToken TokenFromConstraints(
    const MediaStreamConstraints* constraints);

void RecordIdentifiabilityMetric(const IdentifiableSurface& surface,
                                 ExecutionContext* context,
                                 IdentifiableToken token);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_IDENTIFIABILITY_METRICS_H_
