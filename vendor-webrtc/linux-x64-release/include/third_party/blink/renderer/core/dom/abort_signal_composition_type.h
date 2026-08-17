// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_COMPOSITION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_COMPOSITION_TYPE_H_

namespace blink {

// These correspond to the events propagated by source signals to dependent
// composite signals. The signal relationships are independent of each other for
// different composition types.
//
// TODO(crbug.com/1323391): Consider moving to AbortSignalCompositionManager.
enum class AbortSignalCompositionType {
  kAbort = 0,
  kPriority = 1,

  kMaxValue = kPriority
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_COMPOSITION_TYPE_H_
