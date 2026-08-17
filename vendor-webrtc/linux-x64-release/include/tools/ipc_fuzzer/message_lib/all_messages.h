// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// Inclusion of all message files recognized by message_lib. All messages
// received by RenderProcessHost should be included here for the IPC fuzzer.

// Force all multi-include optional files to be included again.
#undef CHROME_COMMON_COMMON_PARAM_TRAITS_MACROS_H_
#undef COMPONENTS_NACL_COMMON_NACL_TYPES_PARAM_TRAITS_H_
#undef COMPONENTS_TRACING_COMMON_TRACING_MESSAGES_H_
#undef CONTENT_COMMON_CONTENT_PARAM_TRAITS_MACROS_H_
#undef CONTENT_COMMON_FRAME_PARAM_MACROS_H_
#undef CONTENT_PUBLIC_COMMON_COMMON_PARAM_TRAITS_MACROS_H_

#include "components/nacl/common/buildflags.h"

#include "chrome/common/all_messages.h"
#if BUILDFLAG(ENABLE_NACL)
#include "components/nacl/common/nacl_host_messages.h"
#endif
#include "content/common/all_messages.h"
