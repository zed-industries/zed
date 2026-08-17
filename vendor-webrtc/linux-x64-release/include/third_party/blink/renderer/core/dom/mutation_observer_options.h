// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_MUTATION_OBSERVER_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_MUTATION_OBSERVER_OPTIONS_H_

namespace blink {

using MutationObserverOptions = unsigned char;
using MutationRecordDeliveryOptions = unsigned char;

// MutationType represents lower three bits of MutationObserverOptions.
// It doesn't use |enum class| because we'd like to do bitwise operations.
enum MutationType {
  kMutationTypeChildList = 1 << 0,
  kMutationTypeAttributes = 1 << 1,
  kMutationTypeCharacterData = 1 << 2,

  kMutationTypeAll = kMutationTypeChildList | kMutationTypeAttributes |
                     kMutationTypeCharacterData
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_MUTATION_OBSERVER_OPTIONS_H_
