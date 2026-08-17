// Copyright 2014 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef HEADER_PACKETED_BIO
#define HEADER_PACKETED_BIO

#include <functional>

#include <openssl/base.h>
#include <openssl/bio.h>

#if defined(OPENSSL_WINDOWS)
#include <winsock2.h>
#else
#include <sys/time.h>
#endif


// PacketedBioCreate creates a filter BIO which implements a reliable in-order
// blocking datagram socket. It uses the value of |*clock| as the clock.
// |get_timeout| should output what the |SSL| object believes is the next
// timeout, or return false if there is none. It will be compared against
// assertions from the runner. |set_mtu| will be called when the runner asks to
// change the MTU.
//
// During a |BIO_read|, the peer may signal the filter BIO to simulate a
// timeout. The operation will fail immediately. The caller must then call
// |PacketedBioAdvanceClock| before retrying |BIO_read|.
bssl::UniquePtr<BIO> PacketedBioCreate(
    timeval *clock, std::function<bool(timeval *)> get_timeout,
    std::function<bool(uint32_t)> set_mtu);

// PacketedBioAdvanceClock advances |bio|'s clock and returns true if there is a
// pending timeout. Otherwise, it returns false.
bool PacketedBioAdvanceClock(BIO *bio);

// PacketedBioAdvanceClock return's |bio|'s clock.
timeval *PacketedBioGetClock(BIO *bio);


#endif  // HEADER_PACKETED_BIO
