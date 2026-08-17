// -*- mode: C++ -*-

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

// Original author: Jim Blandy <jimb@mozilla.com> <jimb@red-bean.com>

// synth_minidump.h: Interface to SynthMinidump: fake minidump generator.
//
// We treat a minidump file as the concatenation of a bunch of
// test_assembler::Sections. The file header, stream directory,
// streams, memory regions, strings, and so on --- each is a Section
// that eventually gets appended to the minidump. Dump, Memory,
// Context, Thread, and so on all inherit from test_assembler::Section.
// For example:
//
//    using google_breakpad::test_assembler::kLittleEndian;
//    using google_breakpad::SynthMinidump::Context;
//    using google_breakpad::SynthMinidump::Dump;
//    using google_breakpad::SynthMinidump::Memory;
//    using google_breakpad::SynthMinidump::Thread;
//    
//    Dump minidump(MD_NORMAL, kLittleEndian);
//    
//    Memory stack1(minidump, 0x569eb0a9);
//    ... build contents of stack1 with test_assembler::Section functions ...
//    
//    MDRawContextX86 x86_context1;
//    x86_context1.context_flags = MD_CONTEXT_X86;
//    x86_context1.eip = 0x7c90eb94;
//    x86_context1.esp = 0x569eb0a9;
//    x86_context1.ebp = x86_context1.esp + something appropriate;
//    Context context1(minidump, x86_context1);
//    
//    Thread thread1(minidump, 0xe4a4821d, stack1, context1);
//    
//    minidump.Add(&stack1);
//    minidump.Add(&context1);
//    minidump.Add(&thread1);
//    minidump.Finish();
//    
//    string contents;
//    EXPECT_TRUE(minidump.GetContents(&contents));
//    // contents now holds the bytes of a minidump file
//
// Because the test_assembler classes let us write Label references to
// sections before the Labels' values are known, this gives us
// flexibility in how we put the dump together: minidump pieces can
// hold the file offsets of other minidump pieces before the
// referents' positions have been decided. As long as everything has
// been placed by the time we call dump.GetContents to obtain the
// bytes, all the Labels' values will be known, and everything will
// get patched up appropriately.
//   
// The dump.Add(thing) functions append THINGS's contents to the
// minidump, but they also do two other things:
//
// - dump.Add(thing) invokes thing->Finish, which tells *thing the
//   offset within the file at which it was placed, and allows *thing
//   to do any final content generation.
//
// - If THING is something which should receive an entry in some sort
//   of list or directory, then dump.Add(THING) automatically creates
//   the appropriate directory or list entry. Streams must appear in
//   the stream directory; memory ranges should be listed in the
//   memory list; threads should be placed in the thread list; and so
//   on.
//
// By convention, Section subclass constructors that take references
// to other Sections do not take care of 'Add'ing their arguments to
// the dump. For example, although the Thread constructor takes
// references to a Memory and a Context, it does not add them to the
// dump on the caller's behalf. Rather, the caller is responsible for
// 'Add'ing every section they create. This allows Sections to be
// cited from more than one place; for example, Memory ranges are
// cited both from Thread objects (as their stack contents) and by the
// memory list stream.
//
// If you forget to Add some Section, the Dump::GetContents call will
// fail, as the test_assembler::Labels used to cite the Section's
// contents from elsewhere will still be undefined.
#ifndef PROCESSOR_SYNTH_MINIDUMP_H_
#define PROCESSOR_SYNTH_MINIDUMP_H_

#include <assert.h>

#include <iostream>
#include <string>

#include "common/test_assembler.h"
#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/common/minidump_format.h"

namespace google_breakpad {

namespace SynthMinidump {

using test_assembler::Endianness;
using test_assembler::kBigEndian;
using test_assembler::kLittleEndian;
using test_assembler::kUnsetEndian;
using test_assembler::Label;

class Dump;
class Memory;
class String;

// A test_assembler::Section which will be appended to a minidump.
class Section: public test_assembler::Section {
 public:
  explicit Section(const Dump& dump);

  // Append an MDLocationDescriptor referring to this section to SECTION.
  // If 'this' is NULL, append a descriptor with a zero length and MDRVA.
  //
  // (I couldn't find the language in the C++ standard that says that
  // invoking member functions of a NULL pointer to a class type is
  // bad, if such language exists. Having this function handle NULL
  // 'this' is convenient, but if it causes trouble, it's not hard to
  // do differently.)
  void CiteLocationIn(test_assembler::Section* section) const;

  // Note that this section's contents are complete, and that it has
  // been placed in the minidump file at OFFSET. The 'Add' member
  // functions call the Finish member function of the object being
  // added for you; if you are 'Add'ing this section, you needn't Finish it.
  virtual void Finish(const Label& offset) { 
    file_offset_ = offset; size_ = Size();
  }

 protected:
  // This section's size and offset within the minidump file.
  Label file_offset_, size_;
};

// A stream within a minidump file. 'Add'ing a stream to a minidump
// creates an entry for it in the minidump's stream directory.
class Stream: public Section {
 public:
  // Create a stream of type TYPE.  You can append whatever contents
  // you like to this stream using the test_assembler::Section methods.
  Stream(const Dump& dump, uint32_t type) : Section(dump), type_(type) { }

  // Append an MDRawDirectory referring to this stream to SECTION.
  void CiteStreamIn(test_assembler::Section* section) const;

 private:
  // The type of this stream.
  uint32_t type_;
};

class SystemInfo: public Stream {
 public:
  // Create an MD_SYSTEM_INFO_STREAM stream belonging to DUMP holding
  // an MDRawSystem info structure initialized with the values from
  // SYSTEM_INFO, except that the csd_version field is replaced with
  // the file offset of the string CSD_VERSION, which can be 'Add'ed
  // to the dump at the desired location.
  // 
  // Remember that you are still responsible for 'Add'ing CSD_VERSION
  // to the dump yourself.
  SystemInfo(const Dump& dump,
             const MDRawSystemInfo& system_info,
             const String& csd_version);

  // Stock MDRawSystemInfo information and associated strings, for
  // writing tests.
  static const MDRawSystemInfo windows_x86;
  static const string windows_x86_csd_version;
};

// An MDString: a string preceded by a 32-bit length.
class String: public Section {
 public:
  String(const Dump& dump, const string& value);

  // Append an MDRVA referring to this string to SECTION.
  void CiteStringIn(test_assembler::Section* section) const;
};

// A range of memory contents. 'Add'ing a memory range to a minidump
// creates n entry for it in the minidump's memory list. By
// convention, the 'start', 'Here', and 'Mark' member functions refer
// to memory addresses.
class Memory: public Section {
 public:
  Memory(const Dump& dump, uint64_t address)
      : Section(dump), address_(address) { start() = address; }

  // Append an MDMemoryDescriptor referring to this memory range to SECTION.
  void CiteMemoryIn(test_assembler::Section* section) const;

 private:
  // The process address from which these memory contents were taken.
  // Shouldn't this be a Label?
  uint64_t address_;
};

class Context: public Section {
 public:
  // Create a context belonging to DUMP whose contents are a copy of CONTEXT.
  Context(const Dump& dump, const MDRawContextX86& context);
  Context(const Dump& dump, const MDRawContextARM& context);
  Context(const Dump& dump, const MDRawContextMIPS& context);
  // Add an empty context to the dump.
  Context(const Dump& dump) : Section(dump) {}
  // Add constructors for other architectures here. Remember to byteswap.
};

class Thread: public Section {
 public:
  // Create a thread belonging to DUMP with the given values, citing
  // STACK and CONTEXT (which you must Add to the dump separately).
  Thread(const Dump& dump,
         uint32_t thread_id,
         const Memory& stack,
         const Context& context,
         uint32_t suspend_count = 0,
         uint32_t priority_class = 0,
         uint32_t priority = 0,
         uint64_t teb = 0);
};

class Module: public Section {
 public:
  // Create a module with the given values. Note that CV_RECORD and
  // MISC_RECORD can be NULL, in which case the corresponding location
  // descriptior in the minidump will have a length of zero.
  Module(const Dump& dump,
         uint64_t base_of_image,
         uint32_t size_of_image,
         const String& name,
         uint32_t time_date_stamp = 1262805309,
         uint32_t checksum = 0,
         const MDVSFixedFileInfo& version_info = Module::stock_version_info,
         const Section* cv_record = nullptr,
         const Section* misc_record = nullptr);

 private:
  // A standard MDVSFixedFileInfo structure to use as a default for
  // minidumps.  There's no reason to make users write out all this crap
  // over and over.
  static const MDVSFixedFileInfo stock_version_info;
};

class UnloadedModule: public Section {
 public:
  UnloadedModule(const Dump& dump,
                 uint64_t base_of_image,
                 uint32_t size_of_image,
                 const String& name,
                 uint32_t checksum = 0,
                 uint32_t time_date_stamp = 1262805309);
};

class Exception : public Stream {
public:
  Exception(const Dump& dump,
            const Context& context,
            uint32_t thread_id = 0,
            uint32_t exception_code = 0,
            uint32_t exception_flags = 0,
            uint64_t exception_address = 0);
};

// A list of entries starting with a 32-bit count, like a memory list
// or a thread list.
template<typename Element>
class List: public Stream {
 public:
  List(const Dump& dump, uint32_t type) : Stream(dump, type), count_(0) {
    D32(count_label_);
  }

  // Add ELEMENT to this list.
  void Add(Element* element) {
    element->Finish(file_offset_ + Size());
    Append(*element);
    count_++;
  }

  // Return true if this List is empty, false otherwise.
  bool Empty() { return count_ == 0; }

  // Finish up the contents of this section, mark it as having been
  // placed at OFFSET.
  virtual void Finish(const Label& offset) {
    Stream::Finish(offset);
    count_label_ = count_;
  }

 private:
  size_t count_;

 protected:
  // This constructor allows derived lists to specify their own layout
  // rather than starting with count as specified in the public constructor.
  List(const Dump& dump, uint32_t type, bool) : Stream(dump, type), count_(0) {}

  Label count_label_;
};

class UnloadedModuleList : public List<UnloadedModule> {
 public:
  UnloadedModuleList(const Dump& dump, uint32_t type);
};

class Dump: public test_assembler::Section {
 public:

  // Create a test_assembler::Section containing a minidump file whose
  // header uses the given values. ENDIANNESS determines the
  // endianness of the signature; we set this section's default
  // endianness by this.
  Dump(uint64_t flags,
       Endianness endianness = kLittleEndian,
       uint32_t version = MD_HEADER_VERSION,
       uint32_t date_time_stamp = 1262805309);

  // The following functions call OBJECT->Finish(), and append the
  // contents of OBJECT to this minidump. They also record OBJECT in
  // whatever directory or list is appropriate for its type. The
  // stream directory, memory list, thread list, and module list are
  // accumulated this way.
  Dump& Add(SynthMinidump::Section* object); // simply append data
  Dump& Add(Stream* object); // append, record in stream directory
  Dump& Add(Memory* object); // append, record in memory list
  Dump& Add(Thread* object); // append, record in thread list
  Dump& Add(Module* object); // append, record in module list
  Dump& Add(UnloadedModule* object); // append, record in unloaded module list

  // Complete the construction of the minidump, given the Add calls
  // we've seen up to this point. After this call, this Dump's
  // contents are complete, all labels should be defined if everything
  // Cited has been Added, and you may call GetContents on it.
  void Finish();

 private:
  // A label representing the start of the minidump file.
  Label file_start_;

  // The stream directory.  We construct this incrementally from
  // Add(Stream*) calls.
  SynthMinidump::Section stream_directory_; // The directory's contents.
  size_t stream_count_;                 // The number of streams so far.
  Label stream_count_label_;            // Cited in file header.
  Label stream_directory_rva_;          // The directory's file offset.

  // This minidump's thread list. We construct this incrementally from
  // Add(Thread*) calls.
  List<Thread> thread_list_;

  // This minidump's module list. We construct this incrementally from
  // Add(Module*) calls.
  List<Module> module_list_;

  // This minidump's unloaded module list. We construct this incrementally from
  // Add(UnloadedModule*) calls.
  UnloadedModuleList unloaded_module_list_;

  // This minidump's memory list. We construct this incrementally from
  // Add(Memory*) calls. This is actually a list of MDMemoryDescriptors,
  // not memory ranges --- thus the odd type.
  List<SynthMinidump::Section> memory_list_;
};

} // namespace SynthMinidump

} // namespace google_breakpad

#endif  // PROCESSOR_SYNTH_MINIDUMP_H_
