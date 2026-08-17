// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_HYPHENATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_HYPHENATION_H_

#include "third_party/blink/renderer/platform/text/hyphenation.h"

namespace blink {

class MockHyphenation : public Hyphenation {
 public:
  wtf_size_t LastHyphenLocation(const StringView&,
                                wtf_size_t before_index) const override;

  static scoped_refptr<MockHyphenation> Create() {
    return base::AdoptRef(new MockHyphenation);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_HYPHENATION_H_
