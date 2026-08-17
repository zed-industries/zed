// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_CONTEXT_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_CONTEXT_H__

#include <algorithm>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/log/absl_log.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {
// Marks which kernel the Rust codegen should generate code for.
enum class Kernel {
  kUpb,
  kCpp,
};

inline absl::string_view KernelRsName(Kernel kernel) {
  switch (kernel) {
    case Kernel::kUpb:
      return "upb";
    case Kernel::kCpp:
      return "cpp";
    default:
      ABSL_LOG(FATAL) << "Unknown kernel type: " << static_cast<int>(kernel);
      return "";
  }
}

// Global options for a codegen invocation.
struct Options {
  Kernel kernel;
  std::string mapping_file_path;
  bool strip_nonfunctional_codegen = false;

  // The name to use for the generated entry point rs file.
  std::string generated_entry_point_rs_file_name = "generated.rs";

  static absl::StatusOr<Options> Parse(absl::string_view param);
};

class RustGeneratorContext {
 public:
  explicit RustGeneratorContext(
      const std::vector<const FileDescriptor*>* files_in_current_crate,
      const absl::flat_hash_map<std::string, std::string>*
          import_path_to_crate_name)
      : files_in_current_crate_(*files_in_current_crate),
        import_path_to_crate_name_(*import_path_to_crate_name) {}

  const FileDescriptor& primary_file() const {
    return *files_in_current_crate_.front();
  }

  bool is_file_in_current_crate(const FileDescriptor& f) const {
    return std::find(files_in_current_crate_.begin(),
                     files_in_current_crate_.end(),
                     &f) != files_in_current_crate_.end();
  }

 private:
  const std::vector<const FileDescriptor*>& files_in_current_crate_;
  const absl::flat_hash_map<std::string, std::string>&
      import_path_to_crate_name_;

  friend class Context;
};

// A context for generating a particular kind of definition.
class Context {
 public:
  Context(const Options* opts,
          const RustGeneratorContext* rust_generator_context,
          io::Printer* printer, std::vector<std::string> modules)
      : opts_(opts),
        rust_generator_context_(rust_generator_context),
        printer_(printer),
        modules_(std::move(modules)) {}

  Context(const Context&) = delete;
  Context& operator=(const Context&) = delete;
  Context(Context&&) = default;
  Context& operator=(Context&&) = default;

  const Options& opts() const { return *opts_; }
  const RustGeneratorContext& generator_context() const {
    return *rust_generator_context_;
  }

  bool is_cpp() const { return opts_->kernel == Kernel::kCpp; }
  bool is_upb() const { return opts_->kernel == Kernel::kUpb; }

  // NOTE: prefer ctx.Emit() over ctx.printer().Emit();
  io::Printer& printer() const { return *printer_; }

  Context WithPrinter(io::Printer* printer) const {
    return Context(opts_, rust_generator_context_, printer, modules_);
  }

  // Forwards to Emit(), which will likely be called all the time.
  void Emit(absl::string_view format,
            io::Printer::SourceLocation loc =
                io::Printer::SourceLocation::current()) const {
    printer_->Emit(format, loc);
  }
  void Emit(absl::Span<const io::Printer::Sub> vars, absl::string_view format,
            io::Printer::SourceLocation loc =
                io::Printer::SourceLocation::current()) const {
    printer_->Emit(vars, format, loc);
  }

  absl::string_view ImportPathToCrateName(absl::string_view import_path) const {
    if (opts_->strip_nonfunctional_codegen) {
      return "test";
    }
    auto it =
        rust_generator_context_->import_path_to_crate_name_.find(import_path);
    if (it == rust_generator_context_->import_path_to_crate_name_.end()) {
      ABSL_LOG(ERROR)
          << "Path " << import_path
          << " not found in crate mapping. Crate mapping contains "
          << rust_generator_context_->import_path_to_crate_name_.size()
          << " entries:";
      for (const auto& entry :
           rust_generator_context_->import_path_to_crate_name_) {
        ABSL_LOG(ERROR) << "  " << entry.first << " : " << entry.second << "\n";
      }
      ABSL_LOG(FATAL) << "Cannot continue with missing crate mapping.";
    }
    return it->second;
  }

  // Opening and closing modules should always be done with PushModule() and
  // PopModule(). Knowing what module we are in is important, because it allows
  // us to unambiguously reference other identifiers in the same crate. We
  // cannot just use crate::, because when we are building with Cargo, the
  // generated code does not necessarily live in the crate root.
  void PushModule(absl::string_view name) {
    Emit({{"mod_name", name}}, "pub mod $mod_name$ {");
    modules_.emplace_back(name);
  }

  void PopModule() {
    Emit({{"mod_name", modules_.back()}}, "}  // pub mod $mod_name$");
    modules_.pop_back();
  }

  // Returns the current depth of module nesting.
  size_t GetModuleDepth() const { return modules_.size(); }

 private:
  const Options* opts_;
  const RustGeneratorContext* rust_generator_context_;
  io::Printer* printer_;
  std::vector<std::string> modules_;
};

bool IsInCurrentlyGeneratingCrate(Context& ctx, const FileDescriptor& file);
bool IsInCurrentlyGeneratingCrate(Context& ctx, const Descriptor& message);
bool IsInCurrentlyGeneratingCrate(Context& ctx, const EnumDescriptor& enum_);

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_CONTEXT_H__
