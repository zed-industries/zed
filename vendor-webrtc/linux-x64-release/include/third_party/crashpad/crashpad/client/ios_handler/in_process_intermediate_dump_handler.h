// Copyright 2021 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_CLIENT_IOS_HANDLER_IN_PROCESS_INTERMEDIATE_DUMP_HANDLER_H_
#define CRASHPAD_CLIENT_IOS_HANDLER_IN_PROCESS_INTERMEDIATE_DUMP_HANDLER_H_

#include <mach-o/loader.h>
#include <mach/mach.h>
#include <sys/types.h>

#include <map>

#include "client/crashpad_info.h"
#include "util/ios/ios_intermediate_dump_writer.h"
#include "util/ios/ios_system_data_collector.h"
#include "util/mach/mach_extensions.h"

namespace crashpad {
namespace internal {

//! \brief Dump all in-process data to iOS intermediate dump.
//! Note: All methods are `RUNS-DURING-CRASH`.
class InProcessIntermediateDumpHandler final {
 public:
  InProcessIntermediateDumpHandler() = delete;
  InProcessIntermediateDumpHandler(const InProcessIntermediateDumpHandler&) =
      delete;
  InProcessIntermediateDumpHandler& operator=(
      const InProcessIntermediateDumpHandler&) = delete;

  //! \brief Set kVersion to 1.
  //!
  //! \param[in] writer The dump writer
  static void WriteHeader(IOSIntermediateDumpWriter* writer);

  //! \brief Write ProcessSnapshot data to the intermediate dump.
  //!
  //! \param[in] writer The dump writer
  //! \param[in] annotations The simple map annotations.
  static void WriteProcessInfo(
      IOSIntermediateDumpWriter* writer,
      const std::map<std::string, std::string>& annotations);

  //! \brief Write SystemSnapshot data to the intermediate dump.
  //!
  //! \param[in] writer The dump writer
  //! \param[in] system_data An object containing various system data points.
  //! \param[in] report_time_nanos Report creation time in nanoseconds as
  //!     returned by ClockMonotonicNanoseconds().
  static void WriteSystemInfo(IOSIntermediateDumpWriter* writer,
                              const IOSSystemDataCollector& system_data,
                              uint64_t report_time_nanos);

  //! \brief Write ThreadSnapshot data to the intermediate dump.
  //!
  //! For uncaught NSExceptions, \a frames and \a num_frames will be added to
  //! the intermediate dump for the exception thread. Otherwise, or for the
  //! remaining threads, use `thread_get_state`.
  //!
  //! \param[in] writer The dump writer
  //! \param[in] frames An array of callstack return addresses.
  //! \param[in] num_frames The number of callstack return address in \a frames.
  static void WriteThreadInfo(IOSIntermediateDumpWriter* writer,
                              const uint64_t* frames,
                              const size_t num_frames);

  //! \brief Write ModuleSnapshot data to the intermediate dump.
  //!
  //! This includes both modules and annotations.
  //!
  //! \param[in] writer The dump writer
  static void WriteModuleInfo(IOSIntermediateDumpWriter* writer);

  //! \brief Write an ExceptionSnapshot from a signal to the intermediate dump.
  //!
  //!  Only one of the WriteExceptionFromSignal, WriteExceptionFromMachException
  //!  and WriteExceptionFromNSException should be called per intermediate dump.
  //!
  //! \param[in] writer The dump writer
  //! \param[in] system_data An object containing various system data points.
  //! \param[in] siginfo A pointer to a `siginfo_t` object received by a signal
  //!     handler.
  //! \param[in] context A pointer to a `ucontext_t` object received by a
  //!     signal.
  static void WriteExceptionFromSignal(
      IOSIntermediateDumpWriter* writer,
      const IOSSystemDataCollector& system_data,
      siginfo_t* siginfo,
      ucontext_t* context);

  //! \brief Write an ExceptionSnapshot from a mach exception to the
  //!     intermediate dump.
  //!
  //! Only one of the WriteExceptionFromSignal, WriteExceptionFromMachException
  //! and WriteExceptionFromNSException should be called per intermediate dump.
  //!
  //! \param[in] writer The dump writer
  //! \param[in] behavior
  //! \param[in] thread
  //! \param[in] exception
  //! \param[in] code
  //! \param[in] code_count
  //! \param[in] flavor
  //! \param[in] old_state
  //! \param[in] old_state_count
  static void WriteExceptionFromMachException(
      IOSIntermediateDumpWriter* writer,
      exception_behavior_t behavior,
      thread_t thread,
      exception_type_t exception,
      const mach_exception_data_type_t* code,
      mach_msg_type_number_t code_count,
      thread_state_flavor_t flavor,
      ConstThreadState old_state,
      mach_msg_type_number_t old_state_count);

  //! \brief Write an ExceptionSnapshot from an NSException to the
  //!     intermediate dump.
  //!
  //!  Only one of the WriteExceptionFromSignal, WriteExceptionFromMachException
  //!  and WriteExceptionFromNSException should be called per intermediate dump.
  //!
  //! \param[in] writer The dump writer
  static void WriteExceptionFromNSException(IOSIntermediateDumpWriter* writer);

 private:
  //! \brief Parse and extract module and annotation information from header.
  static void WriteModuleInfoAtAddress(IOSIntermediateDumpWriter* writer,
                                       uint64_t address,
                                       bool is_dyld);

  //! \brief Extract and write Apple crashreporter_annotations_t data and
  //!     Crashpad annotations. Note that \a segment_vm_read_ptr has already
  //!     been read via vm_read and may be dereferenced without a ScopedVMRead.
  static void WriteDataSegmentAnnotations(
      IOSIntermediateDumpWriter* writer,
      const segment_command_64* segment_vm_read_ptr,
      vm_size_t slide);

  //! \brief Write Crashpad annotations list.
  static void WriteCrashpadAnnotationsList(IOSIntermediateDumpWriter* writer,
                                           CrashpadInfo* crashpad_info);

  //! \brief Write Crashpad extra memory data.
  static void WriteCrashpadExtraMemoryRanges(IOSIntermediateDumpWriter* writer,
                                             CrashpadInfo* crashpad_info);

  //! \brief Write Crashpad intermediate dump extra memory data.
  static void WriteCrashpadIntermediateDumpExtraMemoryRanges(
      IOSIntermediateDumpWriter* writer,
      CrashpadInfo* crashpad_info);
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_CLIENT_IOS_HANDLER_IN_PROCESS_INTERMEDIATE_DUMP_HANDLER_H_
