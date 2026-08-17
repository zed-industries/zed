// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is the Android-specific Chromium dynamic linker (loader of dynamic
// libraries), a tiny shared library implementing a custom dynamic linker that
// can be used to load the real Chromium libraries.
//
// The purpose of this custom linker is to be able to share the RELRO section of
// libcontentshell.so (or equivalent) between the browser process and all other
// processes it asks to create.
//
// This source code *cannot* depend on anything from //base or the C++ standard
// library to keep this DSO small and avoid dependency issues. An exception is
// made for std::unique_ptr as a risky header-only definition.

#ifndef BASE_ANDROID_LINKER_LINKER_JNI_H_
#define BASE_ANDROID_LINKER_LINKER_JNI_H_

#include <android/log.h>
#include <jni.h>
#include <stdlib.h>

#include "build/build_config.h"
#include "third_party/jni_zero/jni_zero.h"

// Set this to 1 to enable debug traces to the Android log.
// Note that LOG() from "base/logging.h" cannot be used, since it is
// in base/ which hasn't been loaded yet.
#define DEBUG 0

#define TAG "cr_ChromiumAndroidLinker"

#if DEBUG
#define LOG_INFO(FORMAT, ...)                                             \
  __android_log_print(ANDROID_LOG_INFO, TAG, "%s: " FORMAT, __FUNCTION__, \
                      ##__VA_ARGS__)
#else
#define LOG_INFO(FORMAT, ...) ((void)0)
#endif
#define LOG_ERROR(FORMAT, ...)                                             \
  __android_log_print(ANDROID_LOG_ERROR, TAG, "%s: " FORMAT, __FUNCTION__, \
                      ##__VA_ARGS__)
#define PLOG_ERROR(FORMAT, ...) \
  LOG_ERROR(FORMAT ": %s", ##__VA_ARGS__, strerror(errno))

#if defined(__arm__) && defined(__ARM_ARCH_7A__)
#define CURRENT_ABI "armeabi-v7a"
#elif defined(__arm__)
#define CURRENT_ABI "armeabi"
#elif defined(__i386__)
#define CURRENT_ABI "x86"
#elif defined(__mips__)
#define CURRENT_ABI "mips"
#elif defined(__x86_64__)
#define CURRENT_ABI "x86_64"
#elif defined(__aarch64__)
#define CURRENT_ABI "arm64-v8a"
#elif defined(__riscv) && (__riscv_xlen == 64)
#define CURRENT_ABI "riscv64"
#else
#error "Unsupported target abi"
#endif

// Copied from //base/posix/eintr_wrapper.h to avoid depending on //base.
#define HANDLE_EINTR(x)                                     \
  ({                                                        \
    decltype(x) eintr_wrapper_result;                       \
    do {                                                    \
      eintr_wrapper_result = (x);                           \
    } while (eintr_wrapper_result == -1 && errno == EINTR); \
    eintr_wrapper_result;                                   \
  })

namespace chromium_android_linker {

// Larger than the largest library we might attempt to load.
static const size_t kAddressSpaceReservationSize = 192 * 1024 * 1024;

// A simple scoped UTF String class that can be initialized from
// a Java jstring handle. Modeled like std::string, which cannot
// be used here.
class String {
 public:
  String(JNIEnv* env, jstring str);

  inline ~String() { ::free(ptr_); }

  inline const char* c_str() const { return ptr_ ? ptr_ : ""; }
  inline size_t size() const { return size_; }

 private:
  char* ptr_;
  size_t size_;
};

inline uintptr_t PageStart(size_t page_size, uintptr_t x) {
  return x & ~(page_size - 1);
}

inline uintptr_t PageEnd(size_t page_size, uintptr_t x) {
  return PageStart(page_size, x + page_size - 1);
}

// Returns true iff casting a java-side |address| to uintptr_t does not lose
// bits.
bool IsValidAddress(jlong address);

// Find the jclass JNI reference corresponding to a given |class_name|.
// |env| is the current JNI environment handle.
// On success, return true and set |*clazz|.
bool InitClassReference(JNIEnv* env, const char* class_name, jclass* clazz);

// Finds the region reserved by the WebView zygote if the current process is
// inherited from the modern enough zygote that has this reservation. If the
// lookup is successful, returns true and sets |address| and |size|. Otherwise
// returns false.
bool FindWebViewReservation(uintptr_t* address, size_t* size);

// Initialize a jfieldID corresponding to the field of a given |clazz|,
// with name |field_name| and signature |field_sig|.
// |env| is the current JNI environment handle.
// On success, return true and set |*field_id|.
bool InitFieldId(JNIEnv* env,
                 jclass clazz,
                 const char* field_name,
                 const char* field_sig,
                 jfieldID* field_id);

// Initialize a jfieldID corresponding to the static field of a given |clazz|,
// with name |field_name| and signature |field_sig|.
// |env| is the current JNI environment handle.
// On success, return true and set |*field_id|.
bool InitStaticFieldId(JNIEnv* env,
                       jclass clazz,
                       const char* field_name,
                       const char* field_sig,
                       jfieldID* field_id);

// A class used to model the field IDs of the org.chromium.base.Linker
// LibInfo inner class, used to communicate data with the Java side
// of the linker.
struct LibInfo_class {
  jfieldID load_address_id;
  jfieldID load_size_id;
  jfieldID relro_start_id;
  jfieldID relro_size_id;
  jfieldID relro_fd_id;

  // Initialize an instance.
  bool Init(JNIEnv* env) {
    jclass clazz;
    if (!InitClassReference(
            env, "org/chromium/base/library_loader/Linker$LibInfo", &clazz)) {
      return false;
    }

    return InitFieldId(env, clazz, "mLoadAddress", "J", &load_address_id) &&
           InitFieldId(env, clazz, "mLoadSize", "J", &load_size_id) &&
           InitFieldId(env, clazz, "mRelroStart", "J", &relro_start_id) &&
           InitFieldId(env, clazz, "mRelroSize", "J", &relro_size_id) &&
           InitFieldId(env, clazz, "mRelroFd", "I", &relro_fd_id);
  }

  void SetLoadInfo(JNIEnv* env,
                   jobject library_info_obj,
                   uintptr_t load_address,
                   size_t load_size) {
    env->SetLongField(library_info_obj, load_address_id, load_address);
    env->SetLongField(library_info_obj, load_size_id, load_size);
  }

  void SetRelroInfo(JNIEnv* env,
                    jobject library_info_obj,
                    uintptr_t relro_start,
                    size_t relro_size,
                    int relro_fd) {
    env->SetLongField(library_info_obj, relro_start_id, relro_start);
    env->SetLongField(library_info_obj, relro_size_id, relro_size);
    env->SetIntField(library_info_obj, relro_fd_id, relro_fd);
  }

  bool GetLoadInfo(JNIEnv* env,
                   jobject library_info_obj,
                   uintptr_t* load_address,
                   size_t* load_size) {
    if (load_address) {
      jlong java_address = env->GetLongField(library_info_obj, load_address_id);
      if (!IsValidAddress(java_address)) {
        return false;
      }
      *load_address = static_cast<uintptr_t>(java_address);
    }
    if (load_size) {
      *load_size = static_cast<uintptr_t>(
          env->GetLongField(library_info_obj, load_size_id));
    }
    return true;
  }

  void GetRelroInfo(JNIEnv* env,
                    jobject library_info_obj,
                    uintptr_t* relro_start,
                    size_t* relro_size,
                    int* relro_fd) {
    if (relro_start) {
      *relro_start = static_cast<uintptr_t>(
          env->GetLongField(library_info_obj, relro_start_id));
    }

    if (relro_size) {
      *relro_size = static_cast<size_t>(
          env->GetLongField(library_info_obj, relro_size_id));
    }

    if (relro_fd) {
      *relro_fd = env->GetIntField(library_info_obj, relro_fd_id);
    }
  }
};

// Used to find out whether RELRO sharing is often rejected due to mismatch of
// the contents.
//
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused. Must be kept in sync with the enum
// in enums.xml. A java @IntDef is generated from this.
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base.library_loader
enum class RelroSharingStatus {
  NOT_ATTEMPTED = 0,
  SHARED = 1,
  NOT_IDENTICAL = 2,
  EXTERNAL_RELRO_FD_NOT_PROVIDED = 3,
  EXTERNAL_RELRO_NOT_FOUND = 4,
  NO_SHMEM_FUNCTIONS = 5,
  REMAP_FAILED = 6,
  CORRUPTED_IN_JAVA = 7,
  EXTERNAL_LOAD_ADDRESS_RESET = 8,
  COUNT = 9,
};

struct SharedMemoryFunctions;

// Holds address ranges of the loaded native library, its RELRO region, along
// with the RELRO FD identifying the shared memory region. Carries the same
// members as the Java-side LibInfo (without mLibFilePath), allowing to
// internally import/export the member values from/to the Java-side counterpart.
//
// Does *not* own the RELRO FD as soon as the latter gets exported to Java
// (as a result of 'spawning' the RELRO region as shared memory.
//
// *Not* threadsafe.
class NativeLibInfo {
 public:
  // Constructs an empty instance. The |java_object| indicates the handle to
  // import and export member fields.
  //
  // Having |env| as |nullptr| disables export to java for the lifetime of the
  // instance. This is useful as a scratch info that is gradually populated for
  // comparison with another NativeLibInfo, and then discarded.
  NativeLibInfo(JNIEnv* env, jobject java_object);

  // Copies the java-side object state to this native instance. Returns false
  // iff an imported value is invalid.
  bool CopyFromJavaObject();

  void set_load_address(uintptr_t a) { load_address_ = a; }

  uintptr_t load_address() const { return load_address_; }

  // Loads the native library using android_dlopen_ext and invokes JNI_OnLoad().
  //
  // On a successful load exports the address range of the library to the
  // Java-side LibInfo.
  //
  // Iff |spawn_relro_region| is true, also finds the RELRO region in the
  // library (PT_GNU_RELRO), converts it to be backed by a shared memory region
  // (here referred as "RELRO FD") and exports the RELRO information to Java
  // (the address range and the RELRO FD).
  //
  // When spawned, the shared memory region is exported only after sealing as
  // read-only and without writable memory mappings. This allows any process to
  // provide RELRO FD before it starts processing arbitrary input. For example,
  // an App Zygote can create a RELRO FD in a sufficiently trustworthy way to
  // make the Browser/Privileged processes share the region with it.
  bool LoadLibrary(const String& library_path, bool spawn_relro_region);

  // Finds the RELRO region in the native library identified by
  // |this->load_address()| and replaces it with the shared memory region
  // identified by |other_lib_info|.
  //
  // The external NativeLibInfo can arrive from a different process.
  //
  // Note on security: The RELRO region is treated as *trusted*, no untrusted
  // user/website/network input can be processed in an isolated process before
  // it sends the RELRO FD. This is because there is no way to check whether the
  // process has a writable mapping of the region remaining.
  bool CompareRelroAndReplaceItBy(const NativeLibInfo& other_lib_info);

  void set_relro_info_for_testing(uintptr_t start, size_t size) {
    relro_start_ = start;
    relro_size_ = size;
  }

  // Creates a shared RELRO region as it normally would during LoadLibrary()
  // with |spawn_relro_region=true|. Exposed here because it is difficult to
  // unittest LoadLibrary() directly.
  bool CreateSharedRelroFdForTesting();

  void set_relro_fd_for_testing(int fd) { relro_fd_ = fd; }
  int get_relro_fd_for_testing() const { return relro_fd_; }
  size_t get_relro_start_for_testing() const { return relro_start_; }
  size_t get_load_size_for_testing() const { return load_size_; }

  static bool SharedMemoryFunctionsSupportedForTesting();

  bool FindRelroAndLibraryRangesInElfForTesting() {
    return FindRelroAndLibraryRangesInElf();
  }

 private:
  NativeLibInfo() = delete;

  // Not copyable or movable.
  NativeLibInfo(const NativeLibInfo&) = delete;
  NativeLibInfo& operator=(const NativeLibInfo&) = delete;

  // Exports the address range of the library described by |this| to the
  // Java-side LibInfo.
  void ExportLoadInfoToJava() const;

  // Exports the address range of the RELRO region and RELRO FD described by
  // |this| to the Java-side LibInfo.
  void ExportRelroInfoToJava() const;

  void CloseRelroFd();

  // Determines the minimal address ranges for the union of all the loadable
  // (and RELRO) segments by parsing ELF starting at |load_address()|. May fail
  // or return incorrect results for some creative ELF libraries.
  bool FindRelroAndLibraryRangesInElf();

  // Loads and initializes the load address ranges: |load_address_|,
  // |load_size_|. Assumes that the memory range is reserved (in Linker.java).
  bool LoadWithDlopenExt(const String& path, void** handle);

  // Initializes |relro_fd_| with a newly created read-only shared memory region
  // sized as the library's RELRO and with identical data.
  bool CreateSharedRelroFd(const SharedMemoryFunctions& functions);

  // Assuming that RELRO-related information is populated, memory-maps the RELRO
  // FD on top of the library's RELRO.
  bool ReplaceRelroWithSharedOne(const SharedMemoryFunctions& functions) const;

  // Returns true iff the RELRO address and size, along with the contents are
  // equal among the two.
  bool RelroIsIdentical(const NativeLibInfo& external_lib_info,
                        const SharedMemoryFunctions& functions) const;

  static constexpr int kInvalidFd = -1;
  uintptr_t load_address_ = 0;
  size_t load_size_ = 0;
  uintptr_t relro_start_ = 0;
  size_t relro_size_ = 0;
  int relro_fd_ = kInvalidFd;
  JNIEnv* const env_;
  const jobject java_object_;
};

// JNI_OnLoad() initialization hook for the linker.
// Sets up JNI and other initializations for native linker code.
// |vm| is the Java VM handle passed to JNI_OnLoad().
// |env| is the current JNI environment handle.
// On success, returns true.
bool LinkerJNIInit(JavaVM* vm, JNIEnv* env);

}  // namespace chromium_android_linker

#endif  // BASE_ANDROID_LINKER_LINKER_JNI_H_
