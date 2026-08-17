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
// module_comparer.h: ModuleComparer reads a string format of symbol file, and
// loads the symbol into both BasicSourceLineResolver::Module and
// FastSourceLineResolve::Module.  It then traverses both Modules and compare
// the content of data to verify the correctness of new fast module.
// ModuleCompare class is a tool to verify correctness of a loaded
// FastSourceLineResolver::Module instance, i.e., in-memory representation of
// parsed symbol.  ModuleComparer class should be used for testing purpose only,
// e.g., in fast_source_line_resolver_unittest.
//
// Author: lambxsy@google.com (Siyang Xie)

#ifndef PROCESSOR_MODULE_COMPARER_H__
#define PROCESSOR_MODULE_COMPARER_H__

#include <string>

#include "processor/basic_source_line_resolver_types.h"
#include "processor/fast_source_line_resolver_types.h"
#include "processor/module_serializer.h"
#include "processor/windows_frame_info.h"

namespace google_breakpad {

class ModuleComparer {
 public:
  ModuleComparer(): fast_resolver_(new FastSourceLineResolver),
                   basic_resolver_(new BasicSourceLineResolver) { }
  ~ModuleComparer() {
    delete fast_resolver_;
    delete basic_resolver_;
  }

  // BasicSourceLineResolver loads its module using the symbol data,
  // ModuleSerializer serialize the loaded module into a memory chunk,
  // FastSourceLineResolver loads its module using the serialized memory chunk,
  // Then, traverse both modules together and compare underlying data
  // return true if both modules contain exactly same data.
  bool Compare(const string& symbol_data);

 private:
  typedef BasicSourceLineResolver::Module BasicModule;
  typedef FastSourceLineResolver::Module FastModule;
  typedef BasicSourceLineResolver::Function BasicFunc;
  typedef FastSourceLineResolver::Function FastFunc;
  typedef BasicSourceLineResolver::Line BasicLine;
  typedef FastSourceLineResolver::Line FastLine;
  typedef BasicSourceLineResolver::PublicSymbol BasicPubSymbol;
  typedef FastSourceLineResolver::PublicSymbol FastPubSymbol;
  typedef WindowsFrameInfo WFI;

  bool CompareModule(const BasicModule *oldmodule,
                     const FastModule *newmodule) const;
  bool CompareFunction(const BasicFunc *oldfunc, const FastFunc *newfunc) const;
  bool CompareLine(const BasicLine *oldline, const FastLine *newline) const;
  bool ComparePubSymbol(const BasicPubSymbol*, const FastPubSymbol*) const;
  bool CompareWFI(const WindowsFrameInfo&, const WindowsFrameInfo&) const;

  // Compare ContainedRangeMap
  bool CompareCRM(const ContainedRangeMap<MemAddr, linked_ptr<WFI> >*,
                  const StaticContainedRangeMap<MemAddr, char>*) const;

  FastSourceLineResolver *fast_resolver_;
  BasicSourceLineResolver *basic_resolver_;
  ModuleSerializer serializer_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_MODULE_COMPARER_H__
