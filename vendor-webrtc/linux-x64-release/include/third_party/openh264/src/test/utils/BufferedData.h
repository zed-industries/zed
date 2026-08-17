#ifndef __BUFFEREDDATA_H__
#define __BUFFEREDDATA_H__

#include <gtest/gtest.h>
#include <stddef.h>
#include <stdlib.h>
#include "../test_stdint.h"
#include <algorithm>

class BufferedData {
 public:
  BufferedData() : data_ (NULL), capacity_ (0), length_ (0) {}

  ~BufferedData() {
    free (data_);
  }

  bool PushBack (uint8_t c) {
    if (!EnsureCapacity (length_ + 1)) {
      return false;
    }
    data_[length_++] = c;
    return true;
  }

  bool PushBack (const uint8_t* data, size_t len) {
    if (!EnsureCapacity (length_ + len)) {
      return false;
    }
    memcpy (data_ + length_, data, len);
    length_ += len;
    return true;
  }

  size_t PopFront (uint8_t* ptr, size_t len) {
    len = std::min (length_, len);
    memcpy (ptr, data_, len);
    memmove (data_, data_ + len, length_ - len);
    if (-1 == SetLength (length_ - len))
      return -1;
    return len;
  }

  void Clear() {
    length_ = 0;
  }

  int SetLength (size_t newLen) {
    if (EnsureCapacity (newLen)) {
      length_ = newLen;
    } else {
      Clear();
      //FAIL () << "unable to alloc memory in SetLength()";
      std::cout << "unable to alloc memory in SetLength()" << std::endl;
      return -1;
    }

    return 0;
  }

  size_t Length() const {
    return length_;
  }

  uint8_t* data() {
    return data_;
  }

 private:
  bool EnsureCapacity (size_t capacity) {
    if (capacity > capacity_) {
      size_t newsize = capacity * 2;

      uint8_t* data = static_cast<uint8_t*> (realloc (data_, newsize));

      if (!data)
        return false;

      data_ = data;
      capacity_ = newsize;
      return true;
    }
    return true;
  }

  uint8_t* data_;
  size_t capacity_;
  size_t length_;
};

#endif //__BUFFEREDDATA_H__
