// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_VARIABLES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_VARIABLES_H_

#include <iosfwd>
#include <optional>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/css_variable_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

// HashTrieNode is the primary building block of an array-mapped trie (AMT),
// an associative array with efficient copy-on-write. AMT supports average-time
// O(log n) and worst-time O(k) get/set (where n is the number of elements
// and k is the number of bits in the key).
//
// We treat the non-guaranteed-zero bits of the AtomicString as a 61-bit string
// (or 29-bit, on 32-bit platforms), starting at the lowest-order bits; it is
// collision-free, and experiments suggest it is no worse than using a hash.
// Based on the lower four bits of this string, we assign each element to one
// out of 16 slots. If two keys have the same slot index, we store a pointer to
// a child node instead of the value; that child node then uses the next four
// bits, and so on.
//
// Copying a tree is as easy as just copying the root pointer and marking the
// nodes as shared. If we need to make modifications to a node in such a tree,
// we need to clone that node and all of its parents (up to the root), but
// all the other nodes can remain as-is, so it is both time- and
// memory-efficient. (We rely on garbage collection to get rid of unused nodes
// eventually.) If the nodes are _not_ shared, we can just mutate the lowest
// node in-place; this creates less garbage. (If we wanted to minimize memory
// usage at the cost of creating a lot more garbage, we could store the slots as
// a sparse array in AdditionalBytes; however, this would mean we would _always_
// have to clone on mutation, and we'd also need fast std::popcount().)
//
// The trie maintains a hash value for the tree as a whole; it is simply the
// XOR of hash(key, value) for all key/value pairs. (XOR allows us to easily
// increment it.) This allows us to reject a large amount of operator== tests
// nearly immediately. (This also means that the common case for operator==
// is equality.) However, note that since some of our types, such as CSSValue,
// can have false negatives in this hash. This is acceptable to us, as we only
// ever use equality here for invalidation, and rare over-invalidation is fine.
//
// We do not support deletions. This could be implemented but requires a bit
// that we implement contraction (i.e., when a child is left with only one node,
// it needs to be pulled up as a value into its parent, possibly recursively).
// However, you can insert nullptr as a value, which we do when we encounter
// “invalid at computed-value time” values.
//
// Non-inherited variables don't need the AMT and could do with a simple
// hash table instead, but for now, it uses the same structure for simplicity.
template <class Data>
class HashTrieNode : public GarbageCollected<HashTrieNode<Data>> {
 public:
  using PassKey = base::PassKey<HashTrieNode<Data>>;
  HashTrieNode() = default;

  // NOTE: Needs to return std::optional to distinguish “did not exist“
  // from “exists, but with nullptr value”.
  std::optional<Data*> Get(const AtomicString& key,
                           unsigned shift = kAlignmentBits) const {
    uintptr_t slot = GetSlot(key, shift);
    if (keys_[slot].IsNull()) {
      if (children_[slot] == nullptr) {
        return std::nullopt;
      } else {
        return children_[slot]->Get(key, shift + kFanoutBits);
      }
    } else {
      if (keys_[slot] == key) {
        return values_[slot].Get();
      } else {
        return std::nullopt;
      }
    }
  }

  // NOTE: You must always store the return value of Set(), since it will
  // point to the new node. This may or may not be the same HashTrieNode
  // as |this|.
  [[nodiscard]] HashTrieNode* Set(const AtomicString& key,
                                  Data* value,
                                  unsigned& hash,
                                  unsigned shift = kAlignmentBits) {
    uintptr_t slot = GetSlot(key, shift);
    if (!keys_[slot].IsNull()) {
      if (keys_[slot] == key) {
        // This key already exists in the map.
        if (base::ValuesEquivalent(values_[slot].Get(), value)) {
          // It was a no-op, so no change needed.
          return this;
        }

        // Overwrite the data.
        UpdateHash(key, values_[slot], hash);
        UpdateHash(key, value, hash);
        HashTrieNode* new_this =
            shared ? MakeGarbageCollected<HashTrieNode<Data>>(PassKey(), *this)
                   : this;
        new_this->values_[slot] = value;
        return new_this;
      } else {
        // There's already a different key here, so we need to split into
        // a new child node.
        UpdateHash(key, value, hash);
        if (shared) {
          HashTrieNode* new_this =
              MakeGarbageCollected<HashTrieNode<Data>>(PassKey(), *this);
          new_this->children_[slot] = CreateSplitNode(
              keys_[slot], values_[slot], key, value, shift + kFanoutBits);
          new_this->keys_[slot] = AtomicString();
          return new_this;
        } else {
          children_[slot] =
              CreateSplitNode(std::move(keys_[slot]), values_[slot], key, value,
                              shift + kFanoutBits);
          keys_[slot] = AtomicString();
          return this;
        }
      }
    } else if (children_[slot]) {
      // There's already a child here, so recurse.
      HashTrieNode* new_child =
          children_[slot]->Set(key, value, hash, shift + kFanoutBits);
      if (new_child == children_[slot]) {
        // It was a no-op or a mutating change, so no change needed for us.
        return this;
      } else {
        // Point this slot to the updated child.
        HashTrieNode* new_this =
            shared ? MakeGarbageCollected<HashTrieNode<Data>>(PassKey(), *this)
                   : this;
        new_this->children_[slot] = new_child;
        return new_this;
      }
    } else {
      // This node is completely empty. Just write the data and we're done.
      UpdateHash(key, value, hash);
      HashTrieNode* new_this =
          shared ? MakeGarbageCollected<HashTrieNode<Data>>(PassKey(), *this)
                 : this;
      new_this->keys_[slot] = key;
      new_this->values_[slot] = value;
      return new_this;
    }
  }

  bool empty() const {
    for (const AtomicString& key : keys_) {
      if (!key.IsNull()) {
        return false;
      }
    }
    for (const Member<HashTrieNode>& child : children_) {
      if (child) {
        return false;
      }
    }
    return true;
  }

  bool operator==(const HashTrieNode<Data>& other) const {
    // We test with memcmp first as a fast path; the common case is
    // that we are entirely equal, since the hash check will
    // instantly reject most _true_ inequalities.
    if (UNSAFE_TODO(memcmp(keys_.data(), other.keys_.data(), sizeof(keys_))) !=
        0) {
      // For AtomicString keys, memcmp() is sufficient as a test.
      return false;
    }
    if (UNSAFE_TODO(memcmp(values_.data(), other.values_.data(),
                           sizeof(values_))) != 0) {
      for (unsigned i = 0; i < kNumSlots; ++i) {
        // NOTE: base::ValuesEquivalent() decompresses the pointers before
        // comparing, which is normally fine, but not when the by far
        // most common case is equality.
        if (values_[i] != other.values_[i]) {
          if (!values_[i] || !other.values_[i]) {
            return false;
          }
          if (*values_[i] != *other.values_[i]) {
            return false;
          }
        }
      }
    }
    if (UNSAFE_TODO(memcmp(children_.data(), other.children_.data(),
                           sizeof(children_))) != 0) {
      for (unsigned i = 0; i < kNumSlots; ++i) {
        if (children_[i] != other.children_[i]) {
          if (!children_[i] || !other.children_[i]) {
            return false;
          }
          if (*children_[i] != *other.children_[i]) {
            return false;
          }
        }
      }
    }
    return true;
  }

  void Trace(Visitor* visitor) const {
    for (const Member<Data>& value : values_) {
      visitor->Trace(value);
    }
    for (const Member<HashTrieNode>& child : children_) {
      visitor->Trace(child);
    }
  }

  // Mark this node (and all of its children) as used by multiple trees,
  // such that any further modifications will be copy-on-write.
  void MakeShared() {
    if (shared) {
      return;
    }
    for (const Member<HashTrieNode>& child : children_) {
      if (child) {
        child->MakeShared();
      }
    }
    shared = true;
  }

  void CollectNames(HashSet<AtomicString>& names) const {
    for (const AtomicString& key : keys_) {
      if (!key.IsNull()) {
        names.insert(key);
      }
    }
    for (const Member<HashTrieNode>& child : children_) {
      if (child) {
        child->CollectNames(names);
      }
    }
  }

  // For debugging/logging.
  template <class Printer>
  std::ostream& Serialize(const Printer& printer, std::ostream& stream) const {
    for (unsigned i = 0; i < kNumSlots; ++i) {
      if (!keys_[i].IsNull()) {
        stream << keys_[i] << ": " << printer(values_[i]) << ", ";
      }
    }
    for (const Member<HashTrieNode>& child : children_) {
      if (child) {
        child->Serialize(printer, stream);
      }
    }
    return stream;
  }

  // Copy constructor.
  HashTrieNode(PassKey, const HashTrieNode& other) : keys_(other.keys_) {
    // It is legal to copy Member<> with memcpy() here, since we are in a
    // constructor. (We could use an initializer, but Clang doesn't
    // manage to combine the loads and stores into larger parts.)
    UNSAFE_TODO(memcpy(&values_, &other.values_, sizeof(values_)));
    UNSAFE_TODO(memcpy(&children_, &other.children_, sizeof(children_)));
  }

 private:
  static constexpr unsigned kFanoutBits = 4;
  static constexpr unsigned kNumSlots = 1 << kFanoutBits;
  static constexpr unsigned kAlignmentBits = sizeof(AtomicString) == 8 ? 4 : 3;

  static unsigned GetSlot(const AtomicString& key, unsigned shift) {
#if DCHECK_IS_ON()
    uintptr_t alignment_bits =
        reinterpret_cast<uintptr_t>(key.Impl()) & ((1u << kAlignmentBits) - 1);
    DCHECK_EQ(0u, alignment_bits)
        << "AtomicString's Impl() was unexpectedly unaligned";
    DCHECK_LT(shift, CHAR_BIT * sizeof(uintptr_t));
#endif
    return (reinterpret_cast<uintptr_t>(key.Impl()) >> shift) & (kNumSlots - 1);
  }

  // Add or remove the given key/value pair from the given hash.
  static void UpdateHash(const AtomicString& key, Data* value, unsigned& hash) {
    if (value) {
      hash ^= WTF::HashInts(key.Hash(), value->Hash());
    }
  }

  // Create a node that has exactly two keys. Used when there is a value
  // collision and we need to split the child into a new node.
  HashTrieNode* CreateSplitNode(AtomicString key1,
                                Data* value1,
                                AtomicString key2,
                                Data* value2,
                                unsigned shift) {
    HashTrieNode* node = MakeGarbageCollected<HashTrieNode>();
    uintptr_t slot1 = GetSlot(key1, shift);
    uintptr_t slot2 = GetSlot(key2, shift);
    if (slot1 == slot2) {
      node->children_[slot1] =
          CreateSplitNode(std::move(key1), value1, std::move(key2), value2,
                          shift + kFanoutBits);
    } else {
      node->keys_[slot1] = std::move(key1);
      node->values_[slot1] = value1;
      node->keys_[slot2] = std::move(key2);
      node->values_[slot2] = value2;
    }
    return node;
  }

  // NOTE: We could have stored values_ and children_ in an union.
  // However, due to Oilpan rules, this would mean we could never transition
  // a node from a value to a child without doing copy-on-write on it
  // (as tracing might happen in a different thread, tracing an union needs to
  // always take the same path during the entire lifetime of the object, unless
  // everything is atomic). We accept using 33% more RAM in steady state to
  // reduce the amount of garbage here.
  //
  // Note that since we don't have an union, we could in theory have _both_
  // a value and a child in the same slot. This would mean slightly better
  // memory usage; however, it doesn't help speed much, and it creates the
  // thorny problem of which element should remain in the parent when the others
  // are moved down to the child (it needs to somehow stay consistent
  // in order to not create problems for operator==).
  std::array<AtomicString, kNumSlots> keys_;
  std::array<Member<Data>, kNumSlots> values_;
  std::array<Member<HashTrieNode>, kNumSlots> children_;

  bool shared = false;
};

// Contains values for custom properties.
//
// Each custom property has "variable data" and optionally a "variable value".
//
// * Data:  A CSSVariableData that contains the tokens used for substitution.
// * Value: An optional CSSValue that may be present if the custom property
//          is registered with a non-universal syntax descriptor.
//
// Note that StyleVariables may explicitly contain a nullptr value for a given
// custom property. This is necessary to be able to mark variables that become
// invalid at computed-value time [1] as such.
//
// If StyleVariables does not contain an entry at all for a given property,
// std::nullopt is returned. This allows us to differentiate between the case
// where we want to try to find the variable elsewhere (e.g. StyleInitialData,
// in the case of std::nullopt), or return nullptr without looking further.
//
// There is currently no way to erase an entry from StyleVariables (see above).
// This means that non-implicit initial/inherited values must be explicitly
// stored.
//
// [1] https://drafts.csswg.org/css-variables/#invalid-at-computed-value-time
class CORE_EXPORT StyleVariables {
  DISALLOW_NEW();

 public:
  StyleVariables()
      : data_root_(MakeGarbageCollected<HashTrieNode<CSSVariableData>>()),
        values_root_(MakeGarbageCollected<HashTrieNode<const CSSValue>>()) {}
  StyleVariables(const StyleVariables& other)
      : data_root_(other.data_root_),
        values_root_(other.values_root_),
        data_hash_(other.data_hash_),
        values_hash_(other.values_hash_) {
    data_root_->MakeShared();
    values_root_->MakeShared();
  }
  StyleVariables(StyleVariables&&) = default;
  StyleVariables& operator=(const StyleVariables& other) {
    data_root_ = other.data_root_;
    values_root_ = other.values_root_;
    data_hash_ = other.data_hash_;
    values_hash_ = other.values_hash_;
    data_root_->MakeShared();
    values_root_->MakeShared();
    return *this;
  }
  StyleVariables& operator=(StyleVariables&&) = default;

  void Trace(Visitor* visitor) const {
    visitor->Trace(data_root_);
    visitor->Trace(values_root_);
  }

  bool operator==(const StyleVariables& other) const;
  bool operator!=(const StyleVariables& other) const {
    return !(*this == other);
  }

  std::optional<CSSVariableData*> GetData(const AtomicString& name) const {
    return data_root_->Get(name);
  }
  std::optional<const CSSValue*> GetValue(const AtomicString& name) const {
    return values_root_->Get(name);
  }
  void SetData(const AtomicString&, CSSVariableData*);
  void SetValue(const AtomicString&, const CSSValue*);

  bool IsEmpty() const;
  void CollectNames(HashSet<AtomicString>&) const;

 private:
  // mutable so that operator== can deduplicate them.
  mutable Member<HashTrieNode<CSSVariableData>> data_root_;
  mutable Member<HashTrieNode<const CSSValue>> values_root_;

  // See HashTrieNode class comment.
  unsigned data_hash_ = 0;
  unsigned values_hash_ = 0;

  friend CORE_EXPORT std::ostream& operator<<(std::ostream& stream,
                                              const StyleVariables& variables);
};

CORE_EXPORT std::ostream& operator<<(std::ostream& stream,
                                     const StyleVariables& variables);

template <typename T>
struct ThreadingTrait<
    T,
    std::enable_if_t<
        std::is_base_of<blink::HashTrieNode<CSSVariableData>, T>::value>> {
  static constexpr ThreadAffinity kAffinity = kMainThreadOnly;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_VARIABLES_H_
