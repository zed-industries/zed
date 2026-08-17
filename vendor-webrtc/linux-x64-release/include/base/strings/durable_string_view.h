// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef BASE_STRINGS_DURABLE_STRING_VIEW_H_
#define BASE_STRINGS_DURABLE_STRING_VIEW_H_

#include <string_view>

#include "base/types/strong_alias.h"

namespace base {

// A strong type alias which denotes a std::string_view having durable storage.
// This allows the programmer to declare that a particular string_view is over
// memory that will not be deallocated. I.e., the programmer's use of this alias
// is a promise that it is safe to read from within the memory bounds.
//
// While the underlying string view data can and should be considered const,
// note that DurableStringView is unable to *guarantee* that underlying data
// is truly const.
using DurableStringView = base::StrongAlias<class DurableTag, std::string_view>;

}  // namespace base

#endif  // BASE_STRINGS_DURABLE_STRING_VIEW_H_
