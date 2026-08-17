/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_TESTING_SUPPORT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_TESTING_SUPPORT_H_

#include "v8/include/v8.h"

namespace blink {
class ScopedMockOverlayScrollbars;
class WebLocalFrame;

class WebTestingSupport {
 public:
  // To be called once at startup for the process. Stores state that will be
  // restored after each test when ResetRuntimeFeatures() is called.
  static void SaveRuntimeFeatures();
  static void ResetRuntimeFeatures();

  // Injects the |internals| object into the frame for Javascript to use.
  static void InjectInternalsObject(WebLocalFrame*);
  static void InjectInternalsObject(v8::Local<v8::Context>);
  // Resets state on the main frame once a test is complete, including:
  // - disposing any isolated worlds created by the test.
  // - resetting the state of the |internals| object, and things that it
  // modifies,
  //   back to their starting state
  static void ResetMainFrame(WebLocalFrame*);

  // Use this to install a mock scrollbar theme for tests. To use, simply
  // inherit your test class from this or instantiate it manually. The
  // constructor and destructor will enable and disable the mock theme. For
  // tests within Blink, use ScopedMockOverlayScrollbars instead.
  class WebScopedMockScrollbars {
   public:
    WebScopedMockScrollbars();
    ~WebScopedMockScrollbars();

   private:
    std::unique_ptr<ScopedMockOverlayScrollbars> use_mock_scrollbars_;
  };
};
}

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_TESTING_SUPPORT_H_
