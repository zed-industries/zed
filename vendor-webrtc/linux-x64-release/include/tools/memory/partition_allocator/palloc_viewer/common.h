// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_PARTITION_ALLOCATOR_PALLOC_VIEWER_COMMON_H_
#define TOOLS_MEMORY_PARTITION_ALLOCATOR_PALLOC_VIEWER_COMMON_H_

#include <stddef.h>

/* can't forward-declare Dwarf_Die, so use void* instead :/ */
typedef struct Dwfl Dwfl;
typedef struct Dwfl_Module Dwfl_Module;
Dwfl* addrlookup_init(pid_t pid);
Dwfl_Module* addrlookup_find_lib(Dwfl* dwfl, const char* name);
void* lookup_cu(Dwfl* dwfl,
                Dwfl_Module* mod,
                const char* expected_name,
                unsigned long* bias_out);
unsigned long addrlookup_get_struct_offset(void* scope,
                                           const char** namespaces,
                                           size_t namespaces_len,
                                           const char* struct_name,
                                           const char* member_name);
unsigned long addrlookup_get_variable_address(void* scope,
                                              unsigned long cu_bias,
                                              const char** namespaces,
                                              size_t namespaces_len,
                                              const char* name);
void addrlookup_finish(Dwfl* dwfl);

#endif  // TOOLS_MEMORY_PARTITION_ALLOCATOR_PALLOC_VIEWER_COMMON_H_
