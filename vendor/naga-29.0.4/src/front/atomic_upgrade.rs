//! Upgrade the types of scalars observed to be accessed as atomics to [`Atomic`] types.
//!
//! In SPIR-V, atomic operations can be applied to any scalar value, but in Naga
//! IR atomic operations can only be applied to values of type [`Atomic`]. Naga
//! IR's restriction matches Metal Shading Language and WGSL, so we don't want
//! to relax that. Instead, when the SPIR-V front end observes a value being
//! accessed using atomic instructions, it promotes the value's type from
//! [`Scalar`] to [`Atomic`]. This module implements `Module::upgrade_atomics`,
//! the function that makes that change.
//!
//! Atomics can only appear in global variables in the [`Storage`] and
//! [`Workgroup`] address spaces. These variables can either have `Atomic` types
//! themselves, or be [`Array`]s of such, or be [`Struct`]s containing such.
//! So we only need to change the types of globals and struct fields.
//!
//! Naga IR [`Load`] expressions and [`Store`] statements can operate directly
//! on [`Atomic`] values, retrieving and depositing ordinary [`Scalar`] values,
//! so changing the types doesn't have much effect on the code that operates on
//! those values.
//!
//! Future work:
//!
//! - The GLSL front end could use this transformation as well.
//!
//! [`Atomic`]: TypeInner::Atomic
//! [`Scalar`]: TypeInner::Scalar
//! [`Storage`]: crate::AddressSpace::Storage
//! [`WorkGroup`]: crate::AddressSpace::WorkGroup
//! [`Array`]: TypeInner::Array
//! [`Struct`]: TypeInner::Struct
//! [`Load`]: crate::Expression::Load
//! [`Store`]: crate::Statement::Store

use alloc::{format, sync::Arc};
use core::sync::atomic::AtomicUsize;

use crate::{GlobalVariable, Handle, Module, Type, TypeInner};

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("encountered an unsupported expression")]
    Unsupported,
    #[error("unexpected end of struct field access indices")]
    UnexpectedEndOfIndices,
    #[error("encountered unsupported global initializer in an atomic variable")]
    GlobalInitUnsupported,
    #[error("expected to find a global variable")]
    GlobalVariableMissing,
    #[error("atomic compare exchange requires a scalar base type")]
    CompareExchangeNonScalarBaseType,
}

#[derive(Clone, Default)]
struct Padding(Arc<AtomicUsize>);

impl core::fmt::Display for Padding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for _ in 0..self.0.load(core::sync::atomic::Ordering::Relaxed) {
            f.write_str("  ")?;
        }
        Ok(())
    }
}

impl Drop for Padding {
    fn drop(&mut self) {
        let _ = self.0.fetch_sub(1, core::sync::atomic::Ordering::Relaxed);
    }
}

impl Padding {
    fn trace(&self, msg: impl core::fmt::Display, t: impl core::fmt::Debug) {
        format!("{msg} {t:#?}")
            .split('\n')
            .for_each(|ln| log::trace!("{self}{ln}"));
    }

    fn debug(&self, msg: impl core::fmt::Display, t: impl core::fmt::Debug) {
        format!("{msg} {t:#?}")
            .split('\n')
            .for_each(|ln| log::debug!("{self}{ln}"));
    }

    fn inc_padding(&self) -> Padding {
        let _ = self.0.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        self.clone()
    }
}

#[derive(Debug, Default)]
pub struct Upgrades {
    /// Global variables that we've accessed using atomic operations.
    ///
    /// This includes globals with composite types (arrays, structs) where we've
    /// only accessed some components (elements, fields) atomically.
    globals: crate::arena::HandleSet<GlobalVariable>,

    /// Struct fields that we've accessed using atomic operations.
    ///
    /// Each key refers to some [`Struct`] type, and each value is a set of
    /// the indices of the fields in that struct that have been accessed
    /// atomically.
    ///
    /// This includes fields with composite types (arrays, structs)
    /// of which we've only accessed some components (elements, fields)
    /// atomically.
    ///
    /// [`Struct`]: crate::TypeInner::Struct
    fields: crate::FastHashMap<Handle<Type>, bit_set::BitSet>,
}

impl Upgrades {
    pub fn insert_global(&mut self, global: Handle<GlobalVariable>) {
        self.globals.insert(global);
    }

    pub fn insert_field(&mut self, struct_type: Handle<Type>, field: usize) {
        self.fields.entry(struct_type).or_default().insert(field);
    }

    pub fn is_empty(&self) -> bool {
        self.globals.is_empty()
    }
}

struct UpgradeState<'a> {
    padding: Padding,
    module: &'a mut Module,

    /// A map from old types to their upgraded versions.
    ///
    /// This ensures we never try to rebuild a type more than once.
    upgraded_types: crate::FastHashMap<Handle<Type>, Handle<Type>>,
}

impl UpgradeState<'_> {
    fn inc_padding(&self) -> Padding {
        self.padding.inc_padding()
    }

    /// Get a type equivalent to `ty`, but with [`Scalar`] leaves upgraded to [`Atomic`] scalars.
    ///
    /// If such a type already exists in `self.module.types`, return its handle.
    /// Otherwise, construct a new one and return that handle.
    ///
    /// If `ty` is a [`Pointer`], [`Array`], [`BindingArray`], recurse into the
    /// type and upgrade its leaf types.
    ///
    /// If `ty` is a [`Struct`], recurse into it and upgrade only those fields
    /// whose indices appear in `field_indices`.
    ///
    /// The existing type is not affected.
    ///
    /// [`Scalar`]: crate::TypeInner::Scalar
    /// [`Atomic`]: crate::TypeInner::Atomic
    /// [`Pointer`]: crate::TypeInner::Pointer
    /// [`Array`]: crate::TypeInner::Array
    /// [`Struct`]: crate::TypeInner::Struct
    /// [`BindingArray`]: crate::TypeInner::BindingArray
    fn upgrade_type(
        &mut self,
        ty: Handle<Type>,
        upgrades: &Upgrades,
    ) -> Result<Handle<Type>, Error> {
        let padding = self.inc_padding();
        padding.trace("visiting type: ", ty);

        // If we've already upgraded this type, return the handle we produced at
        // the time.
        if let Some(&new) = self.upgraded_types.get(&ty) {
            return Ok(new);
        }

        let inner = match self.module.types[ty].inner {
            TypeInner::Scalar(scalar) => {
                log::trace!("{padding}hit the scalar leaf, replacing with an atomic");
                TypeInner::Atomic(scalar)
            }
            TypeInner::Pointer { base, space } => TypeInner::Pointer {
                base: self.upgrade_type(base, upgrades)?,
                space,
            },
            TypeInner::Array { base, size, stride } => TypeInner::Array {
                base: self.upgrade_type(base, upgrades)?,
                size,
                stride,
            },
            TypeInner::Struct { ref members, span } => {
                // If no field or subfield of this struct was ever accessed
                // atomically, no change is needed. We should never have arrived here.
                let Some(fields) = upgrades.fields.get(&ty) else {
                    unreachable!("global or field incorrectly flagged as atomically accessed");
                };

                let mut new_members = members.clone();
                for field in fields {
                    new_members[field].ty = self.upgrade_type(new_members[field].ty, upgrades)?;
                }

                TypeInner::Struct {
                    members: new_members,
                    span,
                }
            }
            TypeInner::BindingArray { base, size } => TypeInner::BindingArray {
                base: self.upgrade_type(base, upgrades)?,
                size,
            },
            _ => return Ok(ty),
        };

        // At this point, we have a `TypeInner` that is the upgraded version of
        // `ty`. Find a suitable `Type` for this, creating a new one if
        // necessary, and return its handle.
        let r#type = &self.module.types[ty];
        let span = self.module.types.get_span(ty);
        let new_type = Type {
            name: r#type.name.clone(),
            inner,
        };
        padding.debug("ty: ", ty);
        padding.debug("from: ", r#type);
        padding.debug("to:   ", &new_type);
        let new_handle = self.module.types.insert(new_type, span);
        self.upgraded_types.insert(ty, new_handle);
        Ok(new_handle)
    }

    fn upgrade_all(&mut self, upgrades: &Upgrades) -> Result<(), Error> {
        for handle in upgrades.globals.iter() {
            let padding = self.inc_padding();

            let global = &self.module.global_variables[handle];
            padding.trace("visiting global variable: ", handle);
            padding.trace("var: ", global);

            if global.init.is_some() {
                return Err(Error::GlobalInitUnsupported);
            }

            let var_ty = global.ty;
            let new_ty = self.upgrade_type(var_ty, upgrades)?;
            if new_ty != var_ty {
                padding.debug("upgrading global variable: ", handle);
                padding.debug("from ty: ", var_ty);
                padding.debug("to ty:   ", new_ty);
                self.module.global_variables[handle].ty = new_ty;
            }
        }

        Ok(())
    }
}

impl Module {
    /// Upgrade `global_var_handles` to have [`Atomic`] leaf types.
    ///
    /// [`Atomic`]: TypeInner::Atomic
    pub(crate) fn upgrade_atomics(&mut self, upgrades: &Upgrades) -> Result<(), Error> {
        let mut state = UpgradeState {
            padding: Default::default(),
            module: self,
            upgraded_types: crate::FastHashMap::with_capacity_and_hasher(
                upgrades.fields.len(),
                Default::default(),
            ),
        };

        state.upgrade_all(upgrades)?;

        Ok(())
    }
}
