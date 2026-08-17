// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUNCTIONAL_UNRETAINED_TRAITS_H_
#define BASE_FUNCTIONAL_UNRETAINED_TRAITS_H_

#include <type_traits>

#include "base/types/is_complete.h"
#include "base/types/same_as_any.h"
#include "build/build_config.h"

// Various opaque system types that should still be usable with the base
// callback system. Please keep sorted.
#define BASE_INTERNAL_LIST_OF_SAFE_FOR_UNRETAINED           \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(ANativeWindow)          \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(DBusMessage)            \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(HWND__)                 \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(VkBuffer_T)             \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(VkDeviceMemory_T)       \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(VkImage_T)              \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(VkSemaphore_T)          \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(VmaAllocation_T)        \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(WGPUAdapterImpl)        \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(fpdf_action_t__)        \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(fpdf_annotation_t__)    \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(fpdf_attachment_t__)    \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(fpdf_bookmark_t__)      \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(fpdf_document_t__)      \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(fpdf_form_handle_t__)   \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(fpdf_page_t__)          \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(fpdf_structelement_t__) \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(hb_set_t)               \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(wl_gpu)                 \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(wl_shm)                 \
  BASE_INTERNAL_SAFE_FOR_UNRETAINED(wl_surface)

#define BASE_INTERNAL_SAFE_FOR_UNRETAINED(x) struct x;
BASE_INTERNAL_LIST_OF_SAFE_FOR_UNRETAINED
#undef BASE_INTERNAL_SAFE_FOR_UNRETAINED

namespace base::internal {

// Determining whether a type can be used with `Unretained()` requires that `T`
// be completely defined. Some system types have an intentionally opaque and
// incomplete representation, but should still be usable with `Unretained()`.
template <typename T>
concept SafeIncompleteTypeForUnretained =
    SameAsAny<std::remove_cvref_t<T>,
#define BASE_INTERNAL_SAFE_FOR_UNRETAINED(x) x,
              BASE_INTERNAL_LIST_OF_SAFE_FOR_UNRETAINED
#undef BASE_INTERNAL_SAFE_FOR_UNRETAINED
              // void* is occasionally used with callbacks; in the future,
              // this may be more restricted/limited, but allow it for now.
              void>;

// Customization point. Specialize this to be `false` for types as needed. In
// general, you should not need this; types that do not support `Unretained()`
// should use `DISALLOW_UNRETAINED()`. However, this is necessary when
// disallowing `Unretained()` for types that do not (or cannot) use //base.
template <typename T>
inline constexpr bool kCustomizeSupportsUnretained = true;

template <typename T>
concept DisallowsUnretained = !kCustomizeSupportsUnretained<T> || requires {
  // Matches against the marker tag created by the `DISALLOW_UNRETAINED()` macro
  // in //base/functional/disallow_unretained.h.
  typename T::DisallowBaseUnretainedMarker;
};

template <typename T>
struct SupportsUnretainedImpl {
  // For context on this "templated struct with a lambda that asserts" pattern,
  // see comments in `Invoker<>`.
  template <bool v = IsComplete<T> || SafeIncompleteTypeForUnretained<T>>
  struct AllowlistIncompleteTypes {
    static constexpr bool value = [] {
// Incrementally enforce the requirement to be completely defined. For now,
// limit the failures to:
//
// - non-test code
// - non-official code (because these builds don't run as part of the default CQ
//   and are slower due to PGO and LTO)
// - Android, Linux or Windows
//
// to make this easier to land without potentially breaking the tree.
//
// TODO(crbug.com/40247956): Enable this on all platforms, then in
// official builds, and then in non-test code as well.
#if defined(FORCE_UNRETAINED_COMPLETENESS_CHECKS_FOR_TESTS) || \
    (!defined(UNIT_TEST) && !defined(OFFICIAL_BUILD) &&        \
     (BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_WIN)))
      static_assert(v,
                    "Argument requires unretained storage, but type is not "
                    "fully defined. This prevents determining whether "
                    "`Unretained()` is supported.");
      return v;
#else
      return true;
#endif
    }();
  };

  template <bool v = !DisallowsUnretained<T>>
  struct AllowsUnretained {
    static constexpr bool value = [] {
      static_assert(v,
                    "Argument requires unretained storage, but type does not "
                    "support `Unretained()`. See "
                    "base/functional/disallow_unretained.h for alternatives.");
      return v;
    }();
  };

  static constexpr bool value =
      std::conjunction_v<AllowlistIncompleteTypes<>, AllowsUnretained<>>;
};

// Not meant for general use: merely checking this concept will
// `static_assert()` if the type does not support unretained. This is meant only
// for use inside the `Bind()` machinery, which wants that assertion.
//
// If we ever want to use this concept outside that machinery, we'll need to not
// only move the "allows unretained" assertion from above to the `Bind()` side,
// we'll also need to hoist or duplicate the incomplete type check there (and
// not assert in that case) so it does not fire multiple `static_assert()`s for
// incomplete types.
template <typename T>
concept SupportsUnretained = SupportsUnretainedImpl<T>::value;

}  // namespace base::internal

#endif  // BASE_FUNCTIONAL_UNRETAINED_TRAITS_H_
