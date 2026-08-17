/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BUFFER_H_
#define RTC_BASE_BUFFER_H_

#include <stdint.h>

#include <algorithm>
#include <cstring>
#include <memory>
#include <type_traits>
#include <utility>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "rtc_base/checks.h"
#include "rtc_base/type_traits.h"
#include "rtc_base/zero_memory.h"

namespace webrtc {

namespace internal {

// (Internal; please don't use outside this file.) Determines if elements of
// type U are compatible with a BufferT<T>. For most types, we just ignore
// top-level const and forbid top-level volatile and require T and U to be
// otherwise equal, but all byte-sized integers (notably char, int8_t, and
// uint8_t) are compatible with each other. (Note: We aim to get rid of this
// behavior, and treat all types the same.)
template <typename T, typename U>
struct BufferCompat {
  static constexpr bool value =
      !std::is_volatile<U>::value &&
      ((std::is_integral<T>::value && sizeof(T) == 1)
           ? (std::is_integral<U>::value && sizeof(U) == 1)
           : (std::is_same<T, typename std::remove_const<U>::type>::value));
};

}  // namespace internal

// Basic buffer class, can be grown and shrunk dynamically.
// Unlike std::string/vector, does not initialize data when increasing size.
// If "ZeroOnFree" is true, any memory is explicitly cleared before releasing.
// The type alias "ZeroOnFreeBuffer" below should be used instead of setting
// "ZeroOnFree" in the template manually to "true".
template <typename T, bool ZeroOnFree = false>
class BufferT {
  // We want T's destructor and default constructor to be trivial, i.e. perform
  // no action, so that we don't have to touch the memory we allocate and
  // deallocate. And we want T to be trivially copyable, so that we can copy T
  // instances with std::memcpy. This is precisely the definition of a trivial
  // type.
  static_assert(std::is_trivial<T>::value, "T must be a trivial type.");

  // This class relies heavily on being able to mutate its data.
  static_assert(!std::is_const<T>::value, "T may not be const");

 public:
  using value_type = T;
  using const_iterator = const T*;

  // An empty BufferT.
  BufferT() : size_(0), capacity_(0), data_(nullptr) {
    RTC_DCHECK(IsConsistent());
  }

  // Disable copy construction and copy assignment, since copying a buffer is
  // expensive enough that we want to force the user to be explicit about it.
  BufferT(const BufferT&) = delete;
  BufferT& operator=(const BufferT&) = delete;

  BufferT(BufferT&& buf)
      : size_(buf.size()),
        capacity_(buf.capacity()),
        data_(std::move(buf.data_)) {
    RTC_DCHECK(IsConsistent());
    buf.OnMovedFrom();
  }

  // Construct a buffer with the specified number of uninitialized elements.
  explicit BufferT(size_t size) : BufferT(size, size) {}

  BufferT(size_t size, size_t capacity)
      : size_(size),
        capacity_(std::max(size, capacity)),
        data_(capacity_ > 0 ? new T[capacity_] : nullptr) {
    RTC_DCHECK(IsConsistent());
  }

  // Construct a buffer and copy the specified number of elements into it.
  template <typename U,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  BufferT(const U* data, size_t size) : BufferT(data, size, size) {}

  template <typename U,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  BufferT(U* data, size_t size, size_t capacity) : BufferT(size, capacity) {
    static_assert(sizeof(T) == sizeof(U), "");
    if (size > 0) {
      RTC_DCHECK(data);
      std::memcpy(data_.get(), data, size * sizeof(U));
    }
  }

  // Construct a buffer from the contents of an array.
  template <typename U,
            size_t N,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  BufferT(U (&array)[N]) : BufferT(array, N) {}

  ~BufferT() { MaybeZeroCompleteBuffer(); }

  // Implicit conversion to absl::string_view if T is compatible with char.
  template <typename U = T>
  operator typename std::enable_if<internal::BufferCompat<U, char>::value,
                                   absl::string_view>::type() const {
    return absl::string_view(data<char>(), size());
  }

  // Get a pointer to the data. Just .data() will give you a (const) T*, but if
  // T is a byte-sized integer, you may also use .data<U>() for any other
  // byte-sized integer U.
  template <typename U = T,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  const U* data() const {
    RTC_DCHECK(IsConsistent());
    return reinterpret_cast<U*>(data_.get());
  }

  template <typename U = T,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  U* data() {
    RTC_DCHECK(IsConsistent());
    return reinterpret_cast<U*>(data_.get());
  }

  bool empty() const {
    RTC_DCHECK(IsConsistent());
    return size_ == 0;
  }

  size_t size() const {
    RTC_DCHECK(IsConsistent());
    return size_;
  }

  size_t capacity() const {
    RTC_DCHECK(IsConsistent());
    return capacity_;
  }

  BufferT& operator=(BufferT&& buf) {
    RTC_DCHECK(buf.IsConsistent());
    MaybeZeroCompleteBuffer();
    size_ = buf.size_;
    capacity_ = buf.capacity_;
    using std::swap;
    swap(data_, buf.data_);
    buf.data_.reset();
    buf.OnMovedFrom();
    return *this;
  }

  bool operator==(const BufferT& buf) const {
    RTC_DCHECK(IsConsistent());
    if (size_ != buf.size_) {
      return false;
    }
    if (std::is_integral<T>::value) {
      // Optimization.
      return std::memcmp(data_.get(), buf.data_.get(), size_ * sizeof(T)) == 0;
    }
    for (size_t i = 0; i < size_; ++i) {
      if (data_[i] != buf.data_[i]) {
        return false;
      }
    }
    return true;
  }

  bool operator!=(const BufferT& buf) const { return !(*this == buf); }

  T& operator[](size_t index) {
    RTC_DCHECK_LT(index, size_);
    return data()[index];
  }

  T operator[](size_t index) const {
    RTC_DCHECK_LT(index, size_);
    return data()[index];
  }

  T* begin() { return data(); }
  T* end() { return data() + size(); }
  const T* begin() const { return data(); }
  const T* end() const { return data() + size(); }
  const T* cbegin() const { return data(); }
  const T* cend() const { return data() + size(); }

  // The SetData functions replace the contents of the buffer. They accept the
  // same input types as the constructors.
  template <typename U,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  void SetData(const U* data, size_t size) {
    RTC_DCHECK(IsConsistent());
    const size_t old_size = size_;
    size_ = 0;
    AppendData(data, size);
    if (ZeroOnFree && size_ < old_size) {
      ZeroTrailingData(old_size - size_);
    }
  }

  template <typename U,
            size_t N,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  void SetData(const U (&array)[N]) {
    SetData(array, N);
  }

  template <typename W,
            typename std::enable_if<
                HasDataAndSize<const W, const T>::value>::type* = nullptr>
  void SetData(const W& w) {
    SetData(w.data(), w.size());
  }

  // Replaces the data in the buffer with at most `max_elements` of data, using
  // the function `setter`, which should have the following signature:
  //
  //   size_t setter(ArrayView<U> view)
  //
  // `setter` is given an appropriately typed ArrayView of length exactly
  // `max_elements` that describes the area where it should write the data; it
  // should return the number of elements actually written. (If it doesn't fill
  // the whole ArrayView, it should leave the unused space at the end.)
  template <typename U = T,
            typename F,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  size_t SetData(size_t max_elements, F&& setter) {
    RTC_DCHECK(IsConsistent());
    const size_t old_size = size_;
    size_ = 0;
    const size_t written = AppendData<U>(max_elements, std::forward<F>(setter));
    if (ZeroOnFree && size_ < old_size) {
      ZeroTrailingData(old_size - size_);
    }
    return written;
  }

  // The AppendData functions add data to the end of the buffer. They accept
  // the same input types as the constructors.
  template <typename U,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  void AppendData(const U* data, size_t size) {
    if (size == 0) {
      return;
    }
    RTC_DCHECK(data);
    RTC_DCHECK(IsConsistent());
    const size_t new_size = size_ + size;
    EnsureCapacityWithHeadroom(new_size, true);
    static_assert(sizeof(T) == sizeof(U), "");
    std::memcpy(data_.get() + size_, data, size * sizeof(U));
    size_ = new_size;
    RTC_DCHECK(IsConsistent());
  }

  template <typename U,
            size_t N,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  void AppendData(const U (&array)[N]) {
    AppendData(array, N);
  }

  template <typename W,
            typename std::enable_if<
                HasDataAndSize<const W, const T>::value>::type* = nullptr>
  void AppendData(const W& w) {
    AppendData(w.data(), w.size());
  }

  template <typename U,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  void AppendData(const U& item) {
    AppendData(&item, 1);
  }

  // Appends at most `max_elements` to the end of the buffer, using the function
  // `setter`, which should have the following signature:
  //
  //   size_t setter(ArrayView<U> view)
  //
  // `setter` is given an appropriately typed ArrayView of length exactly
  // `max_elements` that describes the area where it should write the data; it
  // should return the number of elements actually written. (If it doesn't fill
  // the whole ArrayView, it should leave the unused space at the end.)
  template <typename U = T,
            typename F,
            typename std::enable_if<
                internal::BufferCompat<T, U>::value>::type* = nullptr>
  size_t AppendData(size_t max_elements, F&& setter) {
    RTC_DCHECK(IsConsistent());
    const size_t old_size = size_;
    SetSize(old_size + max_elements);
    U* base_ptr = data<U>() + old_size;
    size_t written_elements = setter(ArrayView<U>(base_ptr, max_elements));

    RTC_CHECK_LE(written_elements, max_elements);
    size_ = old_size + written_elements;
    RTC_DCHECK(IsConsistent());
    return written_elements;
  }

  // Sets the size of the buffer. If the new size is smaller than the old, the
  // buffer contents will be kept but truncated; if the new size is greater,
  // the existing contents will be kept and the new space will be
  // uninitialized.
  void SetSize(size_t size) {
    const size_t old_size = size_;
    EnsureCapacityWithHeadroom(size, true);
    size_ = size;
    if (ZeroOnFree && size_ < old_size) {
      ZeroTrailingData(old_size - size_);
    }
  }

  // Ensure that the buffer size can be increased to at least capacity without
  // further reallocation. (Of course, this operation might need to reallocate
  // the buffer.)
  void EnsureCapacity(size_t capacity) {
    // Don't allocate extra headroom, since the user is asking for a specific
    // capacity.
    EnsureCapacityWithHeadroom(capacity, false);
  }

  // Resets the buffer to zero size without altering capacity. Works even if the
  // buffer has been moved from.
  void Clear() {
    MaybeZeroCompleteBuffer();
    size_ = 0;
    RTC_DCHECK(IsConsistent());
  }

  // Swaps two buffers. Also works for buffers that have been moved from.
  friend void swap(BufferT& a, BufferT& b) {
    using std::swap;
    swap(a.size_, b.size_);
    swap(a.capacity_, b.capacity_);
    swap(a.data_, b.data_);
  }

 private:
  void EnsureCapacityWithHeadroom(size_t capacity, bool extra_headroom) {
    RTC_DCHECK(IsConsistent());
    if (capacity <= capacity_)
      return;

    // If the caller asks for extra headroom, ensure that the new capacity is
    // >= 1.5 times the old capacity. Any constant > 1 is sufficient to prevent
    // quadratic behavior; as to why we pick 1.5 in particular, see
    // https://github.com/facebook/folly/blob/master/folly/docs/FBVector.md and
    // http://www.gahcep.com/cpp-internals-stl-vector-part-1/.
    const size_t new_capacity =
        extra_headroom ? std::max(capacity, capacity_ + capacity_ / 2)
                       : capacity;

    std::unique_ptr<T[]> new_data(new T[new_capacity]);
    if (data_ != nullptr) {
      std::memcpy(new_data.get(), data_.get(), size_ * sizeof(T));
    }
    MaybeZeroCompleteBuffer();
    data_ = std::move(new_data);
    capacity_ = new_capacity;
    RTC_DCHECK(IsConsistent());
  }

  // Zero the complete buffer if template argument "ZeroOnFree" is true.
  void MaybeZeroCompleteBuffer() {
    if (ZeroOnFree && capacity_ > 0) {
      // It would be sufficient to only zero "size_" elements, as all other
      // methods already ensure that the unused capacity contains no sensitive
      // data---but better safe than sorry.
      ExplicitZeroMemory(data_.get(), capacity_ * sizeof(T));
    }
  }

  // Zero the first "count" elements of unused capacity.
  void ZeroTrailingData(size_t count) {
    RTC_DCHECK(IsConsistent());
    RTC_DCHECK_LE(count, capacity_ - size_);
    ExplicitZeroMemory(data_.get() + size_, count * sizeof(T));
  }

  // Precondition for all methods except Clear, operator= and the destructor.
  // Postcondition for all methods except move construction and move
  // assignment, which leave the moved-from object in a possibly inconsistent
  // state.
  bool IsConsistent() const {
    return (data_ || capacity_ == 0) && capacity_ >= size_;
  }

  // Called when *this has been moved from. Conceptually it's a no-op, but we
  // can mutate the state slightly to help subsequent sanity checks catch bugs.
  void OnMovedFrom() {
    RTC_DCHECK(!data_);  // Our heap block should have been stolen.
#if RTC_DCHECK_IS_ON
    // Ensure that *this is always inconsistent, to provoke bugs.
    size_ = 1;
    capacity_ = 0;
#else
    // Make *this consistent and empty. Shouldn't be necessary, but better safe
    // than sorry.
    size_ = 0;
    capacity_ = 0;
#endif
  }

  size_t size_;
  size_t capacity_;
  std::unique_ptr<T[]> data_;
};

// By far the most common sort of buffer.
using Buffer = BufferT<uint8_t>;

// A buffer that zeros memory before releasing it.
template <typename T>
using ZeroOnFreeBuffer = BufferT<T, true>;

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
template <typename T, bool ZeroOnFree = false>
using BufferT = ::webrtc::BufferT<T, ZeroOnFree>;
using ::webrtc::Buffer;
template <typename T>
using ZeroOnFreeBuffer = ::webrtc::ZeroOnFreeBuffer<T>;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_BUFFER_H_
