// Copyright 2010 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// minidump.h: A minidump reader.
//
// The basic structure of this module tracks the structure of the minidump
// file itself.  At the top level, a minidump file is represented by a
// Minidump object.  Like most other classes in this module, Minidump
// provides a Read method that initializes the object with information from
// the file.  Most of the classes in this file are wrappers around the
// "raw" structures found in the minidump file itself, and defined in
// minidump_format.h.  For example, each thread is represented by a
// MinidumpThread object, whose parameters are specified in an MDRawThread
// structure.  A properly byte-swapped MDRawThread can be obtained from a
// MinidumpThread easily by calling its thread() method.
//
// Most of the module lazily reads only the portion of the minidump file
// necessary to fulfill the user's request.  Calling Minidump::Read
// only reads the minidump's directory.  The thread list is not read until
// it is needed, and even once it's read, the memory regions for each
// thread's stack aren't read until they're needed.  This strategy avoids
// unnecessary file input, and allocating memory for data in which the user
// has no interest.  Note that although memory allocations for a typical
// minidump file are not particularly large, it is possible for legitimate
// minidumps to be sizable.  A full-memory minidump, for example, contains
// a snapshot of the entire mapped memory space.  Even a normal minidump,
// with stack memory only, can be large if, for example, the dump was
// generated in response to a crash that occurred due to an infinite-
// recursion bug that caused the stack's limits to be exceeded.  Finally,
// some users of this library will unfortunately find themselves in the
// position of having to process potentially-hostile minidumps that might
// attempt to cause problems by forcing the minidump processor to over-
// allocate memory.
//
// Memory management in this module is based on a strict
// you-don't-own-anything policy.  The only object owned by the user is
// the top-level Minidump object, the creation and destruction of which
// must be the user's own responsibility.  All other objects obtained
// through interaction with this module are ultimately owned by the
// Minidump object, and will be freed upon the Minidump object's destruction.
// Because memory regions can potentially involve large allocations, a
// FreeMemory method is provided by MinidumpMemoryRegion, allowing the user
// to release data when it is no longer needed.  Use of this method is
// optional but recommended.  If freed data is later required, it will
// be read back in from the minidump file again.
//
// There is one exception to this memory management policy:
// Minidump::ReadString will return a string object to the user, and the user
// is responsible for its deletion.
//
// Author: Mark Mentovai

#ifndef GOOGLE_BREAKPAD_PROCESSOR_MINIDUMP_H__
#define GOOGLE_BREAKPAD_PROCESSOR_MINIDUMP_H__

#include <stdint.h>

#ifndef _WIN32
#include <unistd.h>
#endif

#include <iostream>
#include <map>
#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/processor/code_module.h"
#include "google_breakpad/processor/code_modules.h"
#include "google_breakpad/processor/dump_context.h"
#include "google_breakpad/processor/dump_object.h"
#include "google_breakpad/processor/memory_region.h"
#include "google_breakpad/processor/proc_maps_linux.h"


namespace google_breakpad {


using std::map;
using std::vector;


class Minidump;
template<typename AddressType, typename EntryType> class RangeMap;


// MinidumpObject is the base of all Minidump* objects except for Minidump
// itself.
class MinidumpObject : public DumpObject {
 public:
  virtual ~MinidumpObject() = default;

 protected:
  explicit MinidumpObject(Minidump* minidump);

  // Refers to the Minidump object that is the ultimate parent of this
  // Some MinidumpObjects are owned by other MinidumpObjects, but at the
  // root of the ownership tree is always a Minidump.  The Minidump object
  // is kept here for access to its seeking and reading facilities, and
  // for access to data about the minidump file itself, such as whether
  // it should be byte-swapped.
  Minidump* minidump_;
};


// This class exists primarily to provide a virtual destructor in a base
// class common to all objects that might be stored in
// Minidump::mStreamObjects.  Some object types will never be stored in
// Minidump::mStreamObjects, but are represented as streams and adhere to the
// same interface, and may be derived from this class.
class MinidumpStream : public MinidumpObject {
 public:
  MinidumpStream(const MinidumpStream&) = delete;
  void operator=(const MinidumpStream&) = delete;
  ~MinidumpStream() override = default;

 protected:
  explicit MinidumpStream(Minidump* minidump);

 private:
  // Populate (and validate) the MinidumpStream.  minidump_ is expected
  // to be positioned at the beginning of the stream, so that the next
  // read from the minidump will be at the beginning of the stream.
  // expected_size should be set to the stream's length as contained in
  // the MDRawDirectory record or other identifying record.  A class
  // that implements MinidumpStream can compare expected_size to a
  // known size as an integrity check.
  virtual bool Read(uint32_t expected_size) = 0;
};


// MinidumpContext carries a CPU-specific MDRawContext structure, which
// contains CPU context such as register states.  Each thread has its
// own context, and the exception record, if present, also has its own
// context.  Note that if the exception record is present, the context it
// refers to is probably what the user wants to use for the exception
// thread, instead of that thread's own context.  The exception thread's
// context (as opposed to the exception record's context) will contain
// context for the exception handler (which performs minidump generation),
// and not the context that caused the exception (which is probably what the
// user wants).
class MinidumpContext : public DumpContext {
 public:
  MinidumpContext(const MinidumpContext&) = delete;
  void operator=(const MinidumpContext&) = delete;
  ~MinidumpContext() override;

 protected:
  explicit MinidumpContext(Minidump* minidump);

 private:
  friend class MinidumpThread;
  friend class MinidumpException;

  bool Read(uint32_t expected_size);

  // If the minidump contains a SYSTEM_INFO_STREAM, makes sure that the
  // system info stream gives an appropriate CPU type matching the context
  // CPU type in context_cpu_type.  Returns false if the CPU type does not
  // match.  Returns true if the CPU type matches or if the minidump does
  // not contain a system info stream.
  bool CheckAgainstSystemInfo(uint32_t context_cpu_type);

  // Refers to the Minidump object that is the ultimate parent of this
  // Some MinidumpObjects are owned by other MinidumpObjects, but at the
  // root of the ownership tree is always a Minidump.  The Minidump object
  // is kept here for access to its seeking and reading facilities, and
  // for access to data about the minidump file itself, such as whether
  // it should be byte-swapped.
  Minidump* minidump_;
};


// MinidumpMemoryRegion does not wrap any MDRaw structure, and only contains
// a reference to an MDMemoryDescriptor.  This object is intended to wrap
// portions of a minidump file that contain memory dumps.  In normal
// minidumps, each MinidumpThread owns a MinidumpMemoryRegion corresponding
// to the thread's stack memory.  MinidumpMemoryList also gives access to
// memory regions in its list as MinidumpMemoryRegions.  This class
// adheres to MemoryRegion so that it may be used as a data provider to
// the Stackwalker family of classes.
class MinidumpMemoryRegion : public MinidumpObject,
                             public MemoryRegion {
 public:
  ~MinidumpMemoryRegion() override;

  static void set_max_bytes(uint32_t max_bytes) { max_bytes_ = max_bytes; }
  static uint32_t max_bytes() { return max_bytes_; }

  // Returns a pointer to the base of the memory region.  Returns the
  // cached value if available, otherwise, reads the minidump file and
  // caches the memory region.
  const uint8_t* GetMemory() const;

  // The address of the base of the memory region.
  uint64_t GetBase() const override;

  // The size, in bytes, of the memory region.
  uint32_t GetSize() const override;

  // Frees the cached memory region, if cached.
  void FreeMemory();

  // Obtains the value of memory at the pointer specified by address.
  bool GetMemoryAtAddress(uint64_t address, uint8_t* value) const override;
  bool GetMemoryAtAddress(uint64_t address, uint16_t* value) const override;
  bool GetMemoryAtAddress(uint64_t address, uint32_t* value) const override;
  bool GetMemoryAtAddress(uint64_t address, uint64_t* value) const override;

  // Print a human-readable representation of the object to stdout.
  void Print() const override;
  void SetPrintMode(bool hexdump, unsigned int width);

 protected:
  explicit MinidumpMemoryRegion(Minidump* minidump);

 private:
  friend class MinidumpThread;
  friend class MinidumpMemoryList;

  // Identify the base address and size of the memory region, and the
  // location it may be found in the minidump file.
  void SetDescriptor(MDMemoryDescriptor* descriptor);

  // Implementation for GetMemoryAtAddress
  template<typename T> bool GetMemoryAtAddressInternal(uint64_t address,
                                                       T*        value) const;

  // Knobs for controlling display of memory printing.
  bool hexdump_;
  unsigned int hexdump_width_;

  // The largest memory region that will be read from a minidump.
  static uint32_t max_bytes_;

  // Base address and size of the memory region, and its position in the
  // minidump file.
  MDMemoryDescriptor* descriptor_;

  // Cached memory.
  mutable vector<uint8_t>* memory_;
};


// MinidumpThread contains information about a thread of execution,
// including a snapshot of the thread's stack and CPU context.  For
// the thread that caused an exception, the context carried by
// MinidumpException is probably desired instead of the CPU context
// provided here.
// Note that a MinidumpThread may be valid() even if it does not
// contain a memory region or context.
class MinidumpThread : public MinidumpObject {
 public:
  ~MinidumpThread() override;

  const MDRawThread* thread() const { return valid_ ? &thread_ : nullptr; }
  // GetMemory may return NULL even if the MinidumpThread is valid,
  // if the thread memory cannot be read.
  virtual MinidumpMemoryRegion* GetMemory();
  // GetContext may return NULL even if the MinidumpThread is valid.
  virtual MinidumpContext* GetContext();

  // The thread ID is used to determine if a thread is the exception thread,
  // so a special getter is provided to retrieve this data from the
  // MDRawThread structure.  Returns false if the thread ID cannot be
  // determined.
  virtual bool GetThreadID(uint32_t* thread_id) const;

  // Print a human-readable representation of the object to stdout.
  void Print();

  // Returns the start address of the thread stack memory region.  Returns 0 if
  // MinidumpThread is invalid.  Note that this method can be called even when
  // the thread memory cannot be read and GetMemory returns NULL.
  virtual uint64_t GetStartOfStackMemoryRange() const;

 protected:
  explicit MinidumpThread(Minidump* minidump);

 private:
  // These objects are managed by MinidumpThreadList.
  friend class MinidumpThreadList;

  // This works like MinidumpStream::Read, but is driven by
  // MinidumpThreadList.  No size checking is done, because
  // MinidumpThreadList handles that directly.
  bool Read();

  MDRawThread           thread_;
  MinidumpMemoryRegion* memory_;
  MinidumpContext*      context_;
};


// MinidumpThreadList contains all of the threads (as MinidumpThreads) in
// a process.
class MinidumpThreadList : public MinidumpStream {
 public:
  MinidumpThreadList(const MinidumpThreadList&) = delete;
  void operator=(const MinidumpThreadList&) = delete;
  ~MinidumpThreadList() override;

  static void set_max_threads(uint32_t max_threads) {
    max_threads_ = max_threads;
  }
  static uint32_t max_threads() { return max_threads_; }

  virtual unsigned int thread_count() const {
    return valid_ ? thread_count_ : 0;
  }

  // Sequential access to threads.
  virtual MinidumpThread* GetThreadAtIndex(unsigned int index) const;

  // Random access to threads.
  MinidumpThread* GetThreadByID(uint32_t thread_id);

  // Print a human-readable representation of the object to stdout.
  void Print();

 protected:
  explicit MinidumpThreadList(Minidump* aMinidump);

 private:
  friend class Minidump;

  typedef map<uint32_t, MinidumpThread*> IDToThreadMap;
  typedef vector<MinidumpThread> MinidumpThreads;

  static const uint32_t kStreamType = MD_THREAD_LIST_STREAM;

  bool Read(uint32_t aExpectedSize) override;

  // The largest number of threads that will be read from a minidump.  The
  // default is 256.
  static uint32_t max_threads_;

  // Access to threads using the thread ID as the key.
  IDToThreadMap    id_to_thread_map_;

  // The list of threads.
  MinidumpThreads* threads_;
  uint32_t thread_count_;
};

// MinidumpThreadName contains the name of a thread.
class MinidumpThreadName : public MinidumpObject {
 public:
  ~MinidumpThreadName() override;

  const MDRawThreadName* thread_name() const {
    return valid_ ? &thread_name_ : nullptr;
  }

  // Gets the thread ID.
  virtual bool GetThreadID(uint32_t* thread_id) const;

  // Print a human-readable representation of the object to stdout.
  void Print();

  // Returns the name of the thread.
  virtual std::string GetThreadName() const;

 protected:
  explicit MinidumpThreadName(Minidump* minidump);

 private:
  // These objects are managed by MinidumpThreadNameList.
  friend class MinidumpThreadNameList;

  // This works like MinidumpStream::Read, but is driven by
  // MinidumpThreadNameList.  No size checking is done, because
  // MinidumpThreadNameList handles that directly.
  bool Read();

  // Reads indirectly-referenced data, including the thread name.
  bool ReadAuxiliaryData();

  // True after a successful Read.  This is different from valid_, which is not
  // set true until ReadAuxiliaryData also completes successfully.
  // thread_name_valid_ is only used by ReadAuxiliaryData and the functions it
  // calls to determine whether the object is ready for auxiliary data to be
  // read.
  bool thread_name_valid_;

  MDRawThreadName thread_name_;

  // Cached thread name.
  const string* name_;
};

// MinidumpThreadNameList contains all of the names of the threads (as
// MinidumpThreadNames) in a process.
class MinidumpThreadNameList : public MinidumpStream {
 public:
  MinidumpThreadNameList(const MinidumpThreadNameList&) = delete;
  void operator=(const MinidumpThreadNameList&) = delete;
  ~MinidumpThreadNameList() override;

  virtual unsigned int thread_name_count() const {
    return valid_ ? thread_name_count_ : 0;
  }

  // Sequential access to thread names.
  virtual MinidumpThreadName* GetThreadNameAtIndex(unsigned int index) const;

  // Print a human-readable representation of the object to stdout.
  void Print();

 protected:
  explicit MinidumpThreadNameList(Minidump* aMinidump);

 private:
  friend class Minidump;

  typedef vector<MinidumpThreadName> MinidumpThreadNames;

  static const uint32_t kStreamType = MD_THREAD_NAME_LIST_STREAM;

  bool Read(uint32_t aExpectedSize) override;

  // The list of thread names.
  MinidumpThreadNames* thread_names_;
  uint32_t thread_name_count_;
};

// MinidumpModule wraps MDRawModule, which contains information about loaded
// code modules.  Access is provided to various data referenced indirectly
// by MDRawModule, such as the module's name and a specification for where
// to locate debugging information for the module.
class MinidumpModule : public MinidumpObject,
                       public CodeModule {
 public:
  ~MinidumpModule() override;

  static void set_max_cv_bytes(uint32_t max_cv_bytes) {
    max_cv_bytes_ = max_cv_bytes;
  }
  static uint32_t max_cv_bytes() { return max_cv_bytes_; }

  static void set_max_misc_bytes(uint32_t max_misc_bytes) {
    max_misc_bytes_ = max_misc_bytes;
  }
  static uint32_t max_misc_bytes() { return max_misc_bytes_; }

  const MDRawModule* module() const { return valid_ ? &module_ : nullptr; }

  // CodeModule implementation
  uint64_t base_address() const override {
    return valid_ ? module_.base_of_image : static_cast<uint64_t>(-1);
  }
  uint64_t size() const override { return valid_ ? module_.size_of_image : 0; }
  string code_file() const override;
  string code_identifier() const override;
  string debug_file() const override;
  string debug_identifier() const override;
  string version() const override;
  CodeModule* Copy() const override;
  bool is_unloaded() const override { return false; }

  // Getter and setter for shrink_down_delta.  This is used when the address
  // range for a module is shrunk down due to address range conflicts with
  // other modules.  The base_address and size fields are not updated and they
  // should always reflect the original values (reported in the minidump).
  uint64_t shrink_down_delta() const override;
  void SetShrinkDownDelta(uint64_t shrink_down_delta) override;

  // The CodeView record, which contains information to locate the module's
  // debugging information (pdb).  This is returned as uint8_t* because
  // the data can be of types MDCVInfoPDB20* or MDCVInfoPDB70*, or it may be
  // of a type unknown to Breakpad, in which case the raw data will still be
  // returned but no byte-swapping will have been performed.  Check the
  // record's signature in the first four bytes to differentiate between
  // the various types.  Current toolchains generate modules which carry
  // MDCVInfoPDB70 by default.  Returns a pointer to the CodeView record on
  // success, and NULL on failure.  On success, the optional |size| argument
  // is set to the size of the CodeView record.
  const uint8_t* GetCVRecord(uint32_t* size);

  // The miscellaneous debug record, which is obsolete.  Current toolchains
  // do not generate this type of debugging information (dbg), and this
  // field is not expected to be present.  Returns a pointer to the debugging
  // record on success, and NULL on failure.  On success, the optional |size|
  // argument is set to the size of the debugging record.
  const MDImageDebugMisc* GetMiscRecord(uint32_t* size);

  // Print a human-readable representation of the object to stdout.
  void Print();

 private:
  // These objects are managed by MinidumpModuleList.
  friend class MinidumpModuleList;

  explicit MinidumpModule(Minidump* minidump);

  // This works like MinidumpStream::Read, but is driven by
  // MinidumpModuleList.  No size checking is done, because
  // MinidumpModuleList handles that directly.
  bool Read();

  // Reads indirectly-referenced data, including the module name, CodeView
  // record, and miscellaneous debugging record.  This is necessary to allow
  // MinidumpModuleList to fully construct MinidumpModule objects without
  // requiring seeks to read a contiguous set of MinidumpModule objects.
  // All auxiliary data should be available when Read is called, in order to
  // allow the CodeModule getters to be const methods.
  bool ReadAuxiliaryData();

  // The largest number of bytes that will be read from a minidump for a
  // CodeView record or miscellaneous debugging record, respectively.  The
  // default for each is 1024.
  static uint32_t max_cv_bytes_;
  static uint32_t max_misc_bytes_;

  // True after a successful Read.  This is different from valid_, which is
  // not set true until ReadAuxiliaryData also completes successfully.
  // module_valid_ is only used by ReadAuxiliaryData and the functions it
  // calls to determine whether the object is ready for auxiliary data to
  // be read.
  bool              module_valid_;

  // True if debug info was read from the module.  Certain modules
  // may contain debug records in formats we don't support,
  // so we can just set this to false to ignore them.
  bool              has_debug_info_;

  MDRawModule       module_;

  // Cached module name.
  const string*     name_;

  // Cached CodeView record - this is MDCVInfoPDB20 or (likely)
  // MDCVInfoPDB70, or possibly something else entirely.  Stored as a uint8_t
  // because the structure contains a variable-sized string and its exact
  // size cannot be known until it is processed.
  vector<uint8_t>* cv_record_;

  // If cv_record_ is present, cv_record_signature_ contains a copy of the
  // CodeView record's first four bytes, for ease of determinining the
  // type of structure that cv_record_ contains.
  uint32_t cv_record_signature_;

  // Cached MDImageDebugMisc (usually not present), stored as uint8_t
  // because the structure contains a variable-sized string and its exact
  // size cannot be known until it is processed.
  vector<uint8_t>* misc_record_;
};


// MinidumpModuleList contains all of the loaded code modules for a process
// in the form of MinidumpModules.  It maintains a map of these modules
// so that it may easily provide a code module corresponding to a specific
// address.
class MinidumpModuleList : public MinidumpStream,
                           public CodeModules {
 public:
  MinidumpModuleList(const MinidumpModuleList&) = delete;
  void operator=(const MinidumpModuleList&) = delete;
  ~MinidumpModuleList() override;

  static void set_max_modules(uint32_t max_modules) {
    max_modules_ = max_modules;
  }
  static uint32_t max_modules() { return max_modules_; }

  // CodeModules implementation.
  unsigned int module_count() const override {
    return valid_ ? module_count_ : 0;
  }
  const MinidumpModule* GetModuleForAddress(uint64_t address) const override;
  const MinidumpModule* GetMainModule() const override;
  const MinidumpModule* GetModuleAtSequence(
      unsigned int sequence) const override;
  const MinidumpModule* GetModuleAtIndex(unsigned int index) const override;
  const CodeModules* Copy() const override;

  // Returns a vector of all modules which address ranges needed to be shrunk
  // down due to address range conflicts with other modules.
  vector<linked_ptr<const CodeModule>> GetShrunkRangeModules() const override;

  // Print a human-readable representation of the object to stdout.
  void Print();

 protected:
  explicit MinidumpModuleList(Minidump* minidump);

 private:
  friend class Minidump;

  typedef vector<MinidumpModule> MinidumpModules;

  static const uint32_t kStreamType = MD_MODULE_LIST_STREAM;

  bool Read(uint32_t expected_size) override;

  bool StoreRange(const MinidumpModule& module,
                  uint64_t base_address,
                  uint32_t module_index,
                  uint32_t module_count,
                  bool is_android);

  // The largest number of modules that will be read from a minidump.  The
  // default is 1024.
  static uint32_t max_modules_;

  // Access to modules using addresses as the key.
  RangeMap<uint64_t, unsigned int>* range_map_;

  MinidumpModules* modules_;
  uint32_t module_count_;
};


// MinidumpMemoryList corresponds to a minidump's MEMORY_LIST_STREAM stream,
// which references the snapshots of all of the memory regions contained
// within the minidump.  For a normal minidump, this includes stack memory
// (also referenced by each MinidumpThread, in fact, the MDMemoryDescriptors
// here and in MDRawThread both point to exactly the same data in a
// minidump file, conserving space), as well as a 256-byte snapshot of memory
// surrounding the instruction pointer in the case of an exception.  Other
// types of minidumps may contain significantly more memory regions.  Full-
// memory minidumps contain all of a process' mapped memory.
class MinidumpMemoryList : public MinidumpStream {
 public:
  MinidumpMemoryList(const MinidumpMemoryList&) = delete;
  void operator=(const MinidumpMemoryList&) = delete;
  ~MinidumpMemoryList() override;

  static void set_max_regions(uint32_t max_regions) {
    max_regions_ = max_regions;
  }
  static uint32_t max_regions() { return max_regions_; }

  unsigned int region_count() const { return valid_ ? region_count_ : 0; }

  // Sequential access to memory regions.
  MinidumpMemoryRegion* GetMemoryRegionAtIndex(unsigned int index);

  // Random access to memory regions.  Returns the region encompassing
  // the address identified by address.
  virtual MinidumpMemoryRegion* GetMemoryRegionForAddress(uint64_t address);

  // Print a human-readable representation of the object to stdout.
  void Print();

 private:
  friend class Minidump;
  friend class MockMinidumpMemoryList;

  typedef vector<MDMemoryDescriptor>   MemoryDescriptors;
  typedef vector<MinidumpMemoryRegion> MemoryRegions;

  static const uint32_t kStreamType = MD_MEMORY_LIST_STREAM;

  explicit MinidumpMemoryList(Minidump* minidump);

  bool Read(uint32_t expected_size) override;

  // The largest number of memory regions that will be read from a minidump.
  // The default is 256.
  static uint32_t max_regions_;

  // Access to memory regions using addresses as the key.
  RangeMap<uint64_t, unsigned int>* range_map_;

  // The list of descriptors.  This is maintained separately from the list
  // of regions, because MemoryRegion doesn't own its MemoryDescriptor, it
  // maintains a pointer to it.  descriptors_ provides the storage for this
  // purpose.
  MemoryDescriptors* descriptors_;

  // The list of regions.
  MemoryRegions* regions_;
  uint32_t region_count_;
};


// MinidumpException wraps MDRawExceptionStream, which contains information
// about the exception that caused the minidump to be generated, if the
// minidump was generated in an exception handler called as a result of an
// exception.  It also provides access to a MinidumpContext object, which
// contains the CPU context for the exception thread at the time the exception
// occurred.
class MinidumpException : public MinidumpStream {
 public:
  MinidumpException(const MinidumpException&) = delete;
  void operator=(const MinidumpException&) = delete;
  ~MinidumpException() override;

  const MDRawExceptionStream* exception() const {
    return valid_ ? &exception_ : nullptr;
  }

  // The thread ID is used to determine if a thread is the exception thread,
  // so a special getter is provided to retrieve this data from the
  // MDRawExceptionStream structure.  Returns false if the thread ID cannot
  // be determined.
  bool GetThreadID(uint32_t* thread_id) const;

  MinidumpContext* GetContext();

  // Print a human-readable representation of the object to stdout.
  void Print();

 private:
  friend class Minidump;

  static const uint32_t kStreamType = MD_EXCEPTION_STREAM;

  explicit MinidumpException(Minidump* minidump);

  bool Read(uint32_t expected_size) override;

  MDRawExceptionStream exception_;
  MinidumpContext* context_;
};

// MinidumpAssertion wraps MDRawAssertionInfo, which contains information
// about an assertion that caused the minidump to be generated.
class MinidumpAssertion : public MinidumpStream {
 public:
  MinidumpAssertion(const MinidumpAssertion&) = delete;
  void operator=(const MinidumpAssertion&) = delete;
  ~MinidumpAssertion() override;

  const MDRawAssertionInfo* assertion() const {
    return valid_ ? &assertion_ : nullptr;
  }

  string expression() const {
    return valid_ ? expression_ : "";
  }

  string function() const {
    return valid_ ? function_ : "";
  }

  string file() const {
    return valid_ ? file_ : "";
  }

  // Print a human-readable representation of the object to stdout.
  void Print();

 private:
  friend class Minidump;

  static const uint32_t kStreamType = MD_ASSERTION_INFO_STREAM;

  explicit MinidumpAssertion(Minidump* minidump);

  bool Read(uint32_t expected_size) override;

  MDRawAssertionInfo assertion_;
  string expression_;
  string function_;
  string file_;
};


// MinidumpSystemInfo wraps MDRawSystemInfo and provides information about
// the system on which the minidump was generated.  See also MinidumpMiscInfo.
class MinidumpSystemInfo : public MinidumpStream {
 public:
  MinidumpSystemInfo(const MinidumpSystemInfo&) = delete;
  void operator=(const MinidumpSystemInfo&) = delete;
  ~MinidumpSystemInfo() override;

  const MDRawSystemInfo* system_info() const {
    return valid_ ? &system_info_ : nullptr;
  }

  // GetOS and GetCPU return textual representations of the operating system
  // and CPU that produced the minidump.  Unlike most other Minidump* methods,
  // they return string objects, not weak pointers.  Defined values for
  // GetOS() are "mac", "windows", and "linux".  Defined values for GetCPU
  // are "x86" and "ppc".  These methods return an empty string when their
  // values are unknown.
  string GetOS();
  string GetCPU();

  // I don't know what CSD stands for, but this field is documented as
  // returning a textual representation of the OS service pack.  On other
  // platforms, this provides additional information about an OS version
  // level beyond major.minor.micro.  Returns NULL if unknown.
  const string* GetCSDVersion();

  // If a CPU vendor string can be determined, returns a pointer to it,
  // otherwise, returns NULL.  CPU vendor strings can be determined from
  // x86 CPUs with CPUID 0.
  const string* GetCPUVendor();

  // Print a human-readable representation of the object to stdout.
  void Print();

 protected:
  explicit MinidumpSystemInfo(Minidump* minidump);
  MDRawSystemInfo system_info_;

  // Textual representation of the OS service pack, for minidumps produced
  // by MiniDumpWriteDump on Windows.
  const string* csd_version_;

 private:
  friend class Minidump;

  static const uint32_t kStreamType = MD_SYSTEM_INFO_STREAM;

  bool Read(uint32_t expected_size) override;

  // A string identifying the CPU vendor, if known.
  const string* cpu_vendor_;
};


// MinidumpUnloadedModule wraps MDRawUnloadedModule
class MinidumpUnloadedModule : public MinidumpObject,
                               public CodeModule {
 public:
  ~MinidumpUnloadedModule() override;

  const MDRawUnloadedModule* module() const {
    return valid_ ? &unloaded_module_ : nullptr;
  }

  // CodeModule implementation
  uint64_t base_address() const override {
    return valid_ ? unloaded_module_.base_of_image : 0;
  }
  uint64_t size() const override {
    return valid_ ? unloaded_module_.size_of_image : 0;
  }
  string code_file() const override;
  string code_identifier() const override;
  string debug_file() const override;
  string debug_identifier() const override;
  string version() const override;
  CodeModule* Copy() const override;
  bool is_unloaded() const override { return true; }
  uint64_t shrink_down_delta() const override;
  void SetShrinkDownDelta(uint64_t shrink_down_delta) override;

 protected:
  explicit MinidumpUnloadedModule(Minidump* minidump);

 private:
  // These objects are managed by MinidumpUnloadedModuleList
  friend class MinidumpUnloadedModuleList;

  // This works like MinidumpStream::Read, but is driven by
  // MinidumpUnloadedModuleList.
  bool Read(uint32_t expected_size);

  // Reads the module name. This is done separately from Read to
  // allow contiguous reading of code modules by MinidumpUnloadedModuleList.
  bool ReadAuxiliaryData();

  // True after a successful Read. This is different from valid_, which
  // is not set true until ReadAuxiliaryData also completes successfully.
  // module_valid_ is only used by ReadAuxiliaryData and the functions it
  // calls to determine whether the object is ready for auxiliary data to
  // be read.
  bool module_valid_;

  MDRawUnloadedModule unloaded_module_;

  // Cached module name
  const string* name_;
};


// MinidumpUnloadedModuleList contains all the unloaded code modules for a
// process in the form of MinidumpUnloadedModules. It maintains a map of
// these modules so that it may easily provide a code module corresponding
// to a specific address. If multiple modules in the list have identical
// ranges, only the first module encountered is recorded in the range map.
class MinidumpUnloadedModuleList : public MinidumpStream,
                                   public CodeModules {
 public:
  MinidumpUnloadedModuleList(const MinidumpUnloadedModuleList&) = delete;
  void operator=(const MinidumpUnloadedModuleList&) = delete;
  ~MinidumpUnloadedModuleList() override;

  static void set_max_modules(uint32_t max_modules) {
    max_modules_ = max_modules;
  }
  static uint32_t max_modules() { return max_modules_; }

  // CodeModules implementation.
  unsigned int module_count() const override {
    return valid_ ? module_count_ : 0;
  }
  const MinidumpUnloadedModule*
      GetModuleForAddress(uint64_t address) const override;
  const MinidumpUnloadedModule* GetMainModule() const override;
  const MinidumpUnloadedModule*
      GetModuleAtSequence(unsigned int sequence) const override;
  const MinidumpUnloadedModule*
      GetModuleAtIndex(unsigned int index) const override;
  const CodeModules* Copy() const override;
  vector<linked_ptr<const CodeModule>> GetShrunkRangeModules() const override;

 protected:
  explicit MinidumpUnloadedModuleList(Minidump* minidump_);

 private:
  friend class Minidump;

  typedef vector<MinidumpUnloadedModule> MinidumpUnloadedModules;

  static const uint32_t kStreamType = MD_UNLOADED_MODULE_LIST_STREAM;

  bool Read(uint32_t expected_size_) override;

  // The largest number of modules that will be read from a minidump.  The
  // default is 1024.
  static uint32_t max_modules_;

  // Access to module indices using addresses as the key.
  RangeMap<uint64_t, unsigned int>* range_map_;

  MinidumpUnloadedModules* unloaded_modules_;
  uint32_t module_count_;
};


// MinidumpMiscInfo wraps MDRawMiscInfo and provides information about
// the process that generated the minidump, and optionally additional system
// information.  See also MinidumpSystemInfo.
class MinidumpMiscInfo : public MinidumpStream {
 public:
  MinidumpMiscInfo(const MinidumpMiscInfo&) = delete;
  void operator=(const MinidumpMiscInfo&) = delete;

  const MDRawMiscInfo* misc_info() const {
    return valid_ ? &misc_info_ : nullptr;
  }

  // Print a human-readable representation of the object to stdout.
  void Print();

 private:
  friend class Minidump;
  friend class TestMinidumpMiscInfo;

  static const uint32_t kStreamType = MD_MISC_INFO_STREAM;

  explicit MinidumpMiscInfo(Minidump* minidump_);

  bool Read(uint32_t expected_size_) override;

  MDRawMiscInfo misc_info_;

  // Populated by Read.  Contains the converted strings from the corresponding
  // UTF-16 fields in misc_info_
  string standard_name_;
  string daylight_name_;
  string build_string_;
  string dbg_bld_str_;
};


// MinidumpBreakpadInfo wraps MDRawBreakpadInfo, which is an optional stream in
// a minidump that provides additional information about the process state
// at the time the minidump was generated.
class MinidumpBreakpadInfo : public MinidumpStream {
 public:
  MinidumpBreakpadInfo(const MinidumpBreakpadInfo&) = delete;
  void operator=(const MinidumpBreakpadInfo&) = delete;

  const MDRawBreakpadInfo* breakpad_info() const {
    return valid_ ? &breakpad_info_ : nullptr;
  }

  // These thread IDs are used to determine if threads deserve special
  // treatment, so special getters are provided to retrieve this data from
  // the MDRawBreakpadInfo structure.  The getters return false if the thread
  // IDs cannot be determined.
  bool GetDumpThreadID(uint32_t* thread_id) const;
  bool GetRequestingThreadID(uint32_t* thread_id) const;

  // Print a human-readable representation of the object to stdout.
  void Print();

 private:
  friend class Minidump;

  static const uint32_t kStreamType = MD_BREAKPAD_INFO_STREAM;

  explicit MinidumpBreakpadInfo(Minidump* minidump_);

  bool Read(uint32_t expected_size_) override;

  MDRawBreakpadInfo breakpad_info_;
};

// MinidumpMemoryInfo wraps MDRawMemoryInfo, which provides information
// about mapped memory regions in a process, including their ranges
// and protection.
class MinidumpMemoryInfo : public MinidumpObject {
 public:
  const MDRawMemoryInfo* info() const {
    return valid_ ? &memory_info_ : nullptr;
  }

  // The address of the base of the memory region.
  uint64_t GetBase() const { return valid_ ? memory_info_.base_address : 0; }

  // The size, in bytes, of the memory region.
  uint64_t GetSize() const { return valid_ ? memory_info_.region_size : 0; }

  // Return true if the memory protection allows execution.
  bool IsExecutable() const;

  // Return true if the memory protection allows writing.
  bool IsWritable() const;

  // Print a human-readable representation of the object to stdout.
  void Print();

 private:
  // These objects are managed by MinidumpMemoryInfoList.
  friend class MinidumpMemoryInfoList;

  explicit MinidumpMemoryInfo(Minidump* minidump_);

  // This works like MinidumpStream::Read, but is driven by
  // MinidumpMemoryInfoList.  No size checking is done, because
  // MinidumpMemoryInfoList handles that directly.
  bool Read();

  MDRawMemoryInfo memory_info_;
};

// MinidumpMemoryInfoList contains a list of information about
// mapped memory regions for a process in the form of MDRawMemoryInfo.
// It maintains a map of these structures so that it may easily provide
// info corresponding to a specific address.
class MinidumpMemoryInfoList : public MinidumpStream {
 public:
  MinidumpMemoryInfoList(const MinidumpMemoryInfoList&) = delete;
  void operator=(const MinidumpMemoryInfoList&) = delete;
  ~MinidumpMemoryInfoList() override;

  unsigned int info_count() const { return valid_ ? info_count_ : 0; }

  const MinidumpMemoryInfo* GetMemoryInfoForAddress(uint64_t address) const;
  const MinidumpMemoryInfo* GetMemoryInfoAtIndex(unsigned int index) const;

  // Print a human-readable representation of the object to stdout.
  void Print();

 private:
  friend class Minidump;

  typedef vector<MinidumpMemoryInfo> MinidumpMemoryInfos;

  static const uint32_t kStreamType = MD_MEMORY_INFO_LIST_STREAM;

  explicit MinidumpMemoryInfoList(Minidump* minidump_);

  bool Read(uint32_t expected_size) override;

  // Access to memory info using addresses as the key.
  RangeMap<uint64_t, unsigned int>* range_map_;

  MinidumpMemoryInfos* infos_;
  uint32_t info_count_;
};

// MinidumpLinuxMaps wraps information about a single mapped memory region
// from /proc/self/maps.
class MinidumpLinuxMaps : public MinidumpObject {
 public:
  MinidumpLinuxMaps(const MinidumpLinuxMaps&) = delete;
  void operator=(const MinidumpLinuxMaps&) = delete;

  // The memory address of the base of the mapped region.
  uint64_t GetBase() const { return valid_ ? region_.start : 0; }
  // The size of the mapped region.
  uint64_t GetSize() const { return valid_ ? region_.end - region_.start : 0; }

  // The permissions of the mapped region.
  bool IsReadable() const {
    return valid_ ? region_.permissions & MappedMemoryRegion::READ : false;
  }
  bool IsWriteable() const {
    return valid_ ? region_.permissions & MappedMemoryRegion::WRITE : false;
  }
  bool IsExecutable() const {
    return valid_ ? region_.permissions & MappedMemoryRegion::EXECUTE : false;
  }
  bool IsPrivate() const {
    return valid_ ? region_.permissions & MappedMemoryRegion::PRIVATE : false;
  }

  // The offset of the mapped region.
  uint64_t GetOffset() const { return valid_ ? region_.offset : 0; }

  // The major device number.
  uint8_t GetMajorDevice() const { return valid_ ? region_.major_device : 0; }
  // The minor device number.
  uint8_t GetMinorDevice() const { return valid_ ? region_.minor_device : 0; }

  // The inode of the mapped region.
  uint64_t GetInode() const { return valid_ ? region_.inode : 0; }

  // The pathname of the mapped region.
  const string GetPathname() const { return valid_ ? region_.path : ""; }

  // Print the contents of this mapping.
  void Print() const;

 private:
  // These objects are managed by MinidumpLinuxMapsList.
  friend class MinidumpLinuxMapsList;

  // This caller owns the pointer.
  explicit MinidumpLinuxMaps(Minidump* minidump);

  // The memory region struct that this class wraps.
  MappedMemoryRegion region_;
};

// MinidumpLinuxMapsList corresponds to the Linux-exclusive MD_LINUX_MAPS
// stream, which contains the contents of /prod/self/maps, which contains
// the mapped memory regions and their access permissions.
class MinidumpLinuxMapsList : public MinidumpStream {
 public:
  MinidumpLinuxMapsList(const MinidumpLinuxMapsList&) = delete;
  void operator=(const MinidumpLinuxMapsList&) = delete;
  ~MinidumpLinuxMapsList() override;

  // Get number of mappings.
  unsigned int get_maps_count() const { return valid_ ? maps_count_ : 0; }

  // Get mapping at the given memory address. The caller owns the pointer.
  const MinidumpLinuxMaps* GetLinuxMapsForAddress(uint64_t address) const;
  // Get mapping at the given index. The caller owns the pointer.
  const MinidumpLinuxMaps* GetLinuxMapsAtIndex(unsigned int index) const;

  // Print the contents of /proc/self/maps to stdout.
  void Print() const;

 private:
  friend class Minidump;

  typedef vector<MinidumpLinuxMaps*> MinidumpLinuxMappings;

  static const uint32_t kStreamType = MD_LINUX_MAPS;

  // The caller owns the pointer.
  explicit MinidumpLinuxMapsList(Minidump* minidump);

  // Read and load the contents of the process mapping data.
  // The stream should have data in the form of /proc/self/maps.
  // This method returns whether the stream was read successfully.
  bool Read(uint32_t expected_size) override;

  // The list of individual mappings.
  MinidumpLinuxMappings* maps_;
  // The number of mappings.
  uint32_t maps_count_;
};

// MinidumpCrashpadInfo wraps MDRawCrashpadInfo, which is an optional stream in
// a minidump that provides additional information about the process state
// at the time the minidump was generated.
class MinidumpCrashpadInfo : public MinidumpStream {
 public:
  struct AnnotationObject {
    uint16_t type;
    std::string name;
    std::vector<uint8_t> value;
  };

  const MDRawCrashpadInfo* crashpad_info() const {
    return valid_ ? &crashpad_info_ : nullptr;
  }

  const std::vector<std::vector<AnnotationObject>>*
  GetModuleCrashpadInfoAnnotationObjects() const {
    return valid_ ? &module_crashpad_info_annotation_objects_ : nullptr;
  }

  // Print a human-readable representation of the object to stdout.
  void Print();

 private:
  friend class Minidump;

  static const uint32_t kStreamType = MD_CRASHPAD_INFO_STREAM;

  explicit MinidumpCrashpadInfo(Minidump* minidump_);

  bool Read(uint32_t expected_size);

  MDRawCrashpadInfo crashpad_info_;
  std::vector<uint32_t> module_crashpad_info_links_;
  std::vector<MDRawModuleCrashpadInfo> module_crashpad_info_;
  std::vector<std::vector<std::string>> module_crashpad_info_list_annotations_;
  std::vector<std::map<std::string, std::string>>
      module_crashpad_info_simple_annotations_;
  std::vector<std::vector<AnnotationObject>>
      module_crashpad_info_annotation_objects_;

  std::map<std::string, std::string> simple_annotations_;
};


// Minidump is the user's interface to a minidump file.  It wraps MDRawHeader
// and provides access to the minidump's top-level stream directory.
class Minidump {
 public:
  // path is the pathname of a file containing the minidump.
  explicit Minidump(const string& path,
                    bool hexdump=false,
                    unsigned int hexdump_width=16);
  // input is an istream wrapping minidump data. Minidump holds a
  // weak pointer to input, and the caller must ensure that the stream
  // is valid as long as the Minidump object is.
  explicit Minidump(std::istream& input);

  Minidump(const Minidump&) = delete;
  void operator=(const Minidump&) = delete;

  virtual ~Minidump();

  // path may be empty if the minidump was not opened from a file
  virtual string path() const {
    return path_;
  }
  static void set_max_streams(uint32_t max_streams) {
    max_streams_ = max_streams;
  }
  static uint32_t max_streams() { return max_streams_; }

  static void set_max_string_length(uint32_t max_string_length) {
    max_string_length_ = max_string_length;
  }
  static uint32_t max_string_length() { return max_string_length_; }

  virtual const MDRawHeader* header() const {
    return valid_ ? &header_ : nullptr;
  }

  // Reads the CPU information from the system info stream and generates the
  // appropriate CPU flags.  The returned context_cpu_flags are the same as
  // if the CPU type bits were set in the context_flags of a context record.
  // On success, context_cpu_flags will have the flags that identify the CPU.
  // If a system info stream is missing, context_cpu_flags will be 0.
  // Returns true if the current position in the stream was not changed.
  // Returns false when the current location in the stream was changed and the
  // attempt to restore the original position failed.
  bool GetContextCPUFlagsFromSystemInfo(uint32_t* context_cpu_flags);

  // Reads the minidump file's header and top-level stream directory.
  // The minidump is expected to be positioned at the beginning of the
  // header.  Read() sets up the stream list and map, and validates the
  // Minidump object.
  virtual bool Read();

  // The next set of methods are stubs that call GetStream.  They exist to
  // force code generation of the templatized API within the module, and
  // to avoid exposing an ugly API (GetStream needs to accept a garbage
  // parameter).
  virtual MinidumpThreadList* GetThreadList();
  virtual MinidumpThreadNameList* GetThreadNameList();
  virtual MinidumpModuleList* GetModuleList();
  virtual MinidumpMemoryList* GetMemoryList();
  virtual MinidumpException* GetException();
  virtual MinidumpAssertion* GetAssertion();
  virtual MinidumpSystemInfo* GetSystemInfo();
  virtual MinidumpUnloadedModuleList* GetUnloadedModuleList();
  virtual MinidumpMiscInfo* GetMiscInfo();
  virtual MinidumpBreakpadInfo* GetBreakpadInfo();
  virtual MinidumpMemoryInfoList* GetMemoryInfoList();
  MinidumpCrashpadInfo* GetCrashpadInfo();

  // The next method also calls GetStream, but is exclusive for Linux dumps.
  virtual MinidumpLinuxMapsList* GetLinuxMapsList();

  // The next set of methods are provided for users who wish to access
  // data in minidump files directly, while leveraging the rest of
  // this class and related classes to handle the basic minidump
  // structure and known stream types.

  unsigned int GetDirectoryEntryCount() const {
    return valid_ ? header_.stream_count : 0;
  }
  const MDRawDirectory* GetDirectoryEntryAtIndex(unsigned int index) const;

  // The next 2 methods are lower-level I/O routines.  They use fd_.

  // Reads count bytes from the minidump at the current position into
  // the storage area pointed to by bytes.  bytes must be of sufficient
  // size.  After the read, the file position is advanced by count.
  bool ReadBytes(void* bytes, size_t count);

  // Sets the position of the minidump file to offset.
  bool SeekSet(off_t offset);

  // Returns the current position of the minidump file.
  off_t Tell();

  // Medium-level I/O routines.

  // ReadString returns a string which is owned by the caller!  offset
  // specifies the offset that a length-encoded string is stored at in the
  // minidump file.
  string* ReadString(off_t offset);

  bool ReadUTF8String(off_t offset, string* string_utf8);

  bool ReadStringList(off_t offset, std::vector<std::string>* string_list);

  bool ReadSimpleStringDictionary(
      off_t offset,
      std::map<std::string, std::string>* simple_string_dictionary);

  bool ReadCrashpadAnnotationsList(
      off_t offset,
      std::vector<MinidumpCrashpadInfo::AnnotationObject>* annotations_list);

  // SeekToStreamType positions the file at the beginning of a stream
  // identified by stream_type, and informs the caller of the stream's
  // length by setting *stream_length.  Because stream_map maps each stream
  // type to only one stream in the file, this might mislead the user into
  // thinking that the stream that this seeks to is the only stream with
  // type stream_type.  That can't happen for streams that these classes
  // deal with directly, because they're only supposed to be present in the
  // file singly, and that's verified when stream_map_ is built.  Users who
  // are looking for other stream types should be aware of this
  // possibility, and consider using GetDirectoryEntryAtIndex (possibly
  // with GetDirectoryEntryCount) if expecting multiple streams of the same
  // type in a single minidump file.
  bool SeekToStreamType(uint32_t stream_type, uint32_t* stream_length);

  bool swap() const { return valid_ ? swap_ : false; }

  bool is_big_endian() const { return valid_ ? is_big_endian_ : false; }

  // Print a human-readable representation of the object to stdout.
  void Print();

  // Is the OS Android.
  bool IsAndroid();

  // Determines the platform where the minidump was produced. |platform| is
  // valid iff this method returns true.
  bool GetPlatform(MDOSPlatform* platform);

  // Get current hexdump display settings.
  unsigned int HexdumpMode() const { return hexdump_ ? hexdump_width_ : 0; }

 private:
  // MinidumpStreamInfo is used in the MinidumpStreamMap.  It lets
  // the Minidump object locate interesting streams quickly, and
  // provides a convenient place to stash MinidumpStream objects.
  struct MinidumpStreamInfo {
    MinidumpStreamInfo() : stream_index(0), stream(nullptr) {}
    ~MinidumpStreamInfo() { delete stream; }

    // Index into the MinidumpDirectoryEntries vector
    unsigned int    stream_index;

    // Pointer to the stream if cached, or NULL if not yet populated
    MinidumpStream* stream;
  };

  typedef vector<MDRawDirectory> MinidumpDirectoryEntries;
  typedef map<uint32_t, MinidumpStreamInfo> MinidumpStreamMap;

  template<typename T> T* GetStream(T** stream);

  // Opens the minidump file, or if already open, seeks to the beginning.
  bool Open();

  // The largest number of top-level streams that will be read from a minidump.
  // Note that streams are only read (and only consume memory) as needed,
  // when directed by the caller.  The default is 128.
  static uint32_t max_streams_;

  // The maximum length of a UTF-16 string that will be read from a minidump
  // in 16-bit words.  The default is 1024.  UTF-16 strings are converted
  // to UTF-8 when stored in memory, and each UTF-16 word will be represented
  // by as many as 3 bytes in UTF-8.
  static unsigned int max_string_length_;

  MDRawHeader               header_;

  // The list of streams.
  MinidumpDirectoryEntries* directory_;

  // Access to streams using the stream type as the key.
  MinidumpStreamMap*        stream_map_;

  // The pathname of the minidump file to process, set in the constructor.
  // This may be empty if the minidump was opened directly from a stream.
  const string              path_;

  // The stream for all file I/O.  Used by ReadBytes and SeekSet.
  // Set based on the path in Open, or directly in the constructor.
  std::istream*             stream_;

  // swap_ is true if the minidump file should be byte-swapped.  If the
  // minidump was produced by a CPU that is other-endian than the CPU
  // processing the minidump, this will be true.  If the two CPUs are
  // same-endian, this will be false.
  bool                      swap_;

  // true if the minidump was produced by a big-endian cpu.
  bool                      is_big_endian_;

  // Validity of the Minidump structure, false immediately after
  // construction or after a failed Read(); true following a successful
  // Read().
  bool                      valid_;

  // Knobs for controlling display of memory printing.
  bool                      hexdump_;
  unsigned int              hexdump_width_;
};


}  // namespace google_breakpad


#endif  // GOOGLE_BREAKPAD_PROCESSOR_MINIDUMP_H__
