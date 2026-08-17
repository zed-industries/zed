/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 * Copyright (C) 2014 Apple Inc. All rights reserved.
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTRIBUTE_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTRIBUTE_COLLECTION_H_

#include "base/compiler_specific.h"
#include "third_party/blink/renderer/core/dom/attribute.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_table.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

template <typename Container, typename ContainerMemberType = Container>
class AttributeCollectionGeneric {
  STACK_ALLOCATED();

 public:
  using ValueType = typename Container::ValueType;
  using iterator = ValueType*;

  AttributeCollectionGeneric(Container& attributes) : attributes_(attributes) {}

  ValueType& operator[](unsigned index) const { return at(index); }
  ValueType& at(unsigned index) const {
    CHECK_LT(index, size());
    // SAFETY: Check above.
    return UNSAFE_BUFFERS(begin()[index]);
  }

  ValueType* data() { return attributes_.data(); }
  const ValueType* data() const { return attributes_.data(); }

  iterator begin() const { return attributes_.data(); }
  iterator end() const {
    // SAFETY: size() describes the number of elements at data().
    // This form is used in place of end() to avoid a conflict
    // between the pointer type used as an iterator for this class,
    // and an iterator type used by containers.
    return UNSAFE_BUFFERS(begin() + size());
  }

  unsigned size() const { return attributes_.size(); }
  bool IsEmpty() const { return !size(); }

  // Find() returns nullptr if the specified name is not found.
  iterator Find(const QualifiedName&) const;
  iterator Find(const AtomicString& name) const;
  wtf_size_t FindIndex(const QualifiedName&) const;
  wtf_size_t FindIndex(const AtomicString& name) const;

  // FindHinted() and FindIndexHinted() have subtle semantics.
  //
  // The |hint| is a WeakResult that represents whether or not an AtomicString
  // exists for the AttributeCollectionGeneric version of |name| which has two
  // odd quirks:
  //
  //  1) In an HTML context, the hint will be from a lookup of the ASCII
  //     lowercased version of the attribute |name| as is required by spec.
  //  2) The |hint| is a snapshot of a membership query of the
  //     AtomicStringTable from a specific point in time.
  //
  // For (1), the HTML spec says that attribute names without prefixes should
  // be lowercased before comparison. However, if an attribute is added with
  // a namespace using the *NS() attribute APIs then lookup becomes case
  // sensitive. Therefore the API require both non-lowercased |name| and a
  // lowercased |hint|.
  //
  // For (2), the caller must ensure that its logic is robust to changes in
  // the AtomicStringTable between the creation of the |hint| and its use with
  // this API. In particular, one should not modify |collection| between
  // creation of |hint| and execution of any hinted function.
  //
  // A concrete example of a valid usage pattern is:
  //
  // WTF::AtomicStringTable::WeakResult hint =
  //     WTF::AtomicStringTable::WeakFindLowercased(name);
  //   .... Mutate |WTF::AtomicStringTable| but not |collection| ....
  // collection.FindHinted(name, hint);
  //
  // Because FindHinted() is an existence check, as long as collection is not
  // mutated between the hint creation and the lookup, we know that
  //
  //  (a) If hint.IsNull(), it cannot ever be in |collection| since
  //      then the corresponding AtomicString would be found in
  //      the AtomicStringTable.
  //  (b) If !hint.IsNull() and hint is in |collection| then the table
  //      has a reference to the corresponding AtomicString meaning
  //      it will not be removed from the AtomicString.
  //  (c) If the !hint.IsNull() and it is not in |collection|, then it is
  //      possible that the underlying memory buffer for the AtomicString
  //      corresponding to the him can  be reallocated to a different string
  //      making the |hint| semantically invalid. However, because the
  //      |collection| is not mutated, |hint| will not match anything.
  iterator FindHinted(const StringView& name,
                      WTF::AtomicStringTable::WeakResult hint) const;
  wtf_size_t FindIndexHinted(const StringView& name,
                             WTF::AtomicStringTable::WeakResult hint) const;

 protected:
  iterator FindWithPrefix(const StringView& name) const;

  ContainerMemberType attributes_;
};

class AttributeArray {
  DISALLOW_NEW();

 public:
  using ValueType = const Attribute;

  AttributeArray(const Attribute* array, unsigned size)
      : array_(array), size_(size) {}

  const Attribute* data() const { return array_; }
  unsigned size() const { return size_; }

 private:
  const Attribute* array_;
  unsigned size_;
};

class AttributeCollection
    : public AttributeCollectionGeneric<const AttributeArray> {
 public:
  AttributeCollection()
      : AttributeCollectionGeneric<const AttributeArray>(
            AttributeArray(nullptr, 0)) {}

  AttributeCollection(const Attribute* array, unsigned size)
      : AttributeCollectionGeneric<const AttributeArray>(
            AttributeArray(array, size)) {}
};

using AttributeVector = Vector<Attribute, 4>;
class MutableAttributeCollection
    : public AttributeCollectionGeneric<AttributeVector, AttributeVector&> {
 public:
  explicit MutableAttributeCollection(AttributeVector& attributes)
      : AttributeCollectionGeneric<AttributeVector, AttributeVector&>(
            attributes) {}

  // These functions do no error/duplicate checking.
  void Append(const QualifiedName&, const AtomicString& value);
  void Remove(unsigned index);
};

inline void MutableAttributeCollection::Append(const QualifiedName& name,
                                               const AtomicString& value) {
  attributes_.push_back(Attribute(name, value));
}

inline void MutableAttributeCollection::Remove(unsigned index) {
  attributes_.EraseAt(index);
}

template <typename Container, typename ContainerMemberType>
inline typename AttributeCollectionGeneric<Container,
                                           ContainerMemberType>::iterator
AttributeCollectionGeneric<Container, ContainerMemberType>::Find(
    const AtomicString& name) const {
  return FindHinted(name, WTF::AtomicStringTable::WeakResult(name.Impl()));
}

template <typename Container, typename ContainerMemberType>
inline wtf_size_t
AttributeCollectionGeneric<Container, ContainerMemberType>::FindIndexHinted(
    const StringView& name,
    WTF::AtomicStringTable::WeakResult hint) const {
  iterator it = FindHinted(name, hint);
  return it ? wtf_size_t(it - begin()) : kNotFound;
}

template <typename Container, typename ContainerMemberType>
inline wtf_size_t
AttributeCollectionGeneric<Container, ContainerMemberType>::FindIndex(
    const QualifiedName& name) const {
  iterator end = this->end();
  wtf_size_t index = 0;
  for (iterator it = begin(); it != end; UNSAFE_TODO(++it), ++index) {
    if (it->GetName().Matches(name))
      return index;
  }
  return kNotFound;
}

template <typename Container, typename ContainerMemberType>
inline wtf_size_t
AttributeCollectionGeneric<Container, ContainerMemberType>::FindIndex(
    const AtomicString& name) const {
  return FindIndexHinted(name, WTF::AtomicStringTable::WeakResult(name.Impl()));
}

template <typename Container, typename ContainerMemberType>
inline typename AttributeCollectionGeneric<Container,
                                           ContainerMemberType>::iterator
AttributeCollectionGeneric<Container, ContainerMemberType>::FindHinted(
    const StringView& name,
    WTF::AtomicStringTable::WeakResult hint) const {
  // A slow check is required if there are any attributes with prefixes
  // and no unprefixed name matches.
  bool has_attributes_with_prefixes = false;

  // Optimize for the case where the attribute exists and its name exactly
  // matches.
  iterator end = this->end();
  for (iterator it = begin(); it != end; UNSAFE_TODO(++it)) {
    // FIXME: Why check the prefix? Namespaces should be all that matter.
    // Most attributes (all of HTML and CSS) have no namespace.
    if (!it->GetName().HasPrefix()) {
      if (hint == it->LocalName()) {
        return it;
      }
    } else {
      has_attributes_with_prefixes = true;
    }
  }

  // Note that if the attribute has a prefix, the match has to be case
  // sensitive therefore |name| must be used.
  if (has_attributes_with_prefixes)
    return FindWithPrefix(name);
  return nullptr;
}

template <typename Container, typename ContainerMemberType>
inline typename AttributeCollectionGeneric<Container,
                                           ContainerMemberType>::iterator
AttributeCollectionGeneric<Container, ContainerMemberType>::Find(
    const QualifiedName& name) const {
  iterator end = this->end();
  for (iterator it = begin(); it != end; UNSAFE_TODO(++it)) {
    if (it->GetName().Matches(name))
      return it;
  }
  return nullptr;
}

template <typename Container, typename ContainerMemberType>
typename AttributeCollectionGeneric<Container, ContainerMemberType>::iterator
AttributeCollectionGeneric<Container, ContainerMemberType>::FindWithPrefix(
    const StringView& name) const {
  // Check all attributes with prefixes. This is a case sensitive check.
  // Attributes with empty prefixes are expected to be handled outside this
  // function.
  iterator end = this->end();
  for (iterator it = begin(); it != end; UNSAFE_TODO(++it)) {
    if (!it->GetName().HasPrefix()) {
      // Skip attributes with no prefixes because they must be checked in
      // FindIndex(const AtomicString&).
      DCHECK(!(name == it->LocalName()));
    } else {
      // FIXME: Would be faster to do this comparison without calling ToString,
      // which generates a temporary string by concatenation. But this branch is
      // only reached if the attribute name has a prefix, which is rare in HTML.
      if (name == it->GetName().ToString()) {
        return it;
      }
    }
  }
  return nullptr;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTRIBUTE_COLLECTION_H_
