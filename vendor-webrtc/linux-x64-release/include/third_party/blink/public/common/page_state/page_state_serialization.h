// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_STATE_PAGE_STATE_SERIALIZATION_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_STATE_PAGE_STATE_SERIALIZATION_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "base/containers/span.h"
#include "build/build_config.h"
#include "services/network/public/cpp/resource_request_body.h"
#include "services/network/public/mojom/referrer_policy.mojom.h"
#include "third_party/blink/public/mojom/page_state/page_state.mojom.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/point_f.h"
#include "url/gurl.h"
#include "url/origin.h"

namespace blink {

constexpr int kMaxScrollAnchorSelectorLength = 500;

struct BLINK_COMMON_EXPORT ExplodedHttpBody {
  std::optional<std::u16string> http_content_type;
  scoped_refptr<network::ResourceRequestBody> request_body;
  bool contains_passwords;

  ExplodedHttpBody();
  ~ExplodedHttpBody();
};

struct BLINK_COMMON_EXPORT ExplodedFrameState {
  // When adding a new member, also add it to ExplodedFrameState::assign.
  std::optional<std::u16string> url_string;
  std::optional<std::u16string> referrer;
  std::optional<url::Origin> initiator_origin;
  std::optional<std::u16string> initiator_base_url_string;
  std::optional<std::u16string> target;
  std::optional<std::u16string> state_object;
  std::vector<std::optional<std::u16string>> document_state;
  blink::mojom::ScrollRestorationType scroll_restoration_type =
      blink::mojom::ScrollRestorationType::kAuto;
  bool did_save_scroll_or_scale_state = true;
  gfx::PointF visual_viewport_scroll_offset;
  gfx::Point scroll_offset;
  int64_t item_sequence_number = 0;
  int64_t document_sequence_number = 0;
  double page_scale_factor = 0.f;
  network::mojom::ReferrerPolicy referrer_policy =
      network::mojom::ReferrerPolicy::kDefault;
  ExplodedHttpBody http_body;
  std::optional<std::u16string> scroll_anchor_selector;
  gfx::PointF scroll_anchor_offset;
  uint64_t scroll_anchor_simhash = 0;
  std::optional<std::u16string> navigation_api_key;
  std::optional<std::u16string> navigation_api_id;
  std::optional<std::u16string> navigation_api_state;
  bool protect_url_in_navigation_api = false;
  std::vector<ExplodedFrameState> children;

  ExplodedFrameState();
  ExplodedFrameState(const ExplodedFrameState& other);
  ~ExplodedFrameState();
  void operator=(const ExplodedFrameState& other);

 private:
  void assign(const ExplodedFrameState& other);
};

struct BLINK_COMMON_EXPORT ExplodedPageState {
  // TODO(creis, lukasza): Instead of storing them in |referenced_files|,
  // extract referenced files from ExplodedHttpBody.  |referenced_files|
  // currently contains a list from all frames, but cannot be deserialized into
  // the files referenced by each frame.  See http://crbug.com/441966.
  std::vector<std::optional<std::u16string>> referenced_files;
  ExplodedFrameState top;

  ExplodedPageState();
  ~ExplodedPageState();
};

BLINK_COMMON_EXPORT bool DecodePageState(const std::string& encoded,
                                         ExplodedPageState* exploded);
// Similar to |DecodePageState()|, but returns an int indicating the original
// version number of the encoded state. Returns -1 on failure.
BLINK_COMMON_EXPORT int DecodePageStateForTesting(const std::string& encoded,
                                                  ExplodedPageState* exploded);
BLINK_COMMON_EXPORT void EncodePageState(const ExplodedPageState& exploded,
                                         std::string* encoded);
BLINK_COMMON_EXPORT void LegacyEncodePageStateForTesting(
    const ExplodedPageState& exploded,
    int version,
    std::string* encoded);

#if BUILDFLAG(IS_ANDROID)
BLINK_COMMON_EXPORT bool DecodePageStateWithDeviceScaleFactorForTesting(
    const std::string& encoded,
    float device_scale_factor,
    ExplodedPageState* exploded);

// Converts results of EncodeResourceRequestBody (passed in as a pair of |data|
// + |size|) back into a ResourceRequestBody.  Returns nullptr if the
// decoding fails (e.g. if |data| is malformed).
BLINK_COMMON_EXPORT scoped_refptr<network::ResourceRequestBody>
DecodeResourceRequestBody(base::span<const uint8_t> data);

// Encodes |resource_request_body| into |encoded|.
BLINK_COMMON_EXPORT std::string EncodeResourceRequestBody(
    const network::ResourceRequestBody& resource_request_body);
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_STATE_PAGE_STATE_SERIALIZATION_H_
