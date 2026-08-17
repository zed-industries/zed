// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains methods to iterate over processes on the system.

#ifndef BASE_PROCESS_PROCESS_ITERATOR_H_
#define BASE_PROCESS_PROCESS_ITERATOR_H_

#include <stddef.h>

#include <list>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/files/file_path.h"
#include "base/memory/raw_ptr.h"
#include "base/process/process.h"
#include "base/strings/string_util.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include <windows.h>

#include <tlhelp32.h>
#elif BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_OPENBSD)
#include <sys/sysctl.h>
#elif BUILDFLAG(IS_FREEBSD)
#include <sys/user.h>
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
#include <dirent.h>
#endif

namespace base {

#if BUILDFLAG(IS_WIN)
struct ProcessEntry : public PROCESSENTRY32 {
  ProcessId pid() const { return th32ProcessID; }
  ProcessId parent_pid() const { return th32ParentProcessID; }
  const wchar_t* exe_file() const { return szExeFile; }
};
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
struct BASE_EXPORT ProcessEntry {
  ProcessEntry();
  ProcessEntry(const ProcessEntry& other);
  ~ProcessEntry();

  ProcessId pid() const { return pid_; }
  ProcessId parent_pid() const { return ppid_; }
  ProcessId gid() const { return gid_; }
  const char* exe_file() const { return exe_file_.c_str(); }
  const std::vector<std::string>& cmd_line_args() const LIFETIME_BOUND {
    return cmd_line_args_;
  }

  ProcessId pid_;
  ProcessId ppid_;
  ProcessId gid_;
  std::string exe_file_;
  std::vector<std::string> cmd_line_args_;
};
#endif  // BUILDFLAG(IS_WIN)

// Used to filter processes by process ID.
class ProcessFilter {
 public:
  // Returns true to indicate set-inclusion and false otherwise.  This method
  // should not have side-effects and should be idempotent.
  virtual bool Includes(const ProcessEntry& entry) const = 0;

 protected:
  virtual ~ProcessFilter() = default;
};

// This class provides a way to iterate through a list of processes on the
// current machine with a specified filter.
// To use, create an instance and then call NextProcessEntry() until it returns
// false.
class BASE_EXPORT ProcessIterator {
 public:
  typedef std::list<ProcessEntry> ProcessEntries;

  explicit ProcessIterator(const ProcessFilter* filter);

  ProcessIterator(const ProcessIterator&) = delete;
  ProcessIterator& operator=(const ProcessIterator&) = delete;

  virtual ~ProcessIterator();

  // If there's another process that matches the given executable name,
  // returns a const pointer to the corresponding PROCESSENTRY32.
  // If there are no more matching processes, returns NULL.
  // The returned pointer will remain valid until NextProcessEntry()
  // is called again or this NamedProcessIterator goes out of scope.
  const ProcessEntry* NextProcessEntry();

  // Takes a snapshot of all the ProcessEntry found.
  ProcessEntries Snapshot();

 protected:
  virtual bool IncludeEntry();
  const ProcessEntry& entry() const LIFETIME_BOUND { return entry_; }

 private:
  // Determines whether there's another process (regardless of executable)
  // left in the list of all processes.  Returns true and sets entry_ to
  // that process's info if there is one, false otherwise.
  bool CheckForNextProcess();

  // Initializes a PROCESSENTRY32 data structure so that it's ready for
  // use with Process32First/Process32Next.
  void InitProcessEntry(ProcessEntry* entry);

#if BUILDFLAG(IS_WIN)
  HANDLE snapshot_;
  bool started_iteration_ = false;
#elif BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_BSD)
  std::vector<kinfo_proc> kinfo_procs_;
  size_t index_of_kinfo_proc_ = 0;
#elif BUILDFLAG(IS_POSIX)
  struct DIRClose {
    inline void operator()(DIR* x) const {
      if (x) {
        closedir(x);
      }
    }
  };
  std::unique_ptr<DIR, DIRClose> procfs_dir_;
#endif
  ProcessEntry entry_;
  raw_ptr<const ProcessFilter> filter_;
};

// This class provides a way to iterate through the list of processes
// on the current machine that were started from the given executable
// name.  To use, create an instance and then call NextProcessEntry()
// until it returns false.
// If `use_prefix_match` is true, this iterates all processes that
// begin with `executable_name`; for example, "Google Chrome Helper" would
// match "Google Chrome Helper", "Google Chrome Helper (Renderer)" and
// "Google Chrome Helper (GPU)" if `use_prefix_match` is true and otherwise
// only "Google Chrome Helper". This option is only implemented on Mac.
class BASE_EXPORT NamedProcessIterator : public ProcessIterator {
 public:
  NamedProcessIterator(const FilePath::StringType& executable_name,
                       const ProcessFilter* filter,
                       bool use_prefix_match = false);

  NamedProcessIterator(const NamedProcessIterator&) = delete;
  NamedProcessIterator& operator=(const NamedProcessIterator&) = delete;

  ~NamedProcessIterator() override;

 protected:
  bool IncludeEntry() override;

 private:
  FilePath::StringType executable_name_;
  const bool use_prefix_match_;
};

// Returns the number of processes on the machine that are running from the
// given executable name.  If filter is non-null, then only processes selected
// by the filter will be counted.
BASE_EXPORT int GetProcessCount(const FilePath::StringType& executable_name,
                                const ProcessFilter* filter);

}  // namespace base

#endif  // BASE_PROCESS_PROCESS_ITERATOR_H_
