/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_INVALIDATION_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_INVALIDATION_SET_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/invalidation/invalidation_flags.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class Element;
class TracedValue;

enum class InvalidationType {
  kInvalidateDescendants,
  kInvalidateSiblings,
  kInvalidateNthSiblings
};

class InvalidationSet;

struct CORE_EXPORT InvalidationSetDeleter {
  static void Destruct(const InvalidationSet*);
};

// Tracks data to determine which descendants in a DOM subtree, or
// siblings and their descendants, need to have style recalculated.
//
// Some example invalidation sets:
//
// .z {}
//   For class z we will have a DescendantInvalidationSet with invalidatesSelf
//   (the element itself is invalidated).
//
// .y .z {}
//   For class y we will have a DescendantInvalidationSet containing class z.
//
// .x ~ .z {}
//   For class x we will have a SiblingInvalidationSet containing class z, with
//   invalidatesSelf (the sibling itself is invalidated).
//
// .w ~ .y .z {}
//   For class w we will have a SiblingInvalidationSet containing class y, with
//   the SiblingInvalidationSet havings siblingDescendants containing class z.
//
// .v * {}
//   For class v we will have a DescendantInvalidationSet with
//   wholeSubtreeInvalid.
//
// .u ~ * {}
//   For class u we will have a SiblingInvalidationSet with wholeSubtreeInvalid
//   and invalidatesSelf (for all siblings, the sibling itself is invalidated).
//
// .t .v, .t ~ .z {}
//   For class t we will have a SiblingInvalidationSet containing class z, with
//   the SiblingInvalidationSet also holding descendants containing class v.
//
// We avoid virtual functions to minimize space consumption.
class CORE_EXPORT InvalidationSet
    : public WTF::RefCounted<InvalidationSet, InvalidationSetDeleter> {
 public:
  InvalidationSet(const InvalidationSet&) = delete;
  InvalidationSet& operator=(const InvalidationSet&) = delete;

  bool operator==(const InvalidationSet&) const;
  bool operator!=(const InvalidationSet& o) const { return !(*this == o); }

  InvalidationType GetType() const {
    return static_cast<InvalidationType>(type_);
  }
  bool IsDescendantInvalidationSet() const {
    return GetType() == InvalidationType::kInvalidateDescendants;
  }
  bool IsSiblingInvalidationSet() const {
    return GetType() != InvalidationType::kInvalidateDescendants;
  }
  bool IsNthSiblingInvalidationSet() const {
    return GetType() == InvalidationType::kInvalidateNthSiblings;
  }

  bool InvalidatesElement(Element&) const;
  bool InvalidatesTagName(Element&) const;

  void AddClass(const AtomicString& class_name);
  void AddId(const AtomicString& id);
  void AddTagName(const AtomicString& tag_name);
  void AddAttribute(const AtomicString& attribute_local_name);

  void SetInvalidationFlags(InvalidationFlags flags) {
    invalidation_flags_ = flags;
  }

  void SetWholeSubtreeInvalid();
  bool WholeSubtreeInvalid() const {
    return invalidation_flags_.WholeSubtreeInvalid();
  }

  void SetInvalidatesSelf() { invalidates_self_ = true; }
  bool InvalidatesSelf() const { return invalidates_self_; }

  void SetInvalidatesNth() {
    DCHECK(!IsSelfInvalidationSet());
    invalidates_nth_ = true;
  }
  bool InvalidatesNth() const { return invalidates_nth_; }

  void SetTreeBoundaryCrossing() {
    invalidation_flags_.SetTreeBoundaryCrossing(true);
  }
  bool TreeBoundaryCrossing() const {
    return invalidation_flags_.TreeBoundaryCrossing();
  }

  void SetInsertionPointCrossing() {
    invalidation_flags_.SetInsertionPointCrossing(true);
  }
  bool InsertionPointCrossing() const {
    return invalidation_flags_.InsertionPointCrossing();
  }

  void SetCustomPseudoInvalid() {
    invalidation_flags_.SetInvalidateCustomPseudo(true);
  }
  bool CustomPseudoInvalid() const {
    return invalidation_flags_.InvalidateCustomPseudo();
  }

  void SetInvalidatesSlotted() {
    invalidation_flags_.SetInvalidatesSlotted(true);
  }
  bool InvalidatesSlotted() const {
    return invalidation_flags_.InvalidatesSlotted();
  }

  const InvalidationFlags GetInvalidationFlags() const {
    return invalidation_flags_;
  }

  void SetInvalidatesParts() { invalidation_flags_.SetInvalidatesParts(true); }
  bool InvalidatesParts() const {
    return invalidation_flags_.InvalidatesParts();
  }

  void SetInvalidatesTreeCounting() {
    invalidation_flags_.SetInvalidatesTreeCounting(true);
  }
  bool InvalidatesTreeCounting() const {
    return invalidation_flags_.InvalidatesTreeCounting();
  }

  bool IsEmpty() const {
    return HasEmptyBackings() &&
           !invalidation_flags_.InvalidateCustomPseudo() &&
           !invalidation_flags_.InsertionPointCrossing() &&
           !invalidation_flags_.InvalidatesSlotted() &&
           !invalidation_flags_.InvalidatesParts() &&
           !invalidation_flags_.InvalidatesTreeCounting();
  }

  void WriteIntoTrace(perfetto::TracedValue context) const;

  // Format the InvalidationSet for debugging purposes.
  //
  // Examples:
  //
  //         { .a } - Invalidates class |a|.
  //         { #a } - Invalidates id |a|.
  //      { .a #a } - Invalidates class |a| and id |a|.
  //        { div } - Invalidates tag name |div|.
  //     { :hover } - Invalidates pseudo-class :hover.
  //  { .a [name] } - Invalidates class |a| and attribute |name|.
  //          { $ } - Invalidates self.
  //       { .a $ } - Invalidates class |a| and self.
  //       { .b 4 } - Invalidates class |b|. Max direct siblings = 4.
  //   { .a .b $4 } - Combination of the two previous examples.
  //          { W } - Whole subtree invalid.
  //
  // Flags (omitted if false):
  //
  //  $ - Invalidates self.
  //  N - Invalidates Nth sibling.
  //  W - Whole subtree invalid.
  //  C - Invalidates custom pseudo.
  //  T - Tree boundary crossing.
  //  I - Insertion point crossing.
  //  S - Invalidates slotted.
  //  P - Invalidates parts.
  //  ~ - Max direct siblings is kDirectAdjacentMax.
  //  <integer> - Max direct siblings is specified number (omitted if 1).
  String ToString() const;

  void Combine(const InvalidationSet& other);

  // Returns a singleton DescendantInvalidationSet which only has
  // InvalidatesSelf() set and is otherwise empty. As this is a common
  // invalidation set for features only found in rightmost compounds,
  // sharing this singleton between such features saves a lot of memory on
  // sites with a big number of style rules.
  static InvalidationSet* SelfInvalidationSet();
  bool IsSelfInvalidationSet() const { return this == SelfInvalidationSet(); }

  // Returns a singleton DescendantInvalidationSet which invalidates all
  // shadow-including descendants with part attributes.
  static InvalidationSet* PartInvalidationSet();

  // Returns a singleton NthSiblingInvalidationSet which invalidates all
  // siblings whose ComputedStyle depends on tree-counting functions such as
  // sibling-count() and sibling-index(). It does so by setting
  // invalidates_tree_counting_ along with invalidates_self_, making it
  // equivalent to an imaginary ':nth-child(n):has-tree-counting-style'
  // selector, where ':has-tree-counting-style' matches an element with a
  // ComputedStyle that relies on the evaluation of tree-counting functions.
  //
  // This set is scheduled on ContainerNodes marked with
  // ChildrenAffectedByForwardPositionalRules() or
  // ChildrenAffectedByBackwardPositionalRules() when child elements are
  // inserted, removed, or change target slot.
  static InvalidationSet* TreeCountingInvalidationSet();

  enum class BackingType {
    kClasses,
    kIds,
    kTagNames,
    kAttributes
    // These values are used as bit-indices, and must be smaller than 8.
    // See Backing::GetMask.
  };

  template <BackingType>
  union Backing;

  // Each BackingType has a corresponding bit in an instance of this class. A
  // set bit indicates that the Backing at that position is a HashSet. An unset
  // bit indicates an AtomicString (which may be null).
  class BackingFlags {
   private:
    uint8_t bits_ = 0;
    template <BackingType>
    friend union Backing;
  };

  // InvalidationSet needs to maintain HashSets of classes, ids, tag names and
  // attributes to invalidate. However, since it's common for these hash sets
  // to contain only one element (with a total capacity of 8), we avoid creating
  // the actual HashSets until we have more than one item. If a set contains
  // just one item, we store an AtomicString instead, along with a bit
  // indicating either AtomicString or HashSet.
  //
  // The bits (see BackingFlags) associated with each Backing are stored on the
  // outside, to make sizeof(InvalidationSet) as small as possible.
  //
  // WARNING: Backings must be cleared manually in ~InvalidationSet, otherwise
  //          an AtomicString or HashSet will leak.
  template <BackingType type>
  union Backing {
    using Flags = BackingFlags;
    static_assert(static_cast<size_t>(type) < sizeof(BackingFlags::bits_) * 8,
                  "Enough bits in BackingFlags");

    ~Backing() {
      // Destruction is done by Clear(), since we don't know
      // which of the two members are active without any flags.
    }

    // Adds an AtomicString to the associated Backing. If the Backing is
    // currently empty, we simply copy the incoming AtomicString, which AddRefs
    // the underlying StringImpl. If the Backing already has one item, we first
    // "upgrade" to a HashSet, and add the AtomicString.
    void Add(Flags&, const AtomicString&);
    // Clears the associated Backing. If the Backing is an AtomicString, it is
    // destroyed. If the Backing is a HashSet, it is deleted.
    void Clear(Flags&);
    bool Contains(const Flags&, const AtomicString&) const;
    bool IsEmpty(const Flags&) const;
    size_t Size(const Flags&) const;
    bool IsHashSet(const Flags& flags) const { return flags.bits_ & GetMask(); }

    const AtomicString* GetString(const Flags& flags) const {
      return IsHashSet(flags) ? nullptr : &string_;
    }
    const HashSet<AtomicString>* GetHashSet(const Flags& flags) const {
      return IsHashSet(flags) ? hash_set_ : nullptr;
    }

    // A simple forward iterator, which can either "iterate" over a single
    // AtomicString, or act as a wrapper for HashSet<AtomicString>::iterator.
    class Iterator {
     public:
      enum class Type { kString, kHashSet };

      explicit Iterator(const AtomicString& string_impl)
          : type_(Type::kString), string_(string_impl) {}
      explicit Iterator(HashSet<AtomicString>::iterator iterator)
          : type_(Type::kHashSet), hash_set_iterator_(iterator) {}

      bool operator==(const Iterator& other) const {
        if (type_ != other.type_) {
          return false;
        }
        if (type_ == Type::kString) {
          return string_ == other.string_;
        }
        return hash_set_iterator_ == other.hash_set_iterator_;
      }
      bool operator!=(const Iterator& other) const { return !(*this == other); }
      void operator++() {
        if (type_ == Type::kString) {
          string_ = g_null_atom;
        } else {
          ++hash_set_iterator_;
        }
      }

      const AtomicString& operator*() const {
        return type_ == Type::kString ? string_ : *hash_set_iterator_;
      }

     private:
      Type type_;
      // Used when type_ is kString.
      AtomicString string_;
      // Used when type_ is kHashSet.
      HashSet<AtomicString>::iterator hash_set_iterator_;
    };

    class Range {
     public:
      Range(Iterator begin, Iterator end) : begin_(begin), end_(end) {}
      Iterator begin() const { return begin_; }
      Iterator end() const { return end_; }

     private:
      Iterator begin_;
      Iterator end_;
    };

    Range Items(const Flags& flags) const {
      Iterator begin =
          IsHashSet(flags) ? Iterator(hash_set_->begin()) : Iterator(string_);
      Iterator end =
          IsHashSet(flags) ? Iterator(hash_set_->end()) : Iterator(g_null_atom);
      return Range(begin, end);
    }

   private:
    uint8_t GetMask() const { return 1u << static_cast<size_t>(type); }
    void SetIsString(Flags& flags) { flags.bits_ &= ~GetMask(); }
    void SetIsHashSet(Flags& flags) { flags.bits_ |= GetMask(); }

    AtomicString string_{};
    HashSet<AtomicString>* hash_set_;
  };

 protected:
  explicit InvalidationSet(InvalidationType);

  ~InvalidationSet() {
    ClearAllBackings();
  }

 private:
  friend struct InvalidationSetDeleter;
  friend class RuleFeatureSetTest;
  void Destroy() const;
  void ClearAllBackings();
  bool HasEmptyBackings() const;

  bool HasClasses() const { return !classes_.IsEmpty(backing_flags_); }
  bool HasIds() const { return !ids_.IsEmpty(backing_flags_); }
  bool HasTagNames() const { return !tag_names_.IsEmpty(backing_flags_); }
  bool HasAttributes() const { return !attributes_.IsEmpty(backing_flags_); }

  bool HasId(const AtomicString& string) const {
    return ids_.Contains(backing_flags_, string);
  }

  bool HasTagName(const AtomicString& string) const {
    return tag_names_.Contains(backing_flags_, string);
  }

  Backing<BackingType::kClasses>::Range Classes() const {
    return classes_.Items(backing_flags_);
  }

  Backing<BackingType::kIds>::Range Ids() const {
    return ids_.Items(backing_flags_);
  }

  Backing<BackingType::kTagNames>::Range TagNames() const {
    return tag_names_.Items(backing_flags_);
  }

  Backing<BackingType::kAttributes>::Range Attributes() const {
    return attributes_.Items(backing_flags_);
  }

  // Look for any class name on Element that is contained in |classes_|.
  const AtomicString* FindAnyClass(Element&) const;
  // Look for any attribute on Element that is contained in |attributes_|.
  const AtomicString* FindAnyAttribute(Element&) const;

  Backing<BackingType::kClasses> classes_;
  Backing<BackingType::kIds> ids_;
  Backing<BackingType::kTagNames> tag_names_;
  Backing<BackingType::kAttributes> attributes_;

  InvalidationFlags invalidation_flags_;
  BackingFlags backing_flags_;

  unsigned type_ : 2;

  // If true, the element or sibling itself is invalid.
  unsigned invalidates_self_ : 1;

  // If true, scheduling this invalidation set on a node
  // will also schedule nth-child invalidation on its parent
  // (unless we know for sure no child can be affected by a
  // selector of the :nth-child type).
  unsigned invalidates_nth_ : 1;
};

class CORE_EXPORT DescendantInvalidationSet final : public InvalidationSet {
 public:
  static scoped_refptr<DescendantInvalidationSet> Create() {
    return base::AdoptRef(new DescendantInvalidationSet);
  }

 private:
  DescendantInvalidationSet()
      : InvalidationSet(InvalidationType::kInvalidateDescendants) {}
};

class CORE_EXPORT SiblingInvalidationSet : public InvalidationSet {
 public:
  static scoped_refptr<SiblingInvalidationSet> Create(
      scoped_refptr<DescendantInvalidationSet> descendants) {
    return base::AdoptRef(new SiblingInvalidationSet(std::move(descendants)));
  }

  static constexpr unsigned kDirectAdjacentMax =
      std::numeric_limits<unsigned>::max();

  unsigned MaxDirectAdjacentSelectors() const {
    return max_direct_adjacent_selectors_;
  }
  void UpdateMaxDirectAdjacentSelectors(unsigned value) {
    max_direct_adjacent_selectors_ =
        std::max(value, max_direct_adjacent_selectors_);
  }

  DescendantInvalidationSet* SiblingDescendants() const {
    return sibling_descendant_invalidation_set_.get();
  }
  DescendantInvalidationSet& EnsureSiblingDescendants();

  DescendantInvalidationSet* Descendants() const {
    return descendant_invalidation_set_.get();
  }
  DescendantInvalidationSet& EnsureDescendants();

 protected:
  // Base constructor for NthSiblingInvalidationSet.
  SiblingInvalidationSet();

 private:
  explicit SiblingInvalidationSet(
      scoped_refptr<DescendantInvalidationSet> descendants);

  // Indicates the maximum possible number of siblings affected.
  unsigned max_direct_adjacent_selectors_;

  // Indicates the descendants of siblings.
  scoped_refptr<DescendantInvalidationSet> sibling_descendant_invalidation_set_;

  // Null if a given feature (class, attribute, id, pseudo-class) has only
  // a SiblingInvalidationSet and not also a DescendantInvalidationSet.
  scoped_refptr<DescendantInvalidationSet> descendant_invalidation_set_;
};

// For invalidation of :nth-* selectors on dom mutations we use a sibling
// invalidation set which is scheduled on the parent node of the DOM mutation
// affected by the :nth-* selectors. Similarly, we use another
// NthSiblingInvalidationSet for invalidating style for elements whose style
// rely on tree-counting functions such as sibling-index(). See
// InvalidationSet::TreeCountingInvalidationSet() for further documentation.
//
// During invalidation, such sets are pushed into the SiblingData used for
// invalidating the direct children.
//
// Features are collected into this set as if the selectors were preceded by a
// universal selector with an indirect adjacent combinator.
//
// Example: If you have the following selector:
//
// :nth-of-type(2n+1) .x {}
//
// we need to invalidate descendants of class 'x' of an arbitrary number of
// siblings when one of the siblings are added or removed. We then collect
// features to the NthSiblingInvalidationSet as if we had a selector:
//
// * ~ :nth-of-type(2n+1) .x {}
//
// Pushing that set into SiblingData before invalidating the siblings will then
// invalidate descendants with class 'x'.

class NthSiblingInvalidationSet final : public SiblingInvalidationSet {
 public:
  static scoped_refptr<NthSiblingInvalidationSet> Create() {
    return base::AdoptRef(new NthSiblingInvalidationSet());
  }

 private:
  NthSiblingInvalidationSet() = default;
};

using InvalidationSetVector = Vector<scoped_refptr<InvalidationSet>>;

struct InvalidationLists {
  InvalidationSetVector descendants;
  InvalidationSetVector siblings;
};

template <typename InvalidationSet::BackingType type>
void InvalidationSet::Backing<type>::Add(InvalidationSet::BackingFlags& flags,
                                         const AtomicString& string) {
  DCHECK(!string.IsNull());
  if (IsHashSet(flags)) {
    hash_set_->insert(string);
  } else if (string_) {
    if (string_ == string) {
      return;
    }
    AtomicString atomic_string(std::move(string_));
    string_.~AtomicString();
    hash_set_ = new HashSet<AtomicString>();
    hash_set_->insert(atomic_string);
    hash_set_->insert(string);
    SetIsHashSet(flags);
  } else {
    new (&string_) AtomicString(string);
  }
}

template <typename InvalidationSet::BackingType type>
void InvalidationSet::Backing<type>::Clear(
    InvalidationSet::BackingFlags& flags) {
  if (IsHashSet(flags)) {
    if (hash_set_) {
      delete hash_set_;
      new (&string_) AtomicString;
    }
  } else {
    string_ = AtomicString();
  }
  SetIsString(flags);
}

template <typename InvalidationSet::BackingType type>
bool InvalidationSet::Backing<type>::Contains(
    const InvalidationSet::BackingFlags& flags,
    const AtomicString& string) const {
  if (IsHashSet(flags)) {
    return hash_set_->Contains(string);
  }
  return string == string_;
}

template <typename InvalidationSet::BackingType type>
bool InvalidationSet::Backing<type>::IsEmpty(
    const InvalidationSet::BackingFlags& flags) const {
  return !IsHashSet(flags) && !string_;
}

template <typename InvalidationSet::BackingType type>
size_t InvalidationSet::Backing<type>::Size(
    const InvalidationSet::BackingFlags& flags) const {
  if (const HashSet<AtomicString>* set = GetHashSet(flags)) {
    return set->size();
  }
  if (GetString(flags)) {
    return 1;
  }
  return 0;
}

template <>
struct DowncastTraits<DescendantInvalidationSet> {
  static bool AllowFrom(const InvalidationSet& value) {
    return value.IsDescendantInvalidationSet();
  }
};

template <>
struct DowncastTraits<SiblingInvalidationSet> {
  static bool AllowFrom(const InvalidationSet& value) {
    return value.IsSiblingInvalidationSet();
  }
};

template <>
struct DowncastTraits<NthSiblingInvalidationSet> {
  static bool AllowFrom(const InvalidationSet& value) {
    return value.IsNthSiblingInvalidationSet();
  }
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const InvalidationSet&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_INVALIDATION_SET_H_
