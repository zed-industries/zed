// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_DOCUMENT_CREATED_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_DOCUMENT_CREATED_H_

#include "services/metrics/public/cpp/ukm_recorder.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

class BLINK_COMMON_EXPORT IdentifiabilityStudyDocumentCreated {
 public:
  // Construct an IdentifiabilityStudyDocumentCreated for the given |source_id|.
  // The source must be known to UKM.
  explicit IdentifiabilityStudyDocumentCreated(ukm::SourceIdObj source_id);

  // Same as previous constructor, but uses SourceId instead of SourceIdObj.
  explicit IdentifiabilityStudyDocumentCreated(ukm::SourceId source_id);

  ~IdentifiabilityStudyDocumentCreated();

  // Record collected metrics to `recorder`.
  void Record(ukm::UkmRecorder* recorder);

  IdentifiabilityStudyDocumentCreated& SetNavigationSourceId(
      ukm::SourceId navigation_source_id);

  IdentifiabilityStudyDocumentCreated& SetIsMainFrame(bool is_main_frame);

  IdentifiabilityStudyDocumentCreated& SetIsCrossOriginFrame(
      bool is_cross_origin_frame);

  IdentifiabilityStudyDocumentCreated& SetIsCrossSiteFrame(
      bool is_cross_site_frame);

 private:
  const ukm::SourceId source_id_;
  ukm::SourceId navigation_source_id_;
  bool is_main_frame_;
  bool is_cross_origin_frame_;
  bool is_cross_site_frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_DOCUMENT_CREATED_H_
