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
// fast_source_line_resolver.h: FastSourceLineResolver is derived from
// SourceLineResolverBase, and is a concrete implementation of
// SourceLineResolverInterface.
//
// FastSourceLineResolver is a sibling class of BasicSourceLineResolver.  The
// difference is FastSourceLineResolver loads a serialized memory chunk of data
// which can be used directly a Module without parsing or copying of underlying
// data.  Therefore loading a symbol in FastSourceLineResolver is much faster
// and more memory-efficient than BasicSourceLineResolver.
//
// See "source_line_resolver_base.h" and
// "google_breakpad/source_line_resolver_interface.h" for more reference.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef GOOGLE_BREAKPAD_PROCESSOR_FAST_SOURCE_LINE_RESOLVER_H__
#define GOOGLE_BREAKPAD_PROCESSOR_FAST_SOURCE_LINE_RESOLVER_H__

#include <map>
#include <string>

#include "google_breakpad/processor/source_line_resolver_base.h"

namespace google_breakpad {

using std::map;

class FastSourceLineResolver : public SourceLineResolverBase {
 public:
  FastSourceLineResolver();
  virtual ~FastSourceLineResolver() { }

  using SourceLineResolverBase::FillSourceLineInfo;
  using SourceLineResolverBase::FindCFIFrameInfo;
  using SourceLineResolverBase::FindWindowsFrameInfo;
  using SourceLineResolverBase::HasModule;
  using SourceLineResolverBase::IsModuleCorrupt;
  using SourceLineResolverBase::LoadModule;
  using SourceLineResolverBase::LoadModuleUsingMapBuffer;
  using SourceLineResolverBase::LoadModuleUsingMemoryBuffer;
  using SourceLineResolverBase::UnloadModule;

 private:
  // Friend declarations.
  friend class ModuleComparer;
  friend class ModuleSerializer;
  friend class FastModuleFactory;

  // Nested types that will derive from corresponding nested types defined in
  // SourceLineResolverBase.
  struct Line;
  struct Function;
  struct Inline;
  struct InlineOrigin;
  struct PublicSymbol;
  class Module;

  // Deserialize raw memory data to construct a WindowsFrameInfo object.
  static WindowsFrameInfo CopyWFI(const char *raw_memory);

  // FastSourceLineResolver requires the memory buffer stays alive during the
  // lifetime of a corresponding module, therefore it needs to redefine this
  // virtual method.
  virtual bool ShouldDeleteMemoryBufferAfterLoadModule();

  // Disallow unwanted copy ctor and assignment operator
  FastSourceLineResolver(const FastSourceLineResolver&);
  void operator=(const FastSourceLineResolver&);
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_FAST_SOURCE_LINE_RESOLVER_H__
