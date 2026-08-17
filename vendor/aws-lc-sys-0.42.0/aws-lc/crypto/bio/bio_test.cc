// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <algorithm>
#include <string>
#include <utility>

#include <gtest/gtest.h>

#include <openssl/bio.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include <array>

#include "../internal.h"
#include "../test/file_util.h"
#include "../test/test_util.h"

#include <fcntl.h>

#if !defined(OPENSSL_WINDOWS)
static const int kOpenReadOnlyBinary = O_RDONLY;
static const int kOpenReadOnlyText = O_RDONLY;
#else
static const int kOpenReadOnlyBinary = _O_RDONLY | _O_BINARY;
static const int kOpenReadOnlyText = O_RDONLY | _O_TEXT;
#endif

TEST(BIOTest, Printf) {
  // Test a short output, a very long one, and various sizes around
  // 256 (the size of the buffer) to ensure edge cases are correct.
  static const size_t kLengths[] = {5, 250, 251, 252, 253, 254, 1023};

  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);
  ASSERT_EQ(strcmp(BIO_method_name(bio.get()), "memory buffer"), 0);

  for (size_t length : kLengths) {
    SCOPED_TRACE(length);

    std::string in(length, 'a');

    int ret = BIO_printf(bio.get(), "test %s", in.c_str());
    ASSERT_GE(ret, 0);
    EXPECT_EQ(5 + length, static_cast<size_t>(ret));

    const uint8_t *contents;
    size_t len;
    ASSERT_TRUE(BIO_mem_contents(bio.get(), &contents, &len));
    EXPECT_EQ("test " + in,
              std::string(reinterpret_cast<const char *>(contents), len));

    ASSERT_TRUE(BIO_reset(bio.get()));
  }
}

TEST(BIOTest, CloseFlags) {
#if defined(OPENSSL_ANDROID)
  // On Android, when running from an APK, |tmpfile| does not work. See
  // b/36991167#comment8.
  GTEST_SKIP();
#endif

  const char *test_str = "test\ntest\ntest\n";

  // Assert that CRLF line endings get inserted on write and translated back out
  // on read for text mode.
  TempFILE text_bio_file = createTempFILE();
  ASSERT_TRUE(text_bio_file);
  bssl::UniquePtr<BIO> text_bio(
      BIO_new_fp(text_bio_file.get(), BIO_NOCLOSE | BIO_FP_TEXT));
  int bytes_written = BIO_write(text_bio.get(), test_str, strlen(test_str));
  EXPECT_GE(bytes_written, 0);
  ASSERT_TRUE(BIO_flush(text_bio.get()));
  ASSERT_EQ(0, BIO_seek(text_bio.get(), 0));  // 0 indicates success here
  char contents[256];
  OPENSSL_memset(contents, 0, sizeof(contents));
  int bytes_read = BIO_read(text_bio.get(), contents, sizeof(contents));
  EXPECT_GE(bytes_read, bytes_written);
  EXPECT_EQ(test_str, std::string(contents));

  // Windows should have translated '\n' to '\r\n' on write, so validate that
  // by opening the file in raw binary mode (i.e. no BIO_FP_TEXT).
  bssl::UniquePtr<BIO> text_bio_raw(
      BIO_new_fp(text_bio_file.get(), BIO_NOCLOSE));
  ASSERT_EQ(0, BIO_seek(text_bio.get(), 0));  // 0 indicates success here
  OPENSSL_memset(contents, 0, sizeof(contents));
  bytes_read = BIO_read(text_bio_raw.get(), contents, sizeof(contents));
  EXPECT_GT(bytes_read, 0);
#if defined(OPENSSL_WINDOWS)
  EXPECT_EQ("test\r\ntest\r\ntest\r\n", std::string(contents));
#else
  EXPECT_EQ(test_str, std::string(contents));
#endif

  // Assert that CRLF line endings don't get inserted on write for
  // (default) binary mode.
  TempFILE binary_bio_file = createTempFILE();
  ASSERT_TRUE(binary_bio_file);
  bssl::UniquePtr<BIO> binary_bio(
      BIO_new_fp(binary_bio_file.get(), BIO_NOCLOSE));
  bytes_written = BIO_write(binary_bio.get(), test_str, strlen(test_str));
  EXPECT_EQ((int)strlen(test_str), bytes_written);
  ASSERT_TRUE(BIO_flush(binary_bio.get()));
  ASSERT_EQ(0, BIO_seek(binary_bio.get(), 0));  // 0 indicates success here
  OPENSSL_memset(contents, 0, sizeof(contents));
  bytes_read = BIO_read(binary_bio.get(), contents, sizeof(contents));
  EXPECT_GE(bytes_read, bytes_written);
  EXPECT_EQ(test_str, std::string(contents));

  // This test is meant to ensure that we're correctly handling a ftell/fseek
  // bug on Windows documented and reproduced here:
  // https://developercommunity.visualstudio.com/t/fseek-ftell-fail-in-text-mode-for-unix-style-text/425878
  long pos;
  char b1[256], b2[256];
  binary_bio.reset(BIO_new_fp(binary_bio_file.get(), BIO_NOCLOSE));
  ASSERT_EQ(0, BIO_seek(binary_bio.get(), 0));  // 0 indicates success here
  BIO_gets(binary_bio.get(), b1, sizeof(b1));
  pos = BIO_tell(binary_bio.get());
  ASSERT_GT(BIO_gets(binary_bio.get(), b1, sizeof(b1)), 0);
  BIO_seek(binary_bio.get(), pos);
  BIO_gets(binary_bio.get(), b2, sizeof(b2));
  EXPECT_EQ(std::string(b1), std::string(b2));

  // Assert that BIO_CLOSE causes the underlying file to be closed on BIO free
  // (ftell will return < 0)
  FILE *tmp = createRawTempFILE();
  ASSERT_TRUE(tmp);
  BIO *bio = BIO_new_fp(tmp, BIO_CLOSE);
  EXPECT_EQ(0, BIO_tell(bio));
  // save off fd to avoid referencing |tmp| after free and angering valgrind
  int tmp_fd = fileno(tmp);
  EXPECT_LT(0, tmp_fd);
  EXPECT_TRUE(BIO_free(bio));
  // Windows CRT hits an assertion error and stack overflow (exception code
  // 0xc0000409) when calling _tell or lseek on an already-closed file
  // descriptor, so only consider the non-Windows case here.
#if !defined(OPENSSL_WINDOWS)
  EXPECT_EQ(-1, lseek(tmp_fd, 0, SEEK_CUR));
  EXPECT_EQ(EBADF, errno);  // EBADF indicates that |BIO_free| closed the file
#endif

  // Assert that BIO_NOCLOSE does not close the underlying file on BIO free
  tmp = createRawTempFILE();
  ASSERT_TRUE(tmp);
  bio = BIO_new_fp(tmp, BIO_NOCLOSE);
  EXPECT_EQ(0, BIO_tell(bio));
  EXPECT_TRUE(BIO_free(bio));
  EXPECT_TRUE(tmp);
  EXPECT_EQ(0, ftell(tmp));   // 0 indicates file is still open
  EXPECT_EQ(0, fclose(tmp));  // 0 indicates success for fclose
}

TEST(BIOTest, ReadASN1) {
  static const size_t kLargeASN1PayloadLen = 8000;

  struct ASN1Test {
    bool should_succeed;
    std::vector<uint8_t> input;
    // suffix_len is the number of zeros to append to |input|.
    size_t suffix_len;
    // expected_len, if |should_succeed| is true, is the expected length of the
    // ASN.1 element.
    size_t expected_len;
    size_t max_len;
  } kASN1Tests[] = {
      {true, {0x30, 2, 1, 2, 0, 0}, 0, 4, 100},
      {false /* truncated */, {0x30, 3, 1, 2}, 0, 0, 100},
      {false /* should be short len */, {0x30, 0x81, 1, 1}, 0, 0, 100},
      {false /* zero padded */, {0x30, 0x82, 0, 1, 1}, 0, 0, 100},

      // Test a large payload.
      {true,
       {0x30, 0x82, kLargeASN1PayloadLen >> 8, kLargeASN1PayloadLen & 0xff},
       kLargeASN1PayloadLen,
       4 + kLargeASN1PayloadLen,
       kLargeASN1PayloadLen * 2},
      {false /* max_len too short */,
       {0x30, 0x82, kLargeASN1PayloadLen >> 8, kLargeASN1PayloadLen & 0xff},
       kLargeASN1PayloadLen,
       4 + kLargeASN1PayloadLen,
       3 + kLargeASN1PayloadLen},

      // Test an indefinite-length input.
      {true,
       {0x30, 0x80},
       kLargeASN1PayloadLen + 2,
       2 + kLargeASN1PayloadLen + 2,
       kLargeASN1PayloadLen * 2},
      {false /* max_len too short */,
       {0x30, 0x80},
       kLargeASN1PayloadLen + 2,
       2 + kLargeASN1PayloadLen + 2,
       2 + kLargeASN1PayloadLen + 1},
  };

  for (const auto &t : kASN1Tests) {
    std::vector<uint8_t> input = t.input;
    input.resize(input.size() + t.suffix_len, 0);

    bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(input.data(), input.size()));
    ASSERT_TRUE(bio);

    uint8_t *out;
    size_t out_len;
    int ok = BIO_read_asn1(bio.get(), &out, &out_len, t.max_len);
    if (!ok) {
      out = nullptr;
    }
    bssl::UniquePtr<uint8_t> out_storage(out);

    ASSERT_EQ(t.should_succeed, (ok == 1));
    if (t.should_succeed) {
      EXPECT_EQ(Bytes(input.data(), t.expected_len), Bytes(out, out_len));
    }
  }
}

TEST(BIOTest, MemReadOnly) {
  // A memory BIO created from |BIO_new_mem_buf| is a read-only buffer.
  static const char kData[] = "abcdefghijklmno\npqrs";
  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(kData, strlen(kData)));
  ASSERT_TRUE(bio);

  auto check_bio_contents = [&](Bytes b) {
    char *contents = nullptr;
    long len_l = BIO_get_mem_data(bio.get(), &contents);
    ASSERT_GE(len_l, 0);
    EXPECT_EQ(Bytes(contents, len_l), b);

    const uint8_t *contents_c = nullptr;
    size_t len = 0;
    ASSERT_TRUE(BIO_mem_contents(bio.get(), &contents_c, &len));
    EXPECT_EQ(Bytes(contents_c, len), b);

    BUF_MEM *buf = nullptr;
    ASSERT_EQ(BIO_get_mem_ptr(bio.get(), &buf), 1);
    EXPECT_EQ(Bytes(buf->data, buf->length), b);
  };

  // Writing to read-only buffers should fail.
  EXPECT_EQ(BIO_write(bio.get(), kData, strlen(kData)), -1);

  check_bio_contents(Bytes(kData));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Read less than the whole buffer.
  char buf[6];
  int ret = BIO_read(bio.get(), buf, sizeof(buf));
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("abcdef"));
  check_bio_contents(Bytes("ghijklmno\npqrs"));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  ret = BIO_read(bio.get(), buf, sizeof(buf));
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("ghijkl"));
  check_bio_contents(Bytes("mno\npqrs"));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Read stops at newline
  ret = BIO_gets(bio.get(), buf, sizeof(buf));
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("mno\n"));
  check_bio_contents(Bytes("pqrs"));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Read the remainder of the buffer.
  ret = BIO_read(bio.get(), buf, sizeof(buf));
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("pqrs"));
  EXPECT_EQ(BIO_eof(bio.get()), 1);
  check_bio_contents(Bytes(""));
  EXPECT_EQ(BIO_eof(bio.get()), 1);

  // By default, reading from a consumed read-only buffer returns EOF.
  EXPECT_EQ(BIO_read(bio.get(), buf, sizeof(buf)), 0);
  EXPECT_FALSE(BIO_should_read(bio.get()));

  // A memory BIO can be configured to return an error instead of EOF. This is
  // error is returned as retryable. (This is not especially useful here. It
  // makes more sense for a writable BIO.)
  EXPECT_EQ(BIO_set_mem_eof_return(bio.get(), -1), 1);
  EXPECT_EQ(BIO_read(bio.get(), buf, sizeof(buf)), -1);
  EXPECT_TRUE(BIO_should_read(bio.get()));

  // Rewind buffer when buffer pointer aligns with read pointer
  ret = BIO_seek(bio.get(), 3);
  ASSERT_EQ(ret, 3);
  check_bio_contents(Bytes("defghijklmno\npqrs"));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Rewind buffer when buffer pointer does not align with read pointer
  EXPECT_GT(BIO_read(bio.get(), buf, sizeof(buf)), 0);
  ret = BIO_seek(bio.get(), 4);
  ASSERT_EQ(ret, 4);
  check_bio_contents(Bytes("efghijklmno\npqrs"));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Reset buffer when buffer pointer aligns with read pointer
  ret = BIO_reset(bio.get());
  ASSERT_EQ(ret, 1);
  check_bio_contents(Bytes(kData));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Reset buffer when buffer pointer does not align with read pointer
  EXPECT_GT(BIO_read(bio.get(), buf, sizeof(buf)), 0);
  ret = BIO_reset(bio.get());
  ASSERT_EQ(ret, 1);
  check_bio_contents(Bytes(kData));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Read exactly the right number of bytes, to test the boundary condition is
  // correct.
  bio.reset(BIO_new_mem_buf("abc", 3));
  ASSERT_TRUE(bio);
  ret = BIO_read(bio.get(), buf, 3);
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("abc"));
  EXPECT_EQ(BIO_eof(bio.get()), 1);
}

TEST(BIOTest, MemWritable) {
  // A memory BIO created from |BIO_new| is writable.
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  auto check_bio_contents = [&](Bytes b) {
    char *contents = nullptr;
    long len_l = BIO_get_mem_data(bio.get(), &contents);
    ASSERT_GE(len_l, 0);
    EXPECT_EQ(Bytes(contents, len_l), b);

    const uint8_t *contents_c = nullptr;
    size_t len = 0;
    ASSERT_TRUE(BIO_mem_contents(bio.get(), &contents_c, &len));
    EXPECT_EQ(Bytes(contents_c, len), b);

    BUF_MEM *buf = nullptr;
    ASSERT_EQ(BIO_get_mem_ptr(bio.get(), &buf), 1);
    EXPECT_EQ(Bytes(buf->data, buf->length), b);
  };

  // It is initially empty.
  check_bio_contents(Bytes(""));
  EXPECT_EQ(BIO_eof(bio.get()), 1);

  // Reading from it should default to returning a retryable error.
  char buf[32];
  EXPECT_EQ(BIO_read(bio.get(), buf, sizeof(buf)), -1);
  EXPECT_TRUE(BIO_should_read(bio.get()));

  // This can be configured to return an EOF.
  EXPECT_EQ(BIO_set_mem_eof_return(bio.get(), 0), 1);
  EXPECT_EQ(BIO_read(bio.get(), buf, sizeof(buf)), 0);
  EXPECT_FALSE(BIO_should_read(bio.get()));

  // Restore the default. A writable memory |BIO| is typically used in this mode
  // so additional data can be written when exhausted.
  EXPECT_EQ(BIO_set_mem_eof_return(bio.get(), -1), 1);

  // Writes append to the buffer.
  ASSERT_EQ(BIO_write(bio.get(), "abcdef", 6), 6);
  check_bio_contents(Bytes("abcdef"));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Reset clears the buffer
  int ret = BIO_reset(bio.get());
  ASSERT_EQ(ret, 1);
  EXPECT_EQ(BIO_eof(bio.get()), 1);
  EXPECT_EQ(BIO_read(bio.get(), buf, sizeof(buf)), -1);
  EXPECT_TRUE(BIO_should_read(bio.get()));

  // Writes can include embedded NULs.
  ASSERT_EQ(BIO_write(bio.get(), "abcdef\0ghijk", 12), 12);
  check_bio_contents(Bytes("abcdef\0ghijk", 12));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Do a partial read.
  ret = BIO_read(bio.get(), buf, 4);
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("abcd"));
  check_bio_contents(Bytes("ef\0ghijk", 8));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Reads and writes may alternate.
  ASSERT_EQ(BIO_write(bio.get(), "\nlmnopq", 7), 7);
  check_bio_contents(Bytes("ef\0ghijk\nlmnopq", 15));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Reads may consume embedded NULs.
  ret = BIO_read(bio.get(), buf, 4);
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("ef\0g", 4));
  check_bio_contents(Bytes("hijk\nlmnopq"));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Read then rewind.
  ret = BIO_read(bio.get(), buf, 4);
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("hijk", 4));
  ret = BIO_seek(bio.get(), 0);
  ASSERT_EQ(ret, 0);
  check_bio_contents(Bytes("hijk\nlmnopq"));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // Gets stop at newline
  ret = BIO_gets(bio.get(), buf, sizeof(buf));
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("hijk\n", 5));
  check_bio_contents(Bytes("lmnopq"));
  EXPECT_EQ(BIO_eof(bio.get()), 0);

  // The read buffer exceeds the |BIO|, so we consume everything.
  ret = BIO_read(bio.get(), buf, sizeof(buf));
  ASSERT_GT(ret, 0);
  EXPECT_EQ(Bytes(buf, ret), Bytes("lmnopq"));
  check_bio_contents(Bytes(""));
  EXPECT_EQ(BIO_eof(bio.get()), 1);

  // The |BIO| is now empty.
  EXPECT_EQ(BIO_read(bio.get(), buf, sizeof(buf)), -1);
  EXPECT_TRUE(BIO_should_read(bio.get()));

  // Repeat the above, reading exactly the right number of bytes, to test the
  // boundary condition is correct.
  ASSERT_EQ(BIO_write(bio.get(), "abc", 3), 3);
  ret = BIO_read(bio.get(), buf, 3);
  EXPECT_EQ(Bytes(buf, ret), Bytes("abc"));
  EXPECT_EQ(BIO_read(bio.get(), buf, sizeof(buf)), -1);
  EXPECT_TRUE(BIO_should_read(bio.get()));
  EXPECT_EQ(BIO_eof(bio.get()), 1);
}

TEST(BIOTest, BioGetMemLeak) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  bssl::UniquePtr<BUF_MEM> bufmem;
  BUF_MEM *buf_mem = nullptr;
  ASSERT_TRUE(bio);
  ASSERT_EQ(BIO_puts(bio.get(), "Hello World\n"), 12);
  ASSERT_TRUE(BIO_get_mem_ptr(bio.get(), &buf_mem));
  // Take ownership of the pointer so it will free on function exit
  bufmem.reset(buf_mem);
  ASSERT_GT(BIO_set_close(bio.get(), BIO_NOCLOSE), 0);
  bio.reset();
  ASSERT_EQ(Bytes(buf_mem->data, buf_mem->length), Bytes("Hello World\n"));
}

// SetMemBufNoClose, SetMemBufClose, and SetMemBufReplacesState exercise
// a number of code-paths to ensure |BIO_set_mem_buf| functions correctly.
TEST(BIOTest, SetMemBufNoClose) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  BUF_MEM *m = BUF_MEM_new();
  ASSERT_TRUE(m);
  ASSERT_TRUE(BUF_MEM_grow(m, 5));
  OPENSSL_memcpy(m->data, "hello", 5);

  ASSERT_TRUE(BIO_set_mem_buf(bio.get(), m, BIO_NOCLOSE));

  char out[8] = {0};
  EXPECT_EQ(5, BIO_read(bio.get(), out, sizeof(out)));
  EXPECT_EQ(Bytes(out, 5), Bytes("hello", 5));

  // Exercise to surface memory violations. 
  bio.reset();
  BUF_MEM_free(m);
}

TEST(BIOTest, SetMemBufClose) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  BUF_MEM *m = BUF_MEM_new();
  ASSERT_TRUE(m);
  ASSERT_TRUE(BUF_MEM_grow(m, 5));
  OPENSSL_memcpy(m->data, "world", 5);

  ASSERT_TRUE(BIO_set_mem_buf(bio.get(), m, BIO_CLOSE));

  char out[8] = {0};
  EXPECT_EQ(5, BIO_read(bio.get(), out, sizeof(out)));
  EXPECT_EQ(Bytes(out, 5), Bytes("world", 5));

  // Exercise to surface memory violations. 
  bio.reset();
}

TEST(BIOTest, SetMemBufReplacesState) {
  // After BIO_set_mem_buf, reads must see the new buffer, not the
  // zero-initialized BUF_MEM allocated by mem_new().
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_EQ(BIO_puts(bio.get(), "initial"), 7);

  bssl::UniquePtr<BUF_MEM> m(BUF_MEM_new());
  ASSERT_TRUE(BUF_MEM_grow(m.get(), 3));
  OPENSSL_memcpy(m->data, "abc", 3);
  ASSERT_TRUE(BIO_set_mem_buf(bio.get(), m.get(), BIO_NOCLOSE));

  char out[8] = {0};
  EXPECT_EQ(3, BIO_read(bio.get(), out, sizeof(out)));
  EXPECT_EQ(Bytes(out, 3), Bytes("abc", 3));
}

TEST(BIOTest, Gets) {
  const struct {
    std::string bio;
    int gets_len;
    std::string gets_result;
  } kGetsTests[] = {
      // BIO_gets should stop at the first newline. If the buffer is too small,
      // stop there instead. Note the buffer size
      // includes a trailing NUL.
      {"123456789\n123456789", 5, "1234"},
      {"123456789\n123456789", 9, "12345678"},
      {"123456789\n123456789", 10, "123456789"},
      {"123456789\n123456789", 11, "123456789\n"},
      {"123456789\n123456789", 12, "123456789\n"},
      {"123456789\n123456789", 256, "123456789\n"},

      // If we run out of buffer, read the whole buffer.
      {"12345", 5, "1234"},
      {"12345", 6, "12345"},
      {"12345", 10, "12345"},

      // NUL bytes do not terminate gets.
      {std::string("abc\0def\nghi", 11), 256, std::string("abc\0def\n", 8)},

      // An output size of one means we cannot read any bytes. Only the trailing
      // NUL is included.
      {"12345", 1, ""},

      // Empty line.
      {"\nabcdef", 256, "\n"},
      // Empty BIO.
      {"", 256, ""},
  };
  for (const auto &t : kGetsTests) {
    SCOPED_TRACE(t.bio);
    SCOPED_TRACE(t.gets_len);

    auto check_bio_gets = [&](BIO *bio) {
      std::vector<char> buf(t.gets_len, 'a');
      int ret = BIO_gets(bio, buf.data(), t.gets_len);
      ASSERT_GE(ret, 0);
      // |BIO_gets| should write a NUL terminator, not counted in the return
      // value.
      EXPECT_EQ(Bytes(buf.data(), ret + 1),
                Bytes(t.gets_result.data(), t.gets_result.size() + 1));

      // The remaining data should still be in the BIO.
      buf.resize(t.bio.size() + 1);
      ret = BIO_read(bio, buf.data(), static_cast<int>(buf.size()));
      ASSERT_GE(ret, 0);
      EXPECT_EQ(Bytes(buf.data(), ret),
                Bytes(t.bio.substr(t.gets_result.size())));
    };

    {
      SCOPED_TRACE("memory");
      bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(t.bio.data(), t.bio.size()));
      ASSERT_TRUE(bio);
      check_bio_gets(bio.get());
    }

    if (!SkipTempFileTests()) {
      TemporaryFile file;
      ASSERT_TRUE(file.Init(t.bio));

      // TODO(crbug.com/boringssl/585): If the line has an embedded NUL, file
      // BIOs do not currently report the answer correctly.
      if (t.bio.find('\0') == std::string::npos) {
        SCOPED_TRACE("file");

        // Test |BIO_new_file|.
        bssl::UniquePtr<BIO> bio(BIO_new_file(file.path().c_str(), "rb"));
        ASSERT_TRUE(bio);
        check_bio_gets(bio.get());

        // Test |BIO_read_filename|.
        bio.reset(BIO_new(BIO_s_file()));
        ASSERT_TRUE(bio);
        ASSERT_TRUE(BIO_read_filename(bio.get(), file.path().c_str()));
        check_bio_gets(bio.get());

        // Test |BIO_NOCLOSE|.
        ScopedFILE file_obj = file.Open("rb");
        ASSERT_TRUE(file_obj);
        bio.reset(BIO_new_fp(file_obj.get(), BIO_NOCLOSE));
        ASSERT_TRUE(bio);
        check_bio_gets(bio.get());

        // Test |BIO_CLOSE|.
        file_obj = file.Open("rb");
        ASSERT_TRUE(file_obj);
        bio.reset(BIO_new_fp(file_obj.get(), BIO_CLOSE));
        ASSERT_TRUE(bio);
        file_obj.release();  // |BIO_new_fp| took ownership on success.
        check_bio_gets(bio.get());
      }

      {
        SCOPED_TRACE("fd");
#if defined(OPENSSL_WINDOWS)
        int open_flags = _O_RDONLY | _O_BINARY;
#else
        int open_flags = O_RDONLY;
#endif

        // Test |BIO_NOCLOSE|.
        ScopedFD fd = file.OpenFD(open_flags);
        ASSERT_TRUE(fd.is_valid());
        bssl::UniquePtr<BIO> bio(BIO_new_fd(fd.get(), BIO_NOCLOSE));
        ASSERT_TRUE(bio);
        check_bio_gets(bio.get());

        // Test |BIO_CLOSE|.
        fd = file.OpenFD(open_flags);
        ASSERT_TRUE(fd.is_valid());
        bio.reset(BIO_new_fd(fd.get(), BIO_CLOSE));
        ASSERT_TRUE(bio);
        fd.release();  // |BIO_new_fd| took ownership on success.
        check_bio_gets(bio.get());
      }
    }
  }

  // Negative and zero lengths should not output anything, even a trailing NUL.
  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf("12345", -1));
  ASSERT_TRUE(bio);
  char c = 'a';
  EXPECT_EQ(0, BIO_gets(bio.get(), &c, -1));
  EXPECT_EQ(0, BIO_gets(bio.get(), &c, 0));
  EXPECT_EQ(c, 'a');
}

TEST(BIOTest, ExternalData) {
  // Create a |BIO| object
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  int bio_index =
      BIO_get_ex_new_index(0, nullptr, nullptr, nullptr, CustomDataFree);
  ASSERT_GT(bio_index, 0);

  // Associate custom data with the |BIO| using |BIO_set_ex_data| and set an
  // arbitrary number.
  auto *custom_data = static_cast<CustomData *>(malloc(sizeof(CustomData)));
  ASSERT_TRUE(custom_data);
  custom_data->custom_data = 123;
  ASSERT_TRUE(BIO_set_ex_data(bio.get(), bio_index, custom_data));

  // Retrieve the custom data using |BIO_get_ex_data|.
  auto *retrieved_data =
      static_cast<CustomData *>(BIO_get_ex_data(bio.get(), bio_index));
  ASSERT_TRUE(retrieved_data);
  EXPECT_EQ(retrieved_data->custom_data, 123);
}

// Test that, on Windows, |BIO_read_filename| opens files in binary mode.
TEST(BIOTest, FileMode) {
  if (SkipTempFileTests()) {
    GTEST_SKIP();
  }

  TemporaryFile temp;
  ASSERT_TRUE(temp.Init("hello\r\nworld"));

  auto expect_file_contents = [](BIO *bio, const std::string &str) {
    // Read more than expected, to make sure we've reached the end of the file.
    std::vector<char> buf(str.size() + 100);
    int len = BIO_read(bio, buf.data(), static_cast<int>(buf.size()));
    ASSERT_GT(len, 0);
    EXPECT_EQ(Bytes(buf.data(), len), Bytes(str));
  };
  auto expect_binary_mode = [&](BIO *bio) {
    expect_file_contents(bio, "hello\r\nworld");
  };
  auto expect_text_mode = [&](BIO *bio) {
#if defined(OPENSSL_WINDOWS)
    expect_file_contents(bio, "hello\nworld");
#else
    expect_file_contents(bio, "hello\r\nworld");
#endif
  };

  // |BIO_read_filename| should open in binary mode.
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_file()));
  ASSERT_TRUE(bio);
  ASSERT_TRUE(BIO_read_filename(bio.get(), temp.path().c_str()));
  expect_binary_mode(bio.get());
  // |BIO_new_file| does not set |BIO_FP_TEXT|, so expect it to set the file's
  // mode to binary in all cases. This odd behavior, but it's what OpenSSL does.
  bio.reset(BIO_new_file(temp.path().c_str(), "rb"));
  ASSERT_TRUE(bio);
  expect_binary_mode(bio.get());
  bio.reset(BIO_new_file(temp.path().c_str(), "r"));
  ASSERT_TRUE(bio);
  // NOTE: Our behavior here aligns with OpenSSL which is to |_setmode| the file
  // to binary. BoringSSL would |expect_text_mode| below because it respects
  // default mode on Windows which is text and doesn't call |_setmode| (unless
  // |BIO_FP_TEXT| is set, which is not the case here).
  expect_binary_mode(bio.get());
  // |BIO_new_fp| will always set |_O_BINARY| if |BIO_FP_TEXT| is not set in the
  // call to |BIO_new_fp|.
  ScopedFILE file = temp.Open("rb");
  ASSERT_TRUE(file);
  bio.reset(BIO_new_fp(file.get(), BIO_NOCLOSE));
  ASSERT_TRUE(bio);
  expect_binary_mode(bio.get());
  file = temp.Open("r");
  ASSERT_TRUE(file);
  bio.reset(BIO_new_fp(file.get(), BIO_NOCLOSE));
  ASSERT_TRUE(bio);
  // NOTE: Our behavior here aligns with OpenSSL. BoringSSL would
  // |expect_text_mode| below because they don't call |_setmode| unless
  // |BIO_FP_TEXT| is set.
  expect_binary_mode(bio.get());
  // However, |BIO_FP_TEXT| changes the file to be text mode, no matter how it
  // was opened.
  file = temp.Open("rb");
  ASSERT_TRUE(file);
  bio.reset(BIO_new_fp(file.get(), BIO_NOCLOSE | BIO_FP_TEXT));
  ASSERT_TRUE(bio);
  expect_text_mode(bio.get());
  file = temp.Open("r");
  ASSERT_TRUE(file);
  bio.reset(BIO_new_fp(file.get(), BIO_NOCLOSE | BIO_FP_TEXT));
  ASSERT_TRUE(bio);
  expect_text_mode(bio.get());
  // |BIO_new_fd| inherits the FD's existing mode.
  ScopedFD fd = temp.OpenFD(kOpenReadOnlyBinary);
  ASSERT_TRUE(fd.is_valid());
  bio.reset(BIO_new_fd(fd.get(), BIO_NOCLOSE));
  ASSERT_TRUE(bio);
  expect_binary_mode(bio.get());
  fd = temp.OpenFD(kOpenReadOnlyText);
  ASSERT_TRUE(fd.is_valid());
  bio.reset(BIO_new_fd(fd.get(), BIO_NOCLOSE));
  ASSERT_TRUE(bio);
  expect_text_mode(bio.get());
}

// BIOPairTest: A parameterized test fixture for testing BIO pair operations
// This test class runs each test case with four different combinations:
//   1. Normal order (bio1->bio2) with legacy callback
//   2. Normal order (bio1->bio2) with extended callback
//   3. Swapped order (bio2->bio1) with legacy callback
//   4. Swapped order (bio2->bio1) with extended callback
//
// Parameters:
//   std::tuple<bool, bool>:
//     - First bool (get<0>): Controls BIO swapping
//       * false = normal order (bio1->bio2)
//       * true = swapped order (bio2->bio1)
//     - Second bool (get<1>): Controls callback type
//       * false = legacy callback (BIO_set_callback)
//       * true = extended callback (BIO_set_callback_ex)
class BIOPairTest : public testing::TestWithParam<std::tuple<bool, bool>> {};

TEST_P(BIOPairTest, TestPair) {
  BIO *bio1_raw, *bio2_raw;
  ASSERT_TRUE(BIO_new_bio_pair(&bio1_raw, 10, &bio2_raw, 10));
  bssl::UniquePtr<BIO> bio1(bio1_raw), bio2(bio2_raw);

  const bool should_swap = std::get<0>(GetParam());
  if (should_swap) {
    std::swap(bio1, bio2);
  }

  // Check initial states.
  EXPECT_EQ(10u, BIO_ctrl_get_write_guarantee(bio1.get()));
  EXPECT_EQ(0u, BIO_ctrl_get_read_request(bio1.get()));
  EXPECT_FALSE(BIO_eof(bio1.get()));
  EXPECT_EQ(0u, BIO_pending(bio1.get()));
  EXPECT_EQ(0u, BIO_wpending(bio1.get()));

  // Data written in one end may be read out the other.
  uint8_t buf[20];
  EXPECT_EQ(5, BIO_write(bio1.get(), "12345", 5));
  EXPECT_EQ(5u, BIO_ctrl_get_write_guarantee(bio1.get()));
  EXPECT_EQ(5u, BIO_pending(bio2.get()));
  EXPECT_EQ(5u, BIO_wpending(bio1.get()));
  ASSERT_EQ(5, BIO_read(bio2.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes("12345"), Bytes(buf, 5));
  EXPECT_EQ(10u, BIO_ctrl_get_write_guarantee(bio1.get()));
  EXPECT_EQ(0u, BIO_pending(bio2.get()));
  EXPECT_EQ(0u, BIO_wpending(bio1.get()));

  // Attempting to write more than 10 bytes will write partially.
  EXPECT_EQ(10, BIO_write(bio1.get(), "1234567890___", 13));
  EXPECT_EQ(0u, BIO_ctrl_get_write_guarantee(bio1.get()));
  EXPECT_EQ(-1, BIO_write(bio1.get(), "z", 1));
  EXPECT_TRUE(BIO_should_write(bio1.get()));
  ASSERT_EQ(10, BIO_read(bio2.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes("1234567890"), Bytes(buf, 10));
  EXPECT_EQ(10u, BIO_ctrl_get_write_guarantee(bio1.get()));

  // Unsuccessful reads update the read request.
  EXPECT_EQ(-1, BIO_read(bio2.get(), buf, 5));
  EXPECT_TRUE(BIO_should_read(bio2.get()));
  EXPECT_EQ(5u, BIO_ctrl_get_read_request(bio1.get()));

  // The read request is clamped to the size of the buffer.
  EXPECT_EQ(-1, BIO_read(bio2.get(), buf, 20));
  EXPECT_TRUE(BIO_should_read(bio2.get()));
  EXPECT_EQ(10u, BIO_ctrl_get_read_request(bio1.get()));

  // Data may be written and read in chunks.
  EXPECT_EQ(5, BIO_write(bio1.get(), "12345", 5));
  EXPECT_EQ(5u, BIO_ctrl_get_write_guarantee(bio1.get()));
  EXPECT_EQ(5, BIO_write(bio1.get(), "67890___", 8));
  EXPECT_EQ(0u, BIO_ctrl_get_write_guarantee(bio1.get()));
  ASSERT_EQ(3, BIO_read(bio2.get(), buf, 3));
  EXPECT_EQ(Bytes("123"), Bytes(buf, 3));
  EXPECT_EQ(3u, BIO_ctrl_get_write_guarantee(bio1.get()));
  ASSERT_EQ(7, BIO_read(bio2.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes("4567890"), Bytes(buf, 7));
  EXPECT_EQ(10u, BIO_ctrl_get_write_guarantee(bio1.get()));

  // Successful reads reset the read request.
  EXPECT_EQ(0u, BIO_ctrl_get_read_request(bio1.get()));

  // Test writes and reads starting in the middle of the ring buffer and
  // wrapping to front.
  EXPECT_EQ(8, BIO_write(bio1.get(), "abcdefgh", 8));
  EXPECT_EQ(2u, BIO_ctrl_get_write_guarantee(bio1.get()));
  ASSERT_EQ(3, BIO_read(bio2.get(), buf, 3));
  EXPECT_EQ(Bytes("abc"), Bytes(buf, 3));
  EXPECT_EQ(5u, BIO_ctrl_get_write_guarantee(bio1.get()));
  EXPECT_EQ(5, BIO_write(bio1.get(), "ijklm___", 8));
  EXPECT_EQ(0u, BIO_ctrl_get_write_guarantee(bio1.get()));
  ASSERT_EQ(10, BIO_read(bio2.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes("defghijklm"), Bytes(buf, 10));
  EXPECT_EQ(10u, BIO_ctrl_get_write_guarantee(bio1.get()));

  // Data may flow from both ends in parallel.
  EXPECT_EQ(5, BIO_write(bio1.get(), "12345", 5));
  EXPECT_EQ(5, BIO_write(bio2.get(), "67890", 5));
  ASSERT_EQ(5, BIO_read(bio2.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes("12345"), Bytes(buf, 5));
  ASSERT_EQ(5, BIO_read(bio1.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes("67890"), Bytes(buf, 5));

  // Closing the write end causes an EOF on the read half, after draining.
  EXPECT_EQ(5, BIO_write(bio1.get(), "12345", 5));
  EXPECT_TRUE(BIO_shutdown_wr(bio1.get()));
  EXPECT_FALSE(BIO_eof(bio2.get()));
  EXPECT_EQ(5u, BIO_pending(bio2.get()));
  EXPECT_EQ(5u, BIO_wpending(bio1.get()));
  ASSERT_EQ(5, BIO_read(bio2.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes("12345"), Bytes(buf, 5));
  EXPECT_TRUE(BIO_eof(bio2.get()));
  EXPECT_EQ(0u, BIO_pending(bio2.get()));
  EXPECT_EQ(0u, BIO_wpending(bio1.get()));
  EXPECT_EQ(0, BIO_read(bio2.get(), buf, sizeof(buf)));
  EXPECT_TRUE(BIO_eof(bio2.get()));
  EXPECT_EQ(0u, BIO_pending(bio2.get()));
  EXPECT_EQ(0u, BIO_wpending(bio1.get()));

  // A closed write end may not be written to.
  EXPECT_EQ(0u, BIO_ctrl_get_write_guarantee(bio1.get()));
  EXPECT_EQ(-1, BIO_write(bio1.get(), "_____", 5));
  EXPECT_TRUE(ErrorEquals(ERR_get_error(), ERR_LIB_BIO, BIO_R_BROKEN_PIPE));

  // The other end is still functional.
  EXPECT_EQ(5, BIO_write(bio2.get(), "12345", 5));
  ASSERT_EQ(5, BIO_read(bio1.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes("12345"), Bytes(buf, 5));
  EXPECT_FALSE(BIO_eof(bio1.get()));

  // Destroying |bio2| implicitly closes it, and discards unread data.
  EXPECT_EQ(5, BIO_write(bio2.get(), "12345", 5));
  bio2 = nullptr;

  // |bio1| no longer has the "init" flag set, so reads and writes will fail at
  // the BIO framework.
  EXPECT_FALSE(BIO_get_init(bio1.get()));
  EXPECT_EQ(-2, BIO_write(bio1.get(), "12345", 5));
  EXPECT_TRUE(ErrorEquals(ERR_get_error(), ERR_LIB_BIO, BIO_R_UNINITIALIZED));
  EXPECT_EQ(-2, BIO_read(bio1.get(), buf, sizeof(buf)));
  EXPECT_TRUE(ErrorEquals(ERR_get_error(), ERR_LIB_BIO, BIO_R_UNINITIALIZED));

  // Although in this disconnected state, |BIO_ctrl| works. |bio1| should
  // report EOF when it has no peer.
  EXPECT_TRUE(BIO_eof(bio1.get()));

  // BIO_ctrl_get_write_guarantee should return 0 because there is no one to
  // write to.
  EXPECT_EQ(0u, BIO_ctrl_get_write_guarantee(bio1.get()));
}

#define CALL_BACK_FAILURE -1234567
#define CB_TEST_COUNT 2
static int test_count_ex;
static BIO *param_b_ex[CB_TEST_COUNT];
static int param_oper_ex[CB_TEST_COUNT];
static const char *param_argp_ex[CB_TEST_COUNT];
static int param_argi_ex[CB_TEST_COUNT];
static long param_argl_ex[CB_TEST_COUNT];
static long param_ret_ex[CB_TEST_COUNT];
static size_t param_len_ex[CB_TEST_COUNT];
static size_t param_processed_ex[CB_TEST_COUNT];

static long bio_cb_ex(BIO *b, int oper, const char *argp, size_t len, int argi,
                      long argl, int ret, size_t *processed) {
  if (test_count_ex >= CB_TEST_COUNT) {
    return CALL_BACK_FAILURE;
  }
  param_b_ex[test_count_ex] = b;
  param_oper_ex[test_count_ex] = oper;
  param_argp_ex[test_count_ex] = argp;
  param_argi_ex[test_count_ex] = argi;
  param_argl_ex[test_count_ex] = argl;
  param_ret_ex[test_count_ex] = ret;
  param_len_ex[test_count_ex] = len;
  param_processed_ex[test_count_ex] = processed != NULL ? *processed : 0;
  test_count_ex++;
  return ret;
}

static long bio_cb(BIO *b, int oper, const char *argp, int argi,
                      long argl, long ret) {
  if (test_count_ex >= CB_TEST_COUNT) {
    return CALL_BACK_FAILURE;
  }
  param_b_ex[test_count_ex] = b;
  param_oper_ex[test_count_ex] = oper;
  param_argp_ex[test_count_ex] = argp;
  param_argi_ex[test_count_ex] = argi;
  param_argl_ex[test_count_ex] = argl;
  param_ret_ex[test_count_ex] = ret;
  test_count_ex++;
  return ret;
}

static void bio_callback_cleanup() {
  // These mocks are used in multiple tests and need to be reset
  test_count_ex = 0;
  OPENSSL_cleanse(param_b_ex, sizeof(param_b_ex));
  OPENSSL_cleanse(param_oper_ex, sizeof(param_oper_ex));
  OPENSSL_cleanse(param_argp_ex, sizeof(param_argp_ex));
  OPENSSL_cleanse(param_argi_ex, sizeof(param_argi_ex));
  OPENSSL_cleanse(param_argl_ex, sizeof(param_argl_ex));
  OPENSSL_cleanse(param_ret_ex, sizeof(param_ret_ex));
  OPENSSL_cleanse(param_len_ex, sizeof(param_len_ex));
  OPENSSL_cleanse(param_processed_ex, sizeof(param_processed_ex));
}

#define TEST_BUF_LEN 20
#define TEST_DATA_WRITTEN 5
TEST_P(BIOPairTest, TestCallbacks) {
  bio_callback_cleanup();

  BIO *bio1, *bio2;
  ASSERT_TRUE(BIO_new_bio_pair(&bio1, 10, &bio2, 10));

  // Check the second parameter from the test tuple to determine which callback type to use
  // params is a tuple<bool, bool> where:
  //   - get<0> controls bio swapping
  //   - get<1> controls callback type (true = extended, false = legacy)
  const auto& params = GetParam();
  if (std::get<0>(params)) {  // swap_bios
    std::swap(bio1, bio2);
  }

  if (std::get<1>(params)) {
    // Use extended callback (BIO_callback_ex) which provides additional parameters:
    // - len: size of the buffer for read/write operations
    // - processed: pointer to store number of bytes actually processed
    BIO_set_callback_ex(bio2, bio_cb_ex);
  } else {
    // Use legacy callback (BIO_callback) with basic parameters
    BIO_set_callback(bio2, bio_cb);
  }

  // Data written in one end may be read out the other.
  uint8_t buf[TEST_BUF_LEN];
  EXPECT_EQ(TEST_DATA_WRITTEN, BIO_write(bio1, "12345", TEST_DATA_WRITTEN));
  ASSERT_EQ(TEST_DATA_WRITTEN, BIO_read(bio2, buf, sizeof(buf)));
  EXPECT_EQ(Bytes("12345"), Bytes(buf, TEST_DATA_WRITTEN));

  // Check that read or write was called first, then the combo with
  // BIO_CB_RETURN
  ASSERT_EQ(param_oper_ex[0], BIO_CB_READ);
  ASSERT_EQ(param_oper_ex[1], BIO_CB_READ | BIO_CB_RETURN);

  // argp is a pointer to a buffer for read/write operations. We don't care
  // where the buf is, but it should be the same before and after the BIO calls
  ASSERT_EQ(param_argp_ex[0], param_argp_ex[1]);

  // The calls before the BIO operation use 1 for the BIO's return value
  ASSERT_EQ(param_ret_ex[0], 1);



  if (!std::get<1>(params)) {
    // Legacy callback uses ret to hold data processed
    ASSERT_EQ(param_ret_ex[1], TEST_DATA_WRITTEN);
    // Legacy callback argi is used for len
    ASSERT_EQ(param_argi_ex[0], TEST_BUF_LEN);
    ASSERT_EQ(param_argi_ex[1], TEST_BUF_LEN);
    ASSERT_EQ(param_argl_ex[0], 0);
    ASSERT_EQ(param_argl_ex[1], 0);
  }

  // Extended callback specific verifications
  if (std::get<1>(params)) {// If using extended callback
    // For callback_ex ret is set to 1 after setting processed
    ASSERT_EQ(param_ret_ex[1], 1);
    // For callback_ex argi and arl are unused
    ASSERT_EQ(param_argi_ex[0], 0);
    ASSERT_EQ(param_argi_ex[1], 0);
    ASSERT_EQ(param_argl_ex[0], 0);
    ASSERT_EQ(param_argl_ex[1], 0);
    // For callback_ex the |len| param is the requested number of bytes to
    // read/write
    ASSERT_EQ(param_len_ex[0], (size_t)TEST_BUF_LEN);
    ASSERT_EQ(param_len_ex[0], (size_t)TEST_BUF_LEN);
    // processed is null (0 in the array) the first call and the actual data the
    // second time
    ASSERT_EQ(param_processed_ex[0], 0u);
    ASSERT_EQ(param_processed_ex[1], 5u);
  }

  // The mock should be "full" at this point
  ASSERT_EQ(test_count_ex, CB_TEST_COUNT);

  // If we attempt to read or write more from either BIO the callback fails
  // and the callback return value is returned to the caller
  ASSERT_EQ(CALL_BACK_FAILURE, BIO_read(bio2, buf, sizeof(buf)));

  // Run bio_callback_cleanup to reset the mock, without this when BIO_free
  // calls the callback it would fail before freeing the memory and be detected
  // as a memory leak.
  bio_callback_cleanup();
  ASSERT_EQ(BIO_free(bio1), 1);
  ASSERT_EQ(BIO_free(bio2), 1);

  ASSERT_EQ(param_oper_ex[0], BIO_CB_FREE);

  ASSERT_EQ(param_argp_ex[0], nullptr);
  ASSERT_EQ(param_argi_ex[0], 0);
  ASSERT_EQ(param_argl_ex[0], 0);
  ASSERT_EQ(param_ret_ex[0], 1);
  ASSERT_EQ(param_len_ex[0], 0u);
}

namespace {
#if !defined(OPENSSL_NO_SOCK)
static int callback_invoked = 0;

static long callback(BIO *b, int state, int res) {
  callback_invoked = 1;
  EXPECT_EQ(state, 0);
  EXPECT_EQ(res, -1);
  return 0;
}

TEST(BIOTest, InvokeConnectCallback) {
#if defined(TARGET_OS_IPHONE) && TARGET_OS_IPHONE
  GTEST_SKIP() << "InvokeConnectCallback does not run on iOS";
#endif

  ASSERT_EQ(callback_invoked, 0);
  BIO *bio = BIO_new(BIO_s_connect());
  ASSERT_NE(bio, nullptr);

  ASSERT_TRUE(BIO_set_conn_hostname(bio, "localhost"));
  ASSERT_TRUE(BIO_set_conn_port(bio, "5325"));
  ASSERT_TRUE(BIO_callback_ctrl(bio, BIO_CTRL_SET_CALLBACK, callback));

  ASSERT_EQ(BIO_do_connect(bio), 0);
  ASSERT_EQ(callback_invoked, 1);

  ASSERT_TRUE(BIO_free(bio));
}
#endif  // !OPENSSL_NO_SOCK
}  // namespace


// Instantiate the parameterized test suite for BIOPairTest
// This creates test instances for all combinations of the boolean parameters
//
// Parameters:
//   First argument "All": Test suite name prefix
//   Second argument "BIOPairTest": The test fixture class
//   Third argument: Parameter generator using testing::Combine to create Cartesian product
//     - First Bool(): Controls BIO swapping
//       * false: Keep original BIO order (bio1->bio2)
//       * true:  Swap BIOs (bio2->bio1)
//     - Second Bool(): Controls callback type
//       * false: Use legacy callback (BIO_set_callback)
//       * true:  Use extended callback (BIO_set_callback_ex)
//
// This will generate four test combinations:
//   1. (false, false) - Normal order, Legacy callback
//   2. (false, true)  - Normal order, Extended callback
//   3. (true, false)  - Swapped order, Legacy callback
//   4. (true, true)   - Swapped order, Extended callback
INSTANTIATE_TEST_SUITE_P(
All,
  BIOPairTest,
  testing::Combine(
      testing::Bool(),  // swap_bios
      testing::Bool()   // use_extended_callback
  )
  );

TEST(BIOTest, ReadWriteEx) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  // Reading from an initially empty bio should default to returning a error.
  // Check that both |BIO_read| and |BIO_read_ex| fail.
  char buf[32];
  size_t read = 1;
  EXPECT_EQ(BIO_read(bio.get(), buf, sizeof(buf)), -1);
  EXPECT_FALSE(BIO_read_ex(bio.get(), buf, sizeof(buf), &read));
  EXPECT_EQ(read, (size_t)0);

  // Write and read normally from buffer.
  size_t written = 1;
  ASSERT_TRUE(BIO_write_ex(bio.get(), "abcdef", 6, &written));
  EXPECT_EQ(written, (size_t)6);
  ASSERT_TRUE(BIO_read_ex(bio.get(), buf, sizeof(buf), &read));
  EXPECT_EQ(read, (size_t)6);
  EXPECT_EQ(Bytes(buf, read), Bytes("abcdef"));

  // Test NULL |written_bytes| behavior works.
  ASSERT_TRUE(BIO_write_ex(bio.get(), "ghilmnop", 8, nullptr));
  ASSERT_TRUE(BIO_read_ex(bio.get(), buf, sizeof(buf), &read));
  EXPECT_EQ(read, (size_t)8);
  EXPECT_EQ(Bytes(buf, read), Bytes("ghilmnop"));

  // Test NULL |read_bytes| behavior fails.
  ASSERT_TRUE(BIO_write_ex(bio.get(), "ghilmnop", 8, nullptr));
  ASSERT_FALSE(BIO_read_ex(bio.get(), buf, sizeof(buf), nullptr));

  // Test that |BIO_write/read_ex| align with their non-ex counterparts, when
  // encountering NULL data. EOF in |BIO_read| is indicated by returning 0.
  // In |BIO_read_ex| however, EOF returns a failure and sets |read| to 0.
  EXPECT_FALSE(BIO_write(bio.get(), nullptr, 0));
  EXPECT_FALSE(BIO_write_ex(bio.get(), nullptr, 0, &written));
  EXPECT_EQ(written, (size_t)0);
  EXPECT_EQ(BIO_read(bio.get(), nullptr, 0), 0);
  EXPECT_FALSE(BIO_read_ex(bio.get(), nullptr, 0, &read));
  EXPECT_EQ(read, (size_t)0);
}

TEST(BIOTest, TestPutsAsWrite) {
  // By default, |BIO_puts| acts as |BIO_write|.
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  // Test basic puts and read
  uint8_t buf[32];
  EXPECT_EQ(12, BIO_puts(bio.get(), "hello world\n"));
  EXPECT_EQ(12, BIO_read(bio.get(), buf, sizeof(buf)));
}

namespace {
// Define custom BIO and BIO_METHODS to test BIO_puts without write
static int customPuts(BIO *b, const char *in) {
  return 0;
}
static int customNew(BIO *b) {
  b->init=1;
  return 1;
}
static const BIO_METHOD custom_method = {
  BIO_TYPE_NONE, "CustomBioMethod", NULL /* write */,
  NULL,          customPuts,        NULL,
  NULL,          customNew,         NULL,
  NULL
};

static const BIO_METHOD *BIO_cust(void) { return &custom_method; }

TEST(BIOTest, TestCustomPuts) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_cust()));
  ASSERT_TRUE(bio);

  ASSERT_EQ(0, BIO_puts(bio.get(), "hello world"));

  // Test setting new puts method by creating a new BIO
  bssl::UniquePtr<BIO_METHOD> method(BIO_meth_new(0, nullptr));
  ASSERT_TRUE(method);
  ASSERT_TRUE(BIO_meth_set_create(
    method.get(), [](BIO *b) -> int {
      BIO_set_init(b, 1);
      return 1;
  }));
  ASSERT_TRUE(BIO_meth_set_puts(
    method.get(), [](BIO *b, const char *in) -> int {
      return 100;
    }
  ));
  bssl::UniquePtr<BIO> bio1(BIO_new(method.get()));
  ASSERT_TRUE(bio1);
  ASSERT_TRUE(bio1.get()->method->bputs);
  ASSERT_FALSE(bio1.get()->method->bwrite);
  // The new BIO_puts should only return 100
  ASSERT_EQ(100, BIO_puts(bio1.get(), "hello world"));
}

TEST(BIOTest, TestPutsNullMethod) {
  // Create new BIO to test when neither puts nor write is set
  bssl::UniquePtr<BIO_METHOD> method(BIO_meth_new(0, nullptr));
  ASSERT_TRUE(method);
  ASSERT_TRUE(BIO_meth_set_create(
    method.get(), [](BIO *b) -> int {
      BIO_set_init(b, 1);
      return 1;
  }));
  bssl::UniquePtr<BIO> bio(BIO_new(method.get()));
  ASSERT_TRUE(bio);

  ASSERT_FALSE(bio.get()->method->bputs);
  ASSERT_FALSE(bio.get()->method->bwrite);
  ASSERT_EQ(-2, BIO_puts(bio.get(), "hello world"));
}
} //namespace

TEST_P(BIOPairTest, TestPutsCallbacks) {
  bio_callback_cleanup();
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  //   - get<1> controls callback type (true = extended, false = legacy)
  const bool use_extended_callback = std::get<1>(GetParam());

  if (use_extended_callback) {
    // Use extended callback (BIO_callback_ex) which provides additional parameters:
    // - len: size of the buffer for read/write operations
    // - processed: pointer to store number of bytes actually processed
    BIO_set_callback_ex(bio.get(), bio_cb_ex);
  } else {
    // Use legacy callback (BIO_callback) with basic parameters
    BIO_set_callback(bio.get(), bio_cb);
  }

  EXPECT_EQ(TEST_DATA_WRITTEN, BIO_puts(bio.get(), "12345"));

  ASSERT_EQ(param_oper_ex[0], BIO_CB_PUTS);
  ASSERT_EQ(param_oper_ex[1], BIO_CB_PUTS | BIO_CB_RETURN);

  ASSERT_EQ(param_argp_ex[0], param_argp_ex[1]);
  ASSERT_EQ(param_ret_ex[0], 1);

  if (!use_extended_callback) {
    // Legacy callback uses ret for data processed
    ASSERT_EQ(param_ret_ex[1], TEST_DATA_WRITTEN);
  }


  ASSERT_EQ(param_argi_ex[0], 0);
  ASSERT_EQ(param_argi_ex[1], 0);
  ASSERT_EQ(param_argl_ex[0], 0);
  ASSERT_EQ(param_argl_ex[1], 0);

  // Extended callback specific verifications
  if (use_extended_callback) {
    // ret is set to processed and ret is reset back to 1
    ASSERT_EQ(param_ret_ex[1], 1);
    // len unused in puts callback
    ASSERT_FALSE(param_len_ex[0]);
    ASSERT_FALSE(param_len_ex[1]);

    // Verify processed bytes
    ASSERT_EQ(param_processed_ex[0], 0u);
    ASSERT_EQ(param_processed_ex[1], 5u);
  }

  // The mock should be "full" at this point
  ASSERT_EQ(test_count_ex, CB_TEST_COUNT);

  bio_callback_cleanup();
}

TEST_P(BIOPairTest, TestGetsCallback) {
  bio_callback_cleanup();

  bssl::UniquePtr<BIO> bio (BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);
  // write data to BIO, then set callback
  EXPECT_EQ(TEST_DATA_WRITTEN, BIO_write(bio.get(), "12345", TEST_DATA_WRITTEN));
  char buf[TEST_BUF_LEN];

  //   - get<1> controls callback type (true = extended, false = legacy)
  const bool use_extended_callback = std::get<1>(GetParam());

  if (use_extended_callback) {
    // Use extended callback (BIO_callback_ex) which provides additional parameters:
    // - len: size of the buffer for read/write operations
    // - processed: pointer to store number of bytes actually processed
    BIO_set_callback_ex(bio.get(), bio_cb_ex);
  } else {
    // Use legacy callback (BIO_callback) with basic parameters
    BIO_set_callback(bio.get(), bio_cb);
  }

  ASSERT_EQ(TEST_DATA_WRITTEN, BIO_gets(bio.get(), buf, sizeof(buf)));

  ASSERT_EQ(param_oper_ex[0], BIO_CB_GETS);
  ASSERT_EQ(param_oper_ex[1], BIO_CB_GETS | BIO_CB_RETURN);

  ASSERT_EQ(param_argp_ex[0], param_argp_ex[1]);
  ASSERT_EQ(param_ret_ex[0], 1);


  ASSERT_EQ(param_argl_ex[0], 0);
  ASSERT_EQ(param_argl_ex[1], 0);

  // Old-style callback uses argi for len
  if (!use_extended_callback) {
    // use ret for data processed
    ASSERT_EQ(param_ret_ex[1], TEST_DATA_WRITTEN);
    ASSERT_EQ(param_argi_ex[0], TEST_BUF_LEN);
    ASSERT_EQ(param_argi_ex[1], TEST_BUF_LEN);
  }

  // Extended callback specific verifications
  if (use_extended_callback) {
    // ret is set to 1 after setting processed
    ASSERT_EQ(param_ret_ex[1], 1);
    ASSERT_EQ(param_argi_ex[0], 0);
    ASSERT_EQ(param_argi_ex[1], 0);
    ASSERT_EQ(param_len_ex[0], (size_t)TEST_BUF_LEN);
    ASSERT_EQ(param_len_ex[1], (size_t)TEST_BUF_LEN);
    ASSERT_EQ(param_processed_ex[0], 0u);
    ASSERT_EQ(param_processed_ex[1], 5u);
  }


  ASSERT_EQ(test_count_ex, CB_TEST_COUNT);

  bio_callback_cleanup();
}

TEST_P(BIOPairTest, TestCtrlCallback) {
  bio_callback_cleanup();

  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  //   - get<1> controls callback type (true = extended, false = legacy)
  const bool use_extended_callback = std::get<1>(GetParam());

  if (use_extended_callback) {
    // Use extended callback (BIO_callback_ex) which provides additional parameters:
    // - len: size of the buffer for read/write operations
    // - processed: pointer to store number of bytes actually processed
    BIO_set_callback_ex(bio.get(), bio_cb_ex);
  } else {
    // Use legacy callback (BIO_callback) with basic parameters
    BIO_set_callback(bio.get(), bio_cb);
  }

  char buf[TEST_BUF_LEN];
  // Test BIO_ctrl. This is not normally called directly so we can use one of
  // the macros such as BIO_reset to test it
  ASSERT_EQ(BIO_reset(bio.get()), 1);

  ASSERT_EQ(param_oper_ex[0], BIO_CB_CTRL);
  ASSERT_EQ(param_oper_ex[1], BIO_CB_CTRL | BIO_CB_RETURN);

  // argi in this case in the cmd sent to the ctrl method
  ASSERT_EQ(param_argi_ex[0], BIO_CTRL_RESET);
  ASSERT_EQ(param_argi_ex[1], BIO_CTRL_RESET);

  // BIO_ctrl of a memory bio sets ret to 1 when it calls the reset method
  ASSERT_EQ(param_ret_ex[0], 1);
  ASSERT_EQ(param_ret_ex[1], 1);

  // processed is unused in ctrl
  ASSERT_EQ(param_processed_ex[0], 0u);
  ASSERT_EQ(param_processed_ex[1], 0u);

  bio_callback_cleanup();
  // check that BIO_reset was called correctly
  ASSERT_EQ(BIO_gets(bio.get(), buf, sizeof(buf)), 0);

  bio_callback_cleanup();
}

TEST(BIOTest, GetMemDataBackwardsCompat) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  const uint8_t contents[] = {0x72, 0x61, 0x63, 0x63, 0x6f, 0x6f, 0x6e};

  // Write some test data
  int write_len = BIO_write(bio.get(), contents, sizeof(contents));
  ASSERT_EQ(sizeof(contents), (size_t)write_len);

  // Yes, this is something gRPC does
  const uint8_t *ptr = NULL;
  long data_len = BIO_get_mem_data(bio.get(), &ptr);
  ASSERT_EQ((size_t)data_len, sizeof(contents));
  EXPECT_EQ(Bytes(contents, sizeof(contents)), Bytes(ptr, data_len));
}

TEST(BIOTest, Dump) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  static constexpr std::array<uint8_t, 12> data = {{
      0x00, 0x01, 0x02, 0x7f, 0x80, 0xff, 'A', 'B', 'C', '\n', '\r', '\t'
  }};

  int ret = BIO_dump(bio.get(), data.data(), data.size());

  static constexpr std::array<char, 75> kExpectedOutput = {{
    '0', '0', '0', '0', '0', '0', '0', '0',  // offset
    ' ', ' ',                                 // spacing
    '0', '0', ' ', '0', '1', ' ', '0', '2', ' ', '7', 'f', ' ',
    '8', '0', ' ', 'f', 'f', ' ', '4', '1', ' ', '4', '2', ' ', // first 8 bytes hex
    ' ',                                      // group separator
    '4', '3', ' ', '0', 'a', ' ', '0', 'd', ' ', '0', '9', // remaining 4 bytes hex
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', // padding
    '|',                                      // separator
    '.', '.', '.', '.', '.', '.', 'A', 'B', 'C', '.', '.', '.', // ASCII
    '|', '\n'                                // closing
}};

  const uint8_t *contents = nullptr;
  size_t len = 0;
  ASSERT_TRUE(BIO_mem_contents(bio.get(), &contents, &len));
  std::string output(reinterpret_cast<const char *>(contents), len);

  EXPECT_EQ(output, std::string(kExpectedOutput.data(), kExpectedOutput.size()))
    << "BIO_dump output should exactly match expected format";
  EXPECT_EQ(ret, static_cast<int>(kExpectedOutput.size()))
      << "Return value should match expected length";

  // Test edge cases
  bio.reset(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);
  EXPECT_EQ(BIO_dump(bio.get(), data.data(), 0), 0)
      << "Valid data with zero length should return 0";

  EXPECT_EQ(BIO_dump(bio.get(), data.data(), -1), -1)
      << "Negative length should return -1";

  EXPECT_EQ(BIO_dump(bio.get(), nullptr, 0), -1)
      << "BIO_dump with NULL data should return -1";

  EXPECT_EQ(BIO_dump(nullptr, data.data(), data.size()), -1)
      << "BIO_dump with NULL BIO should return -1";

  // Test with larger data
  static constexpr std::array<uint8_t, 32> large_data = {{
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
}};

  bio.reset(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);
  ret = BIO_dump(bio.get(), large_data.data(), large_data.size());

  static constexpr std::array<char, 158> kExpectedLargeOutput = {{
    // First line
    '0', '0', '0', '0', '0', '0', '0', '0',  // offset
    ' ', ' ',                                 // spacing
    '0', '0', ' ', '0', '1', ' ', '0', '2', ' ', '0', '3', ' ',
    '0', '4', ' ', '0', '5', ' ', '0', '6', ' ', '0', '7', ' ', // first 8 bytes hex
    ' ',                                      // group separator
    '0', '8', ' ', '0', '9', ' ', '0', 'a', ' ', '0', 'b', ' ',
    '0', 'c', ' ', '0', 'd', ' ', '0', 'e', ' ', '0', 'f', ' ', // last 8 bytes hex
    ' ', '|',                                // separator
    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', // ASCII
    '|', '\n',                              // closing

    // Second line
    '0', '0', '0', '0', '0', '0', '1', '0',  // offset
    ' ', ' ',                                 // spacing
    '1', '0', ' ', '1', '1', ' ', '1', '2', ' ', '1', '3', ' ',
    '1', '4', ' ', '1', '5', ' ', '1', '6', ' ', '1', '7', ' ', // first 8 bytes hex
    ' ',                                      // group separator
    '1', '8', ' ', '1', '9', ' ', '1', 'a', ' ', '1', 'b', ' ',
    '1', 'c', ' ', '1', 'd', ' ', '1', 'e', ' ', '1', 'f', ' ', // last 8 bytes hex
    ' ', '|',                                // separator
    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', // ASCII
    '|', '\n'                               // closing
}};

  ASSERT_TRUE(BIO_mem_contents(bio.get(), &contents, &len));
  output = std::string(reinterpret_cast<const char *>(contents), len);

  EXPECT_EQ(output, std::string(kExpectedLargeOutput.data(), kExpectedLargeOutput.size()))
    << "BIO_dump output should exactly match expected format";
  EXPECT_EQ(ret, static_cast<int>(kExpectedLargeOutput.size()))
      << "Return value should match expected length";
}
