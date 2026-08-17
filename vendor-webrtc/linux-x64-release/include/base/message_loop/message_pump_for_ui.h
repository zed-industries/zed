// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_MESSAGE_PUMP_FOR_UI_H_
#define BASE_MESSAGE_LOOP_MESSAGE_PUMP_FOR_UI_H_

// This header is a forwarding header to coalesce the various platform specific
// implementations of MessagePumpForUI.

#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/message_loop/message_pump_win.h"
#elif BUILDFLAG(IS_ANDROID)
#include "base/message_loop/message_pump_android.h"
#elif BUILDFLAG(IS_APPLE)
#include "base/message_loop/message_pump.h"
#elif BUILDFLAG(IS_NACL) || BUILDFLAG(IS_AIX)
// No MessagePumpForUI, see below.
#elif defined(USE_GLIB)
#include "base/message_loop/message_pump_glib.h"
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_BSD)
#include "base/message_loop/message_pump_epoll.h"
#elif BUILDFLAG(IS_FUCHSIA)
#include "base/message_loop/message_pump_fuchsia.h"
#endif

namespace base {

#if BUILDFLAG(IS_WIN)
// Windows defines it as-is.
using MessagePumpForUI = MessagePumpForUI;
#elif BUILDFLAG(IS_ANDROID)
using MessagePumpForUI = MessagePumpAndroid;
#elif BUILDFLAG(IS_APPLE)
// MessagePumpForUI isn't bound to a specific impl on Mac. While each impl can
// be represented by a plain MessagePump: message_pump_apple::Create() must be
// used to instantiate the right impl.
using MessagePumpForUI = MessagePump;
#elif BUILDFLAG(IS_NACL) || BUILDFLAG(IS_AIX)
// Currently NaCl and AIX don't have a MessagePumpForUI.
// TODO(abarth): Figure out if we need this.
#elif defined(USE_GLIB)
using MessagePumpForUI = MessagePumpGlib;
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_BSD)
using MessagePumpForUI = MessagePumpEpoll;
#elif BUILDFLAG(IS_FUCHSIA)
using MessagePumpForUI = MessagePumpFuchsia;
#else
#error Platform does not define MessagePumpForUI
#endif

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_MESSAGE_PUMP_FOR_UI_H_
