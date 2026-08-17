// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_OPTION_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_OPTION_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class HTMLSelectElement;
class HTMLOptionElement;

class CORE_EXPORT OptionListIterator final {
  STACK_ALLOCATED();

 public:
  enum class StartingPoint {
    kStart,
    kEnd,
    kLast,
  };
  explicit OptionListIterator(
      const HTMLSelectElement& select,
      StartingPoint starting_point = StartingPoint::kStart)
      : select_(select), current_(nullptr) {
    switch (starting_point) {
      case StartingPoint::kStart:
        Advance(nullptr);
        break;
      case StartingPoint::kLast:
        Retreat(nullptr);
        break;
      case StartingPoint::kEnd:
        break;
    }
  }
  HTMLOptionElement& operator*() {
    DCHECK(current_);
    return *current_;
  }
  HTMLOptionElement* operator->() { return current_; }
  OptionListIterator& operator++() {
    if (current_) {
      Advance(current_);
    }
    return *this;
  }
  OptionListIterator& operator--() {
    if (current_) {
      Retreat(current_);
    }
    return *this;
  }
  operator bool() const { return current_; }
  bool operator==(const OptionListIterator& other) const {
    return current_ == other.current_;
  }
  bool operator!=(const OptionListIterator& other) const {
    return !(*this == other);
  }

 private:
  void Advance(HTMLOptionElement* current);
  void Retreat(HTMLOptionElement* current);

  const HTMLSelectElement& select_;
  HTMLOptionElement* current_;  // nullptr means we reached the end.
};

// OptionList class is a lightweight version of HTMLOptionsCollection.
class OptionList final {
  STACK_ALLOCATED();

 public:
  explicit OptionList(const HTMLSelectElement& select) : select_(select) {}
  using Iterator = OptionListIterator;
  Iterator begin() {
    return Iterator(select_, OptionListIterator::StartingPoint::kStart);
  }
  Iterator end() {
    return Iterator(select_, OptionListIterator::StartingPoint::kEnd);
  }
  Iterator last() {
    return Iterator(select_, OptionListIterator::StartingPoint::kLast);
  }
  bool Empty() {
    return !Iterator(select_, OptionListIterator::StartingPoint::kStart);
  }
  unsigned size() const;
  typedef bool (*OptionMatchingPredicate)(HTMLOptionElement& option);
  HTMLOptionElement* NextFocusableOption(HTMLOptionElement& option,
                                         bool inclusive = false) {
    return FindFocusableOption(option, /*forward*/ true, inclusive);
  }
  HTMLOptionElement* PreviousFocusableOption(HTMLOptionElement& option,
                                             bool inclusive = false) {
    return FindFocusableOption(option, /*forward*/ false, inclusive);
  }

 private:
  HTMLOptionElement* FindFocusableOption(HTMLOptionElement& option,
                                         bool forward,
                                         bool inclusive);

  const HTMLSelectElement& select_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_OPTION_LIST_H_
