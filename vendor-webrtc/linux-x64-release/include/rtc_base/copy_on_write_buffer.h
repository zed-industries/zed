/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_COPY_ON_WRITE_BUFFER_H_
#define RTC_BASE_COPY_ON_WRITE_BUFFER_H_

#include <stdint.h>

#include <algorithm>
#include <cstring>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/strings/string_view.h"
#include "api/scoped_refptr.h"
#include "rtc_base/buffer.h"
#include "rtc_base/checks.h"
#include "rtc_base/ref_counted_object.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/type_traits.h"

namespace webrtc {

class RTC_EXPORT CopyOnWriteBuffer {
 public:
  // An empty buffer.
  CopyOnWriteBuffer();
  // Share the data with an existing buffer.
  CopyOnWriteBuffer(const CopyOnWriteBuffer& buf);
  // Move contents from an existing buffer.
  CopyOnWriteBuffer(CopyOnWriteBuffer&& buf) noexcept;

  // Construct a buffer from a string, convenient for unittests.
  explicit CopyOnWriteBuffer(absl::string_view s);

  // Construct a buffer with the specified number of uninitialized bytes.
  explicit CopyOnWriteBuffer(size_t size);
  CopyOnWriteBuffer(size_t size, size_t capacity);

  // Construct a buffer and copy the specified number of bytes into it. The
  // source array may be (const) uint8_t*, int8_t*, or char*.
  template <typename T,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  CopyOnWriteBuffer(const T* data, size_t size)
      : CopyOnWriteBuffer(data, size, size) {}
  template <typename T,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  CopyOnWriteBuffer(const T* data, size_t size, size_t capacity)
      : CopyOnWriteBuffer(size, capacity) {
    if (buffer_) {
      std::memcpy(buffer_->data(), data, size);
      offset_ = 0;
      size_ = size;
    }
  }

  // Construct a buffer from the contents of an array.
  template <typename T,
            size_t N,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  CopyOnWriteBuffer(const T (&array)[N])  // NOLINT: runtime/explicit
      : CopyOnWriteBuffer(array, N) {}

  // Construct a buffer from a vector like type.
  template <typename VecT,
            typename ElemT = typename std::remove_pointer_t<
                decltype(std::declval<VecT>().data())>,
            typename std::enable_if_t<
                !std::is_same<VecT, CopyOnWriteBuffer>::value &&
                HasDataAndSize<VecT, ElemT>::value &&
                internal::BufferCompat<uint8_t, ElemT>::value>* = nullptr>
  explicit CopyOnWriteBuffer(const VecT& v)
      : CopyOnWriteBuffer(v.data(), v.size()) {}

  // Construct a buffer from a vector like type and a capacity argument
  template <typename VecT,
            typename ElemT = typename std::remove_pointer_t<
                decltype(std::declval<VecT>().data())>,
            typename std::enable_if_t<
                !std::is_same<VecT, CopyOnWriteBuffer>::value &&
                HasDataAndSize<VecT, ElemT>::value &&
                internal::BufferCompat<uint8_t, ElemT>::value>* = nullptr>
  explicit CopyOnWriteBuffer(const VecT& v, size_t capacity)
      : CopyOnWriteBuffer(v.data(), v.size(), capacity) {}

  ~CopyOnWriteBuffer();

  // Get a pointer to the data. Just .data() will give you a (const) uint8_t*,
  // but you may also use .data<int8_t>() and .data<char>().
  template <typename T = uint8_t,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  const T* data() const {
    return cdata<T>();
  }

  // Get writable pointer to the data. This will create a copy of the underlying
  // data if it is shared with other buffers.
  template <typename T = uint8_t,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  T* MutableData() {
    RTC_DCHECK(IsConsistent());
    if (!buffer_) {
      return nullptr;
    }
    UnshareAndEnsureCapacity(capacity());
    return buffer_->data<T>() + offset_;
  }

  // Get const pointer to the data. This will not create a copy of the
  // underlying data if it is shared with other buffers.
  template <typename T = uint8_t,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  const T* cdata() const {
    RTC_DCHECK(IsConsistent());
    if (!buffer_) {
      return nullptr;
    }
    return buffer_->data<T>() + offset_;
  }

  bool empty() const { return size_ == 0; }

  size_t size() const {
    RTC_DCHECK(IsConsistent());
    return size_;
  }

  size_t capacity() const {
    RTC_DCHECK(IsConsistent());
    return buffer_ ? buffer_->capacity() - offset_ : 0;
  }

  const uint8_t* begin() const { return data(); }
  const uint8_t* end() const { return data() + size_; }

  CopyOnWriteBuffer& operator=(const CopyOnWriteBuffer& buf) {
    RTC_DCHECK(IsConsistent());
    RTC_DCHECK(buf.IsConsistent());
    if (&buf != this) {
      buffer_ = buf.buffer_;
      offset_ = buf.offset_;
      size_ = buf.size_;
    }
    return *this;
  }

  CopyOnWriteBuffer& operator=(CopyOnWriteBuffer&& buf) {
    RTC_DCHECK(IsConsistent());
    RTC_DCHECK(buf.IsConsistent());
    buffer_ = std::move(buf.buffer_);
    offset_ = buf.offset_;
    size_ = buf.size_;
    buf.offset_ = 0;
    buf.size_ = 0;
    return *this;
  }

  bool operator==(const CopyOnWriteBuffer& buf) const;

  bool operator!=(const CopyOnWriteBuffer& buf) const {
    return !(*this == buf);
  }

  uint8_t operator[](size_t index) const {
    RTC_DCHECK_LT(index, size());
    return cdata()[index];
  }

  // Replace the contents of the buffer. Accepts the same types as the
  // constructors.
  template <typename T,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  void SetData(const T* data, size_t size) {
    RTC_DCHECK(IsConsistent());
    if (!buffer_) {
      buffer_ = size > 0 ? new RefCountedBuffer(data, size) : nullptr;
    } else if (!buffer_->HasOneRef()) {
      buffer_ = new RefCountedBuffer(data, size, capacity());
    } else {
      buffer_->SetData(data, size);
    }
    offset_ = 0;
    size_ = size;

    RTC_DCHECK(IsConsistent());
  }

  template <typename T,
            size_t N,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  void SetData(const T (&array)[N]) {
    SetData(array, N);
  }

  void SetData(const CopyOnWriteBuffer& buf) {
    RTC_DCHECK(IsConsistent());
    RTC_DCHECK(buf.IsConsistent());
    if (&buf != this) {
      buffer_ = buf.buffer_;
      offset_ = buf.offset_;
      size_ = buf.size_;
    }
  }

  // Append data to the buffer. Accepts the same types as the constructors.
  template <typename T,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  void AppendData(const T* data, size_t size) {
    RTC_DCHECK(IsConsistent());
    if (!buffer_) {
      buffer_ = new RefCountedBuffer(data, size);
      offset_ = 0;
      size_ = size;
      RTC_DCHECK(IsConsistent());
      return;
    }

    UnshareAndEnsureCapacity(std::max(capacity(), size_ + size));

    buffer_->SetSize(offset_ +
                     size_);  // Remove data to the right of the slice.
    buffer_->AppendData(data, size);
    size_ += size;

    RTC_DCHECK(IsConsistent());
  }

  template <typename T,
            size_t N,
            typename std::enable_if<
                internal::BufferCompat<uint8_t, T>::value>::type* = nullptr>
  void AppendData(const T (&array)[N]) {
    AppendData(array, N);
  }

  template <typename VecT,
            typename ElemT = typename std::remove_pointer_t<
                decltype(std::declval<VecT>().data())>,
            typename std::enable_if_t<
                HasDataAndSize<VecT, ElemT>::value &&
                internal::BufferCompat<uint8_t, ElemT>::value>* = nullptr>
  void AppendData(const VecT& v) {
    AppendData(v.data(), v.size());
  }

  // Sets the size of the buffer. If the new size is smaller than the old, the
  // buffer contents will be kept but truncated; if the new size is greater,
  // the existing contents will be kept and the new space will be
  // uninitialized.
  void SetSize(size_t size);

  // Ensure that the buffer size can be increased to at least capacity without
  // further reallocation. (Of course, this operation might need to reallocate
  // the buffer.)
  void EnsureCapacity(size_t capacity);

  // Resets the buffer to zero size without altering capacity. Works even if the
  // buffer has been moved from.
  void Clear();

  // Swaps two buffers.
  friend void swap(CopyOnWriteBuffer& a, CopyOnWriteBuffer& b) {
    a.buffer_.swap(b.buffer_);
    std::swap(a.offset_, b.offset_);
    std::swap(a.size_, b.size_);
  }

  CopyOnWriteBuffer Slice(size_t offset, size_t length) const {
    CopyOnWriteBuffer slice(*this);
    RTC_DCHECK_LE(offset, size_);
    RTC_DCHECK_LE(length + offset, size_);
    slice.offset_ += offset;
    slice.size_ = length;
    return slice;
  }

 private:
  using RefCountedBuffer = FinalRefCountedObject<Buffer>;
  // Create a copy of the underlying data if it is referenced from other Buffer
  // objects or there is not enough capacity.
  void UnshareAndEnsureCapacity(size_t new_capacity);

  // Pre- and postcondition of all methods.
  bool IsConsistent() const {
    if (buffer_) {
      return buffer_->capacity() > 0 && offset_ <= buffer_->size() &&
             offset_ + size_ <= buffer_->size();
    } else {
      return size_ == 0 && offset_ == 0;
    }
  }

  // buffer_ is either null, or points to an webrtc::Buffer with capacity > 0.
  scoped_refptr<RefCountedBuffer> buffer_;
  // This buffer may represent a slice of a original data.
  size_t offset_;  // Offset of a current slice in the original data in buffer_.
                   // Should be 0 if the buffer_ is empty.
  size_t size_;    // Size of a current slice in the original data in buffer_.
                   // Should be 0 if the buffer_ is empty.
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::CopyOnWriteBuffer;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_COPY_ON_WRITE_BUFFER_H_
