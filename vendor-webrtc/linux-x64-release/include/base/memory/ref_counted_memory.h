// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_REF_COUNTED_MEMORY_H_
#define BASE_MEMORY_REF_COUNTED_MEMORY_H_

#include <stddef.h>

#include <memory>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/memory/raw_span.h"
#include "base/memory/ref_counted.h"
#include "base/memory/shared_memory_mapping.h"

namespace base {

class ReadOnlySharedMemoryRegion;

// A generic interface to memory. This object is reference counted because most
// of its subclasses own the data they carry, and this interface needs to
// support heterogeneous containers of these different types of memory.
//
// The RefCountedMemory class provides a const view of the data it holds, as it
// does not require all subclassing implementations to hold mutable data. If a
// mutable view is required, the code must maintain awareness of the subclass
// type, and can access it through there, such as:
// - RefCountedBytes provides `as_vector()` to give mutable access to its data.
// - RefCountedString provides `as_string()` to give mutable access to its data.
class BASE_EXPORT RefCountedMemory
    : public RefCountedThreadSafe<RefCountedMemory> {
 public:
  // Returns true if `other` is byte for byte equal.
  bool Equals(const scoped_refptr<RefCountedMemory>& other) const;

  // Allow explicit conversion to `base::span<const uint8_t>`. Use a span to
  // access the data in a safe way, rather than calling `data()` explicitly.
  //
  // Example:
  // ```
  // auto data = base::MakeRefCounted<base::RefCountedBytes>(
  //     std::vector<uint8_t>{1, 2, 3});
  // base::span<const uint8_t> v = base::span(data);
  // v[2] = uint8_t{4};
  // ```
  const uint8_t* data() const LIFETIME_BOUND { return AsSpan().data(); }
  size_t size() const { return AsSpan().size(); }

  using iterator = base::span<const uint8_t>::iterator;
  iterator begin() const LIFETIME_BOUND { return AsSpan().begin(); }
  iterator end() const LIFETIME_BOUND { return AsSpan().end(); }

  // TODO(danakj): Remove all callers and remove this.
  const uint8_t* front() const LIFETIME_BOUND { return AsSpan().data(); }

  // The data/size members (or begin/end) give conversion to span already, but
  // we provide this operator as an optimization to combine two virtual method
  // calls into one.
  explicit operator base::span<const uint8_t>() const LIFETIME_BOUND {
    return AsSpan();
  }

 protected:
  friend class RefCountedThreadSafe<RefCountedMemory>;
  RefCountedMemory();
  virtual ~RefCountedMemory();

  virtual base::span<const uint8_t> AsSpan() const LIFETIME_BOUND = 0;
};

// An implementation of RefCountedMemory, for pointing to memory with a static
// lifetime. Since the memory exists for the life of the program, the class can
// not and does not need to take ownership of it.
class BASE_EXPORT RefCountedStaticMemory : public RefCountedMemory {
 public:
  RefCountedStaticMemory();
  explicit RefCountedStaticMemory(base::span<const uint8_t> bytes);

  RefCountedStaticMemory(const RefCountedStaticMemory&) = delete;
  RefCountedStaticMemory& operator=(const RefCountedStaticMemory&) = delete;

 private:
  ~RefCountedStaticMemory() override;

  // RefCountedMemory:
  base::span<const uint8_t> AsSpan() const LIFETIME_BOUND override;

  base::raw_span<const uint8_t> bytes_;
};

// An implementation of RefCountedMemory, where the data is stored in a STL
// vector.
class BASE_EXPORT RefCountedBytes : public RefCountedMemory {
 public:
  RefCountedBytes();

  // Constructs a RefCountedBytes object by taking `initializer`.
  explicit RefCountedBytes(std::vector<uint8_t> initializer);

  // Constructs a RefCountedBytes object by copying from `initializer`.
  explicit RefCountedBytes(base::span<const uint8_t> initializer);

  // Constructs a RefCountedBytes object by zero-initializing a new vector of
  // `size` bytes.
  explicit RefCountedBytes(size_t size);

  RefCountedBytes(const RefCountedBytes&) = delete;
  RefCountedBytes& operator=(const RefCountedBytes&) = delete;

  const std::vector<uint8_t>& as_vector() const { return bytes_; }
  std::vector<uint8_t>& as_vector() { return bytes_; }

 private:
  ~RefCountedBytes() override;

  // RefCountedMemory:
  base::span<const uint8_t> AsSpan() const LIFETIME_BOUND override;

  std::vector<uint8_t> bytes_;
};

// An implementation of RefCountedMemory, where the bytes are stored in a STL
// string. Use this if your data naturally arrives in that format.
class BASE_EXPORT RefCountedString : public RefCountedMemory {
 public:
  RefCountedString();
  explicit RefCountedString(std::string value);

  RefCountedString(const RefCountedString&) = delete;
  RefCountedString& operator=(const RefCountedString&) = delete;

  const std::string& as_string() const LIFETIME_BOUND { return string_; }
  std::string& as_string() LIFETIME_BOUND { return string_; }

 private:
  ~RefCountedString() override;

  // RefCountedMemory:
  base::span<const uint8_t> AsSpan() const LIFETIME_BOUND override;

  std::string string_;
};

// An implementation of RefCountedMemory, where the bytes are stored in a
// std::u16string.
class BASE_EXPORT RefCountedString16 : public base::RefCountedMemory {
 public:
  RefCountedString16();
  explicit RefCountedString16(std::u16string value);

  RefCountedString16(const RefCountedString16&) = delete;
  RefCountedString16& operator=(const RefCountedString16&) = delete;

  const std::u16string& as_string() const LIFETIME_BOUND { return string_; }
  std::u16string& as_string() LIFETIME_BOUND { return string_; }

 private:
  ~RefCountedString16() override;

  // RefCountedMemory:
  base::span<const uint8_t> AsSpan() const LIFETIME_BOUND override;

  std::u16string string_;
};

// An implementation of RefCountedMemory, where the bytes are stored in
// ReadOnlySharedMemoryMapping.
class BASE_EXPORT RefCountedSharedMemoryMapping : public RefCountedMemory {
 public:
  // Constructs a RefCountedMemory object by taking ownership of an already
  // mapped ReadOnlySharedMemoryMapping object.
  explicit RefCountedSharedMemoryMapping(ReadOnlySharedMemoryMapping mapping);

  RefCountedSharedMemoryMapping(const RefCountedSharedMemoryMapping&) = delete;
  RefCountedSharedMemoryMapping& operator=(
      const RefCountedSharedMemoryMapping&) = delete;

  // Convenience method to map all of `region` and take ownership of the
  // mapping. Returns a null `scoped_refptr` if the map operation fails.
  static scoped_refptr<RefCountedSharedMemoryMapping> CreateFromWholeRegion(
      const ReadOnlySharedMemoryRegion& region);

 private:
  ~RefCountedSharedMemoryMapping() override;

  // RefCountedMemory:
  base::span<const uint8_t> AsSpan() const LIFETIME_BOUND override;

  const ReadOnlySharedMemoryMapping mapping_;
};

}  // namespace base

#endif  // BASE_MEMORY_REF_COUNTED_MEMORY_H_
