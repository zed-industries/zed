#ifndef TEST_STD_RANGES_RANGE_FACTORIES_RANGE_ISTREAM_UTILS_H
#define TEST_STD_RANGES_RANGE_FACTORIES_RANGE_ISTREAM_UTILS_H

#include <sstream>
#include <string>

template <class CharT, std::size_t N>
auto make_string(const char (&in)[N]) {
  return std::basic_string<CharT>(in + 0, in + (N - 1));
}

template <class CharT, std::size_t N>
auto make_string_stream(const char (&in)[N]) {
  return std::basic_istringstream<CharT>(make_string<CharT>(in));
}

#endif //TEST_STD_RANGES_RANGE_FACTORIES_RANGE_ISTREAM_UTILS_H
