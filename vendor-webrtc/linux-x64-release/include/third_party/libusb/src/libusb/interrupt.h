// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBUSB_SRC_LIBUSB_INTERRUPT_H
#define THIRD_PARTY_LIBUSB_SRC_LIBUSB_INTERRUPT_H

#include "libusb.h"

#ifdef __cplusplus
extern "C" {
#endif

int LIBUSB_CALL libusb_interrupt_handle_event(struct libusb_context* ctx);

#ifdef __cplusplus
}
#endif

#endif  // THIRD_PARTY_LIBUSB_SRC_LIBUSB_INTERRUPT_H
