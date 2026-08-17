// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMEZONE_TIMEZONE_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMEZONE_TIMEZONE_CONTROLLER_H_

#include <memory>

#include "mojo/public/cpp/bindings/receiver.h"
#include "services/device/public/mojom/time_zone_monitor.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// An instance of TimeZoneController manages renderer time zone. It listens to
// the host system time zone change notifications from the browser and when
// received, updates the default ICU time zone and notifies V8 and workers.
//
// Time zone override mode allows clients to temporarily override host system
// time zone with the one specified by the client. The client can change the
// time zone, however no other client will be allowed to set another override
// until the existing override is removed. When the override is removed, the
// current host system time zone is assumed.
class CORE_EXPORT TimeZoneController final
    : public device::mojom::blink::TimeZoneMonitorClient {
 public:
  ~TimeZoneController() override;

  static void Init();

  class TimeZoneOverride {
    friend TimeZoneController;
    TimeZoneOverride() = default;

   public:
    void change(const String& timezone_id) {
      ChangeTimeZoneOverride(timezone_id);
    }

    ~TimeZoneOverride() { ClearTimeZoneOverride(); }
  };

  static std::unique_ptr<TimeZoneOverride> SetTimeZoneOverride(
      const String& timezone_id);

  static bool HasTimeZoneOverride();
  static const String& TimeZoneIdOverride();

  static void ChangeTimeZoneForTesting(const String&);

 private:
  TimeZoneController();
  static TimeZoneController& instance();
  static void ClearTimeZoneOverride();
  static void ChangeTimeZoneOverride(const String&);

  const String& GetHostTimezoneId();

  // device::mojom::blink::TimeZoneMonitorClient:
  void OnTimeZoneChange(const String& timezone_id) override;

  // receiver_ must not use HeapMojoReceiver. TimeZoneController is not managed
  // by Oilpan.
  mojo::Receiver<device::mojom::blink::TimeZoneMonitorClient> receiver_{this};

  String host_timezone_id_;
  String override_timezone_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMEZONE_TIMEZONE_CONTROLLER_H_
