// Copyright 2018 The Crashpad Authors
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

#include "build/build_config.h"

#if BUILDFLAG(IS_FUCHSIA)
#include "util/process/process_memory_fuchsia.h"
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
#include "util/process/process_memory_linux.h"
#elif BUILDFLAG(IS_WIN)
#include "util/process/process_memory_win.h"
#elif BUILDFLAG(IS_APPLE)
#include "util/process/process_memory_mac.h"
#endif

namespace crashpad {

#if BUILDFLAG(IS_FUCHSIA) || DOXYGEN
//! \brief Alias for platform-specific native implementation of ProcessMemory.
using ProcessMemoryNative = ProcessMemoryFuchsia;
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
using ProcessMemoryNative = ProcessMemoryLinux;
#elif BUILDFLAG(IS_WIN)
using ProcessMemoryNative = ProcessMemoryWin;
#elif BUILDFLAG(IS_APPLE)
using ProcessMemoryNative = ProcessMemoryMac;
#else
#error Port.
#endif

}  // namespace crashpad
