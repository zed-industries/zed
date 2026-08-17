// Copyright 2019 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_THIRD_PARTY_LSS_LSS_H_
#define CRASHPAD_THIRD_PARTY_LSS_LSS_H_

#if defined(CRASHPAD_LSS_SOURCE_EXTERNAL)
#include "third_party/lss/linux_syscall_support.h"
#elif defined(CRASHPAD_LSS_SOURCE_EMBEDDED)
#include "third_party/lss/lss/linux_syscall_support.h"
#elif defined(CRASHPAD_LSS_SOURCE_FUCHSIA)
#include "../../../../../third_party/linux-syscall-support/src/linux_syscall_support.h"
#else
#error Unknown lss source
#endif

#endif  // CRASHPAD_THIRD_PARTY_LSS_LSS_H_
