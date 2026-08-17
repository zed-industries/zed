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

#ifndef INCLUDE_PERFETTO_TRACING_DEFAULT_SOCKET_H_
#define INCLUDE_PERFETTO_TRACING_DEFAULT_SOCKET_H_

#include <string>
#include <vector>

#include "perfetto/base/export.h"

namespace perfetto {

PERFETTO_EXPORT_COMPONENT const char* GetConsumerSocket();
// This function is used for tokenize the |producer_socket_names| string into
// multiple producer socket names.
PERFETTO_EXPORT_COMPONENT std::vector<std::string> TokenizeProducerSockets(
    const char* producer_socket_names);
PERFETTO_EXPORT_COMPONENT const char* GetProducerSocket();

// Optionally returns the relay socket name. The relay socket is used
// for forwarding the IPC messages between the local producers and the remote
// tracing service.
PERFETTO_EXPORT_COMPONENT std::string GetRelaySocket();

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_DEFAULT_SOCKET_H_
