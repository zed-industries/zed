// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_VALUE_ITERATORS_H_
#define BASE_VALUE_ITERATORS_H_

#include <memory>
#include <string>
#include <utility>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/flat_map.h"

namespace base {

class Value;

namespace detail {

using DictStorage = base::flat_map<std::string, std::unique_ptr<Value>>;

// This iterator closely resembles DictStorage::iterator, with one
// important exception. It abstracts the underlying unique_ptr away, meaning its
// reference type is a std::pair<const std::string&, Value&>, so that callers
// have read-write access without incurring a copy.
class BASE_EXPORT dict_iterator {
 public:
  using difference_type = DictStorage::iterator::difference_type;
  using value_type = std::pair<const std::string&, Value&>;
  using reference = value_type;
  using iterator_category = std::bidirectional_iterator_tag;

  class pointer {
   public:
    explicit pointer(const reference& ref);
    pointer(const pointer& ptr);
    pointer& operator=(const pointer& ptr) = delete;

    reference* operator->() { return &ref_; }

   private:
    reference ref_;
  };

  constexpr dict_iterator() = default;
  explicit dict_iterator(DictStorage::iterator dict_iter);
  dict_iterator(const dict_iterator& dict_iter);
  dict_iterator& operator=(const dict_iterator& dict_iter);
  ~dict_iterator();

  reference operator*() const;
  pointer operator->() const;

  dict_iterator& operator++();
  dict_iterator operator++(int);
  dict_iterator& operator--();
  dict_iterator operator--(int);

  BASE_EXPORT friend bool operator==(const dict_iterator& lhs,
                                     const dict_iterator& rhs);
  BASE_EXPORT friend bool operator!=(const dict_iterator& lhs,
                                     const dict_iterator& rhs);

  // Currently, there is no easy way to friend Value::Dict. Once dictionary
  // storage is updated to not require a proxy iterator, the implementation can
  // be folded into //base/values.h and a standard friend declaration can be
  // used instead.
  const DictStorage::iterator& GetUnderlyingIteratorDoNotUse() const
      LIFETIME_BOUND {
    return dict_iter_;
  }

 private:
  DictStorage::iterator dict_iter_;
};

// This iterator closely resembles DictStorage::const_iterator, with one
// important exception. It abstracts the underlying unique_ptr away, meaning its
// reference type is a std::pair<const std::string&, const Value&>, so that
// callers have read-only access without incurring a copy.
class BASE_EXPORT const_dict_iterator {
 public:
  using difference_type = DictStorage::const_iterator::difference_type;
  using value_type = std::pair<const std::string&, const Value&>;
  using reference = value_type;
  using iterator_category = std::bidirectional_iterator_tag;

  class pointer {
   public:
    explicit pointer(const reference& ref);
    pointer(const pointer& ptr);
    pointer& operator=(const pointer& ptr) = delete;

    const reference* operator->() const { return &ref_; }

   private:
    const reference ref_;
  };

  constexpr const_dict_iterator() = default;
  explicit const_dict_iterator(DictStorage::const_iterator dict_iter);
  const_dict_iterator(const const_dict_iterator& dict_iter);
  const_dict_iterator& operator=(const const_dict_iterator& dict_iter);
  ~const_dict_iterator();

  reference operator*() const;
  pointer operator->() const;

  const_dict_iterator& operator++();
  const_dict_iterator operator++(int);
  const_dict_iterator& operator--();
  const_dict_iterator operator--(int);

  BASE_EXPORT friend bool operator==(const const_dict_iterator& lhs,
                                     const const_dict_iterator& rhs);
  BASE_EXPORT friend bool operator!=(const const_dict_iterator& lhs,
                                     const const_dict_iterator& rhs);

  // Currently, there is no easy way to friend Value::Dict. Once dictionary
  // storage is updated to not require a proxy iterator, the implementation can
  // be folded into //base/values.h and a standard friend declaration can be
  // used instead.
  const DictStorage::const_iterator& GetUnderlyingIteratorDoNotUse() {
    return dict_iter_;
  }

 private:
  DictStorage::const_iterator dict_iter_;
};

}  // namespace detail

}  // namespace base

#endif  // BASE_VALUE_ITERATORS_H_
