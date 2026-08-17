// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use objc::runtime::{BOOL, NO, YES};

use std::ffi::{c_char, CStr};
use std::ptr;

/// Only available on (macos(10.12), ios(10.0)
///
/// See <https://developer.apple.com/documentation/metal/mtlpatchtype/>
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLPatchType {
    None = 0,
    Triangle = 1,
    Quad = 2,
}

/// See <https://developer.apple.com/documentation/metal/mtlvertexattribute/>
pub enum MTLVertexAttribute {}

foreign_obj_type! {
    type CType = MTLVertexAttribute;
    pub struct VertexAttribute;
}

impl VertexAttributeRef {
    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    pub fn attribute_index(&self) -> u64 {
        unsafe { msg_send![self, attributeIndex] }
    }

    pub fn attribute_type(&self) -> MTLDataType {
        unsafe { msg_send![self, attributeType] }
    }

    pub fn is_active(&self) -> bool {
        unsafe { msg_send_bool![self, isActive] }
    }

    /// Only available on (macos(10.12), ios(10.0)
    pub fn is_patch_data(&self) -> bool {
        unsafe { msg_send_bool![self, isPatchData] }
    }

    /// Only available on (macos(10.12), ios(10.0)
    pub fn is_patch_control_point_data(&self) -> bool {
        unsafe { msg_send_bool![self, isPatchControlPointData] }
    }
}

/// Only available on (macos(10.12), ios(10.0))
///
/// See <https://developer.apple.com/documentation/metal/mtlattribute/>
pub enum MTLAttribute {}

foreign_obj_type! {
    type CType = MTLAttribute;
    pub struct Attribute;
}

impl AttributeRef {
    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    pub fn attribute_index(&self) -> u64 {
        unsafe { msg_send![self, attributeIndex] }
    }

    pub fn attribute_type(&self) -> MTLDataType {
        unsafe { msg_send![self, attributeType] }
    }

    pub fn is_active(&self) -> bool {
        unsafe { msg_send_bool![self, isActive] }
    }

    /// Only available on (macos(10.12), ios(10.0))
    pub fn is_patch_data(&self) -> bool {
        unsafe { msg_send_bool![self, isPatchData] }
    }

    /// Only available on (macos(10.12), ios(10.0))
    pub fn is_patch_control_point_data(&self) -> bool {
        unsafe { msg_send_bool![self, isPatchControlPointData] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlfunctiontype/>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLFunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
    /// Only available on (macos(11.0), ios(14.0))
    Visible = 5,
    /// Only available on (macos(11.0), ios(14.0))
    Intersection = 6,
}

/// Only available on (macos(10.12), ios(10.0))
///
/// See <https://developer.apple.com/documentation/metal/mtlfunctionconstant/>
pub enum MTLFunctionConstant {}

foreign_obj_type! {
    type CType = MTLFunctionConstant;
    pub struct FunctionConstant;
}

impl FunctionConstantRef {
    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    pub fn data_type(&self) -> MTLDataType {
        unsafe { msg_send![self, type] }
    }

    pub fn index(&self) -> NSUInteger {
        unsafe { msg_send![self, index] }
    }

    pub fn required(&self) -> bool {
        unsafe { msg_send_bool![self, required] }
    }
}

bitflags::bitflags! {
    /// Only available on (macos(11.0), ios(14.0))
    ///
    /// See <https://developer.apple.com/documentation/metal/mtlfunctionoptions/>
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MTLFunctionOptions: NSUInteger {
        const None = 0;
        const CompileToBinary = 1 << 0;
    }
}

/// Only available on (macos(11.0), ios(14.0))
///
/// See <https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/>
pub enum MTLFunctionDescriptor {}

foreign_obj_type! {
    type CType = MTLFunctionDescriptor;
    pub struct FunctionDescriptor;
}

impl FunctionDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLFunctionDescriptor);
            msg_send![class, new]
        }
    }
}

impl FunctionDescriptorRef {
    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
            let ns_name = crate::nsstring_from_str(name);
            let () = msg_send![self, setName: ns_name];
        }
    }

    pub fn specialized_name(&self) -> &str {
        unsafe {
            let name = msg_send![self, specializedName];
            crate::nsstring_as_str(name)
        }
    }

    pub fn set_specialized_name(&self, name: &str) {
        unsafe {
            let ns_name = crate::nsstring_from_str(name);
            let () = msg_send![self, setSpecializedName: ns_name];
        }
    }

    pub fn constant_values(&self) -> &FunctionConstantValuesRef {
        unsafe { msg_send![self, constantValues] }
    }

    pub fn set_constant_values(&self, values: &FunctionConstantValuesRef) {
        unsafe { msg_send![self, setConstantValues: values] }
    }

    pub fn options(&self) -> MTLFunctionOptions {
        unsafe { msg_send![self, options] }
    }

    pub fn set_options(&self, options: MTLFunctionOptions) {
        unsafe { msg_send![self, setOptions: options] }
    }
}

/// Only available on (macos(11.0), ios(14.0))
///
/// See <https://developer.apple.com/documentation/metal/mtlintersectionfunctiondescriptor/>
pub enum MTLIntersectionFunctionDescriptor {}

foreign_obj_type! {
    type CType = MTLIntersectionFunctionDescriptor;
    pub struct IntersectionFunctionDescriptor;
    type ParentType = FunctionDescriptor;
}

/// Only available on (macos(11.0), ios(14.0))
///
/// See <https://developer.apple.com/documentation/metal/mtlfunctionhandle/>
pub enum MTLFunctionHandle {}

foreign_obj_type! {
    type CType = MTLFunctionHandle;
    pub struct FunctionHandle;
}

impl FunctionHandleRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn name(&self) -> &str {
        unsafe {
            let ns_name = msg_send![self, name];
            crate::nsstring_as_str(ns_name)
        }
    }

    pub fn function_type(&self) -> MTLFunctionType {
        unsafe { msg_send![self, functionType] }
    }
}

// TODO:
// MTLIntersectionFunctionSignature
// MTLIntersectionFunctionTableDescriptor
// MTLIntersectionFunctionTable

/// See <https://developer.apple.com/documentation/metal/mtlfunction/>
pub enum MTLFunction {}

foreign_obj_type! {
    type CType = MTLFunction;
    pub struct Function;
}

impl FunctionRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    /// Only available on (macos(10.12), ios(10.0))
    pub fn label(&self) -> &str {
        unsafe {
            let ns_label = msg_send![self, label];
            crate::nsstring_as_str(ns_label)
        }
    }

    /// Only available on (macos(10.12), ios(10.0))
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_label = crate::nsstring_from_str(label);
            let () = msg_send![self, setLabel: ns_label];
        }
    }

    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    pub fn function_type(&self) -> MTLFunctionType {
        unsafe { msg_send![self, functionType] }
    }

    /// Only available on (macos(10.12), ios(10.0))
    pub fn patch_type(&self) -> MTLPatchType {
        unsafe { msg_send![self, patchType] }
    }

    /// Only available on (macos(10.12), ios(10.0))
    pub fn patch_control_point_count(&self) -> NSUInteger {
        unsafe { msg_send![self, patchControlPointCount] }
    }

    /// Only available on (macos(10.12), ios(10.0))
    pub fn vertex_attributes(&self) -> &Array<VertexAttribute> {
        unsafe { msg_send![self, vertexAttributes] }
    }

    /// Only available on (macos(10.12), ios(10.0))
    pub fn stage_input_attributes(&self) -> &Array<Attribute> {
        unsafe { msg_send![self, stageInputAttributes] }
    }

    pub fn new_argument_encoder(&self, buffer_index: NSUInteger) -> ArgumentEncoder {
        unsafe {
            let ptr = msg_send![self, newArgumentEncoderWithBufferIndex: buffer_index];
            ArgumentEncoder::from_ptr(ptr)
        }
    }

    pub fn function_constants_dictionary(&self) -> *mut Object {
        unsafe { msg_send![self, functionConstantsDictionary] }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn options(&self) -> MTLFunctionOptions {
        unsafe { msg_send![self, options] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtllanguageversion/>
#[repr(u64)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum MTLLanguageVersion {
    V1_0 = 0x10000,
    V1_1 = 0x10001,
    V1_2 = 0x10002,
    V2_0 = 0x20000,
    V2_1 = 0x20001,
    V2_2 = 0x20002,
    /// available on macOS 11.0+, iOS 14.0+
    V2_3 = 0x20003,
    /// available on macOS 12.0+, iOS 15.0+
    V2_4 = 0x20004,
    /// available on macOS 13.0+, iOS 16.0+
    V3_0 = 0x30000,
    /// available on macOS 14.0+, iOS 17.0+
    V3_1 = 0x30001,
}

/// See <https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/>
pub enum MTLFunctionConstantValues {}

foreign_obj_type! {
    type CType = MTLFunctionConstantValues;
    pub struct FunctionConstantValues;
}

impl FunctionConstantValues {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLFunctionConstantValues);
            msg_send![class, new]
        }
    }
}

impl FunctionConstantValuesRef {
    pub fn set_constant_value_at_index(
        &self,
        value: *const c_void,
        ty: MTLDataType,
        index: NSUInteger,
    ) {
        unsafe { msg_send![self, setConstantValue:value type:ty atIndex:index] }
    }

    pub fn set_constant_values_with_range(
        &self,
        values: *const c_void,
        ty: MTLDataType,
        range: NSRange,
    ) {
        unsafe { msg_send![self, setConstantValues:values type:ty withRange:range] }
    }

    pub fn set_constant_value_with_name(&self, value: *const c_void, ty: MTLDataType, name: &str) {
        unsafe {
            let ns_name = crate::nsstring_from_str(name);
            msg_send![self, setConstantValue:value type:ty withName:ns_name]
        }
    }
}

/// Only available on (macos(11.0), ios(14.0))
///
/// See <https://developer.apple.com/documentation/metal/mtllibrarytype/>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLLibraryType {
    Executable = 0,
    Dynamic = 1,
}

/// See <https://developer.apple.com/documentation/metal/mtlcompileoptions/>
pub enum MTLCompileOptions {}

foreign_obj_type! {
    type CType = MTLCompileOptions;
    pub struct CompileOptions;
}

impl CompileOptions {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLCompileOptions);
            msg_send![class, new]
        }
    }
}

impl CompileOptionsRef {
    pub unsafe fn preprocessor_macros(&self) -> *mut Object {
        msg_send![self, preprocessorMacros]
    }

    pub unsafe fn set_preprocessor_macros(&self, defines: *mut Object) {
        msg_send![self, setPreprocessorMacros: defines]
    }

    pub fn is_fast_math_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, fastMathEnabled] }
    }

    pub fn set_fast_math_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setFastMathEnabled: enabled] }
    }

    /// Only available on (macos(10.11), ios(9.0))
    pub fn language_version(&self) -> MTLLanguageVersion {
        unsafe { msg_send![self, languageVersion] }
    }

    /// Only available on (macos(10.11), ios(9.0))
    pub fn set_language_version(&self, version: MTLLanguageVersion) {
        unsafe { msg_send![self, setLanguageVersion: version] }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn library_type(&self) -> MTLLibraryType {
        unsafe { msg_send![self, libraryType] }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn set_library_type(&self, lib_type: MTLLibraryType) {
        unsafe { msg_send![self, setLibraryType: lib_type] }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn install_name(&self) -> &str {
        unsafe {
            let name = msg_send![self, installName];
            crate::nsstring_as_str(name)
        }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn set_install_name(&self, name: &str) {
        unsafe {
            let install_name = crate::nsstring_from_str(name);
            let () = msg_send![self, setInstallName: install_name];
        }
    }

    /// Only available on (macos(11.0), ios(14.0))
    ///
    /// Marshal to Rust Vec
    pub fn libraries(&self) -> Vec<DynamicLibrary> {
        unsafe {
            let libraries: *mut Object = msg_send![self, libraries];
            let count: NSUInteger = msg_send![libraries, count];
            (0..count)
                .map(|i| {
                    let lib = msg_send![libraries, objectAtIndex: i];
                    DynamicLibrary::from_ptr(lib)
                })
                .collect()
        }
    }

    /// Only available on (macos(11.0), ios(14.0))
    ///
    /// As raw NSArray
    pub fn libraries_as_nsarray(&self) -> &ArrayRef<DynamicLibrary> {
        unsafe { msg_send![self, libraries] }
    }

    /// Only available on (macos(11.0), ios(14.0))
    ///
    /// Marshal from Rust slice
    pub fn set_libraries(&self, libraries: &[&DynamicLibraryRef]) {
        let ns_array = Array::<DynamicLibrary>::from_slice(libraries);
        unsafe { msg_send![self, setLibraries: ns_array] }
    }

    /// Only available on (macos(11.0), ios(14.0))
    ///
    /// From raw NSArray
    pub fn set_libraries_nsarray(&self, libraries: &ArrayRef<DynamicLibrary>) {
        unsafe { msg_send![self, setLibraries: libraries] }
    }

    /// Only available on (macos(11.0), macCatalyst(14.0), ios(13.0))
    pub fn preserve_invariance(&self) -> bool {
        unsafe { msg_send_bool![self, preserveInvariance] }
    }

    /// Only available on (macos(11.0), macCatalyst(14.0), ios(13.0))
    pub fn set_preserve_invariance(&self, preserve: bool) {
        unsafe { msg_send![self, setPreserveInvariance: preserve] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtllibraryerror/>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLLibraryError {
    Unsupported = 1,
    Internal = 2,
    CompileFailure = 3,
    CompileWarning = 4,
    /// Only available on (macos(10.12), ios(10.0))
    FunctionNotFound = 5,
    /// Only available on (macos(10.12), ios(10.0))
    FileNotFound = 6,
}

/// See <https://developer.apple.com/documentation/metal/mtllibrary/>
pub enum MTLLibrary {}

foreign_obj_type! {
    type CType = MTLLibrary;
    pub struct Library;
}

impl LibraryRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(label);
            let () = msg_send![self, setLabel: nslabel];
        }
    }

    /// Retrieve a function from the library.
    ///
    /// Although this method is named `get_function`, the underlying Metal
    /// method is named `newFunctionWithName`, and it returns a retained object
    /// that has not been added to the autorelease pool.
    /// See <https://github.com/gfx-rs/metal-rs/issues/128>.
    // FIXME: should rename to new_function
    pub fn get_function(
        &self,
        name: &str,
        constants: Option<FunctionConstantValues>,
    ) -> Result<Function, String> {
        unsafe {
            let nsname = crate::nsstring_from_str(name);

            let function: *mut MTLFunction = match constants {
                Some(c) => try_objc! { err => msg_send![self,
                    newFunctionWithName: nsname.as_ref()
                    constantValues: c.as_ref()
                    error: &mut err
                ]},
                None => msg_send![self, newFunctionWithName: nsname.as_ref()],
            };

            if !function.is_null() {
                Ok(Function::from_ptr(function))
            } else {
                Err(format!("Function '{}' does not exist", name))
            }
        }
    }

    // TODO: get_function_async with completion handler

    pub fn function_names(&self) -> Vec<String> {
        unsafe {
            let names: *mut Object = msg_send![self, functionNames];
            let count: NSUInteger = msg_send![names, count];
            (0..count)
                .map(|i| {
                    let name = msg_send![names, objectAtIndex: i];
                    nsstring_as_str(name).to_string()
                })
                .collect()
        }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn library_type(&self) -> MTLLibraryType {
        unsafe { msg_send![self, type] }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn install_name(&self) -> Option<&str> {
        unsafe {
            let maybe_name: *mut Object = msg_send![self, installName];
            maybe_name.as_ref().map(crate::nsstring_as_str)
        }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn new_function_with_descriptor(
        &self,
        descriptor: &FunctionDescriptorRef,
    ) -> Result<Function, String> {
        unsafe {
            let function: *mut MTLFunction = try_objc! {
                err => msg_send![self,
                    newFunctionWithDescriptor: descriptor
                    error: &mut err
                ]
            };

            if !function.is_null() {
                Ok(Function::from_ptr(function))
            } else {
                Err(String::from("new_function_with_descriptor() failed"))
            }
        }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn new_intersection_function_with_descriptor(
        &self,
        descriptor: &IntersectionFunctionDescriptorRef,
    ) -> Result<Function, String> {
        unsafe {
            let function: *mut MTLFunction = try_objc! {
                err => msg_send![self,
                    newIntersectionFunctionWithDescriptor: descriptor
                    error: &mut err
                ]
            };

            if !function.is_null() {
                Ok(Function::from_ptr(function))
            } else {
                Err(String::from(
                    "new_intersection_function_with_descriptor() failed",
                ))
            }
        }
    }
}

/// Only available on (macos(11.0), ios(14.0))
///
/// See <https://developer.apple.com/documentation/metal/mtldynamiclibraryerror/>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLDynamicLibraryError {
    None = 0,
    InvalidFile = 1,
    CompilationFailure = 2,
    UnresolvedInstallName = 3,
    DependencyLoadFailure = 4,
    Unsupported = 5,
}

/// See <https://developer.apple.com/documentation/metal/mtldynamiclibrary/>
pub enum MTLDynamicLibrary {}

foreign_obj_type! {
    type CType = MTLDynamicLibrary;
    pub struct DynamicLibrary;
}

impl DynamicLibraryRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(label);
            let () = msg_send![self, setLabel: nslabel];
        }
    }

    pub fn install_name(&self) -> &str {
        unsafe {
            let name = msg_send![self, installName];
            crate::nsstring_as_str(name)
        }
    }

    pub fn serialize_to_url(&self, url: &URLRef) -> Result<bool, String> {
        unsafe { msg_send_bool_error_check![self, serializeToURL: url] }
    }
}

/// macOS 11.0+ iOS 14.0+
///
/// See <https://developer.apple.com/documentation/metal/mtlbinaryarchivedescriptor/>
pub enum MTLBinaryArchiveDescriptor {}

foreign_obj_type! {
    type CType = MTLBinaryArchiveDescriptor;
    pub struct BinaryArchiveDescriptor;
}

impl BinaryArchiveDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLBinaryArchiveDescriptor);
            msg_send![class, new]
        }
    }
}

impl BinaryArchiveDescriptorRef {
    pub fn url(&self) -> &URLRef {
        unsafe { msg_send![self, url] }
    }
    pub fn set_url(&self, url: &URLRef) {
        unsafe { msg_send![self, setUrl: url] }
    }
}

/// macOS 11.0+ iOS 14.0+
///
/// See <https://developer.apple.com/documentation/metal/mtlbinaryarchive/>
pub enum MTLBinaryArchive {}

foreign_obj_type! {
    type CType = MTLBinaryArchive;
    pub struct BinaryArchive;
}

impl BinaryArchiveRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(label);
            let () = msg_send![self, setLabel: nslabel];
        }
    }

    pub fn add_compute_pipeline_functions_with_descriptor(
        &self,
        descriptor: &ComputePipelineDescriptorRef,
    ) -> Result<bool, String> {
        unsafe {
            msg_send_bool_error_check![self, addComputePipelineFunctionsWithDescriptor: descriptor]
        }
    }

    pub fn add_render_pipeline_functions_with_descriptor(
        &self,
        descriptor: &RenderPipelineDescriptorRef,
    ) -> Result<bool, String> {
        unsafe {
            msg_send_bool_error_check![self, addRenderPipelineFunctionsWithDescriptor: descriptor]
        }
    }

    // TODO: addTileRenderPipelineFunctionsWithDescriptor
    // - (BOOL)addTileRenderPipelineFunctionsWithDescriptor:(MTLTileRenderPipelineDescriptor *)descriptor
    // error:(NSError * _Nullable *)error;

    pub fn serialize_to_url(&self, url: &URLRef) -> Result<bool, String> {
        unsafe {
            let mut err: *mut Object = ptr::null_mut();
            let result: BOOL = msg_send![self, serializeToURL:url
                                                        error:&mut err];
            if !err.is_null() {
                // FIXME: copy pasta
                let desc: *mut Object = msg_send![err, localizedDescription];
                let c_msg: *const c_char = msg_send![desc, UTF8String];
                let message = CStr::from_ptr(c_msg).to_string_lossy().into_owned();
                Err(message)
            } else {
                match result {
                    YES => Ok(true),
                    NO => Ok(false),
                    #[cfg(not(target_arch = "aarch64"))]
                    _ => unreachable!(),
                }
            }
        }
    }
}

/// macOS 11.0+ iOS 14.0+
///
/// See <https://developer.apple.com/documentation/metal/mtllinkedfunctions/>
pub enum MTLLinkedFunctions {}

foreign_obj_type! {
    type CType = MTLLinkedFunctions;
    pub struct LinkedFunctions;
}

impl LinkedFunctions {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLLinkedFunctions);
            msg_send![class, new]
        }
    }
}

impl LinkedFunctionsRef {
    /// Marshal to Rust Vec
    pub fn functions(&self) -> Vec<Function> {
        unsafe {
            let functions: *mut Object = msg_send![self, functions];
            let count: NSUInteger = msg_send![functions, count];
            (0..count)
                .map(|i| {
                    let f = msg_send![functions, objectAtIndex: i];
                    Function::from_ptr(f)
                })
                .collect()
        }
    }

    /// Marshal from Rust slice
    pub fn set_functions(&self, functions: &[&FunctionRef]) {
        let ns_array = Array::<Function>::from_slice(functions);
        unsafe { msg_send![self, setFunctions: ns_array] }
    }

    /// Marshal to Rust Vec
    pub fn binary_functions(&self) -> Vec<Function> {
        unsafe {
            let functions: *mut Object = msg_send![self, binaryFunctions];
            let count: NSUInteger = msg_send![functions, count];
            (0..count)
                .map(|i| {
                    let f = msg_send![functions, objectAtIndex: i];
                    Function::from_ptr(f)
                })
                .collect()
        }
    }

    /// Marshal from Rust slice
    pub fn set_binary_functions(&self, functions: &[&FunctionRef]) {
        let ns_array = Array::<Function>::from_slice(functions);
        unsafe { msg_send![self, setBinaryFunctions: ns_array] }
    }

    // TODO: figure out NSDictionary wrapper
    // TODO: groups
    // @property (readwrite, nonatomic, copy, nullable) NSDictionary<NSString*, NSArray<id<MTLFunction>>*> *groups;
}
