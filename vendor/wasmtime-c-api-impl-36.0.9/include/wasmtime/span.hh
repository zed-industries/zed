/**
 * \file wasmtime/span.hh
 */

#ifndef WASMTIME_SPAN_HH
#define WASMTIME_SPAN_HH

#ifdef __has_include
#if __has_include(<span>)
#include <span>
#endif
#endif

#ifndef __cpp_lib_span
#include <cstddef>
#include <limits>
#include <type_traits>
#endif

namespace wasmtime {

#ifdef __cpp_lib_span

/// \brief Alias to C++20 std::span when it is available
template <typename T, std::size_t Extent = std::dynamic_extent>
using Span = std::span<T, Extent>;

#else

/// \brief Means number of elements determined at runtime
inline constexpr size_t dynamic_extent =
    std::numeric_limits<std::size_t>::max();

/**
 * \brief Span class used when c++20 is not available
 * @tparam T Type of data
 * @tparam Extent Static size of data referred by Span class
 */
template <typename T, std::size_t Extent = dynamic_extent> class Span;

/// \brief Check whether a type is `Span`
template <typename T> struct IsSpan : std::false_type {};

template <typename T, std::size_t Extent>
struct IsSpan<Span<T, Extent>> : std::true_type {};

template <typename T, std::size_t Extent> class Span {
  static_assert(Extent == dynamic_extent,
                "The current implementation supports dynamic-extent span only");

public:
  /// \brief Type used to iterate over this span (a raw pointer)
  using iterator = T *;

  /// \brief Constructor of Span class
  Span(T *t, std::size_t n) : ptr_{t}, size_{n} {}

  /// \brief Constructor of Span class for containers
  template <typename C,
            std::enable_if_t<
                !IsSpan<C>::value &&
                    std::is_pointer_v<decltype(std::declval<C &>().data())> &&
                    std::is_convertible_v<
                        std::remove_pointer_t<
                            decltype(std::declval<C &>().data())> (*)[],
                        T (*)[]> &&
                    std::is_convertible_v<decltype(std::declval<C>().size()),
                                          std::size_t>,
                int> = 0>
  Span(C &range) : ptr_{range.data()}, size_{range.size()} {}

  /// \brief Returns item by index
  T &operator[](ptrdiff_t idx) const {
    return ptr_[idx]; // NOLINT
  }

  /// \brief Returns pointer to data
  T *data() const { return ptr_; }

  /// \brief Returns number of data that referred by Span class
  std::size_t size() const { return size_; }

  /// \brief Returns begin iterator
  iterator begin() const { return ptr_; }

  /// \brief Returns end iterator
  iterator end() const {
    return ptr_ + size_; // NOLINT
  }

  /// \brief Returns size in bytes
  std::size_t size_bytes() const { return sizeof(T) * size_; }

private:
  T *ptr_;
  std::size_t size_;
};

#endif

} // namespace wasmtime

#endif // WASMTIME_SPAN_HH
