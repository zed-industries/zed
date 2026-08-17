// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_GARBAGE_COLLECTED_SCRIPT_WRAPPABLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_GARBAGE_COLLECTED_SCRIPT_WRAPPABLE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class GarbageCollectedScriptWrappable : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  GarbageCollectedScriptWrappable(const String&);
  ~GarbageCollectedScriptWrappable() override;

  const String& toString() const { return string_; }

 private:
  String string_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_GARBAGE_COLLECTED_SCRIPT_WRAPPABLE_H_
