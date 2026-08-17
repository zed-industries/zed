// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_REFERENCE_DRIVERS_MULTIPROCESS_REFERENCE_DRIVER_H_
#define IPCZ_SRC_REFERENCE_DRIVERS_MULTIPROCESS_REFERENCE_DRIVER_H_

#include "ipcz/ipcz.h"
#include "reference_drivers/file_descriptor.h"
#include "reference_drivers/socket_transport.h"

namespace ipcz::reference_drivers {

// A basic reference driver which supports multiprocess operation. This is also
// suitable for single-process usage, but unlike kSingleProcessReferenceDriver
// all transmissions through this driver are asynchronous.
extern const IpczDriver kMultiprocessReferenceDriver;

// Creates a new multiprocess-capable driver transport from a SocketTransport
// endpoint and returns an IpczDriverHandle to reference it.
IpczDriverHandle CreateMultiprocessTransport(Ref<SocketTransport> transport);

// Extracts the underlying file descriptor from a socket-based multiprocess
// driver transport. `transport` is effectively consumed and invalidated by this
// call.
FileDescriptor TakeMultiprocessTransportDescriptor(IpczDriverHandle transport);

}  // namespace ipcz::reference_drivers

#endif  // IPCZ_SRC_REFERENCE_DRIVERS_MULTIPROCESS_REFERENCE_DRIVER_H_
