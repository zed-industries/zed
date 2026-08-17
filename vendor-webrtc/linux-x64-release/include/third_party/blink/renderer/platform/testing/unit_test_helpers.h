/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_UNIT_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_UNIT_TEST_HELPERS_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/testing/geometry_test_helpers.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace base {
class FilePath;
}

namespace blink {

namespace test {

// Note: You may want to use TestingPlatformSupportWithMockScheduler to
// provides runUntilIdle() method that can work with URLLoaderMockFactory.
void RunPendingTasks();

// Waits for delayed task to complete or timers to fire for |delay|.
void RunDelayedTasks(base::TimeDelta delay);

void YieldCurrentThread();

// Returns Blink top directory as an absolute path, e.g.
// /src/third_party/blink.
String BlinkRootDir();

// Returns Blink web_tests directory as an absolute path, e.g.
// /src/third_party/blink/web_tests.
String BlinkWebTestsDir();

// Returns directory containing the current executable as absolute path.
String ExecutableDir();

// Returns test data absolute path for blink_unittests in core, i.e.
// <blinkRootDir>/renderer/core/testing/data/<relative_path>.
// It returns the top web test directory if |relative_path| was not specified.
String CoreTestDataPath(const String& relative_path = String());

// Returns test data absolute path for blink_platform_unittests, i.e.
// <blinkRootDir>/renderer/platform/testing/data/<relative_path>.
// It returns the top platform test directory if |relative_path| was not
// specified.
String PlatformTestDataPath(const String& relative_path = String());

// Returns test data absolute path for accessibility unittests, i.e.
// <blinkRootDir>/renderer/modules/accessibility/testing/data/<relative_path>.
// It returns the top accessibility test directory if |relative_path| was not
// specified.
String AccessibilityTestDataPath(const String& relative_path = String());

// Returns Blink web_tests fonts as an absolute path, i.e.
// <blinkRootDir>/web_tests/external/wpt/fonts/<relative_path>.
// It returns the top fonts test directory if |relative_path| was not
// specified.
String BlinkWebTestsFontsTestDataPath(const String& relative_path = String());

// Returns Blink web_tests images as an absolute path, i.e.
// <blinkRootDir>/web_tests/images/resources/<relative_path>.
// It returns the top Blink web_tests image resources directory if
// |relative_path| was not specified.
String BlinkWebTestsImagesTestDataPath(const String& relative_path = String());

// Returns Blink style perftest data as an absolute path, i.e.
// <blinkRootDir>/renderer/core/css/perftest_data/<relative_path>.
// It returns the top perftest data directory if |relative_path| was not
// specified.
String StylePerfTestDataPath(const String& relative_path = String());

// Returns the directory of hyphenation dictionaries for testing.
base::FilePath HyphenationDictionaryDir();

// Reads the file at the given path and returns its data.
// Returns nullopt if the file does not exist or couldn't be read.
std::optional<Vector<char>> ReadFromFile(const String& path);

class LineReader {
  DISALLOW_NEW();

 public:
  LineReader(const String& text);
  bool GetNextLine(String* line);

 private:
  String text_;
  wtf_size_t index_;
};

}  // namespace test
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_UNIT_TEST_HELPERS_H_
