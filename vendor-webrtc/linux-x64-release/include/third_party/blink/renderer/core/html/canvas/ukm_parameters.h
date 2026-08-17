// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_UKM_PARAMETERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_UKM_PARAMETERS_H_

#include "services/metrics/public/cpp/ukm_recorder.h"
#include "services/metrics/public/cpp/ukm_source_id.h"

namespace blink {

struct UkmParameters {
  ukm::UkmRecorder* ukm_recorder;
  ukm::SourceId source_id;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_UKM_PARAMETERS_H_
