/*!
Generating accesses to [`ByteAddressBuffer`] contents.

Naga IR globals in the [`Storage`] address space are rendered as
[`ByteAddressBuffer`]s or [`RWByteAddressBuffer`]s in HLSL. These
buffers don't have HLSL types (structs, arrays, etc.); instead, they
are just raw blocks of bytes, with methods to load and store values of
specific types at particular byte offsets. This means that Naga must
translate chains of [`Access`] and [`AccessIndex`] expressions into
HLSL expressions that compute byte offsets into the buffer.

To generate code for a [`Storage`] access:

- Call [`Writer::fill_access_chain`] on the expression referring to
  the value. This populates [`Writer::temp_access_chain`] with the
  appropriate byte offset calculations, as a vector of [`SubAccess`]
  values.

- Call [`Writer::write_storage_address`] to emit an HLSL expression
  for a given slice of [`SubAccess`] values.

Naga IR expressions can operate on composite values of any type, but
[`ByteAddressBuffer`] and [`RWByteAddressBuffer`] have only a fixed
set of `Load` and `Store` methods, to access one through four
consecutive 32-bit values. To synthesize a Naga access, you can
initialize [`temp_access_chain`] to refer to the composite, and then
temporarily push and pop additional steps on
[`Writer::temp_access_chain`] to generate accesses to the individual
elements/members.

The [`temp_access_chain`] field is a member of [`Writer`] solely to
allow re-use of the `Vec`'s dynamic allocation. Its value is no longer
needed once HLSL for the access has been generated.

Note about DXC and Load/Store functions:

DXC's HLSL has a generic [`Load` and `Store`] function for [`ByteAddressBuffer`] and
[`RWByteAddressBuffer`]. This is not available in FXC's HLSL, so we use
it only for types that are only available in DXC. Notably 64 and 16 bit types.

FXC's HLSL has functions Load, Load2, Load3, and Load4 and Store, Store2, Store3, Store4.
This loads/stores a vector of length 1, 2, 3, or 4. We use that for 32bit types, bitcasting to the
correct type if necessary.

[`Storage`]: crate::AddressSpace::Storage
[`ByteAddressBuffer`]: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/sm5-object-byteaddressbuffer
[`RWByteAddressBuffer`]: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/sm5-object-rwbyteaddressbuffer
[`Access`]: crate::Expression::Access
[`AccessIndex`]: crate::Expression::AccessIndex
[`Writer::fill_access_chain`]: super::Writer::fill_access_chain
[`Writer::write_storage_address`]: super::Writer::write_storage_address
[`Writer::temp_access_chain`]: super::Writer::temp_access_chain
[`temp_access_chain`]: super::Writer::temp_access_chain
[`Writer`]: super::Writer
[`Load` and `Store`]: https://github.com/microsoft/DirectXShaderCompiler/wiki/ByteAddressBuffer-Load-Store-Additions
*/

use alloc::format;
use core::{fmt, mem};

use super::{super::FunctionCtx, BackendResult, Error};
use crate::{
    proc::{Alignment, NameKey, TypeResolution},
    Handle,
};

const STORE_TEMP_NAME: &str = "_value";

/// One step in accessing a [`Storage`] global's component or element.
///
/// [`Writer::temp_access_chain`] holds a series of these structures,
/// describing how to compute the byte offset of a particular element
/// or member of some global variable in the [`Storage`] address
/// space.
///
/// [`Writer::temp_access_chain`]: super::Writer::temp_access_chain
/// [`Storage`]: crate::AddressSpace::Storage
#[derive(Debug)]
pub(super) enum SubAccess {
    BufferOffset {
        group: u32,
        offset: u32,
    },

    /// Add the given byte offset. This is used for struct members, or
    /// known components of a vector or matrix. In all those cases,
    /// the byte offset is a compile-time constant.
    Offset(u32),

    /// Scale `value` by `stride`, and add that to the current byte
    /// offset. This is used to compute the offset of an array element
    /// whose index is computed at runtime.
    Index {
        value: Handle<crate::Expression>,
        stride: u32,
    },
}

pub(super) enum StoreValue {
    Expression(Handle<crate::Expression>),
    TempIndex {
        depth: usize,
        index: u32,
        ty: TypeResolution,
    },
    TempAccess {
        depth: usize,
        base: Handle<crate::Type>,
        member_index: u32,
    },
    // Access to a single column of a Cx2 matrix within a struct
    TempColumnAccess {
        depth: usize,
        base: Handle<crate::Type>,
        member_index: u32,
        column: u32,
    },
}

impl<W: fmt::Write> super::Writer<'_, W> {
    pub(super) fn write_storage_address(
        &mut self,
        module: &crate::Module,
        chain: &[SubAccess],
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        if chain.is_empty() {
            write!(self.out, "0")?;
        }
        for (i, access) in chain.iter().enumerate() {
            if i != 0 {
                write!(self.out, "+")?;
            }
            match *access {
                SubAccess::BufferOffset { group, offset } => {
                    write!(self.out, "__dynamic_buffer_offsets{group}._{offset}")?;
                }
                SubAccess::Offset(offset) => {
                    write!(self.out, "{offset}")?;
                }
                SubAccess::Index { value, stride } => {
                    self.write_expr(module, value, func_ctx)?;
                    write!(self.out, "*{stride}")?;
                }
            }
        }
        Ok(())
    }

    fn write_storage_load_sequence<I: Iterator<Item = (TypeResolution, u32)>>(
        &mut self,
        module: &crate::Module,
        var_handle: Handle<crate::GlobalVariable>,
        sequence: I,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        for (i, (ty_resolution, offset)) in sequence.enumerate() {
            // add the index temporarily
            self.temp_access_chain.push(SubAccess::Offset(offset));
            if i != 0 {
                write!(self.out, ", ")?;
            };
            self.write_storage_load(module, var_handle, ty_resolution, func_ctx)?;
            self.temp_access_chain.pop();
        }
        Ok(())
    }

    /// Emit code to access a [`Storage`] global's component.
    ///
    /// Emit HLSL to access the component of `var_handle`, a global
    /// variable in the [`Storage`] address space, whose type is
    /// `result_ty` and whose location within the global is given by
    /// [`self.temp_access_chain`]. See the [`storage`] module's
    /// documentation for background.
    ///
    /// [`Storage`]: crate::AddressSpace::Storage
    /// [`self.temp_access_chain`]: super::Writer::temp_access_chain
    pub(super) fn write_storage_load(
        &mut self,
        module: &crate::Module,
        var_handle: Handle<crate::GlobalVariable>,
        result_ty: TypeResolution,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        match *result_ty.inner_with(&module.types) {
            crate::TypeInner::Scalar(scalar) => {
                // working around the borrow checker in `self.write_expr`
                let chain = mem::take(&mut self.temp_access_chain);
                let var_name = &self.names[&NameKey::GlobalVariable(var_handle)];
                // See note about DXC and Load/Store in the module's documentation.
                if scalar.width == 4 {
                    let cast = scalar.kind.to_hlsl_cast();
                    write!(self.out, "{cast}({var_name}.Load(")?;
                } else {
                    let ty = scalar.to_hlsl_str()?;
                    write!(self.out, "{var_name}.Load<{ty}>(")?;
                };
                self.write_storage_address(module, &chain, func_ctx)?;
                write!(self.out, ")")?;
                if scalar.width == 4 {
                    write!(self.out, ")")?;
                }
                self.temp_access_chain = chain;
            }
            crate::TypeInner::Vector { size, scalar } => {
                // working around the borrow checker in `self.write_expr`
                let chain = mem::take(&mut self.temp_access_chain);
                let var_name = &self.names[&NameKey::GlobalVariable(var_handle)];
                let size = size as u8;
                // See note about DXC and Load/Store in the module's documentation.
                if scalar.width == 4 {
                    let cast = scalar.kind.to_hlsl_cast();
                    write!(self.out, "{cast}({var_name}.Load{size}(")?;
                } else {
                    let ty = scalar.to_hlsl_str()?;
                    write!(self.out, "{var_name}.Load<{ty}{size}>(")?;
                };
                self.write_storage_address(module, &chain, func_ctx)?;
                write!(self.out, ")")?;
                if scalar.width == 4 {
                    write!(self.out, ")")?;
                }
                self.temp_access_chain = chain;
            }
            crate::TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => {
                write!(
                    self.out,
                    "{}{}x{}(",
                    scalar.to_hlsl_str()?,
                    columns as u8,
                    rows as u8,
                )?;

                // Note: Matrices containing vec3s, due to padding, act like they contain vec4s.
                let row_stride = Alignment::from(rows) * scalar.width as u32;
                let iter = (0..columns as u32).map(|i| {
                    let ty_inner = crate::TypeInner::Vector { size: rows, scalar };
                    (TypeResolution::Value(ty_inner), i * row_stride)
                });
                self.write_storage_load_sequence(module, var_handle, iter, func_ctx)?;
                write!(self.out, ")")?;
            }
            crate::TypeInner::Array {
                base,
                size: crate::ArraySize::Constant(size),
                stride,
            } => {
                let constructor = super::help::WrappedConstructor {
                    ty: result_ty.handle().unwrap(),
                };
                self.write_wrapped_constructor_function_name(module, constructor)?;
                write!(self.out, "(")?;
                let iter = (0..size.get()).map(|i| (TypeResolution::Handle(base), stride * i));
                self.write_storage_load_sequence(module, var_handle, iter, func_ctx)?;
                write!(self.out, ")")?;
            }
            crate::TypeInner::Struct { ref members, .. } => {
                let constructor = super::help::WrappedConstructor {
                    ty: result_ty.handle().unwrap(),
                };
                self.write_wrapped_constructor_function_name(module, constructor)?;
                write!(self.out, "(")?;
                let iter = members
                    .iter()
                    .map(|m| (TypeResolution::Handle(m.ty), m.offset));
                self.write_storage_load_sequence(module, var_handle, iter, func_ctx)?;
                write!(self.out, ")")?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn write_store_value(
        &mut self,
        module: &crate::Module,
        value: &StoreValue,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        match *value {
            StoreValue::Expression(expr) => self.write_expr(module, expr, func_ctx)?,
            StoreValue::TempIndex {
                depth,
                index,
                ty: _,
            } => write!(self.out, "{STORE_TEMP_NAME}{depth}[{index}]")?,
            StoreValue::TempAccess {
                depth,
                base,
                member_index,
            } => {
                let name = &self.names[&NameKey::StructMember(base, member_index)];
                write!(self.out, "{STORE_TEMP_NAME}{depth}.{name}")?
            }
            StoreValue::TempColumnAccess {
                depth,
                base,
                member_index,
                column,
            } => {
                let name = &self.names[&NameKey::StructMember(base, member_index)];
                write!(self.out, "{STORE_TEMP_NAME}{depth}.{name}_{column}")?
            }
        }
        Ok(())
    }

    /// Helper function to write down the Store operation on a `ByteAddressBuffer`.
    pub(super) fn write_storage_store(
        &mut self,
        module: &crate::Module,
        var_handle: Handle<crate::GlobalVariable>,
        value: StoreValue,
        func_ctx: &FunctionCtx,
        level: crate::back::Level,
        within_struct: Option<Handle<crate::Type>>,
    ) -> BackendResult {
        let temp_resolution;
        let ty_resolution = match value {
            StoreValue::Expression(expr) => &func_ctx.info[expr].ty,
            StoreValue::TempIndex {
                depth: _,
                index: _,
                ref ty,
            } => ty,
            StoreValue::TempAccess {
                depth: _,
                base,
                member_index,
            } => {
                let ty_handle = match module.types[base].inner {
                    crate::TypeInner::Struct { ref members, .. } => {
                        members[member_index as usize].ty
                    }
                    _ => unreachable!(),
                };
                temp_resolution = TypeResolution::Handle(ty_handle);
                &temp_resolution
            }
            StoreValue::TempColumnAccess { .. } => {
                unreachable!("attempting write_storage_store for TempColumnAccess");
            }
        };
        match *ty_resolution.inner_with(&module.types) {
            crate::TypeInner::Scalar(scalar) => {
                // working around the borrow checker in `self.write_expr`
                let chain = mem::take(&mut self.temp_access_chain);
                let var_name = &self.names[&NameKey::GlobalVariable(var_handle)];
                // See note about DXC and Load/Store in the module's documentation.
                if scalar.width == 4 {
                    write!(self.out, "{level}{var_name}.Store(")?;
                    self.write_storage_address(module, &chain, func_ctx)?;
                    write!(self.out, ", asuint(")?;
                    self.write_store_value(module, &value, func_ctx)?;
                    writeln!(self.out, "));")?;
                } else {
                    write!(self.out, "{level}{var_name}.Store(")?;
                    self.write_storage_address(module, &chain, func_ctx)?;
                    write!(self.out, ", ")?;
                    self.write_store_value(module, &value, func_ctx)?;
                    writeln!(self.out, ");")?;
                }
                self.temp_access_chain = chain;
            }
            crate::TypeInner::Vector { size, scalar } => {
                // working around the borrow checker in `self.write_expr`
                let chain = mem::take(&mut self.temp_access_chain);
                let var_name = &self.names[&NameKey::GlobalVariable(var_handle)];
                // See note about DXC and Load/Store in the module's documentation.
                if scalar.width == 4 {
                    write!(self.out, "{}{}.Store{}(", level, var_name, size as u8)?;
                    self.write_storage_address(module, &chain, func_ctx)?;
                    write!(self.out, ", asuint(")?;
                    self.write_store_value(module, &value, func_ctx)?;
                    writeln!(self.out, "));")?;
                } else {
                    write!(self.out, "{level}{var_name}.Store(")?;
                    self.write_storage_address(module, &chain, func_ctx)?;
                    write!(self.out, ", ")?;
                    self.write_store_value(module, &value, func_ctx)?;
                    writeln!(self.out, ");")?;
                }
                self.temp_access_chain = chain;
            }
            crate::TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => {
                // Note: Matrices containing vec3s, due to padding, act like they contain vec4s.
                let row_stride = Alignment::from(rows) * scalar.width as u32;

                writeln!(self.out, "{level}{{")?;

                match within_struct {
                    Some(containing_struct) if rows == crate::VectorSize::Bi => {
                        // If we are within a struct, then the struct was already assigned to
                        // a temporary, we don't need to make another.
                        let mut chain = mem::take(&mut self.temp_access_chain);
                        for i in 0..columns as u32 {
                            chain.push(SubAccess::Offset(i * row_stride));
                            // working around the borrow checker in `self.write_expr`
                            let var_name = &self.names[&NameKey::GlobalVariable(var_handle)];
                            let StoreValue::TempAccess { member_index, .. } = value else {
                                unreachable!(
                                    "write_storage_store within_struct but not TempAccess"
                                );
                            };
                            let column_value = StoreValue::TempColumnAccess {
                                depth: level.0, // note not incrementing, b/c no temp
                                base: containing_struct,
                                member_index,
                                column: i,
                            };
                            // See note about DXC and Load/Store in the module's documentation.
                            if scalar.width == 4 {
                                write!(
                                    self.out,
                                    "{}{}.Store{}(",
                                    level.next(),
                                    var_name,
                                    rows as u8
                                )?;
                                self.write_storage_address(module, &chain, func_ctx)?;
                                write!(self.out, ", asuint(")?;
                                self.write_store_value(module, &column_value, func_ctx)?;
                                writeln!(self.out, "));")?;
                            } else {
                                write!(self.out, "{}{var_name}.Store(", level.next())?;
                                self.write_storage_address(module, &chain, func_ctx)?;
                                write!(self.out, ", ")?;
                                self.write_store_value(module, &column_value, func_ctx)?;
                                writeln!(self.out, ");")?;
                            }
                            chain.pop();
                        }
                        self.temp_access_chain = chain;
                    }
                    _ => {
                        // first, assign the value to a temporary
                        let depth = level.0 + 1;
                        write!(
                            self.out,
                            "{}{}{}x{} {}{} = ",
                            level.next(),
                            scalar.to_hlsl_str()?,
                            columns as u8,
                            rows as u8,
                            STORE_TEMP_NAME,
                            depth,
                        )?;
                        self.write_store_value(module, &value, func_ctx)?;
                        writeln!(self.out, ";")?;

                        // then iterate the stores
                        for i in 0..columns as u32 {
                            self.temp_access_chain
                                .push(SubAccess::Offset(i * row_stride));
                            let ty_inner = crate::TypeInner::Vector { size: rows, scalar };
                            let sv = StoreValue::TempIndex {
                                depth,
                                index: i,
                                ty: TypeResolution::Value(ty_inner),
                            };
                            self.write_storage_store(
                                module,
                                var_handle,
                                sv,
                                func_ctx,
                                level.next(),
                                None,
                            )?;
                            self.temp_access_chain.pop();
                        }
                    }
                }
                // done
                writeln!(self.out, "{level}}}")?;
            }
            crate::TypeInner::Array {
                base,
                size: crate::ArraySize::Constant(size),
                stride,
            } => {
                // first, assign the value to a temporary
                writeln!(self.out, "{level}{{")?;
                write!(self.out, "{}", level.next())?;
                self.write_type(module, base)?;
                let depth = level.next().0;
                write!(self.out, " {STORE_TEMP_NAME}{depth}")?;
                self.write_array_size(module, base, crate::ArraySize::Constant(size))?;
                write!(self.out, " = ")?;
                self.write_store_value(module, &value, func_ctx)?;
                writeln!(self.out, ";")?;
                // then iterate the stores
                for i in 0..size.get() {
                    self.temp_access_chain.push(SubAccess::Offset(i * stride));
                    let sv = StoreValue::TempIndex {
                        depth,
                        index: i,
                        ty: TypeResolution::Handle(base),
                    };
                    self.write_storage_store(module, var_handle, sv, func_ctx, level.next(), None)?;
                    self.temp_access_chain.pop();
                }
                // done
                writeln!(self.out, "{level}}}")?;
            }
            crate::TypeInner::Struct { ref members, .. } => {
                // first, assign the value to a temporary
                writeln!(self.out, "{level}{{")?;
                let depth = level.next().0;
                let struct_ty = ty_resolution.handle().unwrap();
                let struct_name = &self.names[&NameKey::Type(struct_ty)];
                write!(
                    self.out,
                    "{}{} {}{} = ",
                    level.next(),
                    struct_name,
                    STORE_TEMP_NAME,
                    depth
                )?;
                self.write_store_value(module, &value, func_ctx)?;
                writeln!(self.out, ";")?;
                // then iterate the stores
                for (i, member) in members.iter().enumerate() {
                    self.temp_access_chain
                        .push(SubAccess::Offset(member.offset));
                    let sv = StoreValue::TempAccess {
                        depth,
                        base: struct_ty,
                        member_index: i as u32,
                    };
                    self.write_storage_store(
                        module,
                        var_handle,
                        sv,
                        func_ctx,
                        level.next(),
                        Some(struct_ty),
                    )?;
                    self.temp_access_chain.pop();
                }
                // done
                writeln!(self.out, "{level}}}")?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    /// Set [`temp_access_chain`] to compute the byte offset of `cur_expr`.
    ///
    /// The `cur_expr` expression must be a reference to a global
    /// variable in the [`Storage`] address space, or a chain of
    /// [`Access`] and [`AccessIndex`] expressions referring to some
    /// component of such a global.
    ///
    /// [`temp_access_chain`]: super::Writer::temp_access_chain
    /// [`Storage`]: crate::AddressSpace::Storage
    /// [`Access`]: crate::Expression::Access
    /// [`AccessIndex`]: crate::Expression::AccessIndex
    pub(super) fn fill_access_chain(
        &mut self,
        module: &crate::Module,
        mut cur_expr: Handle<crate::Expression>,
        func_ctx: &FunctionCtx,
    ) -> Result<Handle<crate::GlobalVariable>, Error> {
        enum AccessIndex {
            Expression(Handle<crate::Expression>),
            Constant(u32),
        }
        enum Parent<'a> {
            Array { stride: u32 },
            Struct(&'a [crate::StructMember]),
        }
        self.temp_access_chain.clear();

        loop {
            let (next_expr, access_index) = match func_ctx.expressions[cur_expr] {
                crate::Expression::GlobalVariable(handle) => {
                    if let Some(ref binding) = module.global_variables[handle].binding {
                        // this was already resolved earlier when we started evaluating an entry point.
                        let bt = self.options.resolve_resource_binding(binding).unwrap();
                        if let Some(dynamic_storage_buffer_offsets_index) =
                            bt.dynamic_storage_buffer_offsets_index
                        {
                            self.temp_access_chain.push(SubAccess::BufferOffset {
                                group: binding.group,
                                offset: dynamic_storage_buffer_offsets_index,
                            });
                        }
                    }
                    return Ok(handle);
                }
                crate::Expression::Access { base, index } => (base, AccessIndex::Expression(index)),
                crate::Expression::AccessIndex { base, index } => {
                    (base, AccessIndex::Constant(index))
                }
                ref other => {
                    return Err(Error::Unimplemented(format!("Pointer access of {other:?}")))
                }
            };

            let parent = match *func_ctx.resolve_type(next_expr, &module.types) {
                crate::TypeInner::Pointer { base, .. } => match module.types[base].inner {
                    crate::TypeInner::Struct { ref members, .. } => Parent::Struct(members),
                    crate::TypeInner::Array { stride, .. } => Parent::Array { stride },
                    crate::TypeInner::Vector { scalar, .. } => Parent::Array {
                        stride: scalar.width as u32,
                    },
                    crate::TypeInner::Matrix { rows, scalar, .. } => Parent::Array {
                        // The stride between matrices is the count of rows as this is how
                        // long each column is.
                        stride: Alignment::from(rows) * scalar.width as u32,
                    },
                    _ => unreachable!(),
                },
                crate::TypeInner::ValuePointer { scalar, .. } => Parent::Array {
                    stride: scalar.width as u32,
                },
                _ => unreachable!(),
            };

            let sub = match (parent, access_index) {
                (Parent::Array { stride }, AccessIndex::Expression(value)) => {
                    SubAccess::Index { value, stride }
                }
                (Parent::Array { stride }, AccessIndex::Constant(index)) => {
                    SubAccess::Offset(stride * index)
                }
                (Parent::Struct(members), AccessIndex::Constant(index)) => {
                    SubAccess::Offset(members[index as usize].offset)
                }
                (Parent::Struct(_), AccessIndex::Expression(_)) => unreachable!(),
            };

            self.temp_access_chain.push(sub);
            cur_expr = next_expr;
        }
    }
}
