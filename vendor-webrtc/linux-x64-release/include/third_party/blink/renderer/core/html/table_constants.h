// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TABLE_CONSTANTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TABLE_CONSTANTS_H_

namespace blink {

// https://html.spec.whatwg.org/C/#dom-colgroup-span
// https://html.spec.whatwg.org/C/#dom-col-span
// https://html.spec.whatwg.org/C/#dom-tdth-colspan
constexpr unsigned kDefaultColSpan = 1u;
constexpr unsigned kMinColSpan = 1u;
constexpr unsigned kMaxColSpan = 1000u;

// https://html.spec.whatwg.org/C/#dom-tdth-rowspan
constexpr unsigned kDefaultRowSpan = 1u;
constexpr unsigned kMaxRowSpan = 65534u;
constexpr unsigned kMinRowSpan = 0;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TABLE_CONSTANTS_H_
