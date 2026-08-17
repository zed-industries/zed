/// The protocol compiler can output a FileDescriptorSet containing the .proto
/// files it parses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDescriptorSet {
    #[prost(message, repeated, tag="1")]
    pub file: ::prost::alloc::vec::Vec<FileDescriptorProto>,
}
/// Describes a complete .proto file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDescriptorProto {
    /// file name, relative to root of source tree
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// e.g. "foo", "foo.bar", etc.
    #[prost(string, optional, tag="2")]
    pub package: ::core::option::Option<::prost::alloc::string::String>,
    /// Names of files imported by this file.
    #[prost(string, repeated, tag="3")]
    pub dependency: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Indexes of the public imported files in the dependency list above.
    #[prost(int32, repeated, packed="false", tag="10")]
    pub public_dependency: ::prost::alloc::vec::Vec<i32>,
    /// Indexes of the weak imported files in the dependency list.
    /// For Google-internal migration only. Do not use.
    #[prost(int32, repeated, packed="false", tag="11")]
    pub weak_dependency: ::prost::alloc::vec::Vec<i32>,
    /// All top-level definitions in this file.
    #[prost(message, repeated, tag="4")]
    pub message_type: ::prost::alloc::vec::Vec<DescriptorProto>,
    #[prost(message, repeated, tag="5")]
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProto>,
    #[prost(message, repeated, tag="6")]
    pub service: ::prost::alloc::vec::Vec<ServiceDescriptorProto>,
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, optional, tag="8")]
    pub options: ::core::option::Option<FileOptions>,
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    #[prost(message, optional, tag="9")]
    pub source_code_info: ::core::option::Option<SourceCodeInfo>,
    /// The syntax of the proto file.
    /// The supported values are "proto2" and "proto3".
    #[prost(string, optional, tag="12")]
    pub syntax: ::core::option::Option<::prost::alloc::string::String>,
}
/// Describes a message type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub field: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, repeated, tag="6")]
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, repeated, tag="3")]
    pub nested_type: ::prost::alloc::vec::Vec<DescriptorProto>,
    #[prost(message, repeated, tag="4")]
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProto>,
    #[prost(message, repeated, tag="5")]
    pub extension_range: ::prost::alloc::vec::Vec<descriptor_proto::ExtensionRange>,
    #[prost(message, repeated, tag="8")]
    pub oneof_decl: ::prost::alloc::vec::Vec<OneofDescriptorProto>,
    #[prost(message, optional, tag="7")]
    pub options: ::core::option::Option<MessageOptions>,
    #[prost(message, repeated, tag="9")]
    pub reserved_range: ::prost::alloc::vec::Vec<descriptor_proto::ReservedRange>,
    /// Reserved field names, which may not be used by fields in the same message.
    /// A given name may only be reserved once.
    #[prost(string, repeated, tag="10")]
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DescriptorProto`.
pub mod descriptor_proto {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExtensionRange {
        /// Inclusive.
        #[prost(int32, optional, tag="1")]
        pub start: ::core::option::Option<i32>,
        /// Exclusive.
        #[prost(int32, optional, tag="2")]
        pub end: ::core::option::Option<i32>,
        #[prost(message, optional, tag="3")]
        pub options: ::core::option::Option<super::ExtensionRangeOptions>,
    }
    /// Range of reserved tag numbers. Reserved tag numbers may not be used by
    /// fields or extension ranges in the same message. Reserved ranges may
    /// not overlap.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReservedRange {
        /// Inclusive.
        #[prost(int32, optional, tag="1")]
        pub start: ::core::option::Option<i32>,
        /// Exclusive.
        #[prost(int32, optional, tag="2")]
        pub end: ::core::option::Option<i32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionRangeOptions {
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
/// Describes a field within a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub number: ::core::option::Option<i32>,
    #[prost(enumeration="field_descriptor_proto::Label", optional, tag="4")]
    pub label: ::core::option::Option<i32>,
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    #[prost(enumeration="field_descriptor_proto::Type", optional, tag="5")]
    pub r#type: ::core::option::Option<i32>,
    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    #[prost(string, optional, tag="6")]
    pub type_name: ::core::option::Option<::prost::alloc::string::String>,
    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as type_name.
    #[prost(string, optional, tag="2")]
    pub extendee: ::core::option::Option<::prost::alloc::string::String>,
    /// For numeric types, contains the original text representation of the value.
    /// For booleans, "true" or "false".
    /// For strings, contains the default text contents (not escaped in any way).
    /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    /// TODO(kenton):  Base-64 encode?
    #[prost(string, optional, tag="7")]
    pub default_value: ::core::option::Option<::prost::alloc::string::String>,
    /// If set, gives the index of a oneof in the containing type's oneof_decl
    /// list.  This field is a member of that oneof.
    #[prost(int32, optional, tag="9")]
    pub oneof_index: ::core::option::Option<i32>,
    /// JSON name of this field. The value is set by protocol compiler. If the
    /// user has set a "json_name" option on this field, that option's value
    /// will be used. Otherwise, it's deduced from the field's name by converting
    /// it to camelCase.
    #[prost(string, optional, tag="10")]
    pub json_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub options: ::core::option::Option<FieldOptions>,
    /// If true, this is a proto3 "optional". When a proto3 field is optional, it
    /// tracks presence regardless of field type.
    ///
    /// When proto3_optional is true, this field must be belong to a oneof to
    /// signal to old proto3 clients that presence is tracked for this field. This
    /// oneof is known as a "synthetic" oneof, and this field must be its sole
    /// member (each proto3 optional field gets its own synthetic oneof). Synthetic
    /// oneofs exist in the descriptor only, and do not generate any API. Synthetic
    /// oneofs must be ordered after all "real" oneofs.
    ///
    /// For message fields, proto3_optional doesn't create any semantic change,
    /// since non-repeated message fields always track presence. However it still
    /// indicates the semantic detail of whether the user wrote "optional" or not.
    /// This can be useful for round-tripping the .proto file. For consistency we
    /// give message fields a synthetic oneof also, even though it is not required
    /// to track presence. This is especially important because the parser can't
    /// tell if a field is a message or an enum, so it must always create a
    /// synthetic oneof.
    ///
    /// Proto2 optional fields do not set this flag, because they already indicate
    /// optional with `LABEL_OPTIONAL`.
    #[prost(bool, optional, tag="17")]
    pub proto3_optional: ::core::option::Option<bool>,
}
/// Nested message and enum types in `FieldDescriptorProto`.
pub mod field_descriptor_proto {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// 0 is reserved for errors.
        /// Order is weird for historical reasons.
        Double = 1,
        Float = 2,
        /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
        /// negative values are likely.
        Int64 = 3,
        Uint64 = 4,
        /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
        /// negative values are likely.
        Int32 = 5,
        Fixed64 = 6,
        Fixed32 = 7,
        Bool = 8,
        String = 9,
        /// Tag-delimited aggregate.
        /// Group type is deprecated and not supported in proto3. However, Proto3
        /// implementations should still be able to parse the group wire format and
        /// treat group fields as unknown fields.
        Group = 10,
        /// Length-delimited aggregate.
        Message = 11,
        /// New in version 2.
        Bytes = 12,
        Uint32 = 13,
        Enum = 14,
        Sfixed32 = 15,
        Sfixed64 = 16,
        /// Uses ZigZag encoding.
        Sint32 = 17,
        /// Uses ZigZag encoding.
        Sint64 = 18,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Label {
        /// 0 is reserved for errors
        Optional = 1,
        Required = 2,
        Repeated = 3,
    }
}
/// Describes a oneof.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneofDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<OneofOptions>,
}
/// Describes an enum type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub value: ::prost::alloc::vec::Vec<EnumValueDescriptorProto>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<EnumOptions>,
    /// Range of reserved numeric values. Reserved numeric values may not be used
    /// by enum values in the same enum declaration. Reserved ranges may not
    /// overlap.
    #[prost(message, repeated, tag="4")]
    pub reserved_range: ::prost::alloc::vec::Vec<enum_descriptor_proto::EnumReservedRange>,
    /// Reserved enum value names, which may not be reused. A given name may only
    /// be reserved once.
    #[prost(string, repeated, tag="5")]
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `EnumDescriptorProto`.
pub mod enum_descriptor_proto {
    /// Range of reserved numeric values. Reserved values may not be used by
    /// entries in the same enum. Reserved ranges may not overlap.
    ///
    /// Note that this is distinct from DescriptorProto.ReservedRange in that it
    /// is inclusive such that it can appropriately represent the entire int32
    /// domain.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumReservedRange {
        /// Inclusive.
        #[prost(int32, optional, tag="1")]
        pub start: ::core::option::Option<i32>,
        /// Inclusive.
        #[prost(int32, optional, tag="2")]
        pub end: ::core::option::Option<i32>,
    }
}
/// Describes a value within an enum.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValueDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="2")]
    pub number: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<EnumValueOptions>,
}
/// Describes a service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub method: ::prost::alloc::vec::Vec<MethodDescriptorProto>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<ServiceOptions>,
}
/// Describes a method of a service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Input and output type names.  These are resolved in the same way as
    /// FieldDescriptorProto.type_name, but must refer to a message type.
    #[prost(string, optional, tag="2")]
    pub input_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub output_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub options: ::core::option::Option<MethodOptions>,
    /// Identifies if client streams multiple client messages
    #[prost(bool, optional, tag="5", default="false")]
    pub client_streaming: ::core::option::Option<bool>,
    /// Identifies if server streams multiple server messages
    #[prost(bool, optional, tag="6", default="false")]
    pub server_streaming: ::core::option::Option<bool>,
}
// ===================================================================
// Options

// Each of the definitions above may have "options" attached.  These are
// just annotations which may cause code to be generated slightly differently
// or may contain hints for code that manipulates protocol messages.
//
// Clients may define custom options as extensions of the *Options messages.
// These extensions may not yet be known at parsing time, so the parser cannot
// store the values in them.  Instead it stores them in a field in the *Options
// message called uninterpreted_option. This field must have the same name
// across all *Options messages. We then use this field to populate the
// extensions when we build a descriptor, at which point all protos have been
// parsed and so all extensions are known.
//
// Extension numbers for custom options may be chosen as follows:
// * For options which will only be used within a single application or
//   organization, or for experimental options, use field numbers 50000
//   through 99999.  It is up to you to ensure that you do not use the
//   same number for multiple options.
// * For options which will be published and used publicly by multiple
//   independent entities, e-mail protobuf-global-extension-registry@google.com
//   to reserve extension numbers. Simply provide your project name (e.g.
//   Objective-C plugin) and your project website (if available) -- there's no
//   need to explain how you intend to use them. Usually you only need one
//   extension number. You can declare multiple options with only one extension
//   number by putting them in a sub-message. See the Custom Options section of
//   the docs for examples:
//   <https://developers.google.com/protocol-buffers/docs/proto#options>
//   If this turns out to be popular, a web service will be set up
//   to automatically assign option numbers.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileOptions {
    /// Sets the Java package where classes generated from this .proto will be
    /// placed.  By default, the proto package is used, but this is often
    /// inappropriate because proto packages do not normally start with backwards
    /// domain names.
    #[prost(string, optional, tag="1")]
    pub java_package: ::core::option::Option<::prost::alloc::string::String>,
    /// If set, all the classes from the .proto file are wrapped in a single
    /// outer class with the given name.  This applies to both Proto1
    /// (equivalent to the old "--one_java_file" option) and Proto2 (where
    /// a .proto always translates to a single class, but you may want to
    /// explicitly choose the class name).
    #[prost(string, optional, tag="8")]
    pub java_outer_classname: ::core::option::Option<::prost::alloc::string::String>,
    /// If set true, then the Java code generator will generate a separate .java
    /// file for each top-level message, enum, and service defined in the .proto
    /// file.  Thus, these types will *not* be nested inside the outer class
    /// named by java_outer_classname.  However, the outer class will still be
    /// generated to contain the file's getDescriptor() method as well as any
    /// top-level extensions defined in the file.
    #[prost(bool, optional, tag="10", default="false")]
    pub java_multiple_files: ::core::option::Option<bool>,
    /// This option does nothing.
    #[deprecated]
    #[prost(bool, optional, tag="20")]
    pub java_generate_equals_and_hash: ::core::option::Option<bool>,
    /// If set true, then the Java2 code generator will generate code that
    /// throws an exception whenever an attempt is made to assign a non-UTF-8
    /// byte sequence to a string field.
    /// Message reflection will do the same.
    /// However, an extension field still accepts non-UTF-8 byte sequences.
    /// This option has no effect on when used with the lite runtime.
    #[prost(bool, optional, tag="27", default="false")]
    pub java_string_check_utf8: ::core::option::Option<bool>,
    #[prost(enumeration="file_options::OptimizeMode", optional, tag="9", default="Speed")]
    pub optimize_for: ::core::option::Option<i32>,
    /// Sets the Go package where structs generated from this .proto will be
    /// placed. If omitted, the Go package will be derived from the following:
    ///   - The basename of the package import path, if provided.
    ///   - Otherwise, the package statement in the .proto file, if present.
    ///   - Otherwise, the basename of the .proto file, without extension.
    #[prost(string, optional, tag="11")]
    pub go_package: ::core::option::Option<::prost::alloc::string::String>,
    /// Should generic services be generated in each language?  "Generic" services
    /// are not specific to any particular RPC system.  They are generated by the
    /// main code generators in each language (without additional plugins).
    /// Generic services were the only kind of service generation supported by
    /// early versions of google.protobuf.
    ///
    /// Generic services are now considered deprecated in favor of using plugins
    /// that generate code specific to your particular RPC system.  Therefore,
    /// these default to false.  Old code which depends on generic services should
    /// explicitly set them to true.
    #[prost(bool, optional, tag="16", default="false")]
    pub cc_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="17", default="false")]
    pub java_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="18", default="false")]
    pub py_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="42", default="false")]
    pub php_generic_services: ::core::option::Option<bool>,
    /// Is this file deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for everything in the file, or it will be completely ignored; in the very
    /// least, this is a formalization for deprecating files.
    #[prost(bool, optional, tag="23", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    /// Enables the use of arenas for the proto messages in this file. This applies
    /// only to generated classes for C++.
    #[prost(bool, optional, tag="31", default="true")]
    pub cc_enable_arenas: ::core::option::Option<bool>,
    /// Sets the objective c class prefix which is prepended to all objective c
    /// generated classes from this .proto. There is no default.
    #[prost(string, optional, tag="36")]
    pub objc_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace for generated classes; defaults to the package.
    #[prost(string, optional, tag="37")]
    pub csharp_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// By default Swift generators will take the proto package and CamelCase it
    /// replacing '.' with underscore and use that to prefix the types/symbols
    /// defined. When this options is provided, they will use this value instead
    /// to prefix the types/symbols defined.
    #[prost(string, optional, tag="39")]
    pub swift_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Sets the php class prefix which is prepended to all php generated classes
    /// from this .proto. Default is empty.
    #[prost(string, optional, tag="40")]
    pub php_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the namespace of php generated classes. Default
    /// is empty. When this option is empty, the package name will be used for
    /// determining the namespace.
    #[prost(string, optional, tag="41")]
    pub php_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the namespace of php generated metadata classes.
    /// Default is empty. When this option is empty, the proto file name will be
    /// used for determining the namespace.
    #[prost(string, optional, tag="44")]
    pub php_metadata_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Use this option to change the package of ruby generated classes. Default
    /// is empty. When this option is not set, the package name will be used for
    /// determining the ruby package.
    #[prost(string, optional, tag="45")]
    pub ruby_package: ::core::option::Option<::prost::alloc::string::String>,
    /// The parser stores options it doesn't recognize here.
    /// See the documentation for the "Options" section above.
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
/// Nested message and enum types in `FileOptions`.
pub mod file_options {
    /// Generated classes can be optimized for speed or code size.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OptimizeMode {
        /// Generate complete code for parsing, serialization,
        Speed = 1,
        /// etc.
        ///
        /// Use ReflectionOps to implement these methods.
        CodeSize = 2,
        /// Generate code using MessageLite and the lite runtime.
        LiteRuntime = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageOptions {
    /// Set true to use the old proto1 MessageSet wire format for extensions.
    /// This is provided for backwards-compatibility with the MessageSet wire
    /// format.  You should not use this for any other reason:  It's less
    /// efficient, has fewer features, and is more complicated.
    ///
    /// The message must be defined exactly as follows:
    ///   message Foo {
    ///     option message_set_wire_format = true;
    ///     extensions 4 to max;
    ///   }
    /// Note that the message cannot have any defined fields; MessageSets only
    /// have extensions.
    ///
    /// All extensions of your type must be singular messages; e.g. they cannot
    /// be int32s, enums, or repeated messages.
    ///
    /// Because this is an option, the above two restrictions are not enforced by
    /// the protocol compiler.
    #[prost(bool, optional, tag="1", default="false")]
    pub message_set_wire_format: ::core::option::Option<bool>,
    /// Disables the generation of the standard "descriptor()" accessor, which can
    /// conflict with a field of the same name.  This is meant to make migration
    /// from proto1 easier; new code should avoid fields named "descriptor".
    #[prost(bool, optional, tag="2", default="false")]
    pub no_standard_descriptor_accessor: ::core::option::Option<bool>,
    /// Is this message deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the message, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating messages.
    #[prost(bool, optional, tag="3", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    /// Whether the message is an automatically generated map entry type for the
    /// maps field.
    ///
    /// For maps fields:
    ///     map<KeyType, ValueType> map_field = 1;
    /// The parsed descriptor looks like:
    ///     message MapFieldEntry {
    ///         option map_entry = true;
    ///         optional KeyType key = 1;
    ///         optional ValueType value = 2;
    ///     }
    ///     repeated MapFieldEntry map_field = 1;
    ///
    /// Implementations may choose not to generate the map_entry=true message, but
    /// use a native map in the target language to hold the keys and values.
    /// The reflection APIs in such implementations still need to work as
    /// if the field is a repeated message field.
    ///
    /// NOTE: Do not set the option in .proto files. Always use the maps syntax
    /// instead. The option should only be implicitly set by the proto compiler
    /// parser.
    #[prost(bool, optional, tag="7")]
    pub map_entry: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldOptions {
    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    #[prost(enumeration="field_options::CType", optional, tag="1", default="String")]
    pub ctype: ::core::option::Option<i32>,
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    #[prost(bool, optional, tag="2")]
    pub packed: ::core::option::Option<bool>,
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    #[prost(enumeration="field_options::JsType", optional, tag="6", default="JsNormal")]
    pub jstype: ::core::option::Option<i32>,
    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    #[prost(bool, optional, tag="5", default="false")]
    pub lazy: ::core::option::Option<bool>,
    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    #[prost(bool, optional, tag="3", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    /// For Google-internal migration only. Do not use.
    #[prost(bool, optional, tag="10", default="false")]
    pub weak: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
/// Nested message and enum types in `FieldOptions`.
pub mod field_options {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CType {
        /// Default mode.
        String = 0,
        Cord = 1,
        StringPiece = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JsType {
        /// Use the default type.
        JsNormal = 0,
        /// Use JavaScript strings.
        JsString = 1,
        /// Use JavaScript numbers.
        JsNumber = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneofOptions {
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumOptions {
    /// Set this option to true to allow mapping different tag names to the same
    /// value.
    #[prost(bool, optional, tag="2")]
    pub allow_alias: ::core::option::Option<bool>,
    /// Is this enum deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating enums.
    #[prost(bool, optional, tag="3", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValueOptions {
    /// Is this enum value deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum value, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating enum values.
    #[prost(bool, optional, tag="1", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceOptions {
    // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
    //   framework.  We apologize for hoarding these numbers to ourselves, but
    //   we were already using them long before we decided to release Protocol
    //   Buffers.

    /// Is this service deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the service, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating services.
    #[prost(bool, optional, tag="33", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodOptions {
    // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
    //   framework.  We apologize for hoarding these numbers to ourselves, but
    //   we were already using them long before we decided to release Protocol
    //   Buffers.

    /// Is this method deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the method, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating methods.
    #[prost(bool, optional, tag="33", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(enumeration="method_options::IdempotencyLevel", optional, tag="34", default="IdempotencyUnknown")]
    pub idempotency_level: ::core::option::Option<i32>,
    /// The parser stores options it doesn't recognize here. See above.
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
/// Nested message and enum types in `MethodOptions`.
pub mod method_options {
    /// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
    /// or neither? HTTP based RPC implementation may choose GET verb for safe
    /// methods, and PUT verb for idempotent methods instead of the default POST.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IdempotencyLevel {
        IdempotencyUnknown = 0,
        /// implies idempotent
        NoSideEffects = 1,
        /// idempotent, but may have side effects
        Idempotent = 2,
    }
}
/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UninterpretedOption {
    #[prost(message, repeated, tag="2")]
    pub name: ::prost::alloc::vec::Vec<uninterpreted_option::NamePart>,
    /// The value of the uninterpreted option, in whatever type the tokenizer
    /// identified it as during parsing. Exactly one of these should be set.
    #[prost(string, optional, tag="3")]
    pub identifier_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="4")]
    pub positive_int_value: ::core::option::Option<u64>,
    #[prost(int64, optional, tag="5")]
    pub negative_int_value: ::core::option::Option<i64>,
    #[prost(double, optional, tag="6")]
    pub double_value: ::core::option::Option<f64>,
    #[prost(bytes="vec", optional, tag="7")]
    pub string_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="8")]
    pub aggregate_value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `UninterpretedOption`.
pub mod uninterpreted_option {
    /// The name of the uninterpreted option.  Each string represents a segment in
    /// a dot-separated name.  is_extension is true iff a segment represents an
    /// extension (denoted with parentheses in options specs in .proto files).
    /// E.g.,{ ["foo", false], ["bar.baz", true], ["qux", false] } represents
    /// "foo.(bar.baz).qux".
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NamePart {
        #[prost(string, required, tag="1")]
        pub name_part: ::prost::alloc::string::String,
        #[prost(bool, required, tag="2")]
        pub is_extension: bool,
    }
}
// ===================================================================
// Optional source code info

/// Encapsulates information about the original source file from which a
/// FileDescriptorProto was generated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCodeInfo {
    /// A Location identifies a piece of source code in a .proto file which
    /// corresponds to a particular definition.  This information is intended
    /// to be useful to IDEs, code indexers, documentation generators, and similar
    /// tools.
    ///
    /// For example, say we have a file like:
    ///   message Foo {
    ///     optional string foo = 1;
    ///   }
    /// Let's look at just the field definition:
    ///   optional string foo = 1;
    ///   ^       ^^     ^^  ^  ^^^
    ///   a       bc     de  f  ghi
    /// We have the following locations:
    ///   span   path               represents
    ///   [a,i)  [ 4, 0, 2, 0 ]     The whole field definition.
    ///   [a,b)  [ 4, 0, 2, 0, 4 ]  The label (optional).
    ///   [c,d)  [ 4, 0, 2, 0, 5 ]  The type (string).
    ///   [e,f)  [ 4, 0, 2, 0, 1 ]  The name (foo).
    ///   [g,h)  [ 4, 0, 2, 0, 3 ]  The number (1).
    ///
    /// Notes:
    /// - A location may refer to a repeated field itself (i.e. not to any
    ///   particular index within it).  This is used whenever a set of elements are
    ///   logically enclosed in a single code segment.  For example, an entire
    ///   extend block (possibly containing multiple extension definitions) will
    ///   have an outer location whose path refers to the "extensions" repeated
    ///   field without an index.
    /// - Multiple locations may have the same path.  This happens when a single
    ///   logical declaration is spread out across multiple places.  The most
    ///   obvious example is the "extend" block again -- there may be multiple
    ///   extend blocks in the same scope, each of which will have the same path.
    /// - A location's span is not always a subset of its parent's span.  For
    ///   example, the "extendee" of an extension declaration appears at the
    ///   beginning of the "extend" block and is shared by all extensions within
    ///   the block.
    /// - Just because a location's span is a subset of some other location's span
    ///   does not mean that it is a descendant.  For example, a "group" defines
    ///   both a type and a field in a single declaration.  Thus, the locations
    ///   corresponding to the type and field and their components will overlap.
    /// - Code which tries to interpret locations should probably be designed to
    ///   ignore those that it doesn't understand, as more types of locations could
    ///   be recorded in the future.
    #[prost(message, repeated, tag="1")]
    pub location: ::prost::alloc::vec::Vec<source_code_info::Location>,
}
/// Nested message and enum types in `SourceCodeInfo`.
pub mod source_code_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Location {
        /// Identifies which part of the FileDescriptorProto was defined at this
        /// location.
        ///
        /// Each element is a field number or an index.  They form a path from
        /// the root FileDescriptorProto to the place where the definition.  For
        /// example, this path:
        ///   [ 4, 3, 2, 7, 1 ]
        /// refers to:
        ///   file.message_type(3)  // 4, 3
        ///       .field(7)         // 2, 7
        ///       .name()           // 1
        /// This is because FileDescriptorProto.message_type has field number 4:
        ///   repeated DescriptorProto message_type = 4;
        /// and DescriptorProto.field has field number 2:
        ///   repeated FieldDescriptorProto field = 2;
        /// and FieldDescriptorProto.name has field number 1:
        ///   optional string name = 1;
        ///
        /// Thus, the above path gives the location of a field name.  If we removed
        /// the last element:
        ///   [ 4, 3, 2, 7 ]
        /// this path refers to the whole field declaration (from the beginning
        /// of the label to the terminating semicolon).
        #[prost(int32, repeated, tag="1")]
        pub path: ::prost::alloc::vec::Vec<i32>,
        /// Always has exactly three or four elements: start line, start column,
        /// end line (optional, otherwise assumed same as start line), end column.
        /// These are packed into a single field for efficiency.  Note that line
        /// and column numbers are zero-based -- typically you will want to add
        /// 1 to each before displaying to a user.
        #[prost(int32, repeated, tag="2")]
        pub span: ::prost::alloc::vec::Vec<i32>,
        /// If this SourceCodeInfo represents a complete declaration, these are any
        /// comments appearing before and after the declaration which appear to be
        /// attached to the declaration.
        ///
        /// A series of line comments appearing on consecutive lines, with no other
        /// tokens appearing on those lines, will be treated as a single comment.
        ///
        /// leading_detached_comments will keep paragraphs of comments that appear
        /// before (but not connected to) the current element. Each paragraph,
        /// separated by empty lines, will be one comment element in the repeated
        /// field.
        ///
        /// Only the comment content is provided; comment markers (e.g. //) are
        /// stripped out.  For block comments, leading whitespace and an asterisk
        /// will be stripped from the beginning of each line other than the first.
        /// Newlines are included in the output.
        ///
        /// Examples:
        ///
        ///   optional int32 foo = 1;  // Comment attached to foo.
        ///   // Comment attached to bar.
        ///   optional int32 bar = 2;
        ///
        ///   optional string baz = 3;
        ///   // Comment attached to baz.
        ///   // Another line attached to baz.
        ///
        ///   // Comment attached to qux.
        ///   //
        ///   // Another line attached to qux.
        ///   optional double qux = 4;
        ///
        ///   // Detached comment for corge. This is not leading or trailing comments
        ///   // to qux or corge because there are blank lines separating it from
        ///   // both.
        ///
        ///   // Detached comment for corge paragraph 2.
        ///
        ///   optional string corge = 5;
        ///   /* Block comment attached
        ///    * to corge.  Leading asterisks
        ///    * will be removed. */
        ///   /* Block comment attached to
        ///    * grault. */
        ///   optional int32 grault = 6;
        ///
        ///   // ignored detached comments.
        #[prost(string, optional, tag="3")]
        pub leading_comments: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag="4")]
        pub trailing_comments: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, repeated, tag="6")]
        pub leading_detached_comments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Describes the relationship between generated code and its original source
/// file. A GeneratedCodeInfo message is associated with only one generated
/// source file, but may contain references to different source .proto files.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratedCodeInfo {
    /// An Annotation connects some span of text in generated code to an element
    /// of its generating .proto file.
    #[prost(message, repeated, tag="1")]
    pub annotation: ::prost::alloc::vec::Vec<generated_code_info::Annotation>,
}
/// Nested message and enum types in `GeneratedCodeInfo`.
pub mod generated_code_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Annotation {
        /// Identifies the element in the original source .proto file. This field
        /// is formatted the same as SourceCodeInfo.Location.path.
        #[prost(int32, repeated, tag="1")]
        pub path: ::prost::alloc::vec::Vec<i32>,
        /// Identifies the filesystem path to the original source .proto.
        #[prost(string, optional, tag="2")]
        pub source_file: ::core::option::Option<::prost::alloc::string::String>,
        /// Identifies the starting offset in bytes in the generated code
        /// that relates to the identified object.
        #[prost(int32, optional, tag="3")]
        pub begin: ::core::option::Option<i32>,
        /// Identifies the ending offset in bytes in the generated code that
        /// relates to the identified offset. The end offset should be one past
        /// the last relevant byte (so the length of the text = end - begin).
        #[prost(int32, optional, tag="4")]
        pub end: ::core::option::Option<i32>,
    }
}
/// `Any` contains an arbitrary serialized protocol buffer message along with a
/// URL that describes the type of the serialized message.
///
/// Protobuf library provides support to pack/unpack Any values in the form
/// of utility functions or additional generated methods of the Any type.
///
/// Example 1: Pack and unpack a message in C++.
///
///     Foo foo = ...;
///     Any any;
///     any.PackFrom(foo);
///     ...
///     if (any.UnpackTo(&foo)) {
///       ...
///     }
///
/// Example 2: Pack and unpack a message in Java.
///
///     Foo foo = ...;
///     Any any = Any.pack(foo);
///     ...
///     if (any.is(Foo.class)) {
///       foo = any.unpack(Foo.class);
///     }
///
///  Example 3: Pack and unpack a message in Python.
///
///     foo = Foo(...)
///     any = Any()
///     any.Pack(foo)
///     ...
///     if any.Is(Foo.DESCRIPTOR):
///       any.Unpack(foo)
///       ...
///
///  Example 4: Pack and unpack a message in Go
///
///      foo := &pb.Foo{...}
///      any, err := anypb.New(foo)
///      if err != nil {
///        ...
///      }
///      ...
///      foo := &pb.Foo{}
///      if err := any.UnmarshalTo(foo); err != nil {
///        ...
///      }
///
/// The pack methods provided by protobuf library will by default use
/// 'type.googleapis.com/full.type.name' as the type URL and the unpack
/// methods only use the fully qualified type name after the last '/'
/// in the type URL, for example "foo.bar.com/x/y.z" will yield type
/// name "y.z".
///
///
/// JSON
/// ====
/// The JSON representation of an `Any` value uses the regular
/// representation of the deserialized, embedded message, with an
/// additional field `@type` which contains the type URL. Example:
///
///     package google.profile;
///     message Person {
///       string first_name = 1;
///       string last_name = 2;
///     }
///
///     {
///       "@type": "type.googleapis.com/google.profile.Person",
///       "firstName": <string>,
///       "lastName": <string>
///     }
///
/// If the embedded message type is well-known and has a custom JSON
/// representation, that representation will be embedded adding a field
/// `value` which holds the custom JSON in addition to the `@type`
/// field. Example (for message \[google.protobuf.Duration][\]):
///
///     {
///       "@type": "type.googleapis.com/google.protobuf.Duration",
///       "value": "1.212s"
///     }
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Any {
    /// A URL/resource name that uniquely identifies the type of the serialized
    /// protocol buffer message. This string must contain at least
    /// one "/" character. The last segment of the URL's path must represent
    /// the fully qualified name of the type (as in
    /// `path/google.protobuf.Duration`). The name should be in a canonical form
    /// (e.g., leading "." is not accepted).
    ///
    /// In practice, teams usually precompile into the binary all types that they
    /// expect it to use in the context of Any. However, for URLs which use the
    /// scheme `http`, `https`, or no scheme, one can optionally set up a type
    /// server that maps type URLs to message definitions as follows:
    ///
    /// * If no scheme is provided, `https` is assumed.
    /// * An HTTP GET on the URL must yield a \[google.protobuf.Type][\]
    ///   value in binary format, or produce an error.
    /// * Applications are allowed to cache lookup results based on the
    ///   URL, or have them precompiled into a binary to avoid any
    ///   lookup. Therefore, binary compatibility needs to be preserved
    ///   on changes to types. (Use versioned type names to manage
    ///   breaking changes.)
    ///
    /// Note: this functionality is not currently available in the official
    /// protobuf release, and it is not used for type URLs beginning with
    /// type.googleapis.com.
    ///
    /// Schemes other than `http`, `https` (or the empty scheme) might be
    /// used with implementation specific semantics.
    ///
    #[prost(string, tag="1")]
    pub type_url: ::prost::alloc::string::String,
    /// Must be a valid serialized protocol buffer of the above specified type.
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceContext {
    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
}
/// A protocol buffer message type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
    /// The fully qualified message name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The list of fields.
    #[prost(message, repeated, tag="2")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    /// The list of types appearing in `oneof` definitions in this type.
    #[prost(string, repeated, tag="3")]
    pub oneofs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The protocol buffer options.
    #[prost(message, repeated, tag="4")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// The source context.
    #[prost(message, optional, tag="5")]
    pub source_context: ::core::option::Option<SourceContext>,
    /// The source syntax.
    #[prost(enumeration="Syntax", tag="6")]
    pub syntax: i32,
}
/// A single field of a message type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    /// The field type.
    #[prost(enumeration="field::Kind", tag="1")]
    pub kind: i32,
    /// The field cardinality.
    #[prost(enumeration="field::Cardinality", tag="2")]
    pub cardinality: i32,
    /// The field number.
    #[prost(int32, tag="3")]
    pub number: i32,
    /// The field name.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// The field type URL, without the scheme, for message or enumeration
    /// types. Example: `"type.googleapis.com/google.protobuf.Timestamp"`.
    #[prost(string, tag="6")]
    pub type_url: ::prost::alloc::string::String,
    /// The index of the field type in `Type.oneofs`, for message or enumeration
    /// types. The first type has index 1; zero means the type is not in the list.
    #[prost(int32, tag="7")]
    pub oneof_index: i32,
    /// Whether to use alternative packed wire representation.
    #[prost(bool, tag="8")]
    pub packed: bool,
    /// The protocol buffer options.
    #[prost(message, repeated, tag="9")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// The field JSON name.
    #[prost(string, tag="10")]
    pub json_name: ::prost::alloc::string::String,
    /// The string value of the default value of this field. Proto2 syntax only.
    #[prost(string, tag="11")]
    pub default_value: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Field`.
pub mod field {
    /// Basic field types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// Field type unknown.
        TypeUnknown = 0,
        /// Field type double.
        TypeDouble = 1,
        /// Field type float.
        TypeFloat = 2,
        /// Field type int64.
        TypeInt64 = 3,
        /// Field type uint64.
        TypeUint64 = 4,
        /// Field type int32.
        TypeInt32 = 5,
        /// Field type fixed64.
        TypeFixed64 = 6,
        /// Field type fixed32.
        TypeFixed32 = 7,
        /// Field type bool.
        TypeBool = 8,
        /// Field type string.
        TypeString = 9,
        /// Field type group. Proto2 syntax only, and deprecated.
        TypeGroup = 10,
        /// Field type message.
        TypeMessage = 11,
        /// Field type bytes.
        TypeBytes = 12,
        /// Field type uint32.
        TypeUint32 = 13,
        /// Field type enum.
        TypeEnum = 14,
        /// Field type sfixed32.
        TypeSfixed32 = 15,
        /// Field type sfixed64.
        TypeSfixed64 = 16,
        /// Field type sint32.
        TypeSint32 = 17,
        /// Field type sint64.
        TypeSint64 = 18,
    }
    /// Whether a field is optional, required, or repeated.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Cardinality {
        /// For fields with unknown cardinality.
        Unknown = 0,
        /// For optional fields.
        Optional = 1,
        /// For required fields. Proto2 syntax only.
        Required = 2,
        /// For repeated fields.
        Repeated = 3,
    }
}
/// Enum type definition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Enum {
    /// Enum type name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Enum value definitions.
    #[prost(message, repeated, tag="2")]
    pub enumvalue: ::prost::alloc::vec::Vec<EnumValue>,
    /// Protocol buffer options.
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// The source context.
    #[prost(message, optional, tag="4")]
    pub source_context: ::core::option::Option<SourceContext>,
    /// The source syntax.
    #[prost(enumeration="Syntax", tag="5")]
    pub syntax: i32,
}
/// Enum value definition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValue {
    /// Enum value name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Enum value number.
    #[prost(int32, tag="2")]
    pub number: i32,
    /// Protocol buffer options.
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
}
/// A protocol buffer option, which can be attached to a message, field,
/// enumeration, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Option {
    /// The option's name. For protobuf built-in options (options defined in
    /// descriptor.proto), this is the short name. For example, `"map_entry"`.
    /// For custom options, it should be the fully-qualified name. For example,
    /// `"google.api.http"`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The option's value packed in an Any message. If the value is a primitive,
    /// the corresponding wrapper type defined in google/protobuf/wrappers.proto
    /// should be used. If the value is an enum, it should be stored as an int32
    /// value using the google.protobuf.Int32Value type.
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<Any>,
}
/// The syntax in which a protocol buffer element is defined.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Syntax {
    /// Syntax `proto2`.
    Proto2 = 0,
    /// Syntax `proto3`.
    Proto3 = 1,
}
/// Api is a light-weight descriptor for an API Interface.
///
/// Interfaces are also described as "protocol buffer services" in some contexts,
/// such as by the "service" keyword in a .proto file, but they are different
/// from API Services, which represent a concrete implementation of an interface
/// as opposed to simply a description of methods and bindings. They are also
/// sometimes simply referred to as "APIs" in other contexts, such as the name of
/// this message itself. See <https://cloud.google.com/apis/design/glossary> for
/// detailed terminology.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Api {
    /// The fully qualified name of this interface, including package name
    /// followed by the interface's simple name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The methods of this interface, in unspecified order.
    #[prost(message, repeated, tag="2")]
    pub methods: ::prost::alloc::vec::Vec<Method>,
    /// Any metadata attached to the interface.
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// A version string for this interface. If specified, must have the form
    /// `major-version.minor-version`, as in `1.10`. If the minor version is
    /// omitted, it defaults to zero. If the entire version field is empty, the
    /// major version is derived from the package name, as outlined below. If the
    /// field is not empty, the version in the package name will be verified to be
    /// consistent with what is provided here.
    ///
    /// The versioning schema uses [semantic
    /// versioning](<http://semver.org>) where the major version number
    /// indicates a breaking change and the minor version an additive,
    /// non-breaking change. Both version numbers are signals to users
    /// what to expect from different versions, and should be carefully
    /// chosen based on the product plan.
    ///
    /// The major version is also reflected in the package name of the
    /// interface, which must end in `v<major-version>`, as in
    /// `google.feature.v1`. For major versions 0 and 1, the suffix can
    /// be omitted. Zero major versions must only be used for
    /// experimental, non-GA interfaces.
    ///
    ///
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    /// Source context for the protocol buffer service represented by this
    /// message.
    #[prost(message, optional, tag="5")]
    pub source_context: ::core::option::Option<SourceContext>,
    /// Included interfaces. See \[Mixin][\].
    #[prost(message, repeated, tag="6")]
    pub mixins: ::prost::alloc::vec::Vec<Mixin>,
    /// The source syntax of the service.
    #[prost(enumeration="Syntax", tag="7")]
    pub syntax: i32,
}
/// Method represents a method of an API interface.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Method {
    /// The simple name of this method.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A URL of the input message type.
    #[prost(string, tag="2")]
    pub request_type_url: ::prost::alloc::string::String,
    /// If true, the request is streamed.
    #[prost(bool, tag="3")]
    pub request_streaming: bool,
    /// The URL of the output message type.
    #[prost(string, tag="4")]
    pub response_type_url: ::prost::alloc::string::String,
    /// If true, the response is streamed.
    #[prost(bool, tag="5")]
    pub response_streaming: bool,
    /// Any metadata attached to the method.
    #[prost(message, repeated, tag="6")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// The source syntax of this method.
    #[prost(enumeration="Syntax", tag="7")]
    pub syntax: i32,
}
/// Declares an API Interface to be included in this interface. The including
/// interface must redeclare all the methods from the included interface, but
/// documentation and options are inherited as follows:
///
/// - If after comment and whitespace stripping, the documentation
///   string of the redeclared method is empty, it will be inherited
///   from the original method.
///
/// - Each annotation belonging to the service config (http,
///   visibility) which is not set in the redeclared method will be
///   inherited.
///
/// - If an http annotation is inherited, the path pattern will be
///   modified as follows. Any version prefix will be replaced by the
///   version of the including interface plus the \[root][\] path if
///   specified.
///
/// Example of a simple mixin:
///
///     package google.acl.v1;
///     service AccessControl {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v1/{resource=**}:getAcl";
///       }
///     }
///
///     package google.storage.v2;
///     service Storage {
///       rpc GetAcl(GetAclRequest) returns (Acl);
///
///       // Get a data record.
///       rpc GetData(GetDataRequest) returns (Data) {
///         option (google.api.http).get = "/v2/{resource=**}";
///       }
///     }
///
/// Example of a mixin configuration:
///
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///
/// The mixin construct implies that all methods in `AccessControl` are
/// also declared with same name and request/response types in
/// `Storage`. A documentation generator or annotation processor will
/// see the effective `Storage.GetAcl` method after inheriting
/// documentation and annotations as follows:
///
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/{resource=**}:getAcl";
///       }
///       ...
///     }
///
/// Note how the version in the path pattern changed from `v1` to `v2`.
///
/// If the `root` field in the mixin is specified, it should be a
/// relative path under which inherited HTTP paths are placed. Example:
///
///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///         root: acls
///
/// This implies the following inherited HTTP annotation:
///
///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/acls/{resource=**}:getAcl";
///       }
///       ...
///     }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mixin {
    /// The fully qualified name of the interface which is included.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If non-empty specifies a path under which inherited HTTP paths
    /// are rooted.
    #[prost(string, tag="2")]
    pub root: ::prost::alloc::string::String,
}
/// A Duration represents a signed, fixed-length span of time represented
/// as a count of seconds and fractions of seconds at nanosecond
/// resolution. It is independent of any calendar and concepts like "day"
/// or "month". It is related to Timestamp in that the difference between
/// two Timestamp values is a Duration and it can be added or subtracted
/// from a Timestamp. Range is approximately +-10,000 years.
///
/// # Examples
///
/// Example 1: Compute Duration from two Timestamps in pseudo code.
///
///     Timestamp start = ...;
///     Timestamp end = ...;
///     Duration duration = ...;
///
///     duration.seconds = end.seconds - start.seconds;
///     duration.nanos = end.nanos - start.nanos;
///
///     if (duration.seconds < 0 && duration.nanos > 0) {
///       duration.seconds += 1;
///       duration.nanos -= 1000000000;
///     } else if (duration.seconds > 0 && duration.nanos < 0) {
///       duration.seconds -= 1;
///       duration.nanos += 1000000000;
///     }
///
/// Example 2: Compute Timestamp from Timestamp + Duration in pseudo code.
///
///     Timestamp start = ...;
///     Duration duration = ...;
///     Timestamp end = ...;
///
///     end.seconds = start.seconds + duration.seconds;
///     end.nanos = start.nanos + duration.nanos;
///
///     if (end.nanos < 0) {
///       end.seconds -= 1;
///       end.nanos += 1000000000;
///     } else if (end.nanos >= 1000000000) {
///       end.seconds += 1;
///       end.nanos -= 1000000000;
///     }
///
/// Example 3: Compute Duration from datetime.timedelta in Python.
///
///     td = datetime.timedelta(days=3, minutes=10)
///     duration = Duration()
///     duration.FromTimedelta(td)
///
/// # JSON Mapping
///
/// In JSON format, the Duration type is encoded as a string rather than an
/// object, where the string ends in the suffix "s" (indicating seconds) and
/// is preceded by the number of seconds, with nanoseconds expressed as
/// fractional seconds. For example, 3 seconds with 0 nanoseconds should be
/// encoded in JSON format as "3s", while 3 seconds and 1 nanosecond should
/// be expressed in JSON format as "3.000000001s", and 3 seconds and 1
/// microsecond should be expressed in JSON format as "3.000001s".
///
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Duration {
    /// Signed seconds of the span of time. Must be from -315,576,000,000
    /// to +315,576,000,000 inclusive. Note: these bounds are computed from:
    /// 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
    #[prost(int64, tag="1")]
    pub seconds: i64,
    /// Signed fractions of a second at nanosecond resolution of the span
    /// of time. Durations less than one second are represented with a 0
    /// `seconds` field and a positive or negative `nanos` field. For durations
    /// of one second or more, a non-zero value for the `nanos` field must be
    /// of the same sign as the `seconds` field. Must be from -999,999,999
    /// to +999,999,999 inclusive.
    #[prost(int32, tag="2")]
    pub nanos: i32,
}
/// `FieldMask` represents a set of symbolic field paths, for example:
///
///     paths: "f.a"
///     paths: "f.b.d"
///
/// Here `f` represents a field in some root message, `a` and `b`
/// fields in the message found in `f`, and `d` a field found in the
/// message in `f.b`.
///
/// Field masks are used to specify a subset of fields that should be
/// returned by a get operation or modified by an update operation.
/// Field masks also have a custom JSON encoding (see below).
///
/// # Field Masks in Projections
///
/// When used in the context of a projection, a response message or
/// sub-message is filtered by the API to only contain those fields as
/// specified in the mask. For example, if the mask in the previous
/// example is applied to a response message as follows:
///
///     f {
///       a : 22
///       b {
///         d : 1
///         x : 2
///       }
///       y : 13
///     }
///     z: 8
///
/// The result will not contain specific values for fields x,y and z
/// (their value will be set to the default, and omitted in proto text
/// output):
///
///
///     f {
///       a : 22
///       b {
///         d : 1
///       }
///     }
///
/// A repeated field is not allowed except at the last position of a
/// paths string.
///
/// If a FieldMask object is not present in a get operation, the
/// operation applies to all fields (as if a FieldMask of all fields
/// had been specified).
///
/// Note that a field mask does not necessarily apply to the
/// top-level response message. In case of a REST get operation, the
/// field mask applies directly to the response, but in case of a REST
/// list operation, the mask instead applies to each individual message
/// in the returned resource list. In case of a REST custom method,
/// other definitions may be used. Where the mask applies will be
/// clearly documented together with its declaration in the API.  In
/// any case, the effect on the returned resource/resources is required
/// behavior for APIs.
///
/// # Field Masks in Update Operations
///
/// A field mask in update operations specifies which fields of the
/// targeted resource are going to be updated. The API is required
/// to only change the values of the fields as specified in the mask
/// and leave the others untouched. If a resource is passed in to
/// describe the updated values, the API ignores the values of all
/// fields not covered by the mask.
///
/// If a repeated field is specified for an update operation, new values will
/// be appended to the existing repeated field in the target resource. Note that
/// a repeated field is only allowed in the last position of a `paths` string.
///
/// If a sub-message is specified in the last position of the field mask for an
/// update operation, then new value will be merged into the existing sub-message
/// in the target resource.
///
/// For example, given the target message:
///
///     f {
///       b {
///         d: 1
///         x: 2
///       }
///       c: \[1\]
///     }
///
/// And an update message:
///
///     f {
///       b {
///         d: 10
///       }
///       c: \[2\]
///     }
///
/// then if the field mask is:
///
///  paths: ["f.b", "f.c"]
///
/// then the result will be:
///
///     f {
///       b {
///         d: 10
///         x: 2
///       }
///       c: [1, 2]
///     }
///
/// An implementation may provide options to override this default behavior for
/// repeated and message fields.
///
/// In order to reset a field's value to the default, the field must
/// be in the mask and set to the default value in the provided resource.
/// Hence, in order to reset all fields of a resource, provide a default
/// instance of the resource and set all fields in the mask, or do
/// not provide a mask as described below.
///
/// If a field mask is not present on update, the operation applies to
/// all fields (as if a field mask of all fields has been specified).
/// Note that in the presence of schema evolution, this may mean that
/// fields the client does not know and has therefore not filled into
/// the request will be reset to their default. If this is unwanted
/// behavior, a specific service may require a client to always specify
/// a field mask, producing an error if not.
///
/// As with get operations, the location of the resource which
/// describes the updated values in the request message depends on the
/// operation kind. In any case, the effect of the field mask is
/// required to be honored by the API.
///
/// ## Considerations for HTTP REST
///
/// The HTTP kind of an update operation which uses a field mask must
/// be set to PATCH instead of PUT in order to satisfy HTTP semantics
/// (PUT must only be used for full updates).
///
/// # JSON Encoding of Field Masks
///
/// In JSON, a field mask is encoded as a single string where paths are
/// separated by a comma. Fields name in each path are converted
/// to/from lower-camel naming conventions.
///
/// As an example, consider the following message declarations:
///
///     message Profile {
///       User user = 1;
///       Photo photo = 2;
///     }
///     message User {
///       string display_name = 1;
///       string address = 2;
///     }
///
/// In proto a field mask for `Profile` may look as such:
///
///     mask {
///       paths: "user.display_name"
///       paths: "photo"
///     }
///
/// In JSON, the same mask is represented as below:
///
///     {
///       mask: "user.displayName,photo"
///     }
///
/// # Field Masks and Oneof Fields
///
/// Field masks treat fields in oneofs just as regular fields. Consider the
/// following message:
///
///     message SampleMessage {
///       oneof test_oneof {
///         string name = 4;
///         SubMessage sub_message = 9;
///       }
///     }
///
/// The field mask can be:
///
///     mask {
///       paths: "name"
///     }
///
/// Or:
///
///     mask {
///       paths: "sub_message"
///     }
///
/// Note that oneof type names ("test_oneof" in this case) cannot be used in
/// paths.
///
/// ## Field Mask Verification
///
/// The implementation of any API method which has a FieldMask type field in the
/// request should verify the included field paths, and return an
/// `INVALID_ARGUMENT` error if any path is unmappable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldMask {
    /// The set of field mask paths.
    #[prost(string, repeated, tag="1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.
///
/// The JSON representation for `Struct` is JSON object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {
    /// Unordered map of dynamically typed values.
    #[prost(btree_map="string, message", tag="1")]
    pub fields: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, Value>,
}
/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of that
/// variants, absence of any variant indicates an error.
///
/// The JSON representation for `Value` is JSON value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// The kind of value.
    #[prost(oneof="value::Kind", tags="1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Represents a null value.
        #[prost(enumeration="super::NullValue", tag="1")]
        NullValue(i32),
        /// Represents a double value.
        #[prost(double, tag="2")]
        NumberValue(f64),
        /// Represents a string value.
        #[prost(string, tag="3")]
        StringValue(::prost::alloc::string::String),
        /// Represents a boolean value.
        #[prost(bool, tag="4")]
        BoolValue(bool),
        /// Represents a structured value.
        #[prost(message, tag="5")]
        StructValue(super::Struct),
        /// Represents a repeated `Value`.
        #[prost(message, tag="6")]
        ListValue(super::ListValue),
    }
}
/// `ListValue` is a wrapper around a repeated field of values.
///
/// The JSON representation for `ListValue` is JSON array.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    /// Repeated field of dynamically typed values.
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.
///
///  The JSON representation for `NullValue` is JSON `null`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NullValue {
    /// Null value.
    NullValue = 0,
}
/// A Timestamp represents a point in time independent of any time zone or local
/// calendar, encoded as a count of seconds and fractions of seconds at
/// nanosecond resolution. The count is relative to an epoch at UTC midnight on
/// January 1, 1970, in the proleptic Gregorian calendar which extends the
/// Gregorian calendar backwards to year one.
///
/// All minutes are 60 seconds long. Leap seconds are "smeared" so that no leap
/// second table is needed for interpretation, using a [24-hour linear
/// smear](<https://developers.google.com/time/smear>).
///
/// The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By
/// restricting to that range, we ensure that we can convert to and from [RFC
/// 3339](<https://www.ietf.org/rfc/rfc3339.txt>) date strings.
///
/// # Examples
///
/// Example 1: Compute Timestamp from POSIX `time()`.
///
///     Timestamp timestamp;
///     timestamp.set_seconds(time(NULL));
///     timestamp.set_nanos(0);
///
/// Example 2: Compute Timestamp from POSIX `gettimeofday()`.
///
///     struct timeval tv;
///     gettimeofday(&tv, NULL);
///
///     Timestamp timestamp;
///     timestamp.set_seconds(tv.tv_sec);
///     timestamp.set_nanos(tv.tv_usec * 1000);
///
/// Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.
///
///     FILETIME ft;
///     GetSystemTimeAsFileTime(&ft);
///     UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;
///
///     // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
///     // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
///     Timestamp timestamp;
///     timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
///     timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
///
/// Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.
///
///     long millis = System.currentTimeMillis();
///
///     Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
///         .setNanos((int) ((millis % 1000) * 1000000)).build();
///
///
/// Example 5: Compute Timestamp from Java `Instant.now()`.
///
///     Instant now = Instant.now();
///
///     Timestamp timestamp =
///         Timestamp.newBuilder().setSeconds(now.getEpochSecond())
///             .setNanos(now.getNano()).build();
///
///
/// Example 6: Compute Timestamp from current time in Python.
///
///     timestamp = Timestamp()
///     timestamp.GetCurrentTime()
///
/// # JSON Mapping
///
/// In JSON format, the Timestamp type is encoded as a string in the
/// [RFC 3339](<https://www.ietf.org/rfc/rfc3339.txt>) format. That is, the
/// format is "{year}-{month}-{day}T{hour}:{min}:{sec}\[.{frac_sec}\]Z"
/// where {year} is always expressed using four digits while {month}, {day},
/// {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
/// seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
/// are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
/// is required. A proto3 JSON serializer should always use UTC (as indicated by
/// "Z") when printing the Timestamp type and a proto3 JSON parser should be
/// able to accept both UTC and other timezones (as indicated by an offset).
///
/// For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
/// 01:30 UTC on January 15, 2017.
///
/// In JavaScript, one can convert a Date object to this format using the
/// standard
/// \[toISOString()\](<https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString>)
/// method. In Python, a standard `datetime.datetime` object can be converted
/// to this format using
/// \[`strftime`\](<https://docs.python.org/2/library/time.html#time.strftime>) with
/// the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one can use
/// the Joda Time's \[`ISODateTimeFormat.dateTime()`\](
/// <http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime%2D%2D>
/// ) to obtain a formatter capable of generating timestamps in this format.
///
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag="1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag="2")]
    pub nanos: i32,
}
