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

#ifndef INCLUDE_PERFETTO_EXT_BASE_CTRL_C_HANDLER_H_
#define INCLUDE_PERFETTO_EXT_BASE_CTRL_C_HANDLER_H_

namespace perfetto {
namespace base {

// On Linux/Android/Mac: installs SIGINT + SIGTERM signal handlers.
// On Windows: installs a SetConsoleCtrlHandler() handler.
// The passed handler must be async safe.
using CtrlCHandlerFunction = void (*)();
void InstallCtrlCHandler(CtrlCHandlerFunction);

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_CTRL_C_HANDLER_H_
