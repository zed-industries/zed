// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CE_REACTIONS_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CE_REACTIONS_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "v8/include/v8.h"

namespace blink {

class CustomElementReaction;
class CustomElementReactionStack;
class Element;

// https://html.spec.whatwg.org/C/#cereactions
class CORE_EXPORT CEReactionsScope final {
  STACK_ALLOCATED();

 public:
  static CEReactionsScope* Current();

  explicit CEReactionsScope(v8::Isolate*);

  CEReactionsScope(const CEReactionsScope&) = delete;
  CEReactionsScope& operator=(const CEReactionsScope&) = delete;

  ~CEReactionsScope();

  void EnqueueToCurrentQueue(CustomElementReactionStack&,
                             Element&,
                             CustomElementReaction&);

 private:
  static CEReactionsScope* top_of_stack_;
  CustomElementReactionStack* stack_ = nullptr;
  CEReactionsScope* prev_;
  v8::TryCatch try_catch_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CE_REACTIONS_SCOPE_H_
