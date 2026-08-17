// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_AD_EVIDENCE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_AD_EVIDENCE_H_

#include "base/check_op.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/ad_tagging/ad_evidence.mojom-shared.h"

namespace blink {

// Returns the stricter of the two load policies, as determined by the order
// of the blink::mojom::FilterListResult enum. Should mirror
// `LoadPolicy::MoreRestrictiveLoadPolicy()`.
mojom::FilterListResult MoreRestrictiveFilterListEvidence(
    mojom::FilterListResult a,
    mojom::FilterListResult b);

// Enumeration of evidence for or against a frame being an ad.
class BLINK_COMMON_EXPORT FrameAdEvidence {
 public:
  explicit FrameAdEvidence(bool parent_is_ad = false);

  FrameAdEvidence(const FrameAdEvidence&);

  ~FrameAdEvidence();

  // Returns whether the fields indicate that the corresponding frame is an ad
  // or not. Should only be called once `is_complete()`.
  bool IndicatesAdFrame() const;

  // Indicates whether the fields on the class are ready to be used for
  // calculation. If false, some fields might represent defaults rather than the
  // truth. Once set (as true), this will not change further. For example, this
  // bit should not be set during an initial navigation while waiting on an IPC
  // message that might change one of the fields from its default value. Once it
  // we know that no more updates will occur for the navigation,
  // `set_is_complete()` should be called.
  bool is_complete() const { return is_complete_; }
  void set_is_complete() { is_complete_ = true; }

  bool parent_is_ad() const { return parent_is_ad_; }

  mojom::FilterListResult latest_filter_list_result() const {
    return latest_filter_list_result_;
  }
  mojom::FilterListResult most_restrictive_filter_list_result() const {
    return most_restrictive_filter_list_result_;
  }

  // Updates the latest filter list result and, if necessary, the most
  // restrictive filter list result as well.
  void UpdateFilterListResult(mojom::FilterListResult value);

  mojom::FrameCreationStackEvidence created_by_ad_script() const {
    return created_by_ad_script_;
  }

  // Should not be called once `is_complete()`.
  void set_created_by_ad_script(mojom::FrameCreationStackEvidence value) {
    DCHECK(!is_complete_);
    DCHECK_LE(created_by_ad_script_, value);
    created_by_ad_script_ = value;
  }

 private:
  // See `is_complete()`.
  bool is_complete_ = false;

  // Whether the frame's parent is an ad. Not const to allow copy assignment.
  // Note, for embedded main frames that are a subresource filter child (e.g.
  // FencedFrame), this will specify if the outer delegate frame is an ad.
  bool parent_is_ad_;

  // Whether any URL for this frame has been checked against the filter list
  // and, if so, the result of the latest lookup. This is set once the filter
  // list evaluates a frame url, or it is known a frame will not consult the
  // the filter list (and has never done so yet).
  // TODO(crbug.com/1148058): Update to only include load policies from
  // navigations that commit.
  mojom::FilterListResult latest_filter_list_result_ =
      mojom::FilterListResult::kNotChecked;

  // The most restrictive value of `latest_filter_list_result_` ever set. This
  // tracks whether any URL for this frame has been checked against the filter
  // list and, if so, the most restrictive result of any lookup.
  mojom::FilterListResult most_restrictive_filter_list_result_ =
      mojom::FilterListResult::kNotChecked;

  // Whether ad script was on the v8 stack at the time this frame was created.
  mojom::FrameCreationStackEvidence created_by_ad_script_ =
      mojom::FrameCreationStackEvidence::kNotCreatedByAdScript;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_AD_EVIDENCE_H_
