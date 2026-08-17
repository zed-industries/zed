// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_GTEST_XML_UNITTEST_RESULT_PRINTER_H_
#define BASE_TEST_GTEST_XML_UNITTEST_RESULT_PRINTER_H_

#include <stdio.h>

#include <optional>
#include <string_view>

#include "base/memory/raw_ptr.h"
#include "base/threading/thread_checker.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base {

class FilePath;

// Generates an XML output file. Format is very close to GTest, but has
// extensions needed by the test launcher.
class XmlUnitTestResultPrinter : public testing::EmptyTestEventListener {
 public:
  XmlUnitTestResultPrinter();

  XmlUnitTestResultPrinter(const XmlUnitTestResultPrinter&) = delete;
  XmlUnitTestResultPrinter& operator=(const XmlUnitTestResultPrinter&) = delete;

  ~XmlUnitTestResultPrinter() override;

  static XmlUnitTestResultPrinter* Get();

  // Add link in the gtest xml output.
  // Please see AddLinkToTestResult in gtest_links.h for detailed
  // explanation and usage.
  void AddLink(const std::string& name, const std::string& url);

  // Add tag in the gtest xml output.
  // Please see AddTagToTestResult in gtest_tags.h for detailed
  // explanation and usage.
  void AddTag(const std::string& name, const std::string& value);

  // Add SubTestResult in the GTest XML output.
  // See gtest_sub_test_results.h for more information.
  void AddSubTestResult(std::string_view name,
                        testing::TimeInMillis elapsed_time,
                        std::optional<std::string_view> failure_message);

  // Must be called before adding as a listener. Returns true on success.
  [[nodiscard]] bool Initialize(const FilePath& output_file_path);

  // CHECK/DCHECK failed. Print file/line and message to the xml.
  void OnAssert(const char* file,
                int line,
                const std::string& summary,
                const std::string& message);

 private:
  // testing::EmptyTestEventListener:
  void OnTestSuiteStart(const testing::TestSuite& test_suite) override;
  void OnTestStart(const testing::TestInfo& test_info) override;
  void OnTestEnd(const testing::TestInfo& test_info) override;
  void OnTestSuiteEnd(const testing::TestSuite& test_suite) override;

  void WriteTestPartResult(const char* file,
                           int line,
                           testing::TestPartResult::Type type,
                           const std::string& summary,
                           const std::string& message);

  static XmlUnitTestResultPrinter* instance_;
  raw_ptr<FILE> output_file_ = nullptr;
  bool open_failed_ = false;

  // Flag that's true iff a test has been started but not yet ended.
  bool test_running_ = false;

  ThreadChecker thread_checker_;
};

}  // namespace base

#endif  // BASE_TEST_GTEST_XML_UNITTEST_RESULT_PRINTER_H_
