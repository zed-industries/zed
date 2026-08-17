/*
 * Copyright (C) 2007, 2008, 2010, 2011, 2012 Apple Inc. All rights reserved.
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
 * the Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SPACE_SPLIT_STRING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SPACE_SPLIT_STRING_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CORE_EXPORT SpaceSplitString {
  DISALLOW_NEW();

 public:
  SpaceSplitString() = default;
  explicit SpaceSplitString(const AtomicString& string) { Set(string); }
  SpaceSplitString(const SpaceSplitString& other) : data_(other.data_) {
    if (data_) {
      data_->MarkShared();
    }
  }
  SpaceSplitString& operator=(SpaceSplitString other) {
    std::swap(data_, other.data_);
    return *this;
  }
  SpaceSplitString(SpaceSplitString&&) = default;
  ~SpaceSplitString() = default;

  bool operator!=(const SpaceSplitString& other) const {
    return data_ != other.data_;
  }

  void Set(const AtomicString&);
  void Clear() { data_ = nullptr; }

  bool Contains(const AtomicString& string) const {
    return data_ && data_->Contains(string);
  }
  bool ContainsAll(const SpaceSplitString& names) const {
    return !names.data_ || (data_ && data_->ContainsAll(*names.data_));
  }
  void Add(const AtomicString&);
  void Remove(const AtomicString&);
  void Remove(wtf_size_t index);
  void ReplaceAt(wtf_size_t index, const AtomicString&);

  // https://dom.spec.whatwg.org/#concept-ordered-set-serializer
  // The ordered set serializer takes a set and returns the concatenation of the
  // strings in set, separated from each other by U+0020, if set is non-empty,
  // and the empty string otherwise.
  AtomicString SerializeToString() const;

  wtf_size_t size() const { return data_ ? data_->size() : 0; }
  bool IsNull() const { return !data_; }
  const AtomicString& operator[](wtf_size_t i) const { return (*data_)[i]; }
  Vector<AtomicString, 4>::const_iterator begin() const {
    return data_ ? data_->begin()
                 : Vector<AtomicString, 4>::const_iterator(nullptr);
  }
  Vector<AtomicString, 4>::const_iterator end() const {
    return data_ ? data_->end()
                 : Vector<AtomicString, 4>::const_iterator(nullptr);
  }

  void Trace(Visitor* visitor) const { visitor->Trace(data_); }

 private:
  class CORE_EXPORT Data : public GarbageCollected<Data> {
   public:
    static Data* Create(const AtomicString&);
    static Data* CreateUnique(const Data&);

    // This constructor always creates a "shared" (non-unique) Data object.
    explicit Data(const AtomicString&);
    // This constructor always creates a non-"shared" (unique) Data object.
    Data(const Data&);

    ~Data() = default;
    Data(Data&&) = delete;
    Data& operator=(const Data&) = delete;
    Data& operator=(Data&&) = delete;

    bool Contains(const AtomicString& string) const {
      return vector_.Contains(string);
    }

    bool ContainsAll(Data&);

    void Add(const AtomicString&);
    void Remove(unsigned index);

    void MarkShared() { might_be_shared_ = true; }
    bool MightBeShared() const { return might_be_shared_; }
    wtf_size_t size() const { return vector_.size(); }
    const AtomicString& operator[](wtf_size_t i) const { return vector_[i]; }
    AtomicString& operator[](wtf_size_t i) { return vector_[i]; }

    Vector<AtomicString, 4>::const_iterator begin() const {
      return vector_.begin();
    }
    Vector<AtomicString, 4>::const_iterator end() const {
      return vector_.end();
    }

    void Trace(Visitor*) const {}

   private:
    void CreateVector(const AtomicString&);
    template <typename CharacterType>
    inline void CreateVector(const AtomicString&,
                             base::span<const CharacterType>);

    bool might_be_shared_;
    Vector<AtomicString, 4> vector_;
  };

  typedef HeapHashMap<AtomicString, WeakMember<Data>> DataMap;

  static DataMap& SharedDataMap();

  void EnsureUnique() {
    if (data_ && data_->MightBeShared()) {
      data_ = Data::CreateUnique(*data_);
    }
  }

  Member<Data> data_;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const SpaceSplitString&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SPACE_SPLIT_STRING_H_
