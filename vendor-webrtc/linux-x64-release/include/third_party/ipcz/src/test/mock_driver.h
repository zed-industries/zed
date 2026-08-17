// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_TEST_MOCK_DRIVER_H_
#define IPCZ_SRC_TEST_MOCK_DRIVER_H_

#include "ipcz/ipcz.h"
#include "testing/gmock/include/gmock/gmock.h"

namespace ipcz::test {

// A mock implementation of the IpczDriver API which tests can use to precisely
// introspect driver API invocations made by ipcz. At most one instance of this
// class may exist at a time, and the kMockDriver driver declared below can only
// be used while such an instance exists.
class MockDriver {
 public:
  MockDriver();
  ~MockDriver();

  MOCK_METHOD(IpczResult, Close, (IpczDriverHandle, uint32_t, const void*));
  MOCK_METHOD(IpczResult,
              Serialize,
              (IpczDriverHandle,
               IpczDriverHandle,
               uint32_t,
               const void*,
               volatile void*,
               size_t*,
               IpczDriverHandle*,
               size_t*));
  MOCK_METHOD(IpczResult,
              Deserialize,
              (const volatile void*,
               size_t,
               const IpczDriverHandle*,
               size_t,
               IpczDriverHandle,
               uint32_t,
               const void*,
               IpczDriverHandle*));
  MOCK_METHOD(IpczResult,
              CreateTransports,
              (IpczDriverHandle,
               IpczDriverHandle,
               uint32_t,
               const void*,
               IpczDriverHandle*,
               IpczDriverHandle*));
  MOCK_METHOD(IpczResult,
              ActivateTransport,
              (IpczDriverHandle,
               IpczHandle,
               IpczTransportActivityHandler,
               uint32_t,
               const void*));
  MOCK_METHOD(IpczResult,
              DeactivateTransport,
              (IpczDriverHandle, uint32_t, const void*));
  MOCK_METHOD(IpczResult,
              Transmit,
              (IpczDriverHandle,
               const void*,
               size_t,
               const IpczDriverHandle*,
               size_t,
               uint32_t,
               const void*));
  MOCK_METHOD(IpczResult,
              ReportBadTransportActivity,
              (IpczDriverHandle, uintptr_t, uint32_t, const void*));
  MOCK_METHOD(IpczResult,
              AllocateSharedMemory,
              (size_t, uint32_t, const void*, IpczDriverHandle*));
  MOCK_METHOD(IpczResult,
              GetSharedMemoryInfo,
              (IpczDriverHandle, uint32_t, const void*, IpczSharedMemoryInfo*));
  MOCK_METHOD(IpczResult,
              DuplicateSharedMemory,
              (IpczDriverHandle, uint32_t, const void*, IpczDriverHandle*));
  MOCK_METHOD(IpczResult,
              MapSharedMemory,
              (IpczDriverHandle,
               uint32_t,
               const void*,
               volatile void**,
               IpczDriverHandle*));
  MOCK_METHOD(IpczResult,
              GenerateRandomBytes,
              (size_t, uint32_t, const void*, void*));
};

// An ipcz driver which forwards its API invocations to the only currently
// existing instance of the MockDriver class above. It is an error to elicit
// driver calls from ipcz while no MockDriver instance exists.
extern const IpczDriver kMockDriver;

}  // namespace ipcz::test

#endif  // IPCZ_SRC_TEST_MOCK_DRIVER_H_
