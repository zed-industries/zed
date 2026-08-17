#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_COMMON_MDSPAN_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_COMMON_MDSPAN_H_

#include <array>
#include <cstddef>
#include <cstring>
#include <functional>
#include <memory>
#include <numeric>
#include <ostream>
#include <type_traits>
#include <utility>

#include "absl/base/nullability.h"
#include "absl/log/absl_check.h"
#include "absl/strings/str_join.h"
#include "absl/types/span.h"

namespace mediapipe::tasks::genai {

// Multi-dimensional span. Adopting limited feature of std::mdspan (c++23).
// Always assuming row-major order. Support rank up to 4.
//
// Example usage:
//
// std::vector<float> data(10 * 10);
// MdSpan<float, 2> span = MakeMdSpan(data.data(), 10, 10);
// EXPECT_EQ(span.size(), 100);
// EXPECT_EQ(span.at(4, 6), data[46]);      // access through multi-indices
// EXPECT_EQ(span[3][7], data[37]);         // span[3] will create subspan
// span[9][9] = 1.25f;                      // span is a reference to data.
// EXPECT_EQ(data[99], 1.25f);
//
// {
//   MdSpan<float, 2> span2 = MakeMdSpan(data.data(), 10, 10, [data =
//   std::move(data)](){}); EXPECT_EQ(span2.size(), 100);
// }  // span2 is destroyed, data is deleted.

template <typename T, size_t Rank>
struct MdSpan;

namespace mdspan_internal {

// Helper to invoke deleter when call reference to MdSpan is destroyed.
struct DeleteHelper {
  explicit DeleteHelper(std::function<void()> dltr) : deleter(std::move(dltr)) {
    ABSL_CHECK(deleter);
  }
  // No copy nor move.
  DeleteHelper(const DeleteHelper&) = delete;
  DeleteHelper& operator=(const DeleteHelper&) = delete;
  DeleteHelper(DeleteHelper&&) noexcept = delete;
  DeleteHelper& operator=(DeleteHelper&&) noexcept = delete;
  ~DeleteHelper() { deleter(); }

  std::function<void()> deleter = nullptr;
};

}  // namespace mdspan_internal

template <typename T>
MdSpan<T, 1> MakeMdSpan(absl::Nonnull<T*> data, size_t d1,
                        std::function<void()> deleter = nullptr) {
  auto delete_helper =
      deleter
          ? std::make_shared<mdspan_internal::DeleteHelper>(std::move(deleter))
          : nullptr;
  return MdSpan<T, 1>(data, {d1}, delete_helper);
}
template <typename T>
MdSpan<T, 2> MakeMdSpan(absl::Nonnull<T*> data, size_t d1, size_t d2,
                        std::function<void()> deleter = nullptr) {
  auto delete_helper =
      deleter
          ? std::make_shared<mdspan_internal::DeleteHelper>(std::move(deleter))
          : nullptr;
  return MdSpan<T, 2>(data, {d1, d2}, delete_helper);
}
template <typename T>
MdSpan<T, 3> MakeMdSpan(absl::Nonnull<T*> data, size_t d1, size_t d2, size_t d3,
                        std::function<void()> deleter = nullptr) {
  auto delete_helper =
      deleter
          ? std::make_shared<mdspan_internal::DeleteHelper>(std::move(deleter))
          : nullptr;
  return MdSpan<T, 3>(data, {d1, d2, d3}, delete_helper);
}
template <typename T>
MdSpan<T, 4> MakeMdSpan(absl::Nonnull<T*> data, size_t d1, size_t d2, size_t d3,
                        size_t d4, std::function<void()> deleter = nullptr) {
  auto delete_helper =
      deleter
          ? std::make_shared<mdspan_internal::DeleteHelper>(std::move(deleter))
          : nullptr;
  return MdSpan<T, 4>(data, {d1, d2, d3, d4}, delete_helper);
}
template <typename T, typename D, typename... Sizes>
auto MakeMdSpan(absl::Nonnull<D*> data, Sizes... sizes) {
  return MakeMdSpan(static_cast<T*>(data), sizes...);
}

template <typename T, size_t Rank>
struct MdSpan {
  using element_type = T;
  using subspan_type = std::conditional_t<Rank == 1, element_type&,
                                          MdSpan<element_type, Rank - 1>>;
  using const_subspan_type =
      std::conditional_t<Rank == 1, const element_type&,
                         MdSpan<const element_type, Rank - 1>>;

  MdSpan() = default;
  // Enable move and copy.
  MdSpan(MdSpan<element_type, Rank>&& other) = default;
  MdSpan(const MdSpan<element_type, Rank>& other) = default;
  MdSpan<element_type, Rank>& operator=(MdSpan<element_type, Rank>&& other) =
      default;
  MdSpan<element_type, Rank>& operator=(
      const MdSpan<element_type, Rank>& other) = default;

  const std::array<size_t, Rank>& shape() const { return shape_internal; }

  size_t size() const {
    return std::accumulate(shape_internal.begin(), shape_internal.end(), 1,
                           std::multiplies<size_t>());
  }

  element_type* data() { return flattened.data(); }
  const element_type* data() const { return flattened.data(); }

  auto begin() { return flattened.begin(); }
  auto begin() const { return flattened.begin(); }
  auto end() { return flattened.end(); }
  auto end() const { return flattened.end(); }

  template <typename... Indices>
  element_type& at(Indices... indices) {
    return at_helper(data(), Rank, indices...);
  }

  template <typename... Indices>
  const element_type& at(size_t idx, Indices... indices) const {
    return const_cast<MdSpan*>(this)->at(idx, indices...);
  }

  subspan_type operator[](size_t idx) {
    ABSL_DCHECK_LT(idx, shape_internal[0]);
    if constexpr (Rank == 1) {
      ABSL_DCHECK_EQ(shape_internal.size(), 1);
      return flattened[idx];
    } else {
      ABSL_DCHECK_GT(shape_internal.size(), 1);
      std::array<size_t, Rank - 1> new_shape;
      std::memcpy(new_shape.data(), shape_internal.data() + 1,
                  sizeof(size_t) * (Rank - 1));
      const size_t subspan_size = std::accumulate(
          new_shape.begin(), new_shape.end(), 1, std::multiplies<size_t>());
      return MdSpan<element_type, Rank - 1>(
          data() + idx * subspan_size, std::move(new_shape), delete_helper);
    }
  }

  const_subspan_type operator[](size_t idx) const {
    ABSL_DCHECK_LT(idx, shape_internal[0]);
    if constexpr (Rank == 1) {
      ABSL_DCHECK_EQ(shape_internal.size(), 1);
      return flattened[idx];
    } else {
      ABSL_DCHECK_GT(shape_internal.size(), 1);
      std::array<size_t, Rank - 1> new_shape;
      std::memcpy(new_shape.data(), shape_internal.data() + 1,
                  sizeof(size_t) * (Rank - 1));
      const size_t subspan_size = std::accumulate(
          new_shape.begin(), new_shape.end(), 1, std::multiplies<size_t>());
      return MdSpan<const element_type, Rank - 1>(
          data() + idx * subspan_size, std::move(new_shape), delete_helper);
    }
  }

 private:
  template <typename U>
  friend MdSpan<U, 1> MakeMdSpan(absl::Nonnull<U*> data, size_t d1,
                                 std::function<void()> deleter);
  template <typename U>
  friend MdSpan<U, 2> MakeMdSpan(absl::Nonnull<U*> data, size_t d1, size_t d2,
                                 std::function<void()> deleter);
  template <typename U>
  friend MdSpan<U, 3> MakeMdSpan(absl::Nonnull<U*> data, size_t d1, size_t d2,
                                 size_t d3, std::function<void()> deleter);
  template <typename U>
  friend MdSpan<U, 4> MakeMdSpan(absl::Nonnull<U*> data, size_t d1, size_t d2,
                                 size_t d3, size_t d4,
                                 std::function<void()> deleter);
  // MdSpan with arbitrary type/rank.
  template <typename U, size_t K>
  friend struct MdSpan;
  // For printing.
  template <typename U, size_t K>
  friend std::ostream& operator<<(std::ostream& os, const MdSpan<U, K>& span);

  std::ostream& print_just_content(std::ostream& os) const {
    constexpr size_t kNicePrintThreshold = 4;
    if constexpr (Rank == 1) {
      if (size() <= kNicePrintThreshold) {
        os << "[" << absl::StrJoin(*this, ",") << "]";
      } else {
        absl::Span<const T> prefix_span =
            absl::MakeConstSpan(data(), kNicePrintThreshold - 1);
        os << "[" << absl::StrJoin(prefix_span, ",") << ", ..., "
           << *(end() - 1) << "]";
      }
      return os;
    } else {
      os << "[";
      bool first_line = true;
      for (size_t i = 0; i < shape()[0]; ++i) {
        if (!first_line) {
          os << "\n";
        } else {
          first_line = false;
        }
        operator[](i).print_just_content(os);
        if (i == kNicePrintThreshold && kNicePrintThreshold < shape()[0] - 2) {
          os << "\n...";
          i = shape()[0] - 2;
        }
      }
      return os << "]";
    }
  }

  MdSpan(absl::Nonnull<T*> data, std::array<size_t, Rank> shape,
         std::shared_ptr<mdspan_internal::DeleteHelper> deleter)
      : shape_internal(std::move(shape)), delete_helper(std::move(deleter)) {
    flattened = absl::MakeSpan(data, size());
  }

  template <typename... Indices>
  element_type& at_helper(element_type* base, size_t rank, size_t idx,
                          Indices... indices) {
    const size_t subspan_size =
        std::accumulate(shape_internal.begin() + Rank - rank + 1,
                        shape_internal.end(), 1, std::multiplies<size_t>());
    base += idx * subspan_size;
    return at_helper(base, rank - 1, indices...);
  }

  template <typename... Indices>
  element_type& at_helper(element_type* base, size_t rank, size_t idx) {
    ABSL_CHECK_EQ(rank, 1);
    base += idx;
    return *base;
  }

  absl::Span<T> flattened;
  std::array<size_t, Rank> shape_internal;
  std::shared_ptr<mdspan_internal::DeleteHelper> delete_helper;
};

template <typename T, size_t Rank>
std::ostream& operator<<(std::ostream& os, const MdSpan<T, Rank>& span) {
  span.print_just_content(os);
  return os << " shape=(" << absl::StrJoin(span.shape(), ",") << ")";
}

}  // namespace mediapipe::tasks::genai

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_COMMON_MDSPAN_H_
