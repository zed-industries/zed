// Copyright 2015 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_CRYPTO_TEST_FILE_TEST_H
#define OPENSSL_HEADER_CRYPTO_TEST_FILE_TEST_H

#include <openssl/base.h>

#include <stdint.h>

#include <functional>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>

// File-based test framework.
//
// This module provides a file-based test framework. The file format is based on
// that of OpenSSL upstream's evp_test and BoringSSL's aead_test. NIST CAVP test
// vector files are also supported. Each input file is a sequence of attributes,
// instructions and blank lines.
//
// Each attribute has the form:
//
//   Name = Value
//
// Instructions are enclosed in square brackets and may appear without a value:
//
//   [Name = Value]
//
// or
//
//   [Name]
//
// Commas in instruction lines are treated as separate instructions. Thus this:
//
//   [Name1,Name2]
//
// is the same as:
//
//   [Name1]
//   [Name2]
//
// Either '=' or ':' may be used to delimit the name from the value. Both the
// name and value have leading and trailing spaces stripped.
//
// Each file contains a number of instruction blocks and test cases.
//
// An instruction block is a sequence of instructions followed by a blank line.
// Instructions apply to all test cases following its appearance, until the next
// instruction block. Instructions are unordered.
//
// A test is a sequence of one or more attributes followed by a blank line.  For
// tests that process multiple kinds of test cases, the first attribute is
// parsed out as the test's type and parameter. Otherwise, attributes are
// unordered. The first attribute is also included in the set of attributes, so
// tests which do not dispatch may ignore this mechanism.
//
// Additional blank lines and lines beginning with # are ignored.
//
// Functions in this module freely output to |stderr| on failure. Tests should
// also do so, and it is recommended they include the corresponding test's line
// number in any output. |PrintLine| does this automatically.
//
// Each attribute in a test and all instructions applying to it must be
// consumed. When a test completes, if any attributes or insturctions haven't
// been processed, the framework reports an error.

class FileTest;
typedef bool (*FileTestFunc)(FileTest *t, void *arg);

class FileTest {
 public:
  enum ReadResult {
    kReadSuccess,
    kReadEOF,
    kReadError,
  };

  class LineReader {
   public:
    virtual ~LineReader() {}
    virtual ReadResult ReadLine(char *out, size_t len) = 0;
  };

  struct Options {
    // path is the path to the input file.
    const char *path = nullptr;
    // callback is called for each test. It should get the parameters from this
    // object and signal any errors by returning false.
    FileTestFunc callback = nullptr;
    // arg is an opaque pointer that is passed to |callback|.
    void *arg = nullptr;
    // silent suppressed the "PASS" string that is otherwise printed after
    // successful runs.
    bool silent = false;
    // comment_callback is called after each comment in the input is parsed.
    std::function<void(const std::string&)> comment_callback;
    // is_kas_test is true if a NIST “KAS” test is being parsed. These tests
    // are inconsistent with the other NIST files to such a degree that they
    // need their own boolean.
    bool is_kas_test = false;
  };

  explicit FileTest(std::unique_ptr<LineReader> reader,
                    std::function<void(const std::string &)> comment_callback,
                    bool is_kas_test);
  ~FileTest();

  // ReadNext reads the next test from the file. It returns |kReadSuccess| if
  // successfully reading a test and |kReadEOF| at the end of the file. On
  // error or if the previous test had unconsumed attributes, it returns
  // |kReadError|.
  ReadResult ReadNext();

  // PrintLine is a variant of printf which prepends the line number and appends
  // a trailing newline.
  void PrintLine(const char *format, ...) OPENSSL_PRINTF_FORMAT_FUNC(2, 3);

  unsigned start_line() const { return start_line_; }

  // GetType returns the name of the first attribute of the current test.
  const std::string &GetType();
  // GetParameter returns the value of the first attribute of the current test.
  const std::string &GetParameter();

  // HasAttribute returns true if the current test has an attribute named |key|.
  bool HasAttribute(const std::string &key);

  // GetAttribute looks up the attribute with key |key|. It sets |*out_value| to
  // the value and returns true if it exists and returns false with an error to
  // |stderr| otherwise.
  bool GetAttribute(std::string *out_value, const std::string &key);

  // GetAttributeOrDie looks up the attribute with key |key| and aborts if it is
  // missing. It should only be used after a |HasAttribute| call.
  const std::string &GetAttributeOrDie(const std::string &key);

  // IgnoreAttribute marks the attribute with key |key| as used.
  void IgnoreAttribute(const std::string &key) { HasAttribute(key); }

  // GetBytes looks up the attribute with key |key| and decodes it as a byte
  // string. On success, it writes the result to |*out| and returns
  // true. Otherwise it returns false with an error to |stderr|. The value may
  // be either a hexadecimal string or a quoted ASCII string. It returns true on
  // success and returns false with an error to |stderr| on failure.
  bool GetBytes(std::vector<uint8_t> *out, const std::string &key);

  // AtNewInstructionBlock returns true if the current test was immediately
  // preceded by an instruction block.
  bool IsAtNewInstructionBlock() const;

  // HasInstruction returns true if the current test has an instruction.
  bool HasInstruction(const std::string &key);

  // IgnoreInstruction marks the instruction with key |key| as used.
  void IgnoreInstruction(const std::string &key) { HasInstruction(key); }

  // IgnoreAllUnusedInstructions disables checking for unused instructions.
  void IgnoreAllUnusedInstructions();

  // GetInstruction looks up the instruction with key |key|. It sets
  // |*out_value| to the value (empty string if the instruction has no value)
  // and returns true if it exists and returns false with an error to |stderr|
  // otherwise.
  bool GetInstruction(std::string *out_value, const std::string &key);

  // GetInstructionOrDie looks up the instruction with key |key| and aborts if
  // it is missing. It should only be used after a |HasInstruction| call.
  const std::string &GetInstructionOrDie(const std::string &key);

  // GetInstructionBytes behaves like GetBytes, but looks up the corresponding
  // instruction.
  bool GetInstructionBytes(std::vector<uint8_t> *out, const std::string &key);

  // CurrentTestToString returns the file content parsed for the current test.
  // If the current test was preceded by an instruction block, the return test
  // case is preceded by the instruction block and a single blank line. All
  // other blank or comment lines are omitted.
  const std::string &CurrentTestToString() const;

  // InjectInstruction adds a key value pair to the most recently parsed set of
  // instructions.
  void InjectInstruction(const std::string &key, const std::string &value);

  // SkipCurrent passes the current test case. Unused attributes are ignored.
  void SkipCurrent();

 private:
  void ClearTest();
  void ClearInstructions();
  void OnKeyUsed(const std::string &key);
  void OnInstructionUsed(const std::string &key);
  bool ConvertToBytes(std::vector<uint8_t> *out, const std::string &value);

  std::unique_ptr<LineReader> reader_;
  // line_ is the number of lines read.
  unsigned line_ = 0;

  // start_line_ is the line number of the first attribute of the test.
  unsigned start_line_ = 0;
  // type_ is the name of the first attribute of the test.
  std::string type_;
  // parameter_ is the value of the first attribute.
  std::string parameter_;
  // attribute_count_ maps unsuffixed attribute names to the number of times
  // they have occurred so far.
  std::map<std::string, size_t> attribute_count_;
  // attributes_ contains all attributes in the test, including the first.
  std::map<std::string, std::string> attributes_;
  // instructions_ contains all instructions in scope for the test.
  std::map<std::string, std::string> instructions_;

  // unused_attributes_ is the set of attributes that have not been queried.
  std::set<std::string> unused_attributes_;

  // unused_instructions_ is the set of instructions that have not been queried.
  std::set<std::string> unused_instructions_;

  std::string current_test_;

  bool is_at_new_instruction_block_ = false;
  bool seen_non_comment_ = false;
  bool is_kas_test_ = false;

  // comment_callback_, if set, is a callback function that is called with the
  // contents of each comment as they are parsed.
  std::function<void(const std::string&)> comment_callback_;

  FileTest(const FileTest &) = delete;
  FileTest &operator=(const FileTest &) = delete;
};

// FileTestMain runs a file-based test out of |path| and returns an exit code
// suitable to return out of |main|. |run_test| should return true on pass and
// false on failure. FileTestMain also implements common handling of the 'Error'
// attribute. A test with that attribute is expected to fail. The value of the
// attribute is the reason string of the expected OpenSSL error code.
//
// Tests are guaranteed to run serially and may affect global state if need be.
// It is legal to use "tests" which, for example, import a private key into a
// list of keys. This may be used to initialize a shared set of keys for many
// tests. However, if one test fails, the framework will continue to run
// subsequent tests.
int FileTestMain(FileTestFunc run_test, void *arg, const char *path);

// FileTestMain accepts a larger number of options via a struct.
int FileTestMain(const FileTest::Options &opts);

// FileTestGTest behaves like FileTestMain, but for GTest. |path| must be the
// name of a test file embedded in the test binary.
void FileTestGTest(const char *path, std::function<void(FileTest *)> run_test);

#endif  // OPENSSL_HEADER_CRYPTO_TEST_FILE_TEST_H
