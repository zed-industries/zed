// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_GC_PLUGIN_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_GC_PLUGIN_H_

// GC_PLUGIN_IGNORE is used to make the Blink GC plugin ignore a particular
// class or field when checking for proper usage.  When using GC_PLUGIN_IGNORE a
// reason must be provided as an argument. In most cases this will be a bug id
// where the bug describes what needs to happen to remove the GC_PLUGIN_IGNORE
// again.
//
// Developer note: this macro must be kept in sync with the definition of
// STACK_ALLOCATED_IGNORE in /base/memory/stack_allocated.h.
#if defined(__clang__)
#define GC_PLUGIN_IGNORE(reason)                     \
  __attribute__((annotate("blink_gc_plugin_ignore"), \
                 annotate("stack_allocated_ignore")))
#else  // !defined(__clang__)
#define GC_PLUGIN_IGNORE(reason)
#endif  // !defined(__clang__)

// GC_SAFE_FIELD is used to make the Blink GC plugin ignore a particular field.
// This must only be used when we can ensure that the pattern it is marking is
// safe despite the exception thrown by the plugin.
#define GC_SAFE_FIELD(reason) GC_PLUGIN_IGNORE(reason)

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_GC_PLUGIN_H_
