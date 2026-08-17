/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_PORTAL_SCOPED_GLIB_H_
#define MODULES_PORTAL_SCOPED_GLIB_H_

#include <gio/gio.h>

#include "rtc_base/checks.h"
#include "rtc_base/system/rtc_export_template.h"

namespace webrtc {

template <class T>
class Scoped {
 public:
  Scoped() {}
  explicit Scoped(T* val) { ptr_ = val; }
  ~Scoped() { RTC_DCHECK_NOTREACHED(); }

  T* operator->() const { return ptr_; }

  explicit operator bool() const { return ptr_ != nullptr; }

  bool operator!() const { return ptr_ == nullptr; }

  T* get() const { return ptr_; }

  T** receive() {
    RTC_CHECK(!ptr_);
    return &ptr_;
  }

  Scoped& operator=(T* val) {
    RTC_DCHECK(val);
    ptr_ = val;
    return *this;
  }

 protected:
  T* ptr_ = nullptr;
};

template <>
Scoped<GError>::~Scoped();
template <>
Scoped<char>::~Scoped();
template <>
Scoped<GVariant>::~Scoped();
template <>
Scoped<GVariantIter>::~Scoped();
template <>
Scoped<GDBusMessage>::~Scoped();
template <>
Scoped<GUnixFDList>::~Scoped();

extern template class RTC_EXPORT_TEMPLATE_DECLARE(RTC_EXPORT) Scoped<GError>;
extern template class RTC_EXPORT_TEMPLATE_DECLARE(RTC_EXPORT) Scoped<char>;
extern template class RTC_EXPORT_TEMPLATE_DECLARE(RTC_EXPORT) Scoped<GVariant>;
extern template class RTC_EXPORT_TEMPLATE_DECLARE(
    RTC_EXPORT) Scoped<GVariantIter>;
extern template class RTC_EXPORT_TEMPLATE_DECLARE(
    RTC_EXPORT) Scoped<GDBusMessage>;
extern template class RTC_EXPORT_TEMPLATE_DECLARE(
    RTC_EXPORT) Scoped<GUnixFDList>;

}  // namespace webrtc

#endif  // MODULES_PORTAL_SCOPED_GLIB_H_
