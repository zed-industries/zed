// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_OFFSET_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_OFFSET_MAP_H_

#include <unicode/edits.h>

#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

// Represents a mapping of text offset when |CaseMap| changes the length of the
// input string. Similar to [icu::Edits], but tracks only when the length
// changes.
//
// For example, when a character is inserted at offset 3, this map has an entry
// {3, 4}, meaning the offset 3 of the source becomes 4 in the result. When a
// character is removed at offset 3, an entry {4, 3} is created.
//
// [icu::Edits]: http://icu-project.org/apiref/icu4c/classicu_1_1Edits.html
class WTF_EXPORT TextOffsetMap {
 public:
  struct Entry {
    Entry(wtf_size_t source, wtf_size_t target)
        : source(source), target(target) {}

    bool operator==(const Entry& rhs) const {
      return source == rhs.source && target == rhs.target;
    }

    wtf_size_t source;
    wtf_size_t target;
  };

  // Create an empty TextOffsetMap instance.
  TextOffsetMap() = default;

  // Suppose that we mapped string-1 to string-2 with producing map12, and
  // we mapped string-2 to string-3 with producing map23. This constructor
  // creates a TextOffsetMap instance for mapping string-1 to string-3.
  TextOffsetMap(const TextOffsetMap& map12, const TextOffsetMap& map23);
  // Suppose that we mapped string-1 of which length is `length1` to string-2
  // of which length is `length2` with producing map12, and we mapped
  // string-2 to string-3 of which length is `length3` with producing map23.
  // This constructor creates a TextOffsetMap instance for mapping string-1
  // to string-3.
  TextOffsetMap(wtf_size_t length1,
                const TextOffsetMap& map12,
                wtf_size_t length2,
                const TextOffsetMap& map23,
                wtf_size_t length3);

  bool IsEmpty() const { return entries_.empty(); }

  const Vector<Entry>& Entries() const { return entries_; }

  void Clear() { entries_.Shrink(0); }

  void Append(wtf_size_t source, wtf_size_t target);
  void Append(const icu::Edits& edits);

  using Length = uint32_t;
  // This returns a list of which size is `new_length`. The Nth element of
  // the list represents the source character length of the Nth character
  // in the destination string.
  Vector<Length> CreateLengthMap(wtf_size_t old_length,
                                 wtf_size_t new_length) const;

 private:
  Vector<Entry> entries_;
};

WTF_EXPORT std::ostream& operator<<(std::ostream&, const TextOffsetMap::Entry&);
WTF_EXPORT std::ostream& operator<<(
    std::ostream& stream,
    const Vector<TextOffsetMap::Entry>& entries);

}  // namespace WTF

using WTF::TextOffsetMap;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_OFFSET_MAP_H_
