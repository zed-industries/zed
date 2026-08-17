// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DISTILLABILITY_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DISTILLABILITY_H_

namespace blink {

struct WebDistillabilityFeatures {
  bool is_mobile_friendly;
  // The rest of the fields are only valid when is_mobile_friendly==false.
  bool open_graph;
  unsigned element_count;
  unsigned anchor_count;
  unsigned form_count;
  unsigned text_input_count;
  unsigned password_input_count;
  unsigned p_count;
  unsigned pre_count;
  // The following scores are derived from the triggering logic in Readability
  // from Mozilla.
  // https://github.com/mozilla/readability/blob/85101066386a0872526a6c4ae164c18fcd6cc1db/Readability.js#L1704
  double moz_score;
  double moz_score_all_sqrt;
  double moz_score_all_linear;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DISTILLABILITY_H_
