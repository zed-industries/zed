// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_SELECTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// TextFragmentSelector represents a single text=... selector of a
// TextFragmentAnchor, parsed into its components.
// TODO(bokan): This should be renamed to TextSelectorParams:
// https://docs.google.com/document/d/1yE75LfQn9GsooOyWWH---obsV46gT4JAUr9uWIYXp28/edit?usp=sharing.
class CORE_EXPORT TextFragmentSelector final {
 public:
  // Parses the serialized form into a TextFragmentSelector object. The
  // serialized argument is of the form: "prefix-,start,end,-suffix", where
  // prefix, end, and suffix are optional. If end is specified, the selector is
  // a range; otherwise, it is an exact match.
  static TextFragmentSelector FromTextDirective(const String& directive);

  enum SelectorType {
    // An invalid text selector.
    kInvalid,
    // An exact selector on the string start_.
    kExact,
    // A range selector on a text range start_ to end_.
    kRange,
  };

  TextFragmentSelector(SelectorType type,
                       const String& start,
                       const String& end,
                       const String& prefix,
                       const String& suffix);
  explicit TextFragmentSelector(SelectorType type);
  ~TextFragmentSelector() = default;

  SelectorType Type() const { return type_; }
  const String& Start() const { return start_; }
  const String& End() const { return end_; }
  const String& Prefix() const { return prefix_; }
  const String& Suffix() const { return suffix_; }
  String ToString() const;

 private:
  const SelectorType type_;
  String start_;
  String end_;
  String prefix_;
  String suffix_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_SELECTOR_H_
