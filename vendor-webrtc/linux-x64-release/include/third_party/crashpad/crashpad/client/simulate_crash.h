// Copyright 2014 The Crashpad Authors
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

#ifndef CRASHPAD_CLIENT_SIMULATE_CRASH_H_
#define CRASHPAD_CLIENT_SIMULATE_CRASH_H_

#include "build/build_config.h"

// IWYU pragma: begin_exports
#if BUILDFLAG(IS_MAC)
#include "client/simulate_crash_mac.h"
#elif BUILDFLAG(IS_IOS)
#include "client/simulate_crash_ios.h"
#elif BUILDFLAG(IS_WIN)
#include "client/simulate_crash_win.h"
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
#include "client/simulate_crash_linux.h"
#endif
// IWYU pragma: end_exports

#endif  // CRASHPAD_CLIENT_SIMULATE_CRASH_H_
