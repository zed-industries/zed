// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_WTF_SIZE_T_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_WTF_SIZE_T_H_

#include <limits.h>
#include <stdint.h>

namespace WTF {

// TLDR: size_t != wtf_size_t
//
// WTF defines wtf_size_t as an unsigned 32 bit integer. This is to align
// with the maximum heap allocation object size and save memory. This deviates
// from Chromium C++ style guide which calls for interfaces to use the
// stdint.h types (e.g. int32_t, size_t) on the exposed interface.
//
// Matching the external API to match the internal API have a number of
// required properties:
//  - Internal storage for Vector and String are all uint32_t based.
//  - Max heap allocation size is kMaxHeapObjectSize (much less than UINTMAX).
//  - Consumers of APIs such as WTF::Vector may store their indices in some
//    other storage and using size_t consumes extra data.
//  - Checked_casts are too slow to use internally.
//  - Conversion from wtf_size_t to size_t is always safe and static_casts
//    are good enough when explicitly required.
//  - Conversion from size_t to wtf_size_t is *not* safe and checked_casts
//    are always required.
//
// It may be possible in the future to move Vector and String to be size_t
// based and this definition may not be necessary, so long as the internal
// type matches the external type.
using wtf_size_t = uint32_t;
const wtf_size_t kNotFound = UINT_MAX;

}  // namespace WTF

using WTF::kNotFound;
using WTF::wtf_size_t;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_WTF_SIZE_T_H_
