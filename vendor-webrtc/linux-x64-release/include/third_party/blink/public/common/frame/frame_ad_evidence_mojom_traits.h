// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_AD_EVIDENCE_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_AD_EVIDENCE_MOJOM_TRAITS_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/frame/frame_ad_evidence.h"
#include "third_party/blink/public/mojom/ad_tagging/ad_evidence.mojom-shared.h"

namespace mojo {

template <>
class BLINK_COMMON_EXPORT StructTraits<blink::mojom::FrameAdEvidenceDataView,
                                       blink::FrameAdEvidence> {
 public:
  static bool is_complete(const blink::FrameAdEvidence& e) {
    return e.is_complete();
  }
  static bool parent_is_ad(const blink::FrameAdEvidence& e) {
    return e.parent_is_ad();
  }
  static blink::mojom::FilterListResult latest_filter_list_result(
      const blink::FrameAdEvidence& e) {
    return e.latest_filter_list_result();
  }
  static blink::mojom::FilterListResult most_restrictive_filter_list_result(
      const blink::FrameAdEvidence& e) {
    return e.most_restrictive_filter_list_result();
  }
  static blink::mojom::FrameCreationStackEvidence created_by_ad_script(
      const blink::FrameAdEvidence& e) {
    return e.created_by_ad_script();
  }

  static bool Read(blink::mojom::FrameAdEvidenceDataView data,
                   blink::FrameAdEvidence* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_AD_EVIDENCE_MOJOM_TRAITS_H_
