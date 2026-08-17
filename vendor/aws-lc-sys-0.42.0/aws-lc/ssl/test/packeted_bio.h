// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef HEADER_PACKETED_BIO
#define HEADER_PACKETED_BIO

#include <openssl/base.h>
#include <openssl/bio.h>

#if defined(OPENSSL_WINDOWS)
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <winsock2.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#else
#include <sys/time.h>
#endif


// PacketedBioCreate creates a filter BIO which implements a reliable in-order
// blocking datagram socket. It uses the value of |*clock| as the clock.
//
// During a |BIO_read|, the peer may signal the filter BIO to simulate a
// timeout. The operation will fail immediately. The caller must then call
// |PacketedBioAdvanceClock| before retrying |BIO_read|.
bssl::UniquePtr<BIO> PacketedBioCreate(timeval *clock);

// PacketedBioAdvanceClock advances |bio|'s clock and returns true if there is a
// pending timeout. Otherwise, it returns false.
bool PacketedBioAdvanceClock(BIO *bio);


#endif  // HEADER_PACKETED_BIO
