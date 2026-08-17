// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>
#include <signal.h>

#include <openssl/base.h>

#include "internal.h"
#include "../test/test_util.h"

#if !defined(OPENSSL_NO_TTY)

#if defined(OPENSSL_WINDOWS)
#include <io.h>
#define dup _dup
#define dup2 _dup2
#define fileno _fileno
#define close _close
#else
#include <unistd.h>
#endif


#if !defined(OPENSSL_ANDROID)
// On Android, when running from an APK, |tmpfile| does not work. See
// b/36991167#comment8.

// Consolidated password testing
class PemPasswdTest : public testing::Test {
 protected:
  void SetUp() override {
#if defined(OPENSSL_WINDOWS)
    _putenv_s("AWSLC_CONSOLE_NO_TTY_DETECT", "1");
#else
    setenv("AWSLC_CONSOLE_NO_TTY_DETECT", "1", 1);
#endif

    // Save original file descriptors
    original_stdin = dup(fileno(stdin));
    original_stderr = dup(fileno(stderr));

    // Create temporary files
    stdin_file = createRawTempFILE();
    stderr_file = createRawTempFILE();
    ASSERT_TRUE(stdin_file != nullptr);
    ASSERT_TRUE(stderr_file != nullptr);

    // Redirect stdin/stderr to our temp files
    ASSERT_NE(-1, dup2(fileno(stdin_file), fileno(stdin)));
    ASSERT_NE(-1, dup2(fileno(stderr_file), fileno(stderr)));

    // Initialize console for each test
    openssl_console_acquire_mutex();
    ASSERT_TRUE(openssl_console_open());
  }

  void TearDown() override {
#if defined(OPENSSL_WINDOWS)
    _putenv_s("AWSLC_CONSOLE_NO_TTY_DETECT", ""); 
#else
    unsetenv("AWSLC_CONSOLE_NO_TTY_DETECT");
#endif

    // Close console for each test
    ASSERT_TRUE(openssl_console_close());
    openssl_console_release_mutex();

    // Restore original streams
    ASSERT_NE(-1, dup2(original_stdin, fileno(stdin)));
    ASSERT_NE(-1, dup2(original_stderr, fileno(stderr)));

    // Close temp files
    if (stdin_file) {
      fclose(stdin_file);
    }
    if (stderr_file) {
      fclose(stderr_file);
    }
  }

  void MockStdinInput(const std::string& input) {
    ASSERT_GT(fwrite(input.c_str(), 1, input.length(), stdin_file), (size_t)0);
    rewind(stdin_file);
  }

  std::string GetStderrOutput() {
    std::string output;
    char buf[1024];
    rewind(stderr_file);
    while (fgets(buf, sizeof(buf), stderr_file) != nullptr) {
      output += buf;
    }

    return output;
  }

  void ResetTempFiles() {
    fclose(stdin_file);
    fclose(stderr_file);

    stdin_file = tmpfile();
    stderr_file = tmpfile();
    ASSERT_TRUE(stdin_file != nullptr);
    ASSERT_TRUE(stderr_file != nullptr);

    // Redirect stdin/stderr to our NEW temp files
    ASSERT_NE(-1, dup2(fileno(stdin_file), fileno(stdin)));
    ASSERT_NE(-1, dup2(fileno(stderr_file), fileno(stderr)));
  }


  FILE* stdin_file = nullptr;
  FILE* stderr_file = nullptr;
  int original_stdin = -1;
  int original_stderr = -1;
  const char* default_prompt = "Enter password:";
};

// Test basic password functionality with various inputs
TEST_F(PemPasswdTest, PasswordInputVariations) {
  struct TestCase {
    std::string description;
    std::string input;
    int min_size;
    int expected_result;
    std::string expected_output;
  };

  std::vector<TestCase> test_cases = {
    // Normal password
    {"Normal password", "test_password\n", 0, 0, "test_password"},
    //
    // // Empty password
    {"Empty password allowed", "\n", 0, 0, ""},
    {"Empty password rejected", "\n", 2, -1, ""},

    // Length requirements
    {"Password too short", "short\n", 10, -1, "short"},
    {"Password meets min length", "longenoughpass\n", 10, 0, "longenoughpass"},

    // Special characters
    {"Special characters", "!@#$%^&*()\n", 0, 0, "!@#$%^&*()"},
    {"Unicode characters", "パスワード\n", 0, 0, "パスワード"}
  };

  for (const auto& tc : test_cases) {
    SCOPED_TRACE(tc.description);

    char buf[1024] = {0};
    MockStdinInput(tc.input);

    ASSERT_TRUE(openssl_console_write(default_prompt));

    ASSERT_EQ(openssl_console_read(buf, tc.min_size, sizeof(buf), 0), tc.expected_result);

    if (tc.expected_result == 0) {
      ASSERT_STREQ(buf, tc.expected_output.c_str());
    }

    // Verify prompt was written
    std::string output = GetStderrOutput();
    ASSERT_TRUE(output.find(default_prompt) != std::string::npos);

    ResetTempFiles();
  }
}

// Test password verification flow (matching and non-matching)
TEST_F(PemPasswdTest, PasswordVerification) {
  struct TestCase {
    std::string description;
    std::string first_password;
    std::string second_password;
    bool should_match;
  };

  std::vector<TestCase> test_cases = {
    {"Matching passwords", "test_password\n", "test_password\n", true},
    {"Non-matching passwords", "password1\n", "password2\n", false}
  };

  for (const auto& tc : test_cases) {
    SCOPED_TRACE(tc.description);

    char buf1[1024] = {0};
    char buf2[1024] = {0};

    // Mock both password inputs
    std::string combined_input = tc.first_password + tc.second_password;
    MockStdinInput(combined_input);

    // First password entry
    ASSERT_TRUE(openssl_console_write(default_prompt));
    ASSERT_EQ(0, openssl_console_read(buf1, 0, sizeof(buf1), 0));

    // Verification prompt
    ASSERT_TRUE(openssl_console_write("Verifying - "));
    ASSERT_TRUE(openssl_console_write(default_prompt));
    ASSERT_EQ(0, openssl_console_read(buf2, 0, sizeof(buf2), 0));

    // Verify match/mismatch as expected
    if (tc.should_match) {
      ASSERT_STREQ(buf1, buf2);
    } else {
      ASSERT_STRNE(buf1, buf2);
    }

    // Verify prompts were written
    std::string output = GetStderrOutput();
    ASSERT_TRUE(output.find(default_prompt) != std::string::npos);
    ASSERT_TRUE(output.find("Verifying - ") != std::string::npos);

    ResetTempFiles();
  }
}

// Test buffer handling (truncation of long passwords)
TEST_F(PemPasswdTest, BufferHandling) {
  // Small buffer to test truncation
  char small_buf[16] = {0};

  // Create a password longer than the buffer
  std::string long_password(32, 'a');
  long_password += "\n";

  MockStdinInput(long_password);
  ASSERT_TRUE(openssl_console_write(default_prompt));
  ASSERT_EQ(0, openssl_console_read(small_buf, 0, sizeof(small_buf),0));

  // Verify the password was truncated to fit the buffer (15 chars + null terminator)
  std::string expected(15, 'a');
  ASSERT_STREQ(small_buf, expected.c_str());
}

// Test echo modes
TEST_F(PemPasswdTest, EchoModes) {
  const char* test_password = "test_password\n";
  char buf_no_echo[1024] = {0};
  char buf_with_echo[1024] = {0};

  // Test with echo disabled
  MockStdinInput(test_password);
  ASSERT_TRUE(openssl_console_write(default_prompt));
  ASSERT_EQ(0, openssl_console_read(buf_no_echo, 0, sizeof(buf_no_echo), 0));

  // Test with echo enabled
  MockStdinInput(test_password);
  ASSERT_TRUE(openssl_console_write(default_prompt));
  ASSERT_EQ(0, openssl_console_read(buf_with_echo, 0, sizeof(buf_with_echo), 1));

  // Both should have the same result
  ASSERT_STREQ(buf_no_echo, "test_password");
  ASSERT_STREQ(buf_with_echo, "test_password");
}
#endif

#endif  // !OPENSSL_NO_TTY
