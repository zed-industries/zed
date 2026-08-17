// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// This file is the public interface to the .proto file parser.

#ifndef GOOGLE_PROTOBUF_COMPILER_IMPORTER_H__
#define GOOGLE_PROTOBUF_COMPILER_IMPORTER_H__

#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/parser.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor_database.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

namespace io {
class ZeroCopyInputStream;
}

namespace compiler {

// Defined in this file.
class Importer;
class MultiFileErrorCollector;
class SourceTree;
class DiskSourceTree;

// TODO:  Move all SourceTree stuff to a separate file?

// An implementation of DescriptorDatabase which loads files from a SourceTree
// and parses them.
//
// Note:  This class is not thread-safe since it maintains a table of source
//   code locations for error reporting.  However, when a DescriptorPool wraps
//   a DescriptorDatabase, it uses mutex locking to make sure only one method
//   of the database is called at a time, even if the DescriptorPool is used
//   from multiple threads.  Therefore, there is only a problem if you create
//   multiple DescriptorPools wrapping the same SourceTreeDescriptorDatabase
//   and use them from multiple threads.
//
// Note:  This class does not implement FindFileContainingSymbol() or
//   FindFileContainingExtension(); these will always return false.
class PROTOBUF_EXPORT SourceTreeDescriptorDatabase : public DescriptorDatabase {
 public:
  SourceTreeDescriptorDatabase(SourceTree* source_tree);

  // If non-NULL, fallback_database will be checked if a file doesn't exist in
  // the specified source_tree.
  SourceTreeDescriptorDatabase(SourceTree* source_tree,
                               DescriptorDatabase* fallback_database);
  ~SourceTreeDescriptorDatabase() override;

  // Instructs the SourceTreeDescriptorDatabase to report any parse errors
  // to the given MultiFileErrorCollector.  This should be called before
  // parsing.  error_collector must remain valid until either this method
  // is called again or the SourceTreeDescriptorDatabase is destroyed.
  void RecordErrorsTo(MultiFileErrorCollector* error_collector) {
    error_collector_ = error_collector;
  }

  // Gets a DescriptorPool::ErrorCollector which records errors to the
  // MultiFileErrorCollector specified with RecordErrorsTo().  This collector
  // has the ability to determine exact line and column numbers of errors
  // from the information given to it by the DescriptorPool.
  DescriptorPool::ErrorCollector* GetValidationErrorCollector() {
    using_validation_error_collector_ = true;
    return &validation_error_collector_;
  }

  // implements DescriptorDatabase -----------------------------------
  bool FindFileByName(const std::string& filename,
                      FileDescriptorProto* output) override;
  bool FindFileContainingSymbol(const std::string& symbol_name,
                                FileDescriptorProto* output) override;
  bool FindFileContainingExtension(const std::string& containing_type,
                                   int field_number,
                                   FileDescriptorProto* output) override;

 private:
  class SingleFileErrorCollector;

  SourceTree* source_tree_;
  DescriptorDatabase* fallback_database_;
  MultiFileErrorCollector* error_collector_;

  class PROTOBUF_EXPORT ValidationErrorCollector
      : public DescriptorPool::ErrorCollector {
   public:
    ValidationErrorCollector(SourceTreeDescriptorDatabase* owner);
    ~ValidationErrorCollector() override;

    // implements ErrorCollector ---------------------------------------
    void RecordError(absl::string_view filename, absl::string_view element_name,
                     const Message* descriptor, ErrorLocation location,
                     absl::string_view message) override;

    void RecordWarning(absl::string_view filename,
                       absl::string_view element_name,
                       const Message* descriptor, ErrorLocation location,
                       absl::string_view message) override;

   private:
    SourceTreeDescriptorDatabase* owner_;
  };
  friend class ValidationErrorCollector;

  bool using_validation_error_collector_;
  SourceLocationTable source_locations_;
  ValidationErrorCollector validation_error_collector_;
};

// Simple interface for parsing .proto files.  This wraps the process
// of opening the file, parsing it with a Parser, recursively parsing all its
// imports, and then cross-linking the results to produce a FileDescriptor.
//
// This is really just a thin wrapper around SourceTreeDescriptorDatabase.
// You may find that SourceTreeDescriptorDatabase is more flexible.
//
// TODO:  I feel like this class is not well-named.
class PROTOBUF_EXPORT Importer {
 public:
  Importer(SourceTree* source_tree, MultiFileErrorCollector* error_collector);
  Importer(const Importer&) = delete;
  Importer& operator=(const Importer&) = delete;
  ~Importer();

  // Import the given file and build a FileDescriptor representing it.  If
  // the file is already in the DescriptorPool, the existing FileDescriptor
  // will be returned.  The FileDescriptor is property of the DescriptorPool,
  // and will remain valid until it is destroyed.  If any errors occur, they
  // will be reported using the error collector and Import() will return NULL.
  //
  // A particular Importer object will only report errors for a particular
  // file once.  All future attempts to import the same file will return NULL
  // without reporting any errors.  The idea is that you might want to import
  // a lot of files without seeing the same errors over and over again.  If
  // you want to see errors for the same files repeatedly, you can use a
  // separate Importer object to import each one (but use the same
  // DescriptorPool so that they can be cross-linked).
  const FileDescriptor* Import(const std::string& filename);

  // The DescriptorPool in which all imported FileDescriptors and their
  // contents are stored.
  inline const DescriptorPool* pool() const { return &pool_; }

  void AddDirectInputFile(absl::string_view file_name,
                          bool unused_import_is_error = false);
  void ClearDirectInputFiles();

#if !defined(PROTOBUF_FUTURE_RENAME_ADD_UNUSED_IMPORT) && !defined(SWIG)
  ABSL_DEPRECATED("Use AddDirectInputFile")
  void AddUnusedImportTrackFile(absl::string_view file_name,
                                bool is_error = false) {
    AddDirectInputFile(file_name, is_error);
  }
  ABSL_DEPRECATED("Use AddDirectInputFile")
  void ClearUnusedImportTrackFiles() { ClearDirectInputFiles(); }
#endif  // !PROTOBUF_FUTURE_RENAME_ADD_UNUSED_IMPORT && !SWIG


 private:
  SourceTreeDescriptorDatabase database_;
  DescriptorPool pool_;
};

// If the importer encounters problems while trying to import the proto files,
// it reports them to a MultiFileErrorCollector.
class PROTOBUF_EXPORT MultiFileErrorCollector {
 public:
  MultiFileErrorCollector() {}
  MultiFileErrorCollector(const MultiFileErrorCollector&) = delete;
  MultiFileErrorCollector& operator=(const MultiFileErrorCollector&) = delete;
  virtual ~MultiFileErrorCollector();

  // Line and column numbers are zero-based.  A line number of -1 indicates
  // an error with the entire file (e.g. "not found").
  virtual void RecordError(absl::string_view filename, int line, int column,
                           absl::string_view message) {
    PROTOBUF_IGNORE_DEPRECATION_START
    AddError(std::string(filename), line, column, std::string(message));
    PROTOBUF_IGNORE_DEPRECATION_STOP
  }
  virtual void RecordWarning(absl::string_view filename, int line, int column,
                             absl::string_view message) {
    PROTOBUF_IGNORE_DEPRECATION_START
    AddWarning(std::string(filename), line, column, std::string(message));
    PROTOBUF_IGNORE_DEPRECATION_STOP
  }

 private:
  // These should never be called directly, but if a legacy class overrides
  // them they'll get routed to by the Record* methods.
  [[deprecated("Use RecordError")]]
  virtual void AddError(const std::string& filename, int line, int column,
                        const std::string& message) {
    ABSL_LOG(FATAL) << "AddError or RecordError must be implemented.";
  }

  [[deprecated("Use RecordWarning")]]
  virtual void AddWarning(const std::string& filename, int line, int column,
                          const std::string& message) {}
};

// Abstract interface which represents a directory tree containing proto files.
// Used by the default implementation of Importer to resolve import statements
// Most users will probably want to use the DiskSourceTree implementation,
// below.
class PROTOBUF_EXPORT SourceTree {
 public:
  SourceTree() {}
  SourceTree(const SourceTree&) = delete;
  SourceTree& operator=(const SourceTree&) = delete;
  virtual ~SourceTree();

  // Open the given file and return a stream that reads it, or NULL if not
  // found.  The caller takes ownership of the returned object.  The filename
  // must be a path relative to the root of the source tree and must not
  // contain "." or ".." components.
  virtual io::ZeroCopyInputStream* Open(absl::string_view filename) = 0;

  // If Open() returns NULL, calling this method immediately will return an
  // description of the error.
  // Subclasses should implement this method and return a meaningful value for
  // better error reporting.
  // TODO: change this to a pure virtual function.
  virtual std::string GetLastErrorMessage();
};

// An implementation of SourceTree which loads files from locations on disk.
// Multiple mappings can be set up to map locations in the DiskSourceTree to
// locations in the physical filesystem.
class PROTOBUF_EXPORT DiskSourceTree : public SourceTree {
 public:
  DiskSourceTree();
  DiskSourceTree(const DiskSourceTree&) = delete;
  DiskSourceTree& operator=(const DiskSourceTree&) = delete;
  ~DiskSourceTree() override;

  // Map a path on disk to a location in the SourceTree.  The path may be
  // either a file or a directory.  If it is a directory, the entire tree
  // under it will be mapped to the given virtual location.  To map a directory
  // to the root of the source tree, pass an empty string for virtual_path.
  //
  // If multiple mapped paths apply when opening a file, they will be searched
  // in order.  For example, if you do:
  //   MapPath("bar", "foo/bar");
  //   MapPath("", "baz");
  // and then you do:
  //   Open("bar/qux");
  // the DiskSourceTree will first try to open foo/bar/qux, then baz/bar/qux,
  // returning the first one that opens successfully.
  //
  // disk_path may be an absolute path or relative to the current directory,
  // just like a path you'd pass to open().
  void MapPath(absl::string_view virtual_path, absl::string_view disk_path);

  // Return type for DiskFileToVirtualFile().
  enum DiskFileToVirtualFileResult {
    SUCCESS,
    SHADOWED,
    CANNOT_OPEN,
    NO_MAPPING
  };

  // Given a path to a file on disk, find a virtual path mapping to that
  // file.  The first mapping created with MapPath() whose disk_path contains
  // the filename is used.  However, that virtual path may not actually be
  // usable to open the given file.  Possible return values are:
  // * SUCCESS: The mapping was found.  *virtual_file is filled in so that
  //   calling Open(*virtual_file) will open the file named by disk_file.
  // * SHADOWED: A mapping was found, but using Open() to open this virtual
  //   path will end up returning some different file.  This is because some
  //   other mapping with a higher precedence also matches this virtual path
  //   and maps it to a different file that exists on disk.  *virtual_file
  //   is filled in as it would be in the SUCCESS case.  *shadowing_disk_file
  //   is filled in with the disk path of the file which would be opened if
  //   you were to call Open(*virtual_file).
  // * CANNOT_OPEN: The mapping was found and was not shadowed, but the
  //   file specified cannot be opened.  When this value is returned,
  //   errno will indicate the reason the file cannot be opened.  *virtual_file
  //   will be set to the virtual path as in the SUCCESS case, even though
  //   it is not useful.
  // * NO_MAPPING: Indicates that no mapping was found which contains this
  //   file.
  DiskFileToVirtualFileResult DiskFileToVirtualFile(
      absl::string_view disk_file, std::string* virtual_file,
      std::string* shadowing_disk_file);

  // Given a virtual path, find the path to the file on disk.
  // Return true and update disk_file with the on-disk path if the file exists.
  // Return false and leave disk_file untouched if the file doesn't exist.
  bool VirtualFileToDiskFile(absl::string_view virtual_file,
                             std::string* disk_file);

  // implements SourceTree -------------------------------------------
  io::ZeroCopyInputStream* Open(absl::string_view filename) override;

  std::string GetLastErrorMessage() override;

 private:
  struct Mapping {
    std::string virtual_path;
    std::string disk_path;

    inline Mapping(std::string virtual_path_param, std::string disk_path_param)
        : virtual_path(std::move(virtual_path_param)),
          disk_path(std::move(disk_path_param)) {}
  };
  std::vector<Mapping> mappings_;
  std::string last_error_message_;

  // Like Open(), but returns the on-disk path in disk_file if disk_file is
  // non-NULL and the file could be successfully opened.
  io::ZeroCopyInputStream* OpenVirtualFile(absl::string_view virtual_file,
                                           std::string* disk_file);

  // Like Open() but given the actual on-disk path.
  io::ZeroCopyInputStream* OpenDiskFile(absl::string_view filename);
};

}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_IMPORTER_H__
