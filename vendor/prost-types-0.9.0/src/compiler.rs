/// The version number of protocol compiler.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    #[prost(int32, optional, tag="1")]
    pub major: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub minor: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub patch: ::core::option::Option<i32>,
    /// A suffix for alpha, beta or rc release, e.g., "alpha-1", "rc2". It should
    /// be empty for mainline stable releases.
    #[prost(string, optional, tag="4")]
    pub suffix: ::core::option::Option<::prost::alloc::string::String>,
}
/// An encoded CodeGeneratorRequest is written to the plugin's stdin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeGeneratorRequest {
    /// The .proto files that were explicitly listed on the command-line.  The
    /// code generator should generate code only for these files.  Each file's
    /// descriptor will be included in proto_file, below.
    #[prost(string, repeated, tag="1")]
    pub file_to_generate: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The generator parameter passed on the command-line.
    #[prost(string, optional, tag="2")]
    pub parameter: ::core::option::Option<::prost::alloc::string::String>,
    /// FileDescriptorProtos for all files in files_to_generate and everything
    /// they import.  The files will appear in topological order, so each file
    /// appears before any file that imports it.
    ///
    /// protoc guarantees that all proto_files will be written after
    /// the fields above, even though this is not technically guaranteed by the
    /// protobuf wire format.  This theoretically could allow a plugin to stream
    /// in the FileDescriptorProtos and handle them one by one rather than read
    /// the entire set into memory at once.  However, as of this writing, this
    /// is not similarly optimized on protoc's end -- it will store all fields in
    /// memory at once before sending them to the plugin.
    ///
    /// Type names of fields and extensions in the FileDescriptorProto are always
    /// fully qualified.
    #[prost(message, repeated, tag="15")]
    pub proto_file: ::prost::alloc::vec::Vec<super::FileDescriptorProto>,
    /// The version number of protocol compiler.
    #[prost(message, optional, tag="3")]
    pub compiler_version: ::core::option::Option<Version>,
}
/// The plugin writes an encoded CodeGeneratorResponse to stdout.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeGeneratorResponse {
    /// Error message.  If non-empty, code generation failed.  The plugin process
    /// should exit with status code zero even if it reports an error in this way.
    ///
    /// This should be used to indicate errors in .proto files which prevent the
    /// code generator from generating correct code.  Errors which indicate a
    /// problem in protoc itself -- such as the input CodeGeneratorRequest being
    /// unparseable -- should be reported by writing a message to stderr and
    /// exiting with a non-zero status code.
    #[prost(string, optional, tag="1")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
    /// A bitmask of supported features that the code generator supports.
    /// This is a bitwise "or" of values from the Feature enum.
    #[prost(uint64, optional, tag="2")]
    pub supported_features: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="15")]
    pub file: ::prost::alloc::vec::Vec<code_generator_response::File>,
}
/// Nested message and enum types in `CodeGeneratorResponse`.
pub mod code_generator_response {
    /// Represents a single generated file.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct File {
        /// The file name, relative to the output directory.  The name must not
        /// contain "." or ".." components and must be relative, not be absolute (so,
        /// the file cannot lie outside the output directory).  "/" must be used as
        /// the path separator, not "\".
        ///
        /// If the name is omitted, the content will be appended to the previous
        /// file.  This allows the generator to break large files into small chunks,
        /// and allows the generated text to be streamed back to protoc so that large
        /// files need not reside completely in memory at one time.  Note that as of
        /// this writing protoc does not optimize for this -- it will read the entire
        /// CodeGeneratorResponse before writing files to disk.
        #[prost(string, optional, tag="1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        /// If non-empty, indicates that the named file should already exist, and the
        /// content here is to be inserted into that file at a defined insertion
        /// point.  This feature allows a code generator to extend the output
        /// produced by another code generator.  The original generator may provide
        /// insertion points by placing special annotations in the file that look
        /// like:
        ///   @@protoc_insertion_point(NAME)
        /// The annotation can have arbitrary text before and after it on the line,
        /// which allows it to be placed in a comment.  NAME should be replaced with
        /// an identifier naming the point -- this is what other generators will use
        /// as the insertion_point.  Code inserted at this point will be placed
        /// immediately above the line containing the insertion point (thus multiple
        /// insertions to the same point will come out in the order they were added).
        /// The double-@ is intended to make it unlikely that the generated code
        /// could contain things that look like insertion points by accident.
        ///
        /// For example, the C++ code generator places the following line in the
        /// .pb.h files that it generates:
        ///   // @@protoc_insertion_point(namespace_scope)
        /// This line appears within the scope of the file's package namespace, but
        /// outside of any particular class.  Another plugin can then specify the
        /// insertion_point "namespace_scope" to generate additional classes or
        /// other declarations that should be placed in this scope.
        ///
        /// Note that if the line containing the insertion point begins with
        /// whitespace, the same whitespace will be added to every line of the
        /// inserted text.  This is useful for languages like Python, where
        /// indentation matters.  In these languages, the insertion point comment
        /// should be indented the same amount as any inserted code will need to be
        /// in order to work correctly in that context.
        ///
        /// The code generator that generates the initial file and the one which
        /// inserts into it must both run as part of a single invocation of protoc.
        /// Code generators are executed in the order in which they appear on the
        /// command line.
        ///
        /// If |insertion_point| is present, |name| must also be present.
        #[prost(string, optional, tag="2")]
        pub insertion_point: ::core::option::Option<::prost::alloc::string::String>,
        /// The file contents.
        #[prost(string, optional, tag="15")]
        pub content: ::core::option::Option<::prost::alloc::string::String>,
        /// Information describing the file content being inserted. If an insertion
        /// point is used, this information will be appropriately offset and inserted
        /// into the code generation metadata for the generated files.
        #[prost(message, optional, tag="16")]
        pub generated_code_info: ::core::option::Option<super::super::GeneratedCodeInfo>,
    }
    /// Sync with code_generator.h.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Feature {
        None = 0,
        Proto3Optional = 1,
    }
}
