// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_MESSAGE_PUMP_FOR_IO_H_
#define BASE_MESSAGE_LOOP_MESSAGE_PUMP_FOR_IO_H_

// This header is a forwarding header to coalesce the various platform specific
// types representing MessagePumpForIO.

#include "base/message_loop/ios_cronet_buildflags.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/message_loop/message_pump_win.h"
#elif BUILDFLAG(IS_IOS) && (BUILDFLAG(IS_IOS_TVOS) || BUILDFLAG(CRONET_BUILD))
#include "base/message_loop/message_pump_io_ios.h"
#elif BUILDFLAG(IS_APPLE)
#include "base/message_loop/message_pump_kqueue.h"
#elif BUILDFLAG(IS_NACL)
#include "base/message_loop/message_pump_default.h"
#elif BUILDFLAG(IS_FUCHSIA)
#include "base/message_loop/message_pump_fuchsia.h"
#elif BUILDFLAG(IS_POSIX)
#include "base/message_loop/message_pump_epoll.h"
#endif

namespace base {

#if BUILDFLAG(IS_WIN)
// Windows defines it as-is.
using MessagePumpForIO = MessagePumpForIO;
#elif BUILDFLAG(IS_IOS) && (BUILDFLAG(IS_IOS_TVOS) || BUILDFLAG(CRONET_BUILD))
using MessagePumpForIO = MessagePumpIOSForIO;
#elif BUILDFLAG(IS_APPLE)
using MessagePumpForIO = MessagePumpKqueue;
#elif BUILDFLAG(IS_NACL)
using MessagePumpForIO = MessagePumpDefault;
#elif BUILDFLAG(IS_FUCHSIA)
using MessagePumpForIO = MessagePumpFuchsia;
#elif BUILDFLAG(IS_POSIX)
using MessagePumpForIO = MessagePumpEpoll;
#else
#error Platform does not define MessagePumpForIO
#endif

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_MESSAGE_PUMP_FOR_IO_H_
