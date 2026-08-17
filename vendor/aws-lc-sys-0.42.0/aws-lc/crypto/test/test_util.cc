// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include "test_util.h"

#include <fstream>
#include <ostream>
#include <inttypes.h>

#include <openssl/bn.h>
#include <openssl/err.h>

#include <thread>
#if !defined(OPENSSL_WINDOWS) && !defined(OPENSSL_WASM_WASI)
 #include <sys/wait.h>
#endif

#include <inttypes.h>

#include <openssl/err.h>

#include "../internal.h"
#include "../ube/fork_ube_detect.h"
#include "openssl/pem.h"
#include "openssl/rand.h"


void hexdump(FILE *fp, const char *msg, const void *in, size_t len) {
  const uint8_t *data = reinterpret_cast<const uint8_t *>(in);

  fputs(msg, fp);
  for (size_t i = 0; i < len; i++) {
    fprintf(fp, "%02x", data[i]);
  }
  fputs("\n", fp);
}

std::ostream &operator<<(std::ostream &os, const Bytes &in) {
  if (in.span_.empty()) {
    return os << "<empty Bytes>";
  }

  // Print a byte slice as hex.
  os << EncodeHex(in.span_);
  return os;
}

bool DecodeHex(std::vector<uint8_t> *out, const std::string &in) {
  out->clear();
  if (in.size() % 2 != 0) {
    return false;
  }
  out->reserve(in.size() / 2);
  for (size_t i = 0; i < in.size(); i += 2) {
    uint8_t hi, lo;
    if (!OPENSSL_fromxdigit(&hi, in[i]) ||
        !OPENSSL_fromxdigit(&lo, in[i + 1])) {
      return false;
    }
    out->push_back((hi << 4) | lo);
  }
  return true;
}

std::vector<uint8_t> HexToBytes(const char *str) {
  std::vector<uint8_t> ret;
  if (!DecodeHex(&ret, str)) {
    abort();
  }
  return ret;
}

std::string EncodeHex(bssl::Span<const uint8_t> in) {
  static const char kHexDigits[] = "0123456789abcdef";
  std::string ret;
  ret.reserve(in.size() * 2);
  for (uint8_t b : in) {
    ret += kHexDigits[b >> 4];
    ret += kHexDigits[b & 0xf];
  }
  return ret;
}

testing::AssertionResult ErrorEquals(uint32_t err, int lib, int reason) {
  if (ERR_GET_LIB(err) == lib && ERR_GET_REASON(err) == reason) {
    return testing::AssertionSuccess();
  }

  char buf[128], expected[128];
  return testing::AssertionFailure()
         << "Got \"" << ERR_error_string_n(err, buf, sizeof(buf))
         << "\", wanted \""
         << ERR_error_string_n(ERR_PACK(lib, reason), expected,
                               sizeof(expected))
         << "\"";
}
// CertFromPEM parses the given, NUL-terminated pem block and returns an
// |X509*|.
bssl::UniquePtr<X509> CertFromPEM(const char *pem) {
  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(pem, strlen(pem)));
  if (!bio) {
    return nullptr;
  }
  return bssl::UniquePtr<X509>(
      PEM_read_bio_X509(bio.get(), nullptr, nullptr, nullptr));
}

bssl::UniquePtr<RSA> RSAFromPEM(const char *pem) {
  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(pem, strlen(pem)));
  if (!bio) {
    return nullptr;
  }
  return bssl::UniquePtr<RSA>(
      PEM_read_bio_RSAPrivateKey(bio.get(), nullptr, nullptr, nullptr));
}

bssl::UniquePtr<X509> MakeTestCert(const char *issuer,
                                          const char *subject, EVP_PKEY *key,
                                          bool is_ca) {
  bssl::UniquePtr<X509> cert(X509_new());
  if (!cert ||  //
      !X509_set_version(cert.get(), X509_VERSION_3) ||
      !X509_NAME_add_entry_by_txt(
          X509_get_issuer_name(cert.get()), "CN", MBSTRING_UTF8,
          reinterpret_cast<const uint8_t *>(issuer), -1, -1, 0) ||
      !X509_NAME_add_entry_by_txt(
          X509_get_subject_name(cert.get()), "CN", MBSTRING_UTF8,
          reinterpret_cast<const uint8_t *>(subject), -1, -1, 0) ||
      !X509_set_pubkey(cert.get(), key) ||
      !ASN1_TIME_adj(X509_getm_notBefore(cert.get()), kReferenceTime, -1, 0) ||
      !ASN1_TIME_adj(X509_getm_notAfter(cert.get()), kReferenceTime, 1, 0)) {
    return nullptr;
  }
  bssl::UniquePtr<BASIC_CONSTRAINTS> bc(BASIC_CONSTRAINTS_new());
  if (!bc) {
    return nullptr;
  }
  bc->ca = is_ca ? ASN1_BOOLEAN_TRUE : ASN1_BOOLEAN_FALSE;
  if (!X509_add1_ext_i2d(cert.get(), NID_basic_constraints, bc.get(),
                         /*crit=*/1, /*flags=*/0)) {
    return nullptr;
  }
  return cert;
}

bssl::UniquePtr<STACK_OF(X509)> CertsToStack(
    const std::vector<X509 *> &certs) {
  bssl::UniquePtr<STACK_OF(X509)> stack(sk_X509_new_null());
  if (!stack) {
    return nullptr;
  }
  for (auto cert : certs) {
    if (!bssl::PushToStack(stack.get(), bssl::UpRef(cert))) {
      return nullptr;
    }
  }
  return stack;
}

bool PEM_to_DER(const char *pem_str, uint8_t **out_der, long *out_der_len) {
  char *name = nullptr;
  char *header = nullptr;

  // Create BIO from memory
  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(pem_str, strlen(pem_str)));
  if (!bio) {
    return false;
  }

  // Read PEM into DER
  if (PEM_read_bio(bio.get(), &name, &header, out_der, out_der_len) <= 0) {
    OPENSSL_free(name);
    OPENSSL_free(header);
    OPENSSL_free(*out_der);
    *out_der = nullptr;
    return false;
  }

  OPENSSL_free(name);
  OPENSSL_free(header);
  return true;
}

#if defined(OPENSSL_WINDOWS)
// GetTempPathA falls back to the Windows directory (e.g. C:\Windows\) when the
// TMP, TEMP, and USERPROFILE environment variables are all unset. This commonly
// happens when running as SYSTEM in Docker containers or CI agents. The Windows
// directory has special protections that cause file rename operations to fail
// intermittently. Detect this case and redirect to C:\Windows\Temp\ instead.
static DWORD GetSafeTempPathA(DWORD nBufferLength, LPSTR lpBuffer) {
  DWORD ret = GetTempPathA(nBufferLength, lpBuffer);
  if (ret == 0 || ret >= nBufferLength) {
    return ret;
  }
  char win_dir[PATH_MAX];
  UINT win_len = GetWindowsDirectoryA(win_dir, sizeof(win_dir));
  if (win_len == 0 || win_len >= sizeof(win_dir)) {
    return ret;
  }
  // Append trailing backslash to match GetTempPathA's format for comparison.
  if (win_len + 1 >= sizeof(win_dir)) {
    return ret;
  }
  win_dir[win_len] = '\\';
  win_dir[win_len + 1] = '\0';
  if (_stricmp(lpBuffer, win_dir) == 0) {
    int written = snprintf(lpBuffer, nBufferLength, "%sTemp\\", win_dir);
    if (written < 0 || (DWORD)written >= nBufferLength) {
      return 0;
    }
    ret = (DWORD)written;
  }
  return ret;
}

size_t createTempFILEpath(char buffer[PATH_MAX]) {
  // On Windows, tmpfile() may attempt to create temp files in the root
  // directory of the drive, which requires Admin privileges, resulting in test
  // failure.
  //
  // We deliberately avoid GetTempFileNameA for unique-name generation: it
  // silently truncates the name prefix to 3 characters and, when uUnique is 0,
  // combines that prefix with a 16-bit time-derived value. That gives only
  // 65,536 possible filenames, and the empty stub file it creates on disk
  // persists. In long CI runs (e.g. the Windows SDE job, which executes the
  // full gtest binary multiple times) many tests accumulate "aws????.tmp"
  // files in the shared temp directory. Once the namespace is crowded, the
  // internal collision-retry loop inside GetTempFileNameA can fail to find a
  // free name and return 0, producing intermittent test failures.
  //
  // Instead, mirror createTempDirPath: generate a 64-bit random suffix with
  // RAND_bytes and create the file atomically with CREATE_NEW so that any
  // collision with a concurrent caller is detected and retried.
  char temp_path[PATH_MAX];
  if (0 == GetSafeTempPathA(PATH_MAX, temp_path)) {
    return 0;
  }

  static const int kMaxAttempts = 10;
  for (int attempt = 0; attempt < kMaxAttempts; attempt++) {
    union {
      uint8_t bytes[8];
      uint64_t value;
    } random_bytes;
    if (!RAND_bytes(random_bytes.bytes, sizeof(random_bytes.bytes))) {
      return 0;
    }

    int written = snprintf(buffer, PATH_MAX, "%sawslctest_%" PRIX64 ".tmp",
                           temp_path, random_bytes.value);
    // Check for truncation of the path.
    if (written < 0 || written >= PATH_MAX) {
      return 0;
    }

    // CREATE_NEW atomically fails with ERROR_FILE_EXISTS if the file already
    // exists, so we never race with another caller that picked the same name.
    HANDLE h = CreateFileA(buffer, GENERIC_WRITE, 0, NULL, CREATE_NEW,
                           FILE_ATTRIBUTE_NORMAL, NULL);
    if (h != INVALID_HANDLE_VALUE) {
      CloseHandle(h);
      return (size_t)written;
    }
    if (GetLastError() != ERROR_FILE_EXISTS) {
      return 0;
    }
  }
  return 0;
}

size_t createTempDirPath(char buffer[PATH_MAX]) {
  char temp_path[PATH_MAX];
  union {
    uint8_t bytes[8];
    uint64_t value;
  } random_bytes;

  // Get the temporary path
  if (0 == GetSafeTempPathA(PATH_MAX, temp_path)) {
    return 0;
  }

  if (!RAND_bytes(random_bytes.bytes, sizeof(random_bytes.bytes))) {
    return 0;
  }

  int written = snprintf(buffer, PATH_MAX, "%s\\awslctest_%" PRIX64, temp_path, random_bytes.value);

  // Check for truncation of dirname
  if (written < 0 || written >= PATH_MAX) {
    return 0;
  }

  if (!CreateDirectoryA(buffer, NULL)) {
    return 0;
  }

  return (size_t)written;
}

FILE* createRawTempFILE() {
  char filename[PATH_MAX];
  if(createTempFILEpath(filename) == 0) {
    return nullptr;
  }
  return fopen(filename, "w+b");
}

testing::AssertionResult WaitForFileAccessible(const char *path) {
  // On Windows, antivirus software, file indexing services, or other
  // background processes can briefly lock files after they are written,
  // causing transient ERROR_SHARING_VIOLATION failures when callers
  // immediately try to reopen the file for reading. Retry opening the file
  // with a short delay to wait out the lock. These values mirror the retry
  // strategy used by WIN32_rename in tool-openssl/ca.cc.
  //
  // We use CreateFileA with GENERIC_READ rather than fopen(): GetLastError()
  // is only contractually reliable after a direct Win32 API call, and the
  // MSVC CRT may clobber it during fopen()'s internal cleanup path. The
  // FILE_SHARE flags ensure the probe does not itself introduce a lock that
  // would interfere with the caller's subsequent open.
  static const int kMaxRetries = 10;
  static const DWORD kRetryDelayMs = 200;
  for (int attempt = 0; attempt <= kMaxRetries; attempt++) {
    if (attempt > 0) {
      Sleep(kRetryDelayMs);
    }
    HANDLE h = CreateFileA(path, GENERIC_READ,
                           FILE_SHARE_READ | FILE_SHARE_WRITE |
                               FILE_SHARE_DELETE,
                           NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (h != INVALID_HANDLE_VALUE) {
      CloseHandle(h);
      return testing::AssertionSuccess();
    }
    DWORD err = GetLastError();
    // ERROR_ACCESS_DENIED is deliberately retried alongside the obvious
    // sharing/lock violations: on Windows it can manifest transiently from
    // pending-deletion state, AV scans, or the Search Indexer briefly holding
    // the file. If the permission failure is genuine, the retries will all
    // fail identically and the test fails correctly after the retry budget.
    if (err != ERROR_ACCESS_DENIED && err != ERROR_SHARING_VIOLATION &&
        err != ERROR_LOCK_VIOLATION) {
      break;
    }
  }
  return testing::AssertionFailure()
         << "File not accessible after retries: " << path;
}

#elif defined(OPENSSL_WASM_WASI)
// WASI doesn't have mkstemp, mkdtemp, or tmpfile. Use counter-based naming
// with random suffix for uniqueness.
#include <cstdlib>
#include <unistd.h>
#include <openssl/rand.h>

size_t createTempFILEpath(char buffer[PATH_MAX]) {
  static int temp_counter = 0;
  uint32_t random_val = 0;
  RAND_bytes(reinterpret_cast<uint8_t*>(&random_val), sizeof(random_val));
  int written = snprintf(buffer, PATH_MAX, "awslctest_%d_%08x.tmp",
                         temp_counter++, random_val);
  if (written < 0 || written >= PATH_MAX) {
    return 0;
  }
  // Create the file
  FILE *f = fopen(buffer, "w");
  if (f == NULL) {
    return 0;
  }
  fclose(f);
  return strnlen(buffer, PATH_MAX);
}

size_t createTempDirPath(char buffer[PATH_MAX]) {
  static int dir_counter = 0;
  uint32_t random_val = 0;
  RAND_bytes(reinterpret_cast<uint8_t*>(&random_val), sizeof(random_val));
  int written = snprintf(buffer, PATH_MAX, "awslctest_dir_%d_%08x",
                         dir_counter++, random_val);
  if (written < 0 || written >= PATH_MAX) {
    return 0;
  }
  // WASI supports mkdir
  if (mkdir(buffer, 0700) != 0) {
    return 0;
  }
  return strnlen(buffer, PATH_MAX);
}

FILE* createRawTempFILE() {
  char buffer[PATH_MAX];
  if (createTempFILEpath(buffer) == 0) {
    return nullptr;
  }
  return fopen(buffer, "w+b");
}
#else
#include <cstdlib>
#include <unistd.h>
size_t createTempFILEpath(char buffer[PATH_MAX]) {
  snprintf(buffer, PATH_MAX, "awslcTestTmpFileXXXXXX");

  int fd = mkstemp(buffer);
  if (fd == -1) {
    return 0;
  }

  close(fd);
  return strnlen(buffer, PATH_MAX);
}

size_t createTempDirPath(char buffer[PATH_MAX]) {
  snprintf(buffer, PATH_MAX, "/tmp/awslcTestDirXXXXXX");
  if (mkdtemp(buffer) == NULL) {
    return 0;
  }
  return strnlen(buffer, PATH_MAX);
}

FILE* createRawTempFILE() {
  return tmpfile();
}
#endif


TempFILE createTempFILE() {
  return TempFILE(createRawTempFILE());
}

void CustomDataFree(void *parent, void *ptr, CRYPTO_EX_DATA *ad,
                           int index, long argl, void *argp) {
  free(ptr);
}

bool osIsAmazonLinux(void) {
  bool res = false;
#if defined(OPENSSL_LINUX)
  // Per https://docs.aws.amazon.com/linux/al2023/ug/naming-and-versioning.html.
  std::ifstream amazonLinuxSpecificFile("/etc/amazon-linux-release-cpe");
  if (amazonLinuxSpecificFile.is_open()) {
    // Definitely on Amazon Linux.
    amazonLinuxSpecificFile.close();
    return true;
  }

  // /etc/amazon-linux-release-cpe was introduced in AL2023. For earlier, parse
  // and read /etc/system-release-cpe.
  std::ifstream osRelease("/etc/system-release-cpe");
  if (!osRelease.is_open()) {
    return false;
  }

  std::string line;
  while (std::getline(osRelease, line)) {
    // AL2:
    // $ cat /etc/system-release-cpe
    // cpe:2.3:o:amazon:amazon_linux:2
    //
    // AL2023:
    // $ cat /etc/system-release-cpe
    // cpe:2.3:o:amazon:amazon_linux:2023
    if (line.find("amazon") != std::string::npos) {
      res = true;
    } else if (line.find("amazon_linux") != std::string::npos) {
      res = true;
    }
  }
  osRelease.close();
#endif
  return res;
}

bool threadTest(const size_t numberOfThreads, std::function<void(bool*)> testFunc) {
  bool res = true;

#if defined(OPENSSL_THREADS)
  // char to be able to pass-as-reference.
  std::vector<char> retValueVec(numberOfThreads, 0);
  std::vector<std::thread> threadVec;

  for (size_t i = 0; i < numberOfThreads; i++) {
    threadVec.emplace_back(testFunc, reinterpret_cast<bool*>(&retValueVec[i]));
  }

  for (auto& thread : threadVec) {
    thread.join();
  }

  for (size_t i = 0; i < numberOfThreads; i++) {
    if (!static_cast<bool>(retValueVec[i])) {
      fprintf(stderr, "Thread %lu failed\n", (long unsigned int) i);
      res = false;
    }
  }

#else
  testFunc(&res);
#endif

  return res;
}

bool forkAndRunTest(std::function<bool()> child_func,
  std::function<bool()> parent_func) {

#if defined(OPENSSL_WINDOWS) || defined(OPENSSL_WASM_WASI)
  // fork() is not supported on Windows or WASI.
  return false;
#else
  pid_t pid = fork();
  if (pid == 0) { // Child
    bool success = child_func();
    exit(success ? 0 : 1);
  } else if (pid > 0) { // Parent
    bool parent_success = parent_func();
    int status;
    waitpid(pid, &status, 0);
    return parent_success && WIFEXITED(status) && WEXITSTATUS(status) == 0;
  }

  // Fork failed
  return false;
#endif
}

void maybeDisableSomeForkUbeDetectMechanisms(void) {
  if (getenv("AWSLC_IGNORE_FORK_UBE_DETECTION")) {
    CRYPTO_fork_detect_ignore_wipeonfork_FOR_TESTING();
    CRYPTO_fork_detect_ignore_inheritzero_FOR_TESTING();
  }
}

bool runtimeEmulationIsIntelSde(void) {
  if (getenv("RUNTIME_EMULATION_SDE")) {
    return true;
  }
  return false;
}

bool addressSanitizerIsEnabled(void) {
#if defined(OPENSSL_ASAN)
  return true;
#else
  return false;
#endif
}

bssl::UniquePtr<BIGNUM> HexToBIGNUM(const char *hex) {
  BIGNUM *bn = nullptr;
  BN_hex2bn(&bn, hex);
  return bssl::UniquePtr<BIGNUM>(bn);
}

std::string BIGNUMToHex(const BIGNUM *bn) {
  bssl::UniquePtr<char> hex(BN_bn2hex(bn));
  if (hex == nullptr) {
    return "error";
  }
  return hex.get();
}
