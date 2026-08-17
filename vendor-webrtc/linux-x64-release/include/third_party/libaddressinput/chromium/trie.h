// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_TRIE_H_
#define THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_TRIE_H_

#include <stdint.h>
#include <map>
#include <set>
#include <vector>

namespace autofill {

// A prefix search tree. Can return all objects whose keys start with a prefix
// byte sequence.
//
// Maps keys to multiple objects. This property is useful when mapping region
// names to region objects. For example, there's a "St. Petersburg" in Florida,
// and there's a "St. Petersburg" in Russia. A lookup for "St. Petersburg" key
// should return two distinct objects.
template <typename T>
class Trie {
 public:
  Trie();
  ~Trie();

  // Returns true if no data was added in AddDataForKey().
  bool empty() const { return data_list_.empty() && sub_nodes_.empty(); }

  // Adds a mapping from the 0 terminated |key| to |data_item|. Can be called
  // with the same |key| multiple times.
  void AddDataForKey(const std::vector<uint8_t>& key, const T& data_item);

  // Adds all objects whose keys start with the 0 terminated |key_prefix| to the
  // |results| parameter. The |results| parameter should not be NULL.
  void FindDataForKeyPrefix(const std::vector<uint8_t>& key_prefix,
                            std::set<T>* results) const;

 private:
  // All objects for this node in the Trie. This field is a collection to enable
  // mapping the same key to multiple objects.
  std::set<T> data_list_;

  // Trie sub nodes. The root Trie stores the objects for the empty key. The
  // children of the root Trie store the objects for the one-byte keys. The
  // grand-children of the root Trie store the objects for the two-byte keys,
  // and so on.
  std::map<uint8_t, Trie<T> > sub_nodes_;
};

}  // namespace autofill

#endif  // THIRD_PARTY_LIBADDRESSINPUT_CHROMIUM_TRIE_H_
