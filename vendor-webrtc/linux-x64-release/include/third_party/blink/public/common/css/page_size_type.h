// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_PAGE_SIZE_TYPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_PAGE_SIZE_TYPE_H_

namespace blink {

// Use for passing information about CSS @page size descriptor for a given page
// between Blink and the printing implementation.
enum class PageSizeType {
  // Explicit or implicit @page { size: auto }.
  kAuto,
  // size:portrait with no specified size.
  kPortrait,
  // size:landscape with no specified size.
  kLandscape,
  // Page styled with a specified size.
  kFixed,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_PAGE_SIZE_TYPE_H_
