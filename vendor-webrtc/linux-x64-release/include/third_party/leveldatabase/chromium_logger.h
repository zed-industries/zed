// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LEVELDATABASE_CHROMIUM_LOGGER_H_
#define THIRD_PARTY_LEVELDATABASE_CHROMIUM_LOGGER_H_

#include <string>
#include <utility>

#include "base/containers/span.h"
#include "base/files/file.h"
#include "base/format_macros.h"
#include "base/i18n/time_formatting.h"
#include "base/strings/strcat.h"
#include "base/strings/stringprintf.h"
#include "base/threading/platform_thread.h"
#include "base/time/time.h"
#include "third_party/leveldatabase/src/include/leveldb/env.h"

namespace leveldb {

class ChromiumLogger : public Logger {
 public:
  explicit ChromiumLogger(base::File file) : file_(std::move(file)) {}

  ~ChromiumLogger() override = default;

  void Logv(const char* format, va_list arguments) override {
    std::string str = base::StrCat(
        {base::UnlocalizedTimeFormatWithPattern(base::Time::Now(),
                                                "yyyy/MM/dd-HH:mm:ss.SSS"),
         base::StringPrintf(
             " %" PRIx64 " ",
             static_cast<uint64_t>(base::PlatformThread::CurrentId())),
         base::StringPrintV(format, arguments)});
    if (str.back() != '\n') {
      str.push_back('\n');
    }

    file_.WriteAtCurrentPosAndCheck(base::as_byte_span(str));
  }

 private:
  base::File file_;
};

}  // namespace leveldb

#endif  // THIRD_PARTY_LEVELDATABASE_CHROMIUM_LOGGER_H_
