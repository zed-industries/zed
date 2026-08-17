/*
 * Copyright (C) 2021 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_GETOPT_H_
#define INCLUDE_PERFETTO_EXT_BASE_GETOPT_H_

// This is the header that should be included in all places that need getopt.h.
// This either routes on the sysroot getopt.h, for OSes that have one (all but
// Windows) or routes on the home-brewed getopt_compat.h.

#include "perfetto/base/build_config.h"

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
#include "perfetto/ext/base/getopt_compat.h"

// getopt_compat.h puts everything in a nested namespace, to allow
// getopt_compat_unittest.cc to use both <getopt.h> and "getopt_compat.h"
// without collisions.

// Here we expose them into the root namespace, because we want offer a drop-in
// replacement to the various main.cc, which can't know about the nested
// namespace.
using ::perfetto::base::getopt_compat::optarg;
using ::perfetto::base::getopt_compat::opterr;
using ::perfetto::base::getopt_compat::optind;
using ::perfetto::base::getopt_compat::option;
using ::perfetto::base::getopt_compat::optopt;
constexpr auto getopt = ::perfetto::base::getopt_compat::getopt;
constexpr auto getopt_long = ::perfetto::base::getopt_compat::getopt_long;
constexpr auto no_argument = ::perfetto::base::getopt_compat::no_argument;
constexpr auto required_argument =
    ::perfetto::base::getopt_compat::required_argument;

#else
#include <getopt.h>  // IWYU pragma: export
#endif

#endif  // INCLUDE_PERFETTO_EXT_BASE_GETOPT_H_
