// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include "file_test.h"

#include <algorithm>
#include <utility>

#include <assert.h>
#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>

#include "../internal.h"
#include "./test_util.h"


FileTest::FileTest(std::unique_ptr<FileTest::LineReader> reader,
                   std::function<void(const std::string &)> comment_callback,
                   bool is_kas_test)
    : reader_(std::move(reader)),
      is_kas_test_(is_kas_test),
      comment_callback_(std::move(comment_callback)) {}

FileTest::~FileTest() {}

// FindDelimiter returns a pointer to the first '=' or ':' in |str| or nullptr
// if there is none.
static const char *FindDelimiter(const char *str) {
  while (*str) {
    if (*str == ':' || *str == '=') {
      return str;
    }
    str++;
  }
  return nullptr;
}

// StripSpace returns a string containing up to |len| characters from |str| with
// leading and trailing whitespace removed.
static std::string StripSpace(const char *str, size_t len) {
  // Remove leading space.
  while (len > 0 && OPENSSL_isspace(*str)) {
    str++;
    len--;
  }
  while (len > 0 && OPENSSL_isspace(str[len - 1])) {
    len--;
  }
  return std::string(str, len);
}

static std::pair<std::string, std::string> ParseKeyValue(const char *str, const size_t len) {
  const char *delimiter = FindDelimiter(str);
  std::string key, value;
  if (delimiter == nullptr) {
    key = StripSpace(str, len);
  } else {
    key = StripSpace(str, delimiter - str);
    value = StripSpace(delimiter + 1, str + len - delimiter - 1);
  }
  return {key, value};
}

FileTest::ReadResult FileTest::ReadNext() {
  // If the previous test had unused attributes or instructions, it is an error.
  if (!unused_attributes_.empty()) {
    for (const std::string &key : unused_attributes_) {
      PrintLine("Unused attribute: %s", key.c_str());
    }
    return kReadError;
  }
  if (!unused_instructions_.empty()) {
    for (const std::string &key : unused_instructions_) {
      PrintLine("Unused instruction: %s", key.c_str());
    }
    return kReadError;
  }

  ClearTest();

  static const size_t kBufLen = 8192 * 4;
  std::unique_ptr<char[]> buf(new char[kBufLen]);

  bool in_instruction_block = false;
  is_at_new_instruction_block_ = false;

  while (true) {
    // Read the next line.
    switch (reader_->ReadLine(buf.get(), kBufLen)) {
      case kReadError:
        fprintf(stderr, "Error reading from input at line %u.\n", line_ + 1);
        return kReadError;
      case kReadEOF:
        // EOF is a valid terminator for a test.
        return start_line_ > 0 ? kReadSuccess : kReadEOF;
      case kReadSuccess:
        break;
    }

    line_++;
    size_t len = strlen(buf.get());
    if (buf[0] == '\n' || buf[0] == '\r' || buf[0] == '\0') {
      // Empty lines delimit tests.
      if (start_line_ > 0) {
        return kReadSuccess;
      }
      if (in_instruction_block) {
        in_instruction_block = false;
        // Delimit instruction block from test with a blank line.
        current_test_ += "\r\n";
      } else if (is_kas_test_) {
        // KAS tests have random blank lines scattered around.
        current_test_ += "\r\n";
      }
    } else if (buf[0] == '#') {
      if (is_kas_test_ && seen_non_comment_) {
        // KAS tests have comments after the initial comment block which need
        // to be included in the corresponding place in the output.
        current_test_ += std::string(buf.get());
      } else if (comment_callback_) {
        comment_callback_(buf.get());
      }
      // Otherwise ignore comments.
    } else if (strcmp("[B.4.2 Key Pair Generation by Testing Candidates]\r\n",
                      buf.get()) == 0) {
      // The above instruction-like line is ignored because the FIPS lab's
      // request files are hopelessly inconsistent.
    } else if (buf[0] == '[') {  // Inside an instruction block.
      is_at_new_instruction_block_ = true;
      seen_non_comment_ = true;
      if (start_line_ != 0) {
        // Instructions should be separate blocks.
        fprintf(stderr, "Line %u is an instruction in a test case.\n", line_);
        return kReadError;
      }
      if (!in_instruction_block) {
        ClearInstructions();
        in_instruction_block = true;
      }

      // Parse the line as an instruction ("[key = value]" or "[key]").

      // KAS tests contain invalid syntax.
      std::string kv = buf.get();
      const bool is_broken_kas_instruction =
          is_kas_test_ &&
          (kv == "[SHA(s) supported (Used for hashing Z): SHA512 \r\n");

      if (!is_broken_kas_instruction) {
        kv = StripSpace(buf.get(), len);
        if (kv[kv.size() - 1] != ']') {
          fprintf(stderr, "Line %u, invalid instruction: '%s'\n", line_,
                  kv.c_str());
          return kReadError;
        }
      } else {
        // Just remove the newline for the broken instruction.
        kv = kv.substr(0, kv.size() - 2);
      }

      current_test_ += kv + "\r\n";
      kv = std::string(kv.begin() + 1, kv.end() - 1);

      for (;;) {
        size_t idx = kv.find(',');
        if (idx == std::string::npos) {
          idx = kv.size();
        }
        std::string key, value;
        std::tie(key, value) = ParseKeyValue(kv.c_str(), idx);
        instructions_[key] = value;
        if (idx == kv.size())
          break;
        kv = kv.substr(idx + 1);
      }
    } else {
      // Parsing a test case.
      if (in_instruction_block) {
        // Some NIST CAVP test files (TDES) have a test case immediately
        // following an instruction block, without a separate blank line, some
        // of the time.
        in_instruction_block = false;
      }

      current_test_ += std::string(buf.get(), len);
      std::string key, value;
      std::tie(key, value) = ParseKeyValue(buf.get(), len);

      // Duplicate keys are rewritten to have “/2”, “/3”, … suffixes.
      std::string mapped_key = key;
      // If absent, the value will be zero-initialized.
      const size_t num_occurrences = ++attribute_count_[key];
      if (num_occurrences > 1) {
        mapped_key += "/" + std::to_string(num_occurrences);
      }

      unused_attributes_.insert(mapped_key);
      attributes_[mapped_key] = value;
      if (start_line_ == 0) {
        // This is the start of a test.
        type_ = mapped_key;
        parameter_ = value;
        start_line_ = line_;
        for (const auto &kv : instructions_) {
          unused_instructions_.insert(kv.first);
        }
      }
    }
  }
}

void FileTest::PrintLine(const char *format, ...) {
  va_list args;
  va_start(args, format);

  fprintf(stderr, "Line %u: ", start_line_);
  vfprintf(stderr, format, args);
  fprintf(stderr, "\n");

  va_end(args);
}

const std::string &FileTest::GetType() {
  OnKeyUsed(type_);
  return type_;
}

const std::string &FileTest::GetParameter() {
  OnKeyUsed(type_);
  return parameter_;
}

bool FileTest::HasAttribute(const std::string &key) {
  OnKeyUsed(key);
  return attributes_.count(key) > 0;
}

bool FileTest::GetAttribute(std::string *out_value, const std::string &key) {
  OnKeyUsed(key);
  auto iter = attributes_.find(key);
  if (iter == attributes_.end()) {
    PrintLine("Missing attribute '%s'.", key.c_str());
    return false;
  }
  *out_value = iter->second;
  return true;
}

const std::string &FileTest::GetAttributeOrDie(const std::string &key) {
  if (!HasAttribute(key)) {
    abort();
  }
  return attributes_[key];
}

bool FileTest::HasInstruction(const std::string &key) {
  OnInstructionUsed(key);
  return instructions_.count(key) > 0;
}

bool FileTest::GetInstruction(std::string *out_value, const std::string &key) {
  OnInstructionUsed(key);
  auto iter = instructions_.find(key);
  if (iter == instructions_.end()) {
    PrintLine("Missing instruction '%s'.", key.c_str());
    return false;
  }
  *out_value = iter->second;
  return true;
}

void FileTest::IgnoreAllUnusedInstructions() {
  unused_instructions_.clear();
}

const std::string &FileTest::GetInstructionOrDie(const std::string &key) {
  if (!HasInstruction(key)) {
    abort();
  }
  return instructions_[key];
}

bool FileTest::GetInstructionBytes(std::vector<uint8_t> *out,
                                   const std::string &key) {
  std::string value;
  return GetInstruction(&value, key) && ConvertToBytes(out, value);
}

const std::string &FileTest::CurrentTestToString() const {
  return current_test_;
}

bool FileTest::GetBytes(std::vector<uint8_t> *out, const std::string &key) {
  std::string value;
  return GetAttribute(&value, key) && ConvertToBytes(out, value);
}

void FileTest::ClearTest() {
  start_line_ = 0;
  type_.clear();
  parameter_.clear();
  attribute_count_.clear();
  attributes_.clear();
  unused_attributes_.clear();
  unused_instructions_.clear();
  current_test_ = "";
}

void FileTest::ClearInstructions() {
  instructions_.clear();
  unused_attributes_.clear();
}

void FileTest::OnKeyUsed(const std::string &key) {
  unused_attributes_.erase(key);
}

void FileTest::OnInstructionUsed(const std::string &key) {
  unused_instructions_.erase(key);
}

bool FileTest::ConvertToBytes(std::vector<uint8_t> *out,
                              const std::string &value) {
  if (value.size() >= 2 && value[0] == '"' && value[value.size() - 1] == '"') {
    out->assign(value.begin() + 1, value.end() - 1);
    return true;
  }

  if (!DecodeHex(out, value)) {
    PrintLine("Error decoding value: %s", value.c_str());
    return false;
  }
  return true;
}

bool FileTest::IsAtNewInstructionBlock() const {
  return is_at_new_instruction_block_;
}

void FileTest::InjectInstruction(const std::string &key,
                                 const std::string &value) {
  instructions_[key] = value;
}

class FileLineReader : public FileTest::LineReader {
 public:
  explicit FileLineReader(const char *path) : file_(fopen(path, "r")) {}
  ~FileLineReader() override {
    if (file_ != nullptr) {
      fclose(file_);
    }
  }

  // is_open returns true if the file was successfully opened.
  bool is_open() const { return file_ != nullptr; }

  FileTest::ReadResult ReadLine(char *out, size_t len) override {
    assert(len > 0);
    if (file_ == nullptr) {
      return FileTest::kReadError;
    }

    len = std::min(len, size_t{INT_MAX});
    if (fgets(out, static_cast<int>(len), file_) == nullptr) {
      return feof(file_) ? FileTest::kReadEOF : FileTest::kReadError;
    }

    if (strlen(out) == len - 1 && out[len - 2] != '\n' && !feof(file_)) {
      fprintf(stderr, "Line too long.\n");
      return FileTest::kReadError;
    }

    return FileTest::kReadSuccess;
  }

 private:
  FILE *file_;

  FileLineReader(const FileLineReader &) = delete;
  FileLineReader &operator=(const FileLineReader &) = delete;
};

int FileTestMain(FileTestFunc run_test, void *arg, const char *path) {
  FileTest::Options opts;
  opts.callback = run_test;
  opts.arg = arg;
  opts.path = path;

  return FileTestMain(opts);
}

int FileTestMain(const FileTest::Options &opts) {
  std::unique_ptr<FileLineReader> reader(
      new FileLineReader(opts.path));
  if (!reader->is_open()) {
    fprintf(stderr, "Could not open file %s: %s.\n", opts.path,
            strerror(errno));
    return 1;
  }

  FileTest t(std::move(reader), opts.comment_callback, opts.is_kas_test);

  bool failed = false;
  while (true) {
    FileTest::ReadResult ret = t.ReadNext();
    if (ret == FileTest::kReadError) {
      return 1;
    } else if (ret == FileTest::kReadEOF) {
      break;
    }

    bool result = opts.callback(&t, opts.arg);
    if (t.HasAttribute("Error")) {
      if (result) {
        t.PrintLine("Operation unexpectedly succeeded.");
        failed = true;
        continue;
      }
      uint32_t err = ERR_peek_error();
      if (ERR_reason_error_string(err) != t.GetAttributeOrDie("Error")) {
        t.PrintLine("Unexpected error; wanted '%s', got '%s'.",
                    t.GetAttributeOrDie("Error").c_str(),
                    ERR_reason_error_string(err));
        failed = true;
        ERR_clear_error();
        continue;
      }
      ERR_clear_error();
    } else if (!result) {
      // In case the test itself doesn't print output, print something so the
      // line number is reported.
      t.PrintLine("Test failed");
      ERR_print_errors_fp(stderr);
      failed = true;
      continue;
    }
  }

  if (!opts.silent && !failed) {
    printf("PASS\n");
  }

  return failed ? 1 : 0;
}

void FileTest::SkipCurrent() {
  ClearTest();
}
