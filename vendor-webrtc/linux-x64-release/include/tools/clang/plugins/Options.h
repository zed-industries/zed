// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_OPTIONS_H_
#define TOOLS_CLANG_PLUGINS_OPTIONS_H_

#include <string>
#include <vector>

namespace chrome_checker {

struct Options {
  bool check_base_classes = false;
  bool check_blink_data_member_type = false;
  bool check_ipc = false;
  bool check_layout_object_methods = false;
  bool check_stack_allocated = false;
  bool check_ptrs_to_non_string_literals = false;
  bool check_span_fields = false;
  bool enable_match_profiling = false;
  std::string exclude_fields_file;
};

}  // namespace chrome_checker

#endif  // TOOLS_CLANG_PLUGINS_OPTIONS_H_
