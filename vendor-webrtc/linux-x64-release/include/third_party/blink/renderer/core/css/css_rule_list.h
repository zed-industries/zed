/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * (C) 2002-2003 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2002, 2006, 2012 Apple Computer, Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RULE_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RULE_LIST_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSRule;
class CSSStyleSheet;

using RuleIndexList = GCedHeapVector<std::pair<Member<CSSRule>, int>>;

class CSSRuleList : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSRuleList(const CSSRuleList&) = delete;
  CSSRuleList& operator=(const CSSRuleList&) = delete;

  virtual unsigned length() const = 0;
  CSSRule* item(unsigned index) const { return Item(index); }

  virtual CSSStyleSheet* GetStyleSheet() const = 0;
  virtual CSSRule* Item(unsigned index, bool trigger_use_counters) const = 0;
  CSSRule* Item(unsigned index) const {
    return Item(index, /*trigger_use_counters=*/true);
  }

  // Get an item, but signal that it's been requested internally from the
  // engine, and not directly from a script.
  CSSRule* ItemInternal(unsigned index) const {
    return Item(index, /*trigger_use_counters=*/false);
  }

 protected:
  CSSRuleList() = default;
};

template <class Rule>
class LiveCSSRuleList final : public CSSRuleList {
 public:
  LiveCSSRuleList(Rule* rule) : rule_(rule) {}

  void Trace(Visitor* visitor) const override {
    visitor->Trace(rule_);
    CSSRuleList::Trace(visitor);
  }

 private:
  unsigned length() const override { return rule_->length(); }
  CSSRule* Item(unsigned index, bool trigger_use_counters) const override {
    return rule_->Item(index, trigger_use_counters);
  }
  CSSStyleSheet* GetStyleSheet() const override {
    return rule_->parentStyleSheet();
  }

  Member<Rule> rule_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RULE_LIST_H_
