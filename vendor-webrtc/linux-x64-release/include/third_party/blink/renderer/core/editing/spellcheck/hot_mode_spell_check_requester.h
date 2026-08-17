// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_HOT_MODE_SPELL_CHECK_REQUESTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_HOT_MODE_SPELL_CHECK_REQUESTER_H_

#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class SpellCheckRequester;

// This class is only supposed to be used by IdleSpellCheckController in hot
// mode invocation. Not to be confused with SpellCheckRequester.
// See design doc for details: https://goo.gl/zONC3v
class HotModeSpellCheckRequester {
  STACK_ALLOCATED();

 public:
  explicit HotModeSpellCheckRequester(SpellCheckRequester&);
  HotModeSpellCheckRequester(const HotModeSpellCheckRequester&) = delete;
  HotModeSpellCheckRequester& operator=(const HotModeSpellCheckRequester&) =
      delete;
  void CheckSpellingAt(const Position&);

 private:
  HeapVector<Member<const Element>> processed_root_editables_;
  SpellCheckRequester* requester_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_HOT_MODE_SPELL_CHECK_REQUESTER_H_
