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
//
// module_serializer.h: ModuleSerializer serializes a loaded symbol,
// i.e., a loaded BasicSouceLineResolver::Module instance, into a memory
// chunk of data. The serialized data can be read and loaded by
// FastSourceLineResolver without CPU & memory-intensive parsing.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_MODULE_SERIALIZER_H__
#define PROCESSOR_MODULE_SERIALIZER_H__

#include <stdint.h>

#include <map>
#include <string>

#include "google_breakpad/processor/basic_source_line_resolver.h"
#include "google_breakpad/processor/fast_source_line_resolver.h"
#include "processor/basic_source_line_resolver_types.h"
#include "processor/fast_source_line_resolver_types.h"
#include "processor/linked_ptr.h"
#include "processor/map_serializers-inl.h"
#include "processor/simple_serializer-inl.h"
#include "processor/windows_frame_info.h"

namespace google_breakpad {

// ModuleSerializer serializes a loaded BasicSourceLineResolver::Module into a
// chunk of memory data. ModuleSerializer also provides interface to compute
// memory size of the serialized data, write serialized data directly into
// memory, convert ASCII format symbol data into serialized binary data, and
// convert loaded BasicSourceLineResolver::Module into
// FastSourceLineResolver::Module.
class ModuleSerializer {
 public:
  // Compute the size of memory required to serialize a module.  Return the
  // total size needed for serialization.
  size_t SizeOf(const BasicSourceLineResolver::Module& module);

  // Write a module into an allocated memory chunk with required size.
  // Return the "end" of data, i.e., the address after the final byte of data.
  char* Write(const BasicSourceLineResolver::Module& module, char* dest);

  // Serializes a loaded Module object into a chunk of memory data and returns
  // the address of memory chunk.  If size != NULL, *size is set to the memory
  // size allocated for the serialized data.
  // Caller takes the ownership of the memory chunk (allocated on heap), and
  // owner should call delete [] to free the memory after use.
  char* Serialize(const BasicSourceLineResolver::Module& module,
                  size_t* size = nullptr);

  // Given the string format symbol_data, produces a chunk of serialized data.
  // Caller takes ownership of the serialized data (on heap), and owner should
  // call delete [] to free the memory after use.
  char* SerializeSymbolFileData(const string& symbol_data,
                                size_t* size = nullptr);

  // Serializes one loaded module with given moduleid in the basic source line
  // resolver, and loads the serialized data into the fast source line resolver.
  // Return false if the basic source line doesn't have a module with the given
  // moduleid.
  bool ConvertOneModule(const string& moduleid,
                        const BasicSourceLineResolver* basic_resolver,
                        FastSourceLineResolver* fast_resolver);

  // Serializes all the loaded modules in a basic source line resolver, and
  // loads the serialized data into a fast source line resolver.
  void ConvertAllModules(const BasicSourceLineResolver* basic_resolver,
                         FastSourceLineResolver* fast_resolver);

 private:
  // Convenient type names.
  typedef BasicSourceLineResolver::Line Line;
  typedef BasicSourceLineResolver::Function Function;
  typedef BasicSourceLineResolver::PublicSymbol PublicSymbol;
  typedef BasicSourceLineResolver::InlineOrigin InlineOrigin;

  // Internal implementation for ConvertOneModule and ConvertAllModules methods.
  bool SerializeModuleAndLoadIntoFastResolver(
      const BasicSourceLineResolver::ModuleMap::const_iterator& iter,
      FastSourceLineResolver* fast_resolver);

  // Number of Maps that Module class contains.
  static const int32_t kNumberMaps_ =
      FastSourceLineResolver::Module::kNumberMaps_;

  // Memory sizes required to serialize map components in Module.
  uint64_t map_sizes_[kNumberMaps_];

  // Serializers for each individual map component in Module class.
  StdMapSerializer<int, string> files_serializer_;
  RangeMapSerializer<MemAddr, linked_ptr<Function> > functions_serializer_;
  AddressMapSerializer<MemAddr, linked_ptr<PublicSymbol> > pubsym_serializer_;
  ContainedRangeMapSerializer<MemAddr,
                              linked_ptr<WindowsFrameInfo> > wfi_serializer_;
  RangeMapSerializer<MemAddr, string> cfi_init_rules_serializer_;
  StdMapSerializer<MemAddr, string> cfi_delta_rules_serializer_;
  StdMapSerializer<int, linked_ptr<InlineOrigin>> inline_origin_serializer_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_MODULE_SERIALIZER_H__
