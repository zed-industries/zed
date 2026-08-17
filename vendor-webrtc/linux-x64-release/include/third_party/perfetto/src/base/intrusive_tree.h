/*
 * Copyright (C) 2025 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_BASE_INTRUSIVE_TREE_H_
#define SRC_BASE_INTRUSIVE_TREE_H_

#include <cstddef>
#include <cstdint>
#include <functional>

#include "perfetto/base/logging.h"

// An intrusive tree implementation, inspired from BSD kernel's tree.h
// Unlike std::set<>, the nodes being inserted into the tree need to explicitly
// declare a RBNode structure (one for each tree they are part of).
// The user must specify a TreeTraits for each tree the struct is part of.
// The traits struct defines the type of the key and how to get to the node
// entry from the outer object.
// Usage example:
// class Person {
//  public:
//   struct Traits {
//     using KeyType = std::string;
//     static const KeyType& GetKey(const Person& p) { return p->unique_id; }
//     static constexpr size_t NodeOffset() { return offsetof(Person, node); }
//   };
//   std::string unique_id;
//   std::string name;
//   std::string surname;
//   IntrusiveTreeNode node{};
// }
//  IntrusiveTree<Person, Person::Traits> tree;
//  tree.insert(&person1);
//  ...

namespace perfetto::base {

namespace internal {

enum RBColor : uint8_t {
  BLACK = 0,
  RED = 1,
};

struct RBNode {
  RBNode* left = nullptr;
  RBNode* right = nullptr;
  RBNode* parent = nullptr;
  RBColor color = RBColor::BLACK;
};

void RBInsertColor(RBNode** root, RBNode* elm);
void RBRemove(RBNode** root, RBNode* elm);

// Returns nullptr after reaching the last leaf (the max element).
const RBNode* RBNext(const RBNode* node);

// KeyCompare tries first to use the CompareKey function in Traits, if present.
// The signature of that function is int(const KeyType&, const KeyType&).
// If the comparator function doesn't exist, falls backk on std::less<KeyType>.
template <
    class Traits,
    class = std::enable_if_t<std::is_function_v<typename Traits::CompareKey>,
                             void> >
int KeyCompare(const typename Traits::KeyType& k1,
               const typename Traits::KeyType& k2) {
  return Traits::CompareKey(k1, k2);
}

// SFINAE fallback on std::less<KeyType>
template <class Traits>
int KeyCompare(const typename Traits::KeyType& k1,
               const typename Traits::KeyType& k2) {
  std::less<typename Traits::KeyType> less_cmp;
  return less_cmp(k1, k2) ? -1 : (less_cmp(k2, k1) ? 1 : 0);
}

}  // namespace internal

using IntrusiveTreeNode = internal::RBNode;

// T is the class that has one or more IntrusiveTreeNode as fiels.
// Traits defines the key type, getter and offset between node and T.
// Traits is separate to allow the same T to be part of different trees (which
// necessitate a different Traits, at very least for the offset).
template <typename T, typename Traits>
class IntrusiveTree {
 public:
  using Key = typename Traits::KeyType;

  class Iterator {
   public:
    Iterator() = default;
    explicit Iterator(const internal::RBNode* node) : node_(node) {}
    ~Iterator() = default;
    Iterator(const Iterator&) = default;
    Iterator& operator=(const Iterator&) = default;
    Iterator(Iterator&&) noexcept = default;
    Iterator& operator=(Iterator&&) noexcept = default;

    bool operator==(const Iterator& o) const { return node_ == o.node_; }
    bool operator!=(const Iterator& o) const { return !(*this == o); }
    const T* operator->() const { return entryof(node_); }
    const T& operator*() const {
      PERFETTO_DCHECK(node_ != nullptr);
      return *operator->();
    }
    T* operator->() { return const_cast<T*>(entryof(node_)); }
    T& operator*() {
      PERFETTO_DCHECK(node_ != nullptr);
      return *operator->();
    }
    explicit operator bool() const { return node_ != nullptr; }

    Iterator& operator++() {
      node_ = internal::RBNext(node_);
      return *this;
    }

   private:
    const internal::RBNode* node_ = nullptr;
  };  // Iterator

  using value_type = T;
  using const_pointer = const T*;
  using const_iterator = Iterator;

  std::pair<Iterator, bool> Insert(T& entry) {
    // The insertion preamble is inlined because it's few instructions and
    // out-lining it would require std::function indirections for getting the
    // key and the comparator.
    int comp = 0;
    internal::RBNode* tmp = root_;
    internal::RBNode* parent = nullptr;
    internal::RBNode* const entry_node = nodeof(&entry);
    while (tmp) {
      parent = tmp;
      comp = key_compare(entry_node, parent);
      if (comp < 0) {
        tmp = tmp->left;
      } else if (comp > 0) {
        tmp = tmp->right;
      } else {
        return {Iterator(tmp), false};  // The key exists already.
      }
    }  // while(tmp)
    entry_node->left = entry_node->right = nullptr;
    entry_node->parent = parent;
    entry_node->color = internal::RBColor::RED;
    if (parent) {
      if (comp < 0) {
        PERFETTO_DCHECK(parent->left == nullptr);
        parent->left = entry_node;
      } else {
        PERFETTO_DCHECK(parent->right == nullptr);
        parent->right = entry_node;
      }
    } else {
      root_ = entry_node;
    }
    internal::RBInsertColor(&root_, entry_node);
    ++size_;
    return {Iterator(entry_node), true};
  }

  Iterator Find(const Key& key) const {
    internal::RBNode* tmp = root_;
    while (tmp) {
      int comp =
          internal::KeyCompare<Traits>(key, Traits::GetKey(*entryof(tmp)));
      if (comp < 0) {
        tmp = tmp->left;
      } else if (comp > 0) {
        tmp = tmp->right;
      } else {
        return Iterator(tmp);
      }
    }
    return Iterator(nullptr);
  }

  bool Remove(const Key& key) {
    Iterator it = Find(key);
    if (!it)
      return false;
    internal::RBRemove(&root_, nodeof(std::addressof(*it)));
    --size_;
    return true;
  }

  Iterator Remove(T& entry) { return Remove(Iterator(nodeof(&entry))); }

  Iterator Remove(Iterator it) {
    Iterator next = it;
    ++next;
    internal::RBRemove(&root_, nodeof(std::addressof(*it)));
    --size_;
    return next;
  }

  size_t Size() const { return size_; }

  Iterator begin() const {
    const internal::RBNode* node = root_;
    while (node && node->left)
      node = node->left;
    return Iterator(node);
  }

  Iterator end() const { return Iterator(nullptr); }

 private:
  static constexpr size_t off_ = Traits::NodeOffset();
  static internal::RBNode* nodeof(T* t) {
    PERFETTO_DCHECK(t != nullptr);
    return reinterpret_cast<internal::RBNode*>(reinterpret_cast<uintptr_t>(t) +
                                               off_);
  }
  static const T* entryof(const internal::RBNode* n) {
    PERFETTO_DCHECK(n != nullptr);
    return reinterpret_cast<T*>(reinterpret_cast<uintptr_t>(n) - off_);
  }
  static int key_compare(const internal::RBNode* node_a,
                         const internal::RBNode* node_b) {
    auto* a = entryof(node_a);
    auto* b = entryof(node_b);
    return internal::KeyCompare<Traits>(Traits::GetKey(*a), Traits::GetKey(*b));
  }

  internal::RBNode* root_ = nullptr;
  size_t size_ = 0;
};

}  // namespace perfetto::base

#endif  // SRC_BASE_INTRUSIVE_TREE_H_
