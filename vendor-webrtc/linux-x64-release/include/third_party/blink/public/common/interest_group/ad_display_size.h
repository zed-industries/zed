// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_DISPLAY_SIZE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_DISPLAY_SIZE_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/interest_group/ad_display_size.mojom-shared.h"
#include "url/gurl.h"

namespace blink {

struct BLINK_COMMON_EXPORT AdSize {
  using LengthUnit = blink::mojom::AdSize_LengthUnit;

  AdSize();
  explicit AdSize(double width,
                  LengthUnit width_units,
                  double height,
                  LengthUnit height_units);
  AdSize(const AdSize&);
  AdSize(AdSize&&);
  AdSize& operator=(const AdSize&);
  AdSize& operator=(AdSize&&);
  bool operator==(const AdSize& other) const;
  bool operator!=(const AdSize& other) const;
  bool operator<(const AdSize& other) const;
  ~AdSize();

  double width;
  LengthUnit width_units;

  double height;
  LengthUnit height_units;
};

struct BLINK_COMMON_EXPORT AdDescriptor {
  AdDescriptor();
  explicit AdDescriptor(GURL url, std::optional<AdSize> size = std::nullopt);
  AdDescriptor(const AdDescriptor&);
  AdDescriptor(AdDescriptor&&);
  AdDescriptor& operator=(const AdDescriptor&);
  AdDescriptor& operator=(AdDescriptor&&);
  bool operator==(const AdDescriptor&) const;
  ~AdDescriptor();

  bool operator<(const AdDescriptor& other) const;

  GURL url;
  std::optional<AdSize> size;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_DISPLAY_SIZE_H_
