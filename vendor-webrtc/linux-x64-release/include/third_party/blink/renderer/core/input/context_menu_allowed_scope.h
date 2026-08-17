// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_CONTEXT_MENU_ALLOWED_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_CONTEXT_MENU_ALLOWED_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT ContextMenuAllowedScope {
  STACK_ALLOCATED();

 public:
  ContextMenuAllowedScope();
  ContextMenuAllowedScope(const ContextMenuAllowedScope&) = delete;
  ContextMenuAllowedScope& operator=(const ContextMenuAllowedScope&) = delete;
  ~ContextMenuAllowedScope();

  static bool IsContextMenuAllowed();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_CONTEXT_MENU_ALLOWED_SCOPE_H_
