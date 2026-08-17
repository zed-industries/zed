// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_PRIVATE_MEMBERSHIP_BASE_PRIVATE_MEMBERSHIP_EXPORT_H_
#define THIRD_PARTY_PRIVATE_MEMBERSHIP_BASE_PRIVATE_MEMBERSHIP_EXPORT_H_

// PRIVATE_MEMBERSHIP_EXPORT is used to mark symbols as imported or
// exported when private_membership is built or used as a shared library.
// When private_membership is built as a static library the
// PRIVATE_MEMBERSHIP_EXPORT macro expands to nothing.

#ifdef PRIVATE_MEMBERSHIP_ENABLE_SYMBOL_EXPORT

#ifdef PRIVATE_MEMBERSHIP_WIN_EXPORT

#ifdef IS_PRIVATE_MEMBERSHIP_LIBRARY_IMPL
#define PRIVATE_MEMBERSHIP_EXPORT __declspec(dllexport)
#else
#define PRIVATE_MEMBERSHIP_EXPORT __declspec(dllimport)
#endif

#else  // PRIVATE_MEMBERSHIP_WIN_EXPORT

#if __has_attribute(visibility) && defined(IS_PRIVATE_MEMBERSHIP_LIBRARY_IMPL)
#define PRIVATE_MEMBERSHIP_EXPORT __attribute__((visibility("default")))
#endif

#endif  // PRIVATE_MEMBERSHIP_WIN_EXPORT

#endif  // PRIVATE_MEMBERSHIP_ENABLE_SYMBOL_EXPORT

#ifndef PRIVATE_MEMBERSHIP_EXPORT
#define PRIVATE_MEMBERSHIP_EXPORT
#endif

#endif  // THIRD_PARTY_PRIVATE_MEMBERSHIP_BASE_PRIVATE_MEMBERSHIP_EXPORT_H_
