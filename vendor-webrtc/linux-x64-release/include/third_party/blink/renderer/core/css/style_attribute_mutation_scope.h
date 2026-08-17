/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2012 Apple Inc. All
 * rights reserved.
 * Copyright (C) 2011 Research In Motion Limited. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ATTRIBUTE_MUTATION_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ATTRIBUTE_MUTATION_SCOPE_H_

#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class AbstractPropertySetCSSStyleDeclaration;
class MutationObserverInterestGroup;
class MutationRecord;

class StyleAttributeMutationScope {
  STACK_ALLOCATED();

 public:
  StyleAttributeMutationScope(AbstractPropertySetCSSStyleDeclaration*);
  StyleAttributeMutationScope(const StyleAttributeMutationScope&) = delete;
  StyleAttributeMutationScope& operator=(const StyleAttributeMutationScope&) =
      delete;

  ~StyleAttributeMutationScope();

  void EnqueueMutationRecord() { should_deliver_ = true; }

  void DidInvalidateStyleAttr() { should_notify_inspector_ = true; }

 private:
  static unsigned scope_count_;
  static AbstractPropertySetCSSStyleDeclaration* current_decl_;
  static bool should_notify_inspector_;
  static bool should_deliver_;

  MutationObserverInterestGroup* mutation_recipients_ = nullptr;
  MutationRecord* mutation_ = nullptr;
  AtomicString old_value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ATTRIBUTE_MUTATION_SCOPE_H_
