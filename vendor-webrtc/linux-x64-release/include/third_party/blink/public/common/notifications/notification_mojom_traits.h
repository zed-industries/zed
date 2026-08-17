// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_NOTIFICATION_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_NOTIFICATION_MOJOM_TRAITS_H_

#include <optional>
#include <string>

#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "mojo/public/cpp/base/string16_mojom_traits.h"
#include "mojo/public/cpp/base/time_mojom_traits.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "skia/public/mojom/bitmap_skbitmap_mojom_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/notifications/notification_resources.h"
#include "third_party/blink/public/common/notifications/platform_notification_data.h"
#include "url/gurl.h"
#include "url/mojom/url_gurl_mojom_traits.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::NotificationDataDataView,
                                        blink::PlatformNotificationData> {
  static const std::u16string& title(
      const blink::PlatformNotificationData& data) {
    return data.title;
  }

  static blink::mojom::NotificationDirection direction(
      const blink::PlatformNotificationData& data) {
    return data.direction;
  }

  static const std::string& lang(const blink::PlatformNotificationData& data) {
    return data.lang;
  }

  static const std::u16string& body(
      const blink::PlatformNotificationData& data) {
    return data.body;
  }

  static const std::string& tag(const blink::PlatformNotificationData& data) {
    return data.tag;
  }

  static const GURL& image(const blink::PlatformNotificationData& data) {
    return data.image;
  }

  static const GURL& icon(const blink::PlatformNotificationData& data) {
    return data.icon;
  }

  static const GURL& badge(const blink::PlatformNotificationData& data) {
    return data.badge;
  }

  static const base::span<const int32_t> vibration_pattern(
      const blink::PlatformNotificationData& data) {
    // TODO(https://crbug.com/798466): Store as int32s to avoid this cast.
    return UNSAFE_TODO(base::span(
        reinterpret_cast<const int32_t*>(data.vibration_pattern.data()),
        data.vibration_pattern.size()));
  }

  static double timestamp(const blink::PlatformNotificationData& data) {
    return data.timestamp.InMillisecondsFSinceUnixEpoch();
  }

  static bool renotify(const blink::PlatformNotificationData& data) {
    return data.renotify;
  }

  static bool silent(const blink::PlatformNotificationData& data) {
    return data.silent;
  }

  static bool require_interaction(const blink::PlatformNotificationData& data) {
    return data.require_interaction;
  }

  static const base::span<const uint8_t> data(
      const blink::PlatformNotificationData& data) {
    // TODO(https://crbug.com/798466): Align data types to avoid this cast.
    return base::as_byte_span(data.data);
  }

  static const std::vector<blink::mojom::NotificationActionPtr>& actions(
      const blink::PlatformNotificationData& data) {
    return data.actions;
  }

  static std::optional<base::Time> show_trigger_timestamp(
      const blink::PlatformNotificationData& data) {
    return data.show_trigger_timestamp;
  }

  static blink::mojom::NotificationScenario scenario(
      const blink::PlatformNotificationData& data) {
    return data.scenario;
  }

  static bool Read(blink::mojom::NotificationDataDataView notification_data,
                   blink::PlatformNotificationData* platform_notification_data);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::NotificationResourcesDataView,
                 blink::NotificationResources> {
  static const SkBitmap& image(const blink::NotificationResources& resources) {
    return resources.image;
  }

  static const SkBitmap& icon(const blink::NotificationResources& resources) {
    return resources.notification_icon;
  }

  static const SkBitmap& badge(const blink::NotificationResources& resources) {
    return resources.badge;
  }

  static const std::vector<SkBitmap>& action_icons(
      const blink::NotificationResources& resources) {
    return resources.action_icons;
  }

  static bool Read(blink::mojom::NotificationResourcesDataView in,
                   blink::NotificationResources* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NOTIFICATIONS_NOTIFICATION_MOJOM_TRAITS_H_
