#ifndef TEST_SUPPORT_FORMAT_STRING_H
#define TEST_SUPPORT_FORMAT_STRING_H

#include <cstdio>
#include <string>
#include <memory>
#include <array>
#include <cstdarg>

namespace format_string_detail {
inline std::string format_string_imp(const char* msg, ...) {
  // we might need a second shot at this, so pre-emptively make a copy
  struct GuardVAList {
    va_list& xtarget;
    bool active;
    GuardVAList(va_list& val) : xtarget(val), active(true) {}

    void clear() {
      if (active)
        va_end(xtarget);
      active = false;
    }
    ~GuardVAList() {
      if (active)
        va_end(xtarget);
    }
  };
  va_list args;
  va_start(args, msg);
  GuardVAList args_guard(args);

  va_list args_cp;
  va_copy(args_cp, args);
  GuardVAList args_copy_guard(args_cp);

  std::array<char, 256> local_buff;
  std::size_t size = local_buff.size();
  auto ret = ::vsnprintf(local_buff.data(), size, msg, args_cp);

  args_copy_guard.clear();

  // handle empty expansion
  if (ret == 0)
    return std::string{};
  if (static_cast<std::size_t>(ret) < size)
    return std::string(local_buff.data());

  // we did not provide a long enough buffer on our first attempt.
  // add 1 to size to account for null-byte in size cast to prevent overflow
  size = static_cast<std::size_t>(ret) + 1;
  auto buff_ptr = std::unique_ptr<char[]>(new char[size]);
  ret = ::vsnprintf(buff_ptr.get(), size, msg, args);
  return std::string(buff_ptr.get());
}

const char* unwrap(std::string& s) { return s.c_str(); }
template <class Arg>
Arg const& unwrap(Arg& a) {
  static_assert(!std::is_class<Arg>::value, "cannot pass class here");
  return a;
}

} // namespace format_string_detail

template <class... Args>
std::string format_string(const char* fmt, Args const&... args) {
  return format_string_detail::format_string_imp(
      fmt, format_string_detail::unwrap(const_cast<Args&>(args))...);
}

#endif // TEST_SUPPORT_FORMAT_STRING_HPP
