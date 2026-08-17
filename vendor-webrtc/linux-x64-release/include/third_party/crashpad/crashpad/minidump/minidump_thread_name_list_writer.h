// Copyright 2022 The Crashpad Authors
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

#ifndef CRASHPAD_MINIDUMP_MINIDUMP_THREAD_NAME_LIST_WRITER_H_
#define CRASHPAD_MINIDUMP_MINIDUMP_THREAD_NAME_LIST_WRITER_H_

#include <windows.h>
#include <dbghelp.h>
#include <stdint.h>
#include <sys/types.h>

#include <memory>
#include <vector>

#include "minidump/minidump_stream_writer.h"
#include "minidump/minidump_string_writer.h"
#include "minidump/minidump_thread_id_map.h"
#include "minidump/minidump_writable.h"

namespace crashpad {

//! \brief The writer for a MINIDUMP_THREAD_NAME object in a minidump file.
//!
//! Because MINIDUMP_THREAD_NAME objects only appear as elements of
//! MINIDUMP_THREAD_NAME_LIST objects, this class does not write any data on its
//! own. It makes its MINIDUMP_THREAD_NAME data available to its
//! MinidumpThreadNameListWriter parent, which writes it as part of a
//! MINIDUMP_THREAD_NAME_LIST.
class MinidumpThreadNameWriter final : public internal::MinidumpWritable {
 public:
  MinidumpThreadNameWriter();

  MinidumpThreadNameWriter(const MinidumpThreadNameWriter&) = delete;
  MinidumpThreadNameWriter& operator=(const MinidumpThreadNameWriter&) = delete;

  ~MinidumpThreadNameWriter() override;

  //! \brief Initializes the MINIDUMP_THREAD_NAME based on \a thread_snapshot.
  //!
  //! \param[in] thread_snapshot The thread snapshot to use as source data.
  //! \param[in] thread_id_map A MinidumpThreadIDMap to be consulted to
  //!     determine the 32-bit minidump thread ID to use for \a thread_snapshot.
  //!
  //! \note Valid in #kStateMutable.
  void InitializeFromSnapshot(const ThreadSnapshot* thread_snapshot,
                              const MinidumpThreadIDMap& thread_id_map);

  //! \brief Sets the ThreadId for MINIDUMP_THREAD_NAME::ThreadId.
  void SetThreadId(uint32_t thread_id) { thread_id_ = thread_id; }

  //! \brief Gets the ThreadId for MINIDUMP_THREAD_NAME::ThreadId.
  //!
  //! \note Valid in #kStateWritable.
  uint32_t ThreadId() const;

  //! \brief Sets MINIDUMP_THREAD_NAME::RvaOfThreadName.
  void SetThreadName(const std::string& thread_name);

  //! \brief Returns an RVA64 which has been updated with the relative address
  //!    of the thread name.
  //!
  //! This method is expected to be called by a MinidumpThreadNameListWriter in
  //! order to obtain the RVA64 of the thread name.
  //!
  //! \note Valid in #kStateWritable.
  RVA64 RvaOfThreadName() const;

 private:
  // MinidumpWritable:
  bool Freeze() override;
  size_t SizeOfObject() override;
  std::vector<MinidumpWritable*> Children() override;
  bool WriteObject(FileWriterInterface* file_writer) override;

  // This exists as a separate field so MinidumpWritable::RegisterRVA() can be
  // used on a guaranteed-aligned pointer (MINIDUMP_THREAD_NAME::RvaOfThreadName
  // is not 64-bit aligned, causing issues on ARM).
  RVA64 rva_of_thread_name_;

  // Although this class manages the data for a MINIDUMP_THREAD_NAME, it does
  // not directly hold a MINIDUMP_THREAD_NAME, as that struct contains a
  // non-aligned RVA64 field which prevents it use with
  // MinidumpWritable::RegisterRVA().
  //
  // Instead, this class individually holds the fields of the
  // MINIDUMP_THREAD_NAME which are fetched by MinidumpThreadNameListWriter.
  uint32_t thread_id_;

  std::unique_ptr<internal::MinidumpUTF16StringWriter> name_;
};

//! \brief The writer for a MINIDUMP_THREAD_NAME_LIST stream in a minidump file,
//!     containing a list of MINIDUMP_THREAD_NAME objects.
class MinidumpThreadNameListWriter final
    : public internal::MinidumpStreamWriter {
 public:
  MinidumpThreadNameListWriter();

  MinidumpThreadNameListWriter(const MinidumpThreadNameListWriter&) = delete;
  MinidumpThreadNameListWriter& operator=(const MinidumpThreadNameListWriter&) =
      delete;

  ~MinidumpThreadNameListWriter() override;

  //! \brief Adds an initialized MINIDUMP_THREAD_NAME for each thread in \a
  //!     thread_snapshots to the MINIDUMP_THREAD_NAME_LIST.
  //!
  //! \param[in] thread_snapshots The thread snapshots to use as source data.
  //! \param[in] thread_id_map A MinidumpThreadIDMap previously built by
  //!     MinidumpThreadListWriter::InitializeFromSnapshot().
  //!
  //! \note Valid in #kStateMutable.
  void InitializeFromSnapshot(
      const std::vector<const ThreadSnapshot*>& thread_snapshots,
      const MinidumpThreadIDMap& thread_id_map);

  //! \brief Adds a MinidumpThreadNameWriter to the MINIDUMP_THREAD_LIST.
  //!
  //! This object takes ownership of \a thread_name and becomes its parent in
  //! the overall tree of internal::MinidumpWritable objects.
  //!
  //! \note Valid in #kStateMutable.
  void AddThreadName(std::unique_ptr<MinidumpThreadNameWriter> thread_name);

 private:
  // MinidumpWritable:
  bool Freeze() override;
  size_t SizeOfObject() override;
  std::vector<MinidumpWritable*> Children() override;
  bool WriteObject(FileWriterInterface* file_writer) override;

  // MinidumpStreamWriter:
  MinidumpStreamType StreamType() const override;

  std::vector<std::unique_ptr<MinidumpThreadNameWriter>> thread_names_;
  MINIDUMP_THREAD_NAME_LIST thread_name_list_;
};

}  // namespace crashpad

#endif  // CRASHPAD_MINIDUMP_MINIDUMP_THREAD_NAME_LIST_WRITER_H_
