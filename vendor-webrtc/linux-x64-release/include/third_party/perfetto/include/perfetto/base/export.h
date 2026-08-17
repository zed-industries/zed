/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_BASE_EXPORT_H_
#define INCLUDE_PERFETTO_BASE_EXPORT_H_

#include "perfetto/base/build_config.h"
#include "perfetto/public/abi/export.h"

// PERFETTO_EXPORT_COMPONENT: Exports a symbol among C++ components when
// building with is_component = true (mostly used by chromium build).
#if PERFETTO_BUILDFLAG(PERFETTO_COMPONENT_BUILD)

#if defined(PERFETTO_IMPLEMENTATION)
#define PERFETTO_EXPORT_COMPONENT PERFETTO_INTERNAL_DLL_EXPORT
#else
#define PERFETTO_EXPORT_COMPONENT PERFETTO_INTERNAL_DLL_IMPORT
#endif

#else  // !PERFETTO_BUILDFLAG(PERFETTO_COMPONENT_BUILD)

#if !defined(PERFETTO_EXPORT_COMPONENT)
#define PERFETTO_EXPORT_COMPONENT
#endif  // !defined(PERFETTO_EXPORT_COMPONENT)

#endif  // PERFETTO_BUILDFLAG(PERFETTO_COMPONENT_BUILD)

#endif  // INCLUDE_PERFETTO_BASE_EXPORT_H_
