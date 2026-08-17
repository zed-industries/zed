#ifndef TESTS_IS_QUIET_NAN_H
#define TESTS_IS_QUIET_NAN_H

namespace flatbuffers {
namespace tests {

#if defined(FLATBUFFERS_HAS_NEW_STRTOD) && (FLATBUFFERS_HAS_NEW_STRTOD > 0)
// The IEEE-754 quiet_NaN is not simple binary constant.
// All binary NaN bit strings have all the bits of the biased exponent field E
// set to 1. A quiet NaN bit string should be encoded with the first bit d[1]
// of the trailing significand field T being 1 (d[0] is implicit bit).
// It is assumed that endianness of floating-point is same as integer.
template<typename T, typename U, U qnan_base> bool is_quiet_nan_impl(T v) {
  static_assert(sizeof(T) == sizeof(U), "unexpected");
  U b = 0;
  std::memcpy(&b, &v, sizeof(T));
  return ((b & qnan_base) == qnan_base);
}
#  if defined(__mips__) || defined(__hppa__)
inline bool is_quiet_nan(float v) {
  return is_quiet_nan_impl<float, uint32_t, 0x7FC00000u>(v) ||
         is_quiet_nan_impl<float, uint32_t, 0x7FBFFFFFu>(v);
}
inline bool is_quiet_nan(double v) {
  return is_quiet_nan_impl<double, uint64_t, 0x7FF8000000000000ul>(v) ||
         is_quiet_nan_impl<double, uint64_t, 0x7FF7FFFFFFFFFFFFu>(v);
}
#  else
inline bool is_quiet_nan(float v) {
  return is_quiet_nan_impl<float, uint32_t, 0x7FC00000u>(v);
}
inline bool is_quiet_nan(double v) {
  return is_quiet_nan_impl<double, uint64_t, 0x7FF8000000000000ul>(v);
}
#  endif
#endif

}  // namespace tests
}  // namespace flatbuffers

#endif  // TESTS_IS_QUIET_NAN_H
