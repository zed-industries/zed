// Copyright 2014 The Crashpad Authors
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

#ifndef CRASHPAD_MINIDUMP_MINIDUMP_CONTEXT_WRITER_H_
#define CRASHPAD_MINIDUMP_MINIDUMP_CONTEXT_WRITER_H_

#include <sys/types.h>

#include <memory>

#include "minidump/minidump_context.h"
#include "minidump/minidump_writable.h"

namespace crashpad {

struct CPUContext;
struct CPUContextX86;
struct CPUContextX86_64;
class MinidumpMiscInfoWriter;

//! \brief The base class for writers of CPU context structures in minidump
//!     files.
class MinidumpContextWriter : public internal::MinidumpWritable {
 public:
  MinidumpContextWriter(const MinidumpContextWriter&) = delete;
  MinidumpContextWriter& operator=(const MinidumpContextWriter&) = delete;

  ~MinidumpContextWriter() override;

  //! \brief Creates a MinidumpContextWriter based on \a context_snapshot.
  //!
  //! \param[in] context_snapshot The context snapshot to use as source data.
  //!
  //! \return A MinidumpContextWriter subclass, such as MinidumpContextWriterX86
  //!     or MinidumpContextWriterAMD64, appropriate to the CPU type of \a
  //!     context_snapshot. The returned object is initialized using the source
  //!     data in \a context_snapshot. If \a context_snapshot is an unknown CPU
  //!     type’s context, logs a message and returns `nullptr`.
  static std::unique_ptr<MinidumpContextWriter> CreateFromSnapshot(
      const CPUContext* context_snapshot);

  //! \brief Returns the size of the context structure that this object will
  //!     write.
  //!
  //! \note This method will force this to #kStateFrozen, if it is not already.
  size_t FreezeAndGetSizeOfObject();

 protected:
  MinidumpContextWriter() : MinidumpWritable() {}

  //! \brief Returns the size of the context structure that this object will
  //!     write.
  //!
  //! \note This method will only be called in #kStateFrozen or a subsequent
  //!     state.
  virtual size_t ContextSize() const = 0;

  // MinidumpWritable:
  size_t SizeOfObject() final;
};

//! \brief The writer for a MinidumpContextX86 structure in a minidump file.
class MinidumpContextX86Writer final : public MinidumpContextWriter {
 public:
  MinidumpContextX86Writer();

  MinidumpContextX86Writer(const MinidumpContextX86Writer&) = delete;
  MinidumpContextX86Writer& operator=(const MinidumpContextX86Writer&) = delete;

  ~MinidumpContextX86Writer() override;

  //! \brief Initializes the MinidumpContextX86 based on \a context_snapshot.
  //!
  //! \param[in] context_snapshot The context snapshot to use as source data.
  //!
  //! \note Valid in #kStateMutable. No mutation of context() may be done before
  //!     calling this method, and it is not normally necessary to alter
  //!     context() after calling this method.
  void InitializeFromSnapshot(const CPUContextX86* context_snapshot);

  //! \brief Returns a pointer to the context structure that this object will
  //!     write.
  //!
  //! \attention This returns a non-`const` pointer to this object’s private
  //!     data so that a caller can populate the context structure directly.
  //!     This is done because providing setter interfaces to each field in the
  //!     context structure would be unwieldy and cumbersome. Care must be taken
  //!     to populate the context structure correctly. The context structure
  //!     must only be modified while this object is in the #kStateMutable
  //!     state.
  MinidumpContextX86* context() { return &context_; }

 protected:
  // MinidumpWritable:
  bool WriteObject(FileWriterInterface* file_writer) override;

  // MinidumpContextWriter:
  size_t ContextSize() const override;

 private:
  MinidumpContextX86 context_;
};

//! \brief Wraps an xsave feature that knows where and how big it is.
class MinidumpXSaveFeatureAMD64 {
 public:
  virtual ~MinidumpXSaveFeatureAMD64() = default;
  // Number of bytes that will be written. May need to vary by CPUID (see
  // Intel 13.5).
  virtual size_t Size() const = 0;
  // Intel 13.4.2 XCOMP_BV.
  virtual uint8_t XCompBVBit() const = 0;
  // Write data to dst. Does not write padding.
  virtual bool Copy(void* dst) const = 0;
};

//! \brief XSAVE_CET_U_FORMAT
class MinidumpXSaveAMD64CetU final : public MinidumpXSaveFeatureAMD64 {
 public:
  MinidumpXSaveAMD64CetU() {}
  ~MinidumpXSaveAMD64CetU() {}
  MinidumpXSaveAMD64CetU(const MinidumpXSaveAMD64CetU&) = delete;
  MinidumpXSaveAMD64CetU& operator=(const MinidumpXSaveAMD64CetU&) = delete;

  size_t Size() const override { return sizeof(cet_u_); }
  uint8_t XCompBVBit() const override { return XSTATE_CET_U; }
  bool Copy(void* dst) const override;
  bool InitializeFromSnapshot(const CPUContextX86_64* context_snapshot);

 private:
  MinidumpAMD64XSaveFormatCetU cet_u_;
};

//! \brief The writer for a MinidumpContextAMD64 structure in a minidump file.
class MinidumpContextAMD64Writer final : public MinidumpContextWriter {
 public:
  MinidumpContextAMD64Writer();

  MinidumpContextAMD64Writer(const MinidumpContextAMD64Writer&) = delete;
  MinidumpContextAMD64Writer& operator=(const MinidumpContextAMD64Writer&) =
      delete;

  ~MinidumpContextAMD64Writer() override;

  // Ensure proper alignment of heap-allocated objects. This should not be
  // necessary in C++17.
  static void* operator new(size_t size);
  static void operator delete(void* ptr);

  // Prevent unaligned heap-allocated arrays. Provisions could be made to allow
  // these if necessary, but there is currently no use for them.
  static void* operator new[](size_t size) = delete;
  static void operator delete[](void* ptr) = delete;

  //! \brief Initializes the MinidumpContextAMD64 based on \a context_snapshot.
  //!
  //! \param[in] context_snapshot The context snapshot to use as source data.
  //!
  //! \note Valid in #kStateMutable. No mutation of context() may be done before
  //!     calling this method, and it is not normally necessary to alter
  //!     context() after calling this method.
  void InitializeFromSnapshot(const CPUContextX86_64* context_snapshot);

  //! \brief Returns a pointer to the context structure that this object will
  //!     write.
  //!
  //! \attention This returns a non-`const` pointer to this object’s private
  //!     data so that a caller can populate the context structure directly.
  //!     This is done because providing setter interfaces to each field in the
  //!     context structure would be unwieldy and cumbersome. Care must be taken
  //!     to populate the context structure correctly. The context structure
  //!     must only be modified while this object is in the #kStateMutable
  //!     state.
  MinidumpContextAMD64* context() { return &context_; }

 protected:
  // MinidumpWritable:
  size_t Alignment() override;
  bool WriteObject(FileWriterInterface* file_writer) override;

  // MinidumpContextWriter:
  size_t ContextSize() const override;

 private:
  MinidumpContextAMD64 context_;
  // These should be in order of XCompBVBit().
  std::vector<std::unique_ptr<MinidumpXSaveFeatureAMD64>> xsave_entries_;
};

//! \brief The writer for a MinidumpContextARM structure in a minidump file.
class MinidumpContextARMWriter final : public MinidumpContextWriter {
 public:
  MinidumpContextARMWriter();

  MinidumpContextARMWriter(const MinidumpContextARMWriter&) = delete;
  MinidumpContextARMWriter& operator=(const MinidumpContextARMWriter&) = delete;

  ~MinidumpContextARMWriter() override;

  //! \brief Initializes the MinidumpContextARM based on \a context_snapshot.
  //!
  //! \param[in] context_snapshot The context snapshot to use as source data.
  //!
  //! \note Valid in #kStateMutable. No mutation of context() may be done before
  //!     calling this method, and it is not normally necessary to alter
  //!     context() after calling this method.
  void InitializeFromSnapshot(const CPUContextARM* context_snapshot);

  //! \brief Returns a pointer to the context structure that this object will
  //!     write.
  //!
  //! \attention This returns a non-`const` pointer to this object’s private
  //!     data so that a caller can populate the context structure directly.
  //!     This is done because providing setter interfaces to each field in the
  //!     context structure would be unwieldy and cumbersome. Care must be taken
  //!     to populate the context structure correctly. The context structure
  //!     must only be modified while this object is in the #kStateMutable
  //!     state.
  MinidumpContextARM* context() { return &context_; }

 protected:
  // MinidumpWritable:
  bool WriteObject(FileWriterInterface* file_writer) override;

  // MinidumpContextWriter:
  size_t ContextSize() const override;

 private:
  MinidumpContextARM context_;
};

//! \brief The writer for a MinidumpContextARM64 structure in a minidump file.
class MinidumpContextARM64Writer final : public MinidumpContextWriter {
 public:
  MinidumpContextARM64Writer();

  MinidumpContextARM64Writer(const MinidumpContextARM64Writer&) = delete;
  MinidumpContextARM64Writer& operator=(const MinidumpContextARM64Writer&) =
      delete;

  ~MinidumpContextARM64Writer() override;

  //! \brief Initializes the MinidumpContextARM64 based on \a context_snapshot.
  //!
  //! \param[in] context_snapshot The context snapshot to use as source data.
  //!
  //! \note Valid in #kStateMutable. No mutation of context() may be done before
  //!     calling this method, and it is not normally necessary to alter
  //!     context() after calling this method.
  void InitializeFromSnapshot(const CPUContextARM64* context_snapshot);

  //! \brief Returns a pointer to the context structure that this object will
  //!     write.
  //!
  //! \attention This returns a non-`const` pointer to this object’s private
  //!     data so that a caller can populate the context structure directly.
  //!     This is done because providing setter interfaces to each field in the
  //!     context structure would be unwieldy and cumbersome. Care must be taken
  //!     to populate the context structure correctly. The context structure
  //!     must only be modified while this object is in the #kStateMutable
  //!     state.
  MinidumpContextARM64* context() { return &context_; }

 protected:
  // MinidumpWritable:
  bool WriteObject(FileWriterInterface* file_writer) override;

  // MinidumpContextWriter:
  size_t ContextSize() const override;

 private:
  MinidumpContextARM64 context_;
};

//! \brief The writer for a MinidumpContextMIPS structure in a minidump file.
class MinidumpContextMIPSWriter final : public MinidumpContextWriter {
 public:
  MinidumpContextMIPSWriter();

  MinidumpContextMIPSWriter(const MinidumpContextMIPSWriter&) = delete;
  MinidumpContextMIPSWriter& operator=(const MinidumpContextMIPSWriter&) =
      delete;

  ~MinidumpContextMIPSWriter() override;

  //! \brief Initializes the MinidumpContextMIPS based on \a context_snapshot.
  //!
  //! \param[in] context_snapshot The context snapshot to use as source data.
  //!
  //! \note Valid in #kStateMutable. No mutation of context() may be done before
  //!     calling this method, and it is not normally necessary to alter
  //!     context() after calling this method.
  void InitializeFromSnapshot(const CPUContextMIPS* context_snapshot);

  //! \brief Returns a pointer to the context structure that this object will
  //!     write.
  //!
  //! \attention This returns a non-`const` pointer to this object’s private
  //!     data so that a caller can populate the context structure directly.
  //!     This is done because providing setter interfaces to each field in the
  //!     context structure would be unwieldy and cumbersome. Care must be taken
  //!     to populate the context structure correctly. The context structure
  //!     must only be modified while this object is in the #kStateMutable
  //!     state.
  MinidumpContextMIPS* context() { return &context_; }

 protected:
  // MinidumpWritable:
  bool WriteObject(FileWriterInterface* file_writer) override;

  // MinidumpContextWriter:
  size_t ContextSize() const override;

 private:
  MinidumpContextMIPS context_;
};

//! \brief The writer for a MinidumpContextMIPS64 structure in a minidump file.
class MinidumpContextMIPS64Writer final : public MinidumpContextWriter {
 public:
  MinidumpContextMIPS64Writer();

  MinidumpContextMIPS64Writer(const MinidumpContextMIPS64Writer&) = delete;
  MinidumpContextMIPS64Writer& operator=(const MinidumpContextMIPS64Writer&) =
      delete;

  ~MinidumpContextMIPS64Writer() override;

  //! \brief Initializes the MinidumpContextMIPS based on \a context_snapshot.
  //!
  //! \param[in] context_snapshot The context snapshot to use as source data.
  //!
  //! \note Valid in #kStateMutable. No mutation of context() may be done before
  //!     calling this method, and it is not normally necessary to alter
  //!     context() after calling this method.
  void InitializeFromSnapshot(const CPUContextMIPS64* context_snapshot);

  //! \brief Returns a pointer to the context structure that this object will
  //!     write.
  //!
  //! \attention This returns a non-`const` pointer to this object’s private
  //!     data so that a caller can populate the context structure directly.
  //!     This is done because providing setter interfaces to each field in the
  //!     context structure would be unwieldy and cumbersome. Care must be taken
  //!     to populate the context structure correctly. The context structure
  //!     must only be modified while this object is in the #kStateMutable
  //!     state.
  MinidumpContextMIPS64* context() { return &context_; }

 protected:
  // MinidumpWritable:
  bool WriteObject(FileWriterInterface* file_writer) override;

  // MinidumpContextWriter:
  size_t ContextSize() const override;

 private:
  MinidumpContextMIPS64 context_;
};

//! \brief The writer for a MinidumpContextRISCV64 structure in a minidump file.
class MinidumpContextRISCV64Writer final : public MinidumpContextWriter {
 public:
  MinidumpContextRISCV64Writer();

  MinidumpContextRISCV64Writer(const MinidumpContextRISCV64Writer&) = delete;
  MinidumpContextRISCV64Writer& operator=(const MinidumpContextRISCV64Writer&) =
      delete;

  ~MinidumpContextRISCV64Writer() override;

  //! \brief Initializes the MinidumpContextRISCV64 based on \a
  //! context_snapshot.
  //!
  //! \param[in] context_snapshot The context snapshot to use as source data.
  //!
  //! \note Valid in #kStateMutable. No mutation of context() may be done before
  //!     calling this method, and it is not normally necessary to alter
  //!     context() after calling this method.
  void InitializeFromSnapshot(const CPUContextRISCV64* context_snapshot);

  //! \brief Returns a pointer to the context structure that this object will
  //!     write.
  //!
  //! \attention This returns a non-`const` pointer to this object’s private
  //!     data so that a caller can populate the context structure directly.
  //!     This is done because providing setter interfaces to each field in the
  //!     context structure would be unwieldy and cumbersome. Care must be taken
  //!     to populate the context structure correctly. The context structure
  //!     must only be modified while this object is in the #kStateMutable
  //!     state.
  MinidumpContextRISCV64* context() { return &context_; }

 protected:
  // MinidumpWritable:
  bool WriteObject(FileWriterInterface* file_writer) override;

  // MinidumpContextWriter:
  size_t ContextSize() const override;

 private:
  MinidumpContextRISCV64 context_;
};

}  // namespace crashpad

#endif  // CRASHPAD_MINIDUMP_MINIDUMP_CONTEXT_WRITER_H_
