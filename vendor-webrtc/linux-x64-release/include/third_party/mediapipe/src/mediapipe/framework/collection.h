// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_COLLECTION_H_
#define MEDIAPIPE_FRAMEWORK_COLLECTION_H_

#include <cstdlib>
#include <iterator>
#include <map>
#include <set>
#include <string>
#include <typeinfo>
#include <vector>

#include "absl/base/macros.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/memory/memory.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/tool/tag_map.h"
#include "mediapipe/framework/tool/tag_map_helper.h"
#include "mediapipe/framework/tool/validate_name.h"
#include "mediapipe/framework/type_map.h"

namespace mediapipe {
namespace internal {

// A class to handle errors that occur in Collection.  For most
// collections, these errors should be fatal.  However, for a collection
// more like PacketTypeSet, the errors should be deferred and handled
// later.
//
// This class is thread compatible.
template <typename T>
struct CollectionErrorHandlerFatal {
  // An error occurred during object lookup for the provided tag and
  // index.  The returned object reference will be provided instead.
  //
  // Since there isn't any state and we're not returning anything, we
  // get away with only one version of this function (which is const
  // but returns a non-const reference).
  T& GetFallback(const absl::string_view tag, int index) const {
    ABSL_LOG(FATAL) << "Failed to get tag \"" << tag << "\" index " << index;
    std::abort();
  }
};

enum class CollectionStorage { kStoreValue = 0, kStorePointer };

// A collection of objects of type T.
//
// If storage == kStorePointer then T* will be stored instead of T, but
// the accessor functions will still return T types.  The T objects must
// be owned elsewhere and remain alive as long as the collection is used.
// To set the pointers use the GetPtr() function.
//
// The ErrorHandler object allows errors to be deferred to a later time.
//
// This class is thread compatible as long as the ErrorHandler object is also
// thread compatible.
template <typename T,
          CollectionStorage storage = CollectionStorage::kStoreValue,
          typename ErrorHandler = CollectionErrorHandlerFatal<T>>
class Collection {
 private:
  template <typename ItType>
  class DoubleDerefIterator;

 public:
  using value_type = T;

  // The iterator is over value_type, requiring a double dereference if
  // storage == kStorePointer.
  using iterator =
      typename std::conditional<storage == CollectionStorage::kStorePointer,
                                DoubleDerefIterator<value_type>,
                                value_type*>::type;
  using const_iterator =
      typename std::conditional<storage == CollectionStorage::kStorePointer,
                                DoubleDerefIterator<const value_type>,
                                const value_type*>::type;
  using difference_type = ptrdiff_t;
  using size_type = size_t;
  using pointer = value_type*;
  using reference = value_type&;

  // The type that is stored by data_;
  using stored_type =
      typename std::conditional<storage == CollectionStorage::kStorePointer,
                                value_type*, value_type>::type;

  // Collection must be initialized on construction.
  Collection() = delete;
  Collection(const Collection&) = delete;
  Collection& operator=(const Collection&) = delete;
  // Makes a Collection using the given TagMap (which should be shared
  // between collections).
  // Refer to mediapipe::tool::CreateTagMap for examples of how to construct a
  // collection from a vector of "TAG:<index>:name" strings, or from an integer
  // number of indexes, etc.
  explicit Collection(std::shared_ptr<tool::TagMap> tag_map);
  // Makes a Collection using the information in the TagAndNameInfo.
  ABSL_DEPRECATED("Use Collection(tool::TagMap)")
  explicit Collection(const tool::TagAndNameInfo& info);
  // Convenience constructor which initializes a collection to use
  // indexes and have num_entries inputs.
  ABSL_DEPRECATED("Use Collection(tool::TagMap)")
  explicit Collection(int num_entries);
  // Convenience constructor which initializes a collection to use tags
  // with the given names.
  // Note: initializer_list constructor should not be marked explicit.
  ABSL_DEPRECATED("Use Collection(tool::TagMap)")
  Collection(const std::initializer_list<std::string>& tag_names);

  // Access the data at a given CollectionItemId.  This is the most efficient
  // way to access data within the collection.
  //
  // Do not assume that Index(2) == Get(collection.TagMap()->BeginId() + 2).
  value_type& Get(CollectionItemId id);
  const value_type& Get(CollectionItemId id) const;

  // Convenience functions.
  value_type& Get(absl::string_view tag, int index);
  const value_type& Get(absl::string_view tag, int index) const;

  // Equivalent to Get("", index);
  value_type& Index(int index);
  const value_type& Index(int index) const;

  // Equivalent to Get(tag, 0);
  value_type& Tag(absl::string_view tag);
  const value_type& Tag(absl::string_view tag) const;

  // These functions only exist for collections with storage ==
  // kStorePointer.  GetPtr returns the stored ptr value rather than
  // the value_type.  The non-const version returns a reference so that
  // the pointer can be set.
  value_type*& GetPtr(CollectionItemId id);
  // Const version returns a pointer to a const value (a const-ref to
  // a pointer wouldn't be useful in this context).
  const value_type* GetPtr(CollectionItemId id) const;

  // Returns true if the collection has a tag other than "".
  // TODO Deprecate and remove this function.
  bool UsesTags() const;

  // Returns a description of the collection.
  std::string DebugString() const;

  // Return the tag_map.
  const std::shared_ptr<tool::TagMap>& TagMap() const;

  // Iteration functions for use of the collection in a range based
  // for loop.  The items are provided in sorted tag order with indexes
  // sequential within tags.
  iterator begin();
  iterator end();
  const_iterator begin() const;
  const_iterator end() const;

  // Returns the error handler object.
  const ErrorHandler& GetErrorHandler() const { return error_handler_; }

  ////////////////////////////////////////
  // The remaining public functions directly call their equivalent
  // in tool::TagMap.  They are guaranteed to be equivalent for any
  // Collection initialized using an equivalent tool::TagMap.
  ////////////////////////////////////////

  // Returns true if the provided tag is available (not necessarily set yet).
  bool HasTag(const absl::string_view tag) const {
    return tag_map_->HasTag(tag);
  }

  // Returns the number of entries in this collection.
  int NumEntries() const { return tag_map_->NumEntries(); }

  // Returns the number of entries with the provided tag.
  int NumEntries(const absl::string_view tag) const {
    return tag_map_->NumEntries(tag);
  }

  // Get the id for the tag and index.  This id is guaranteed valid for
  // any Collection which was initialized with an equivalent tool::TagMap.
  // If the tag or index are invalid then an invalid CollectionItemId
  // is returned (with id.IsValid() == false).
  //
  // The id for indexes within the same tag are guaranteed to
  // be sequential.  Meaning, if tag "BLAH" has 3 indexes, then
  // ++GetId("BLAH", 1) == GetId("BLAH", 2)
  // However, be careful in using this fact, as it circumvents the
  // validity checks in GetId() (i.e. ++GetId("BLAH", 2) looks like it
  // is valid, while GetId("BLAH", 3) is not valid).
  CollectionItemId GetId(const absl::string_view tag, int index) const {
    return tag_map_->GetId(tag, index);
  }

  // Returns the names of the tags in this collection.
  std::set<std::string> GetTags() const { return tag_map_->GetTags(); }

  // Get a tag and index for the specified id.  If the id is not valid,
  // then {"", -1} will be returned.
  std::pair<std::string, int> TagAndIndexFromId(CollectionItemId id) const {
    return tag_map_->TagAndIndexFromId(id);
  }

  // The CollectionItemId corresponding to the first element in the collection.
  // Looping over all elements can be done as follows.
  //   for (CollectionItemId id = collection.BeginId();
  //        id < collection.EndId(); ++id) {
  //   }
  // However, if only one collection is involved, prefer using a range
  // based for loop.
  //   for (Packet packet : Inputs()) {
  //   }
  CollectionItemId BeginId() const { return tag_map_->BeginId(); }
  // The CollectionItemId corresponding to an element immediately after
  // the last element of the collection.
  CollectionItemId EndId() const { return tag_map_->EndId(); }

  // Same as BeginId()/EndId() but for only one tag.  If the tag doesn't
  // exist then an invalid CollectionItemId is returned.  It is guaranteed
  // that a loop constructed in this way will successfully not be entered
  // for invalid tags.
  //   for (CollectionItemId id = collection.BeginId(tag);
  //        id < collection.EndId(tag); ++id) {
  //   }
  CollectionItemId BeginId(const absl::string_view tag) const {
    return tag_map_->BeginId(tag);
  }
  CollectionItemId EndId(const absl::string_view tag) const {
    return tag_map_->EndId(tag);
  }

  // Equal Collections contain equal mappings and equal elements.
  bool operator==(const Collection<T>& other) const {
    if (tag_map_->Mapping() != other.TagMap()->Mapping()) {
      return false;
    }
    for (CollectionItemId id = BeginId(); id < EndId(); ++id) {
      if (Get(id) != other.Get(id)) {
        return false;
      }
    }
    return true;
  }
  bool operator!=(const Collection<T>& other) const {
    return !(*this == other);
  }

 private:
  // An iterator which is identical to ItType** except that the
  // dereference operator (operator*) does a double dereference and
  // returns an ItType.
  //
  // This class is thread compatible.
  template <typename ItType>
  class DoubleDerefIterator {
   public:
    using iterator_category = std::random_access_iterator_tag;
    using value_type = ItType;
    using difference_type = std::ptrdiff_t;
    using pointer = ItType*;
    using reference = ItType&;

    DoubleDerefIterator() : ptr_(nullptr) {}

    reference operator*() { return **ptr_; }

    pointer operator->() { return *ptr_; }

    reference operator[](difference_type d) { return **(ptr_ + d); }

    // Member operators.
    DoubleDerefIterator& operator++() {
      ++ptr_;
      return *this;
    }
    DoubleDerefIterator operator++(int) {
      DoubleDerefIterator output(ptr_);
      ++ptr_;
      return output;
    }
    DoubleDerefIterator& operator--() {
      --ptr_;
      return *this;
    }
    DoubleDerefIterator operator--(int) {
      DoubleDerefIterator output(ptr_);
      --ptr_;
      return output;
    }
    DoubleDerefIterator& operator+=(difference_type d) {
      ptr_ += d;
      return *this;
    }
    DoubleDerefIterator& operator-=(difference_type d) {
      ptr_ -= d;
      return *this;
    }

    // Non-member binary operators.
    friend bool operator==(DoubleDerefIterator lhs, DoubleDerefIterator rhs) {
      return lhs.ptr_ == rhs.ptr_;
    }
    friend bool operator!=(DoubleDerefIterator lhs, DoubleDerefIterator rhs) {
      return lhs.ptr_ != rhs.ptr_;
    }
    friend bool operator<(DoubleDerefIterator lhs, DoubleDerefIterator rhs) {
      return lhs.ptr_ < rhs.ptr_;
    }
    friend bool operator<=(DoubleDerefIterator lhs, DoubleDerefIterator rhs) {
      return lhs.ptr_ <= rhs.ptr_;
    }
    friend bool operator>(DoubleDerefIterator lhs, DoubleDerefIterator rhs) {
      return lhs.ptr_ > rhs.ptr_;
    }
    friend bool operator>=(DoubleDerefIterator lhs, DoubleDerefIterator rhs) {
      return lhs.ptr_ >= rhs.ptr_;
    }

    friend DoubleDerefIterator operator+(DoubleDerefIterator lhs,
                                         difference_type d) {
      return lhs.ptr_ + d;
    }
    friend DoubleDerefIterator operator+(difference_type d,
                                         DoubleDerefIterator rhs) {
      return rhs.ptr_ + d;
    }
    friend DoubleDerefIterator& operator-(DoubleDerefIterator lhs,
                                          difference_type d) {
      return lhs.ptr_ - d;
    }
    friend difference_type operator-(DoubleDerefIterator lhs,
                                     DoubleDerefIterator rhs) {
      return lhs.ptr_ - rhs.ptr_;
    }

   private:
    explicit DoubleDerefIterator(ItType* const* data) : ptr_(data) {}

    ItType* const* ptr_;

    friend class Collection;
  };

  // TagMap for the collection.
  std::shared_ptr<tool::TagMap> tag_map_;

  // Indexed by Id.  Use an array directly so that the type does not
  // have to be copy constructable.  The array has tag_map_->NumEntries()
  // elements.
  std::unique_ptr<stored_type[]> data_;

  // A class which allows errors to be reported flexibly.  The default
  // instantiation performs a ABSL_LOG(FATAL) and does not have any member
  // variables (zero size).
  ErrorHandler error_handler_;
};

// Definitions of templated functions for Collection.

template <typename T, CollectionStorage storage, typename ErrorHandler>
Collection<T, storage, ErrorHandler>::Collection(
    std::shared_ptr<tool::TagMap> tag_map)
    : tag_map_(std::move(tag_map)) {
  if (tag_map_->NumEntries() != 0) {
    data_ = absl::make_unique<stored_type[]>(tag_map_->NumEntries());
  }
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
Collection<T, storage, ErrorHandler>::Collection(
    const tool::TagAndNameInfo& info)
    : Collection(tool::TagMap::Create(info).value()) {}

template <typename T, CollectionStorage storage, typename ErrorHandler>
Collection<T, storage, ErrorHandler>::Collection(const int num_entries)
    : Collection(tool::CreateTagMap(num_entries).value()) {}

template <typename T, CollectionStorage storage, typename ErrorHandler>
Collection<T, storage, ErrorHandler>::Collection(
    const std::initializer_list<std::string>& tag_names)
    : Collection(tool::CreateTagMapFromTags(tag_names).value()) {}

template <typename T, CollectionStorage storage, typename ErrorHandler>
bool Collection<T, storage, ErrorHandler>::UsesTags() const {
  auto& mapping = tag_map_->Mapping();
  if (mapping.size() > 1) {
    // At least one tag is not "".
    return true;
  }
  if (mapping.empty()) {
    // The mapping is empty, it doesn't use tags.
    return false;
  }
  // If the one tag present is non-empty then we are using tags.
  return !mapping.begin()->first.empty();
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
typename Collection<T, storage, ErrorHandler>::value_type&
Collection<T, storage, ErrorHandler>::Get(CollectionItemId id) {
  ABSL_CHECK_LE(BeginId(), id);
  ABSL_CHECK_LT(id, EndId());
  return begin()[id.value()];
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
const typename Collection<T, storage, ErrorHandler>::value_type&
Collection<T, storage, ErrorHandler>::Get(CollectionItemId id) const {
  ABSL_CHECK_LE(BeginId(), id);
  ABSL_CHECK_LT(id, EndId());
  return begin()[id.value()];
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
typename Collection<T, storage, ErrorHandler>::value_type*&
Collection<T, storage, ErrorHandler>::GetPtr(CollectionItemId id) {
  static_assert(storage == CollectionStorage::kStorePointer,
                "mediapipe::internal::Collection<T>::GetPtr() is only "
                "available for collections that were defined with template "
                "argument storage == CollectionStorage::kStorePointer.");
  ABSL_CHECK_LE(BeginId(), id);
  ABSL_CHECK_LT(id, EndId());
  return data_[id.value()];
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
const typename Collection<T, storage, ErrorHandler>::value_type*
Collection<T, storage, ErrorHandler>::GetPtr(CollectionItemId id) const {
  static_assert(storage == CollectionStorage::kStorePointer,
                "mediapipe::internal::Collection<T>::GetPtr() is only "
                "available for collections that were defined with template "
                "argument storage == CollectionStorage::kStorePointer.");
  ABSL_CHECK_LE(BeginId(), id);
  ABSL_CHECK_LT(id, EndId());
  return data_[id.value()];
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
typename Collection<T, storage, ErrorHandler>::value_type&
Collection<T, storage, ErrorHandler>::Get(const absl::string_view tag,
                                          int index) {
  CollectionItemId id = GetId(tag, index);
  if (!id.IsValid()) {
    return error_handler_.GetFallback(tag, index);
  }
  return begin()[id.value()];
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
const typename Collection<T, storage, ErrorHandler>::value_type&
Collection<T, storage, ErrorHandler>::Get(const absl::string_view tag,
                                          int index) const {
  CollectionItemId id = GetId(tag, index);
  if (!id.IsValid()) {
    return error_handler_.GetFallback(tag, index);
  }
  return begin()[id.value()];
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
typename Collection<T, storage, ErrorHandler>::value_type&
Collection<T, storage, ErrorHandler>::Index(int index) {
  return Get("", index);
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
const typename Collection<T, storage, ErrorHandler>::value_type&
Collection<T, storage, ErrorHandler>::Index(int index) const {
  return Get("", index);
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
typename Collection<T, storage, ErrorHandler>::value_type&
Collection<T, storage, ErrorHandler>::Tag(const absl::string_view tag) {
  return Get(tag, 0);
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
const typename Collection<T, storage, ErrorHandler>::value_type&
Collection<T, storage, ErrorHandler>::Tag(const absl::string_view tag) const {
  return Get(tag, 0);
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
std::string Collection<T, storage, ErrorHandler>::DebugString() const {
  std::string output =
      absl::StrCat("Collection of \"", MediaPipeTypeStringOrDemangled<T>(),
                   "\" with\n", tag_map_->DebugString());
  return output;
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
const std::shared_ptr<tool::TagMap>&
Collection<T, storage, ErrorHandler>::TagMap() const {
  return tag_map_;
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
typename Collection<T, storage, ErrorHandler>::iterator
Collection<T, storage, ErrorHandler>::begin() {
  return iterator(data_.get());
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
typename Collection<T, storage, ErrorHandler>::iterator
Collection<T, storage, ErrorHandler>::end() {
  return iterator(data_.get() + tag_map_->NumEntries());
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
typename Collection<T, storage, ErrorHandler>::const_iterator
Collection<T, storage, ErrorHandler>::begin() const {
  return const_iterator(data_.get());
}

template <typename T, CollectionStorage storage, typename ErrorHandler>
typename Collection<T, storage, ErrorHandler>::const_iterator
Collection<T, storage, ErrorHandler>::end() const {
  return const_iterator(data_.get() + tag_map_->NumEntries());
}

}  // namespace internal

// Returns c.HasTag(tag) && !Tag(tag)->IsEmpty() (just for convenience).
// This version is used with Calculator.
template <class S>
bool HasTagValue(const internal::Collection<S*>& c,
                 const absl::string_view tag) {
  return c.HasTag(tag) && !c.Tag(tag)->IsEmpty();
}

// Returns c.HasTag(tag) && !Tag(tag).IsEmpty() (just for convenience).
// This version is used with CalculatorBase.
template <class S>
bool HasTagValue(const internal::Collection<S>& c,
                 const absl::string_view tag) {
  return c.HasTag(tag) && !c.Tag(tag).IsEmpty();
}

// Returns c.HasTag(tag) && !Tag(tag).IsEmpty() (just for convenience).
// This version is used with Calculator or CalculatorBase.
template <class C>
bool HasTagValue(const C& c, const absl::string_view tag) {
  return HasTagValue(c->Inputs(), tag);
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_COLLECTION_H_
