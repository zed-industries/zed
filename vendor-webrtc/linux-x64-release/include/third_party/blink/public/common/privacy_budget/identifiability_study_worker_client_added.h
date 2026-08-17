// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_WORKER_CLIENT_ADDED_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_WORKER_CLIENT_ADDED_H_

#include "services/metrics/public/cpp/ukm_recorder.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_surface.h"

namespace blink {

class BLINK_COMMON_EXPORT IdentifiabilityStudyWorkerClientAdded {
 public:
  // Constructs an IdentifiabilityStudyWorkerClientAdded for the given SourceId.
  explicit IdentifiabilityStudyWorkerClientAdded(ukm::SourceId source_id);

  ~IdentifiabilityStudyWorkerClientAdded();

  // Record collected metrics to `recorder`.
  void Record(ukm::UkmRecorder* recorder);

  IdentifiabilityStudyWorkerClientAdded& SetClientSourceId(
      ukm::SourceId client_source_id);

  IdentifiabilityStudyWorkerClientAdded& SetWorkerType(
      blink::IdentifiableSurface::WorkerType worker_type);

 private:
  const ukm::SourceId source_id_;
  ukm::SourceId client_source_id_;
  blink::IdentifiableSurface::WorkerType worker_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_WORKER_CLIENT_ADDED_H_
