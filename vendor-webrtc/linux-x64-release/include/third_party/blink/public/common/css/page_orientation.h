// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_PAGE_ORIENTATION_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_PAGE_ORIENTATION_H_

namespace blink {

// CSS @page page-orientation descriptor values - to be used both in Blink and
// the printing implementation.
enum class PageOrientation { kUpright, kRotateLeft, kRotateRight };

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_PAGE_ORIENTATION_H_
