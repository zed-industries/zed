#ifndef FILESYSTEM_TEST_HELPER_H
#define FILESYSTEM_TEST_HELPER_H

#include <filesystem>

#include <sys/stat.h> // for stat, mkdir, mkfifo
#ifndef _WIN32
#include <unistd.h> // for ftruncate, link, symlink, getcwd, chdir
#include <sys/statvfs.h>
#else
#include <io.h>
#include <direct.h>
#include <windows.h> // for CreateSymbolicLink, CreateHardLink
#endif

#include <cassert>
#include <cerrno>
#include <chrono>
#include <cstdint>
#include <cstdio> // for printf
#include <cstring>
#include <string>
#include <system_error>
#include <type_traits>
#include <vector>

#include "assert_macros.h"
#include "make_string.h"
#include "test_macros.h"
#include "format_string.h"

// For creating socket files
#if !defined(__FreeBSD__) && !defined(__APPLE__) && !defined(_WIN32)
# include <sys/socket.h>
# include <sys/un.h>
#endif
namespace fs = std::filesystem;

namespace utils {
#ifdef _WIN32
    inline int mkdir(const char* path, int mode) { (void)mode; return ::_mkdir(path); }
    inline int symlink(const char* oldname, const char* newname, bool is_dir) {
        DWORD flags = is_dir ? SYMBOLIC_LINK_FLAG_DIRECTORY : 0;
        if (CreateSymbolicLinkA(newname, oldname,
                                flags | SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE))
          return 0;
        if (GetLastError() != ERROR_INVALID_PARAMETER)
          return 1;
        return !CreateSymbolicLinkA(newname, oldname, flags);
    }
    inline int link(const char *oldname, const char* newname) {
        return !CreateHardLinkA(newname, oldname, NULL);
    }
    inline int setenv(const char *var, const char *val, int overwrite) {
        (void)overwrite;
        return ::_putenv((std::string(var) + "=" + std::string(val)).c_str());
    }
    inline int unsetenv(const char *var) {
        return ::_putenv((std::string(var) + "=").c_str());
    }
    inline bool space(std::string path, std::uintmax_t &capacity,
                      std::uintmax_t &free, std::uintmax_t &avail) {
        ULARGE_INTEGER FreeBytesAvailableToCaller, TotalNumberOfBytes,
                       TotalNumberOfFreeBytes;
        if (!GetDiskFreeSpaceExA(path.c_str(), &FreeBytesAvailableToCaller,
                                 &TotalNumberOfBytes, &TotalNumberOfFreeBytes))
          return false;
        capacity = TotalNumberOfBytes.QuadPart;
        free = TotalNumberOfFreeBytes.QuadPart;
        avail = FreeBytesAvailableToCaller.QuadPart;
        assert(capacity > 0);
        assert(free > 0);
        assert(avail > 0);
        return true;
    }
#else
    using ::mkdir;
    inline int symlink(const char* oldname, const char* newname, bool is_dir) { (void)is_dir; return ::symlink(oldname, newname); }
    using ::link;
    using ::setenv;
    using ::unsetenv;
    inline bool space(std::string path, std::uintmax_t &capacity,
                      std::uintmax_t &free, std::uintmax_t &avail) {
        struct statvfs expect;
        if (::statvfs(path.c_str(), &expect) == -1)
          return false;
        assert(expect.f_bavail > 0);
        assert(expect.f_bfree > 0);
        assert(expect.f_bsize > 0);
        assert(expect.f_blocks > 0);
        assert(expect.f_frsize > 0);
        auto do_mult = [&](std::uintmax_t val) {
            std::uintmax_t fsize = expect.f_frsize;
            std::uintmax_t new_val = val * fsize;
            assert(new_val / fsize == val); // Test for overflow
            return new_val;
        };
        capacity = do_mult(expect.f_blocks);
        free = do_mult(expect.f_bfree);
        avail = do_mult(expect.f_bavail);
        return true;
    }
#endif

    // N.B. libc might define some of the foo[64] identifiers using macros from
    // foo64 -> foo or vice versa.
#if defined(_WIN32)
    using off64_t = std::int64_t;
#elif defined(__MVS__) || defined(__LP64__)
    using off64_t = ::off_t;
#else
    using ::off64_t;
#endif

    inline FILE* fopen64(const char* pathname, const char* mode) {
        // Bionic does not distinguish between fopen and fopen64, but fopen64
        // wasn't added until API 24.
#if defined(_WIN32) || defined(__MVS__) || defined(__LP64__) || defined(__BIONIC__)
        return ::fopen(pathname, mode);
#else
        return ::fopen64(pathname, mode);
#endif
    }

    inline int ftruncate64(int fd, off64_t length) {
#if defined(_WIN32)
        // _chsize_s sets errno on failure and also returns the error number.
        return ::_chsize_s(fd, length) ? -1 : 0;
#elif defined(__MVS__) || defined(__LP64__)
        return ::ftruncate(fd, length);
#else
        return ::ftruncate64(fd, length);
#endif
    }

    inline std::string getcwd() {
        // Assume that path lengths are not greater than this.
        // This should be fine for testing purposes.
        char buf[4096];
        char* ret = ::getcwd(buf, sizeof(buf));
        assert(ret && "getcwd failed");
        return std::string(ret);
    }

    inline bool exists(std::string const& path) {
        struct ::stat tmp;
        return ::stat(path.c_str(), &tmp) == 0;
    }
} // namespace utils

struct scoped_test_env
{
    scoped_test_env() : test_root(available_cwd_path()) {
#ifdef _WIN32
        // Windows mkdir can create multiple recursive directories
        // if needed.
        std::string cmd = "mkdir " + test_root.string();
#else
        std::string cmd = "mkdir -p " + test_root.string();
#endif
        int ret = std::system(cmd.c_str());
        assert(ret == 0);

        // Ensure that the root_path is fully resolved, i.e. it contains no
        // symlinks. The filesystem tests depend on that. We do this after
        // creating the root_path, because `fs::canonical` requires the
        // path to exist.
        test_root = fs::canonical(test_root);
    }

    ~scoped_test_env() {
#ifdef _WIN32
        std::string cmd = "rmdir /s /q " + test_root.string();
        int ret = std::system(cmd.c_str());
        assert(ret == 0);
#else
#if defined(__MVS__)
        // The behaviour of chmod -R on z/OS prevents recursive
        // permission change for directories that do not have read permission.
        std::string cmd = "find  " + test_root.string() + " -exec chmod 777 {} \\;";
#else
        std::string cmd = "chmod -R 777 " + test_root.string();
#endif // defined(__MVS__)
        int ret = std::system(cmd.c_str());
#  if !defined(_AIX) && !defined(__ANDROID__)
        // On AIX the chmod command will return non-zero when trying to set
        // the permissions on a directory that contains a bad symlink. This triggers
        // the assert, despite being able to delete everything with the following
        // `rm -r` command.
        //
        // Android's chmod was buggy in old OSs, but skipping this assert is
        // sufficient to ensure that the `rm -rf` succeeds for almost all tests:
        //  - Android L: chmod aborts after one error
        //  - Android L and M: chmod -R tries to set permissions of a symlink
        //    target.
        // LIBCXX-ANDROID-FIXME: Other fixes to consider: place a toybox chmod
        // onto old devices, re-enable this assert for devices running Android N
        // and up, rewrite this chmod+rm in C or C++.
        assert(ret == 0);
#  endif

        cmd = "rm -rf " + test_root.string();
        ret = std::system(cmd.c_str());
        assert(ret == 0);
#endif
    }

    scoped_test_env(scoped_test_env const &) = delete;
    scoped_test_env & operator=(scoped_test_env const &) = delete;

    fs::path make_env_path(std::string p) { return sanitize_path(p); }

    std::string sanitize_path(std::string raw) {
        assert(raw.find("..") == std::string::npos);
        std::string root = test_root.string();
        if (root.compare(0, root.size(), raw, 0, root.size()) != 0) {
            assert(raw.front() != '\\');
            fs::path tmp(test_root);
            tmp /= raw;
            return tmp.string();
        }
        return raw;
    }

    // Purposefully using a size potentially larger than off_t here so we can
    // test the behavior of libc++fs when it is built with _FILE_OFFSET_BITS=64
    // but the caller is not (std::filesystem also uses uintmax_t rather than
    // off_t). On a 32-bit system this allows us to create a file larger than
    // 2GB.
    std::string create_file(fs::path filename_path, std::uintmax_t size = 0) {
        std::string filename = sanitize_path(filename_path.string());

        if (size >
            static_cast<typename std::make_unsigned<utils::off64_t>::type>(
                std::numeric_limits<utils::off64_t>::max())) {
            std::fprintf(stderr, "create_file(%s, %ju) too large\n",
                         filename.c_str(), size);
            std::abort();
        }

#if defined(_WIN32) || defined(__MVS__)
#  define FOPEN_CLOEXEC_FLAG ""
#else
#  define FOPEN_CLOEXEC_FLAG "e"
#endif
        FILE* file = utils::fopen64(filename.c_str(), "w" FOPEN_CLOEXEC_FLAG);
        if (file == nullptr) {
            std::fprintf(stderr, "fopen %s failed: %s\n", filename.c_str(),
                         std::strerror(errno));
            std::abort();
        }

        if (utils::ftruncate64(
                fileno(file), static_cast<utils::off64_t>(size)) == -1) {
            std::fprintf(stderr, "ftruncate %s %ju failed: %s\n", filename.c_str(),
                         size, std::strerror(errno));
            std::fclose(file);
            std::abort();
        }

        std::fclose(file);
        return filename;
    }

    std::string create_dir(fs::path filename_path) {
        std::string filename = filename_path.string();
        filename = sanitize_path(std::move(filename));
        int ret = utils::mkdir(filename.c_str(), 0777); // rwxrwxrwx mode
        assert(ret == 0);
        return filename;
    }

    std::string create_file_dir_symlink(fs::path source_path,
                                        fs::path to_path,
                                        bool sanitize_source = true,
                                        bool is_dir = false) {
        std::string source = source_path.string();
        std::string to = to_path.string();
        if (sanitize_source)
            source = sanitize_path(std::move(source));
        to = sanitize_path(std::move(to));
        int ret = utils::symlink(source.c_str(), to.c_str(), is_dir);
        assert(ret == 0);
        return to;
    }

    std::string create_symlink(fs::path source_path,
                               fs::path to_path,
                               bool sanitize_source = true) {
        return create_file_dir_symlink(source_path, to_path, sanitize_source,
                                       false);
    }

    std::string create_directory_symlink(fs::path source_path,
                                         fs::path to_path,
                                         bool sanitize_source = true) {
        return create_file_dir_symlink(source_path, to_path, sanitize_source,
                                       true);
    }

    std::string create_hardlink(fs::path source_path, fs::path to_path) {
        std::string source = source_path.string();
        std::string to = to_path.string();
        source = sanitize_path(std::move(source));
        to = sanitize_path(std::move(to));
        int ret = utils::link(source.c_str(), to.c_str());
        assert(ret == 0);
        return to;
    }

#ifndef _WIN32
    std::string create_fifo(std::string file) {
        file = sanitize_path(std::move(file));
        int ret = ::mkfifo(file.c_str(), 0666); // rw-rw-rw- mode
        assert(ret == 0);
        return file;
    }
#endif

  // Some platforms doesn't support socket files so we shouldn't even
  // allow tests to call this unguarded.
#if !defined(__FreeBSD__) && !defined(__APPLE__) && !defined(_WIN32)
    std::string create_socket(std::string file) {
      file = sanitize_path(std::move(file));

      ::sockaddr_un address;
      address.sun_family = AF_UNIX;

// If file.size() is too big, try to create a file directly inside
// /tmp to make sure file path is short enough.
// Android platform warns about tmpnam, since the problem does not appear
// on Android, let's not apply it for Android.
#  if !defined(__ANDROID__)
    if (file.size() > sizeof(address.sun_path)) {
      file = std::tmpnam(nullptr);
    }
#  endif
    assert(file.size() <= sizeof(address.sun_path));
    ::strncpy(address.sun_path, file.c_str(), sizeof(address.sun_path));
    int fd = ::socket(AF_UNIX, SOCK_STREAM, 0);
    assert(::bind(fd, reinterpret_cast<::sockaddr*>(&address), sizeof(address)) == 0);
    return file;
  }
#endif

    fs::path test_root;

private:
    // This could potentially introduce a filesystem race if multiple
    // scoped_test_envs were created concurrently in the same test (hence
    // sharing the same cwd). However, it is fairly unlikely to happen as
    // we generally don't use scoped_test_env from multiple threads, so
    // this is deemed acceptable.
    // The cwd.filename() itself isn't unique across all tests in the suite,
    // so start the numbering from a hash of the full cwd, to avoid
    // different tests interfering with each other.
    static inline fs::path available_cwd_path() {
        fs::path const cwd = utils::getcwd();
        fs::path const tmp = fs::temp_directory_path();
        std::string base = cwd.filename().string();
        std::size_t i = std::hash<std::string>()(cwd.string());
        fs::path p = tmp / (base + "-static_env." + std::to_string(i));
        while (utils::exists(p.string())) {
            p = tmp / (base + "-static_env." + std::to_string(++i));
        }
        return p;
    }
};

/// This class generates the following tree:
///
///     static_test_env
///     |-- bad_symlink -> dne
///     |-- dir1
///     |   |-- dir2
///     |   |   |-- afile3
///     |   |   |-- dir3
///     |   |   |   `-- file5
///     |   |   |-- file4
///     |   |   `-- symlink_to_dir3 -> dir3
///     |   `-- file1
///     |   `-- file2
///     |-- empty_file
///     |-- non_empty_file
///     |-- symlink_to_dir -> dir1
///     `-- symlink_to_empty_file -> empty_file
///
class static_test_env {
    scoped_test_env env_;
public:
    static_test_env() {
        env_.create_symlink("dne", "bad_symlink", false);
        env_.create_dir("dir1");
        env_.create_dir("dir1/dir2");
        env_.create_file("dir1/dir2/afile3");
        env_.create_dir("dir1/dir2/dir3");
        env_.create_file("dir1/dir2/dir3/file5");
        env_.create_file("dir1/dir2/file4");
        env_.create_directory_symlink("dir3", "dir1/dir2/symlink_to_dir3", false);
        env_.create_file("dir1/file1");
        env_.create_file("dir1/file2", 42);
        env_.create_file("empty_file");
        env_.create_file("non_empty_file", 42);
        env_.create_directory_symlink("dir1", "symlink_to_dir", false);
        env_.create_symlink("empty_file", "symlink_to_empty_file", false);
    }

    const fs::path Root = env_.test_root;

    fs::path makePath(fs::path const& p) const {
        // env_path is expected not to contain symlinks.
        fs::path const& env_path = Root;
        return env_path / p;
    }

    const std::vector<fs::path> TestFileList = {
        makePath("empty_file"),
        makePath("non_empty_file"),
        makePath("dir1/file1"),
        makePath("dir1/file2")
    };

    const std::vector<fs::path> TestDirList = {
        makePath("dir1"),
        makePath("dir1/dir2"),
        makePath("dir1/dir2/dir3")
    };

    const fs::path File          = TestFileList[0];
    const fs::path Dir           = TestDirList[0];
    const fs::path Dir2          = TestDirList[1];
    const fs::path Dir3          = TestDirList[2];
    const fs::path SymlinkToFile = makePath("symlink_to_empty_file");
    const fs::path SymlinkToDir  = makePath("symlink_to_dir");
    const fs::path BadSymlink    = makePath("bad_symlink");
    const fs::path DNE           = makePath("DNE");
    const fs::path EmptyFile     = TestFileList[0];
    const fs::path NonEmptyFile  = TestFileList[1];
    const fs::path CharFile      = "/dev/null"; // Hopefully this exists

    const std::vector<fs::path> DirIterationList = {
        makePath("dir1/dir2"),
        makePath("dir1/file1"),
        makePath("dir1/file2")
    };

    const std::vector<fs::path> DirIterationListDepth1 = {
        makePath("dir1/dir2/afile3"),
        makePath("dir1/dir2/dir3"),
        makePath("dir1/dir2/symlink_to_dir3"),
        makePath("dir1/dir2/file4"),
    };

    const std::vector<fs::path> RecDirIterationList = {
        makePath("dir1/dir2"),
        makePath("dir1/file1"),
        makePath("dir1/file2"),
        makePath("dir1/dir2/afile3"),
        makePath("dir1/dir2/dir3"),
        makePath("dir1/dir2/symlink_to_dir3"),
        makePath("dir1/dir2/file4"),
        makePath("dir1/dir2/dir3/file5")
    };

    const std::vector<fs::path> RecDirFollowSymlinksIterationList = {
        makePath("dir1/dir2"),
        makePath("dir1/file1"),
        makePath("dir1/file2"),
        makePath("dir1/dir2/afile3"),
        makePath("dir1/dir2/dir3"),
        makePath("dir1/dir2/file4"),
        makePath("dir1/dir2/dir3/file5"),
        makePath("dir1/dir2/symlink_to_dir3"),
        makePath("dir1/dir2/symlink_to_dir3/file5"),
    };
};

struct CWDGuard {
  std::string oldCwd_;
  CWDGuard() : oldCwd_(utils::getcwd()) { }
  ~CWDGuard() {
    int ret = ::chdir(oldCwd_.c_str());
    assert(ret == 0 && "chdir failed");
  }

  CWDGuard(CWDGuard const&) = delete;
  CWDGuard& operator=(CWDGuard const&) = delete;
};

// We often need to test that the error_code was cleared if no error occurs
// this function returns an error_code which is set to an error that will
// never be returned by the filesystem functions.
inline std::error_code GetTestEC(unsigned Idx = 0) {
  using std::errc;
  auto GetErrc = [&]() {
    switch (Idx) {
    case 0:
      return errc::address_family_not_supported;
    case 1:
      return errc::address_not_available;
    case 2:
      return errc::address_in_use;
    case 3:
      return errc::argument_list_too_long;
    default:
      assert(false && "Idx out of range");
      std::abort();
    }
  };
  return std::make_error_code(GetErrc());
}

inline bool ErrorIsImp(const std::error_code& ec,
                       std::vector<std::errc> const& errors) {
  std::error_condition cond = ec.default_error_condition();
  for (auto errc : errors) {
    if (cond.value() == static_cast<int>(errc))
      return true;
  }
  return false;
}

template <class... ErrcT>
inline bool ErrorIs(const std::error_code& ec, std::errc First, ErrcT... Rest) {
  std::vector<std::errc> errors = {First, Rest...};
  return ErrorIsImp(ec, errors);
}

// Provide our own Sleep routine since std::this_thread::sleep_for is not
// available in single-threaded mode.
template <class Dur> void SleepFor(Dur dur) {
    using namespace std::chrono;
#if !_LIBCPP_HAS_MONOTONIC_CLOCK
    using Clock = system_clock;
#else
    using Clock = steady_clock;
#endif
    const auto wake_time = Clock::now() + dur;
    while (Clock::now() < wake_time)
        ;
}

inline fs::perms NormalizeExpectedPerms(fs::perms P) {
#ifdef _WIN32
  // On Windows, fs::perms only maps down to one bit stored in the filesystem,
  // a boolean readonly flag.
  // Normalize permissions to the format it gets returned; all fs entries are
  // read+exec for all users; writable ones also have the write bit set for
  // all users.
  P |= fs::perms::owner_read | fs::perms::group_read | fs::perms::others_read;
  P |= fs::perms::owner_exec | fs::perms::group_exec | fs::perms::others_exec;
  fs::perms Write =
      fs::perms::owner_write | fs::perms::group_write | fs::perms::others_write;
  if ((P & Write) != fs::perms::none)
    P |= Write;
#endif
  return P;
}

struct ExceptionChecker {
  std::errc expected_err;
  fs::path expected_path1;
  fs::path expected_path2;
  unsigned num_paths;
  const char* func_name;
  std::string opt_message;

  explicit ExceptionChecker(std::errc first_err, const char* fun_name,
                            std::string opt_msg = {})
      : expected_err{first_err}, num_paths(0), func_name(fun_name),
        opt_message(opt_msg) {}
  explicit ExceptionChecker(fs::path p, std::errc first_err,
                            const char* fun_name, std::string opt_msg = {})
      : expected_err(first_err), expected_path1(p), num_paths(1),
        func_name(fun_name), opt_message(opt_msg) {}

  explicit ExceptionChecker(fs::path p1, fs::path p2, std::errc first_err,
                            const char* fun_name, std::string opt_msg = {})
      : expected_err(first_err), expected_path1(p1), expected_path2(p2),
        num_paths(2), func_name(fun_name), opt_message(opt_msg) {}

  void operator()(fs::filesystem_error const& Err) {
    assert(ErrorIsImp(Err.code(), {expected_err}));
    assert(Err.path1() == expected_path1);
    assert(Err.path2() == expected_path2);
#ifndef _WIN32
    // On Windows, the error strings are windows error code strings, and don't
    // match textually with the strings generated for generic std::errc::*.
    LIBCPP_ONLY(check_libcxx_string(Err));
#endif
  }

  void check_libcxx_string(fs::filesystem_error const& Err) {
    std::string message = std::make_error_code(expected_err).message();

    std::string additional_msg = "";
    if (!opt_message.empty()) {
      additional_msg = opt_message + ": ";
    }
    auto transform_path = [](const fs::path& p) {
      return "\"" + p.string() + "\"";
    };
    std::string format = [&]() -> std::string {
      switch (num_paths) {
      case 0:
        return format_string("filesystem error: in %s: %s%s", func_name,
                             additional_msg, message);
      case 1:
        return format_string("filesystem error: in %s: %s%s [%s]", func_name,
                             additional_msg, message,
                             transform_path(expected_path1).c_str());
      case 2:
        return format_string("filesystem error: in %s: %s%s [%s] [%s]",
                             func_name, additional_msg, message,
                             transform_path(expected_path1).c_str(),
                             transform_path(expected_path2).c_str());
      default:
        TEST_FAIL("unexpected case");
        return "";
      }
    }();
    assert(format == Err.what());
    if (format != Err.what()) {
      std::fprintf(stderr, "filesystem_error::what() does not match expected output:\n");
      std::fprintf(stderr, "  expected: \"%s\"\n", format.c_str());
      std::fprintf(stderr, "  actual:   \"%s\"\n\n", Err.what());
    }
  }

  ExceptionChecker(ExceptionChecker const&) = delete;
  ExceptionChecker& operator=(ExceptionChecker const&) = delete;

};

inline fs::path GetWindowsInaccessibleDir() {
  // Only makes sense on windows, but the code can be compiled for
  // any platform.
  const fs::path dir("C:\\System Volume Information");
  std::error_code ec;
  const fs::path root("C:\\");
  for (const auto &ent : fs::directory_iterator(root, ec)) {
    if (ent != dir)
      continue;
    // Basic sanity checks on the directory_entry
    if (!ent.exists() || !ent.is_directory()) {
      std::fprintf(stderr, "The expected inaccessible directory \"%s\" was found "
                           "but doesn't behave as expected, skipping tests "
                           "regarding it\n", dir.string().c_str());
      return fs::path();
    }
    // Check that it indeed is inaccessible as expected
    (void)fs::exists(ent, ec);
    if (!ec) {
      std::fprintf(stderr, "The expected inaccessible directory \"%s\" was found "
                           "but seems to be accessible, skipping tests "
                           "regarding it\n", dir.string().c_str());
      return fs::path();
    }
    return ent;
  }
  std::fprintf(stderr, "No inaccessible directory \"%s\" found, skipping tests "
                       "regarding it\n", dir.string().c_str());
  return fs::path();
}

#endif /* FILESYSTEM_TEST_HELPER_H */
