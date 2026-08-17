// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_LANGUAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_LANGUAGE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class CORE_EXPORT NavigatorLanguage : public GarbageCollectedMixin {
 public:
  explicit NavigatorLanguage(ExecutionContext*);

  AtomicString language();
  const Vector<String>& languages();
  bool IsLanguagesDirty() const;
  void SetLanguagesDirty();

  // Accepts a comma-separated list of languages.
  void SetLanguagesForTesting(const String& languages);

  void Trace(Visitor*) const override;

 protected:
  virtual String GetAcceptLanguages() = 0;

 private:
  void EnsureUpdatedLanguage();

  // NavigatorLanguage can be instantiated after a frame detachment and
  // |execution_context_| may be null at the time of instantiation.
  WeakMember<ExecutionContext> execution_context_;
  Vector<String> languages_;
  bool languages_dirty_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_LANGUAGE_H_
