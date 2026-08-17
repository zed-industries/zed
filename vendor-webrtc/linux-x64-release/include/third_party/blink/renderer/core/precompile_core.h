// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef THIRD_PARTY_BLINK_RENDERER_CORE_PRECOMPILE_CORE_H_
#error You shouldn't include the precompiled header file more than once.
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PRECOMPILE_CORE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PRECOMPILE_CORE_H_

#if defined(_MSC_VER)
#include "third_party/blink/renderer/build/win/precompile.h"
#elif defined(__APPLE__)
#include "third_party/blink/renderer/build/mac/prefix.h"

// In Blink a lot of operations center around dom and Document, or around
// layout/rendering and LayoutObject. Those two headers are in turn pulling
// in large parts of Blink's other headers which means that every compilation
// unit is compiling large parts of Blink. By precompiling document.h
// and layout_object.h we only have to compile those parts once rather
// than 1500 times. It can make a large difference in compilation
// times (3-4 times faster).

// Precompiling these headers has not been found to be helpful on Windows
// compiles in 2020.
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#else
#include "third_party/blink/renderer/build/linux/prefix.h"
#endif

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PRECOMPILE_CORE_H_
