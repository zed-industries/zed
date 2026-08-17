// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_MOCK_DEVICES_CHANGED_OBSERVER_H_
#define BASE_TEST_MOCK_DEVICES_CHANGED_OBSERVER_H_

#include "base/system/system_monitor.h"
#include "testing/gmock/include/gmock/gmock.h"

namespace base {

class MockDevicesChangedObserver
    : public base::SystemMonitor::DevicesChangedObserver {
 public:
  MockDevicesChangedObserver();

  MockDevicesChangedObserver(const MockDevicesChangedObserver&) = delete;
  MockDevicesChangedObserver& operator=(const MockDevicesChangedObserver&) =
      delete;

  ~MockDevicesChangedObserver() override;

  MOCK_METHOD1(OnDevicesChanged,
               void(base::SystemMonitor::DeviceType device_type));
};

}  // namespace base

#endif  // BASE_TEST_MOCK_DEVICES_CHANGED_OBSERVER_H_
