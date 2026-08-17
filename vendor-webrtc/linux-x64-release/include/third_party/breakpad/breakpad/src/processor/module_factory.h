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
// module_factory.h: ModuleFactory a factory that provides
// an interface for creating a Module and deferring instantiation to subclasses
// BasicModuleFactory and FastModuleFactory.

// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_MODULE_FACTORY_H__
#define PROCESSOR_MODULE_FACTORY_H__

#include "processor/basic_source_line_resolver_types.h"
#include "processor/fast_source_line_resolver_types.h"
#include "processor/source_line_resolver_base_types.h"

namespace google_breakpad {

class ModuleFactory {
 public:
  virtual ~ModuleFactory() { };
  virtual SourceLineResolverBase::Module* CreateModule(
      const string& name) const = 0;
};

class BasicModuleFactory : public ModuleFactory {
 public:
  virtual ~BasicModuleFactory() { }
  virtual BasicSourceLineResolver::Module* CreateModule(
      const string& name) const {
    return new BasicSourceLineResolver::Module(name);
  }
};

class FastModuleFactory : public ModuleFactory {
 public:
  virtual ~FastModuleFactory() { }
  virtual FastSourceLineResolver::Module* CreateModule(
      const string& name) const {
    return new FastSourceLineResolver::Module(name);
  }
};

}  // namespace google_breakpad

#endif  // PROCESSOR_MODULE_FACTORY_H__
