// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_SPELL_CHECK_TEST_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_SPELL_CHECK_TEST_BASE_H_

#include "third_party/blink/renderer/core/editing/testing/editing_test_base.h"

namespace blink {

class SpellChecker;

class SpellCheckTestBase : public EditingTestBase {
 protected:
  void SetUp() override;

  SpellChecker& GetSpellChecker() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_SPELL_CHECK_TEST_BASE_H_
