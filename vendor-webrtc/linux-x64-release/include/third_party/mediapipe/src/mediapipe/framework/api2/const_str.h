#ifndef MEDIAPIPE_FRAMEWORK_API2_CONST_STR_H_
#define MEDIAPIPE_FRAMEWORK_API2_CONST_STR_H_

#include <string>

namespace mediapipe {
namespace api2 {

// This class stores a constant string that can be inspected at compile time
// in constexpr code.
class const_str {
 public:
  constexpr const_str(std::size_t size, const char* data)
      : len_(size - 1), data_(data) {}

  template <std::size_t N>
  explicit constexpr const_str(const char (&str)[N]) : const_str(N, str) {}

  constexpr std::size_t len() const { return len_; }
  constexpr const char* data() const { return data_; }

  constexpr bool operator==(const const_str& other) const {
    return len_ == other.len_ && equal(len_, data_, other.data_);
  }

  constexpr char operator[](const std::size_t idx) const {
    return idx <= len_ ? data_[idx] : '\0';
  }

 private:
  static constexpr bool equal(std::size_t len, const char* const p,
                              const char* const q) {
    return len == 0 || (*p == *q && equal(len - 1, p + 1, q + 1));
  }

  const std::size_t len_;
  const char* const data_;
};

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_API2_CONST_STR_H_
