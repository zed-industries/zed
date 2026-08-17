/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SELECTOR_WATCH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SELECTOR_WATCH_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/hash_counted_set.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CORE_EXPORT CSSSelectorWatch final
    : public GarbageCollected<CSSSelectorWatch>,
      public Supplement<Document> {
 public:
  static const char kSupplementName[];

  explicit CSSSelectorWatch(Document&);
  ~CSSSelectorWatch() = default;

  static CSSSelectorWatch& From(Document&);
  static CSSSelectorWatch* FromIfExists(Document&);

  void WatchCSSSelectors(const Vector<String>& selectors);
  const HeapVector<Member<StyleRule>>& WatchedCallbackSelectors() const {
    return watched_callback_selectors_;
  }

  void UpdateSelectorMatches(const Vector<String>& removed_selectors,
                             const Vector<String>& added_selectors);

  void Trace(Visitor*) const override;

 private:
  void CallbackSelectorChangeTimerFired(TimerBase*);

  HeapVector<Member<StyleRule>> watched_callback_selectors_;

  // Maps a CSS selector string with a -webkit-callback property to the number
  // of matching ComputedStyle objects in this document.
  HashCountedSet<String> matching_callback_selectors_;
  // Selectors are relative to |matching_callback_selectors_|'s contents at
  // the previous call to selectorMatchChanged.
  HashSet<String> added_selectors_;
  HashSet<String> removed_selectors_;

  HeapTaskRunnerTimer<CSSSelectorWatch> callback_selector_change_timer_;

  // When an element is reparented, the new location's style is evaluated after
  // the expriation of the relayout timer.  We don't want to send redundant
  // callbacks to the embedder, so this counter lets us wait another time around
  // the event loop.
  int timer_expirations_;

  friend class CSSSelectorWatchTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SELECTOR_WATCH_H_
