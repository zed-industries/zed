/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_FUZZERS_FUZZ_DATA_HELPER_H_
#define TEST_FUZZERS_FUZZ_DATA_HELPER_H_

#include <limits>

#include "api/array_view.h"
#include "modules/rtp_rtcp/source/byte_io.h"

namespace webrtc {
namespace test {

// Helper class to take care of the fuzzer input, read from it, and keep track
// of when the end of the data has been reached.
class FuzzDataHelper {
 public:
  explicit FuzzDataHelper(ArrayView<const uint8_t> data);

  // Returns true if n bytes can be read.
  bool CanReadBytes(size_t n) const { return data_ix_ + n <= data_.size(); }

  // Reads and returns data of type T.
  template <typename T>
  T Read() {
    RTC_CHECK(CanReadBytes(sizeof(T)));
    T x = ByteReader<T>::ReadLittleEndian(&data_[data_ix_]);
    data_ix_ += sizeof(T);
    return x;
  }

  // Reads and returns data of type T. Returns default_value if not enough
  // fuzzer input remains to read a T.
  template <typename T>
  T ReadOrDefaultValue(T default_value) {
    if (!CanReadBytes(sizeof(T))) {
      return default_value;
    }
    return Read<T>();
  }

  // Like ReadOrDefaultValue, but replaces the value 0 with default_value.
  template <typename T>
  T ReadOrDefaultValueNotZero(T default_value) {
    static_assert(std::is_integral<T>::value, "");
    T x = ReadOrDefaultValue(default_value);
    return x == 0 ? default_value : x;
  }

  // Returns one of the elements from the provided input array. The selection
  // is based on the fuzzer input data. If not enough fuzzer data is available,
  // the method will return the first element in the input array. The reason for
  // not flagging this as an error is to allow the method to be called from
  // class constructors, and in constructors we typically do not handle
  // errors. The code will work anyway, and the fuzzer will likely see that
  // providing more data will actually make this method return something else.
  template <typename T, size_t N>
  T SelectOneOf(const T (&select_from)[N]) {
    static_assert(N <= std::numeric_limits<uint8_t>::max(), "");
    // Read an index between 0 and select_from.size() - 1 from the fuzzer data.
    uint8_t index = ReadOrDefaultValue<uint8_t>(0) % N;
    return select_from[index];
  }

  ArrayView<const uint8_t> ReadByteArray(size_t bytes) {
    if (!CanReadBytes(bytes)) {
      return ArrayView<const uint8_t>(nullptr, 0);
    }
    const size_t index_to_return = data_ix_;
    data_ix_ += bytes;
    return data_.subview(index_to_return, bytes);
  }

  // If sizeof(T) > BytesLeft then the remaining bytes will be used and the rest
  // of the object will be zero initialized.
  template <typename T>
  void CopyTo(T* object) {
    memset(object, 0, sizeof(T));

    size_t bytes_to_copy = std::min(BytesLeft(), sizeof(T));
    memcpy(object, data_.data() + data_ix_, bytes_to_copy);
    data_ix_ += bytes_to_copy;
  }

  size_t BytesRead() const { return data_ix_; }

  size_t BytesLeft() const { return data_.size() - data_ix_; }

 private:
  ArrayView<const uint8_t> data_;
  size_t data_ix_ = 0;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_FUZZERS_FUZZ_DATA_HELPER_H_
