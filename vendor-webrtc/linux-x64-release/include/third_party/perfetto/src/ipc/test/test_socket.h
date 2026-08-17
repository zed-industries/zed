/*
 * Copyright (C) 2017 The Android Open Source Project
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

#ifndef SRC_IPC_TEST_TEST_SOCKET_H_
#define SRC_IPC_TEST_TEST_SOCKET_H_

#include <stdint.h>
#include <stdio.h>

#include <cinttypes>

#include "perfetto/base/build_config.h"
#include "perfetto/ext/base/unix_socket.h"

namespace perfetto {
namespace ipc {

struct TestSocket {
  explicit constexpr TestSocket(const char* test_name)
      : test_name_(test_name) {}

  const char* test_name_;
  char buf_[64]{};

  // Inline to avoid multiple definition linker warnings (and avoid a .cc file).
  inline base::SockFamily family();
  inline const char* name();
  inline void Destroy();
};

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)

const char* TestSocket::name() {
  uint64_t hash = 5381;
  for (const char* c = test_name_; *c; c++)
    hash = 33 * hash + static_cast<uint64_t>(*c);
  snprintf(buf_, sizeof(buf_), "127.0.0.1:%" PRIu64, 40000 + (hash % 20000));
  return buf_;
}
base::SockFamily TestSocket::family() {
  return base::SockFamily::kInet;
}
void TestSocket::Destroy() {}

#elif PERFETTO_BUILDFLAG(PERFETTO_OS_ANDROID)

const char* TestSocket::name() {
  snprintf(buf_, sizeof(buf_), "@%s", test_name_);
  return buf_;
}
base::SockFamily TestSocket::family() {
  return base::SockFamily::kUnix;
}
void TestSocket::Destroy() {}

#elif PERFETTO_BUILDFLAG(PERFETTO_OS_FUCHSIA)

const char* TestSocket::name() {
  return "zx_socket";
}
base::SockFamily TestSocket::family() {
  return base::SockFamily::kUnix;
}
void TestSocket::Destroy() {}

#else

const char* TestSocket::name() {
  snprintf(buf_, sizeof(buf_), "/tmp/%s.sock", test_name_);
  return buf_;
}
base::SockFamily TestSocket::family() {
  return base::SockFamily::kUnix;
}
void TestSocket::Destroy() {
  remove(name());
}
#endif

}  // namespace ipc
}  // namespace perfetto

#endif  // SRC_IPC_TEST_TEST_SOCKET_H_
