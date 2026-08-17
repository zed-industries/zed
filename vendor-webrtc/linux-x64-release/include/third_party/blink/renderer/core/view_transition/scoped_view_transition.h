// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_SCOPED_VIEW_TRANSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_SCOPED_VIEW_TRANSITION_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class DOMViewTransition;
class Element;
class ExceptionState;
class ScriptState;
class V8ViewTransitionCallback;
class ViewTransitionOptions;

class CORE_EXPORT ScopedViewTransition {
 public:
  static DOMViewTransition* startViewTransition(
      ScriptState*,
      Element&,
      V8ViewTransitionCallback* callback,
      ExceptionState&);
  static DOMViewTransition* startViewTransition(ScriptState*,
                                                Element&,
                                                ViewTransitionOptions* options,
                                                ExceptionState&);
  static DOMViewTransition* startViewTransition(ScriptState*,
                                                Element&,
                                                ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_SCOPED_VIEW_TRANSITION_H_
