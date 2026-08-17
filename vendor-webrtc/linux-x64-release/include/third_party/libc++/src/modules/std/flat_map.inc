// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

export namespace std {
#if _LIBCPP_STD_VER >= 23
  // [flat.map], class template flat_map
  using std::flat_map;

  using std::sorted_unique;
  using std::sorted_unique_t;

  using std::uses_allocator;

  // [flat.map.erasure], erasure for flat_map
  using std::erase_if;

  // [flat.multimap], class template flat_multimap
  using std::flat_multimap;

  using std::sorted_equivalent;
  using std::sorted_equivalent_t;

  // [flat.multimap.erasure], erasure for flat_multimap
#endif // _LIBCPP_STD_VER >= 23
} // namespace std
