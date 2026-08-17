// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include "file_test.h"

#include <assert.h>
#include <string.h>

#include <memory>
#include <string>
#include <utility>

#include <gtest/gtest.h>

#include <openssl/err.h>


std::string GetTestData(const char *path);

class StringLineReader : public FileTest::LineReader {
 public:
  explicit StringLineReader(const std::string &data)
      : data_(data), offset_(0) {}

  FileTest::ReadResult ReadLine(char *out, size_t len) override {
    assert(len > 0);
    if (offset_ == data_.size()) {
      return FileTest::kReadEOF;
    }

    size_t idx = data_.find('\n', offset_);
    if (idx == std::string::npos) {
      idx = data_.size();
    } else {
      idx++;  // Include the newline.
    }

    if (idx - offset_ > len - 1) {
      ADD_FAILURE() << "Line too long.";
      return FileTest::kReadError;
    }

    memcpy(out, data_.data() + offset_, idx - offset_);
    out[idx - offset_] = '\0';
    offset_ = idx;
    return FileTest::kReadSuccess;
  }

 private:
  std::string data_;
  size_t offset_;

  StringLineReader(const StringLineReader &) = delete;
  StringLineReader &operator=(const StringLineReader &) = delete;
};

void FileTestGTest(const char *path, std::function<void(FileTest *)> run_test) {
  std::unique_ptr<StringLineReader> reader(
      new StringLineReader(GetTestData(path)));
  FileTest t(std::move(reader), nullptr, false);

  while (true) {
    switch (t.ReadNext()) {
      case FileTest::kReadError:
        ADD_FAILURE() << "Error reading test.";
        return;
      case FileTest::kReadEOF:
        return;
      case FileTest::kReadSuccess:
        break;
    }

    const testing::TestResult *test_result =
        testing::UnitTest::GetInstance()->current_test_info()->result();
    int before_part_count = test_result->total_part_count();

    SCOPED_TRACE(testing::Message() << path << ", line " << t.start_line());
    run_test(&t);

    // Check for failures from the most recent test.
    bool failed = false;
    for (int i = before_part_count; i < test_result->total_part_count(); i++) {
      if (test_result->GetTestPartResult(i).failed()) {
        failed = true;
        break;
      }
    }

    // Clean up the error queue for the next test, reporting it on failure.
    if (failed) {
      ERR_print_errors_fp(stdout);
    } else {
      ERR_clear_error();
    }
  }
}
