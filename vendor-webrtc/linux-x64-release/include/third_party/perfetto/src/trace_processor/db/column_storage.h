/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_STORAGE_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_STORAGE_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <vector>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/containers/bit_vector.h"

namespace perfetto::trace_processor {

// Base class for allowing type erasure when defining plug-in implementations
// of backing storage for columns.
class ColumnStorageBase {
 public:
  ColumnStorageBase() = default;
  virtual ~ColumnStorageBase();

  ColumnStorageBase(const ColumnStorageBase&) = delete;
  ColumnStorageBase& operator=(const ColumnStorageBase&) = delete;

  ColumnStorageBase(ColumnStorageBase&&) = default;
  ColumnStorageBase& operator=(ColumnStorageBase&&) noexcept = default;

  virtual const void* data() const = 0;
  virtual const BitVector* bv() const = 0;
  virtual uint32_t size() const = 0;
  virtual uint32_t non_null_size() const = 0;
};

// Class used for implementing storage for non-null columns.
template <typename T>
class ColumnStorage final : public ColumnStorageBase {
 public:
  ColumnStorage() = default;

  explicit ColumnStorage(const ColumnStorage&) = delete;
  ColumnStorage& operator=(const ColumnStorage&) = delete;

  ColumnStorage(ColumnStorage&&) = default;
  ColumnStorage& operator=(ColumnStorage&&) noexcept = default;

  T Get(uint32_t idx) const { return vector_[idx]; }
  void Append(T val) { vector_.emplace_back(val); }
  void Append(const std::vector<T>& vals) {
    vector_.insert(vector_.end(), vals.begin(), vals.end());
  }
  void AppendMultiple(T val, uint32_t count) {
    vector_.insert(vector_.end(), count, val);
  }
  void Set(uint32_t idx, T val) { vector_[idx] = val; }
  PERFETTO_NO_INLINE void ShrinkToFit() { vector_.shrink_to_fit(); }
  const std::vector<T>& vector() const { return vector_; }

  const void* data() const final { return vector_.data(); }
  const BitVector* bv() const final { return nullptr; }
  uint32_t size() const final { return static_cast<uint32_t>(vector_.size()); }
  uint32_t non_null_size() const final { return size(); }

  template <bool IsDense>
  static ColumnStorage<T> Create() {
    static_assert(!IsDense, "Invalid for non-null storage to be dense.");
    return ColumnStorage<T>();
  }

  // Create non-null storage from nullable storage without nulls.
  static ColumnStorage<T> CreateFromAssertNonNull(
      ColumnStorage<std::optional<T>> null_storage) {
    PERFETTO_CHECK(null_storage.size() == null_storage.non_null_size());
    ColumnStorage<T> x;
    x.vector_ = std::move(null_storage).non_null_vector();
    return x;
  }

 private:
  std::vector<T> vector_;
};

// Class used for implementing storage for nullable columns.
template <typename T>
class ColumnStorage<std::optional<T>> final : public ColumnStorageBase {
 public:
  ColumnStorage() = default;

  explicit ColumnStorage(const ColumnStorage&) = delete;
  ColumnStorage& operator=(const ColumnStorage&) = delete;

  ColumnStorage(ColumnStorage&&) = default;
  ColumnStorage& operator=(ColumnStorage&&) noexcept = default;

  std::optional<T> Get(uint32_t idx) const {
    bool contains = valid_.IsSet(idx);
    if (mode_ == Mode::kDense) {
      return contains ? std::make_optional(data_[idx]) : std::nullopt;
    }
    return contains ? std::make_optional(data_[valid_.CountSetBits(idx)])
                    : std::nullopt;
  }
  void Append(T val) {
    data_.emplace_back(val);
    valid_.AppendTrue();
  }
  void Append(std::optional<T> val) {
    if (val) {
      Append(*val);
    } else {
      AppendNull();
    }
  }
  void AppendMultipleNulls(uint32_t count) {
    if (mode_ == Mode::kDense) {
      data_.resize(data_.size() + static_cast<uint32_t>(count));
    }
    valid_.Resize(valid_.size() + static_cast<uint32_t>(count), false);
  }
  void AppendMultiple(T val, uint32_t count) {
    data_.insert(data_.end(), count, val);
    valid_.Resize(valid_.size() + static_cast<uint32_t>(count), true);
  }
  void Append(const std::vector<T>& vals) {
    data_.insert(data_.end(), vals.begin(), vals.end());
    valid_.Resize(valid_.size() + static_cast<uint32_t>(vals.size()), true);
  }
  void Set(uint32_t idx, T val) {
    if (mode_ == Mode::kDense) {
      valid_.Set(idx);
      data_[idx] = val;
    } else {
      // Generally, we will be setting a null row to non-null so optimize for
      // that path.
      uint32_t row = valid_.CountSetBits(idx);
      bool was_set = valid_.Set(idx);
      if (PERFETTO_UNLIKELY(was_set)) {
        data_[row] = val;
      } else {
        data_.insert(data_.begin() + static_cast<ptrdiff_t>(row), val);
      }
    }
  }
  bool IsDense() const { return mode_ == Mode::kDense; }
  PERFETTO_NO_INLINE void ShrinkToFit() {
    data_.shrink_to_fit();
    valid_.ShrinkToFit();
  }
  // For dense columns the size of the vector is equal to size of the bit
  // vector. For sparse it's equal to count set bits of the bit vector.
  const std::vector<T>& non_null_vector() const& { return data_; }
  const BitVector& non_null_bit_vector() const { return valid_; }

  const void* data() const final { return non_null_vector().data(); }
  const BitVector* bv() const final { return &non_null_bit_vector(); }
  uint32_t size() const final { return valid_.size(); }
  uint32_t non_null_size() const final {
    return static_cast<uint32_t>(non_null_vector().size());
  }

  template <bool IsDense>
  static ColumnStorage<std::optional<T>> Create() {
    return IsDense ? ColumnStorage<std::optional<T>>(Mode::kDense)
                   : ColumnStorage<std::optional<T>>(Mode::kSparse);
  }

  std::vector<T> non_null_vector() && { return std::move(data_); }

 private:
  enum class Mode {
    // Sparse mode is the default mode and ensures that nulls are stored using
    // only
    // a single bit (at the cost of making setting null entries to non-null
    // O(n)).
    kSparse,

    // Dense mode forces the reservation of space for null entries which
    // increases
    // memory usage but allows for O(1) set operations.
    kDense,
  };

  explicit ColumnStorage(Mode mode) : mode_(mode) {}

  void AppendNull() {
    if (mode_ == Mode::kDense) {
      data_.emplace_back();
    }
    valid_.AppendFalse();
  }

  Mode mode_ = Mode::kSparse;
  std::vector<T> data_;
  BitVector valid_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_STORAGE_H_
