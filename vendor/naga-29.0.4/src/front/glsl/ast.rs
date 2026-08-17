use alloc::{borrow::Cow, string::String, vec::Vec};
use core::fmt;

use super::{builtins::MacroCall, Span};
use crate::{
    AddressSpace, BinaryOperator, Binding, Constant, Expression, Function, GlobalVariable, Handle,
    Interpolation, Literal, Override, Sampling, StorageAccess, Type, UnaryOperator,
};

#[derive(Debug, Clone, Copy)]
pub enum GlobalLookupKind {
    Variable(Handle<GlobalVariable>),
    Constant(Handle<Constant>, Handle<Type>),
    Override(Handle<Override>, Handle<Type>),
    BlockSelect(Handle<GlobalVariable>, u32),
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalLookup {
    pub kind: GlobalLookupKind,
    pub entry_arg: Option<usize>,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub qualifier: ParameterQualifier,
    /// Whether the parameter should be treated as a depth image instead of a
    /// sampled image.
    pub depth: bool,
}

/// How the function is implemented
#[derive(Clone, Copy)]
pub enum FunctionKind {
    /// The function is user defined
    Call(Handle<Function>),
    /// The function is a builtin
    Macro(MacroCall),
}

impl fmt::Debug for FunctionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Call(_) => write!(f, "Call"),
            Self::Macro(_) => write!(f, "Macro"),
        }
    }
}

#[derive(Debug)]
pub struct Overload {
    /// Normalized function parameters, modifiers are not applied
    pub parameters: Vec<Handle<Type>>,
    pub parameters_info: Vec<ParameterInfo>,
    /// How the function is implemented
    pub kind: FunctionKind,
    /// Whether this function was already defined or is just a prototype
    pub defined: bool,
    /// Whether this overload is the one provided by the language or has
    /// been redeclared by the user (builtins only)
    pub internal: bool,
    /// Whether or not this function returns void (nothing)
    pub void: bool,
}

bitflags::bitflags! {
    /// Tracks the variations of the builtin already generated, this is needed because some
    /// builtins overloads can't be generated unless explicitly used, since they might cause
    /// unneeded capabilities to be requested
    #[derive(Default)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct BuiltinVariations: u32 {
        /// Request the standard overloads
        const STANDARD = 1 << 0;
        /// Request overloads that use the double type
        const DOUBLE = 1 << 1;
        /// Request overloads that use `samplerCubeArray(Shadow)`
        const CUBE_TEXTURES_ARRAY = 1 << 2;
        /// Request overloads that use `sampler2DMSArray`
        const D2_MULTI_TEXTURES_ARRAY = 1 << 3;
    }
}

#[derive(Debug, Default)]
pub struct FunctionDeclaration {
    pub overloads: Vec<Overload>,
    /// Tracks the builtin overload variations that were already generated
    pub variations: BuiltinVariations,
}

#[derive(Debug)]
pub struct EntryArg {
    pub name: Option<String>,
    pub binding: Binding,
    pub handle: Handle<GlobalVariable>,
    pub storage: StorageQualifier,
}

#[derive(Debug, Clone)]
pub struct VariableReference {
    pub expr: Handle<Expression>,
    /// Whether the variable is of a pointer type (and needs loading) or not
    pub load: bool,
    /// Whether the value of the variable can be changed or not
    pub mutable: bool,
    pub constant: Option<(Handle<Constant>, Handle<Type>)>,
    pub entry_arg: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct HirExpr {
    pub kind: HirExprKind,
    pub meta: Span,
}

#[derive(Debug, Clone)]
pub enum HirExprKind {
    /// Represents a sequence of expressions. It returns the type and value of the last (i.e. right-most) expression.
    Sequence {
        exprs: Vec<Handle<HirExpr>>,
    },
    Access {
        base: Handle<HirExpr>,
        index: Handle<HirExpr>,
    },
    Select {
        base: Handle<HirExpr>,
        field: String,
    },
    Literal(Literal),
    Binary {
        left: Handle<HirExpr>,
        op: BinaryOperator,
        right: Handle<HirExpr>,
    },
    Unary {
        op: UnaryOperator,
        expr: Handle<HirExpr>,
    },
    Variable(VariableReference),
    Call(FunctionCall),
    /// Represents the ternary operator in glsl (`:?`)
    Conditional {
        /// The expression that will decide which branch to take, must evaluate to a boolean
        condition: Handle<HirExpr>,
        /// The expression that will be evaluated if [`condition`] returns `true`
        ///
        /// [`condition`]: Self::Conditional::condition
        accept: Handle<HirExpr>,
        /// The expression that will be evaluated if [`condition`] returns `false`
        ///
        /// [`condition`]: Self::Conditional::condition
        reject: Handle<HirExpr>,
    },
    Assign {
        tgt: Handle<HirExpr>,
        value: Handle<HirExpr>,
    },
    /// A prefix/postfix operator like `++`
    PrePostfix {
        /// The operation to be performed
        op: BinaryOperator,
        /// Whether this is a postfix or a prefix
        postfix: bool,
        /// The target expression
        expr: Handle<HirExpr>,
    },
    /// A method call like `what.something(a, b, c)`
    Method {
        /// expression the method call applies to (`what` in the example)
        expr: Handle<HirExpr>,
        /// the method name (`something` in the example)
        name: String,
        /// the arguments to the method (`a`, `b`, and `c` in the example)
        args: Vec<Handle<HirExpr>>,
    },
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum QualifierKey<'a> {
    String(Cow<'a, str>),
    /// Used for `std140` and `std430` layout qualifiers
    Layout,
    /// Used for image formats
    Format,
    /// Used for `index` layout qualifiers
    Index,
}

#[derive(Debug)]
pub enum QualifierValue {
    None,
    Uint(u32),
    Layout(StructLayout),
    Format(crate::StorageFormat),
}

#[derive(Debug, Default)]
pub struct TypeQualifiers<'a> {
    pub span: Span,
    pub storage: (StorageQualifier, Span),
    pub invariant: Option<Span>,
    pub interpolation: Option<(Interpolation, Span)>,
    pub precision: Option<(Precision, Span)>,
    pub sampling: Option<(Sampling, Span)>,
    /// Memory qualifiers used in the declaration to set the storage access to be used
    /// in declarations that support it (storage images and buffers)
    pub storage_access: Option<(StorageAccess, Span)>,
    pub layout_qualifiers: crate::FastHashMap<QualifierKey<'a>, (QualifierValue, Span)>,
}

impl<'a> TypeQualifiers<'a> {
    /// Appends `errors` with errors for all unused qualifiers
    pub fn unused_errors(&self, errors: &mut Vec<super::Error>) {
        if let Some(meta) = self.invariant {
            errors.push(super::Error {
                kind: super::ErrorKind::SemanticError(
                    "Invariant qualifier can only be used in in/out variables".into(),
                ),
                meta,
            });
        }

        if let Some((_, meta)) = self.interpolation {
            errors.push(super::Error {
                kind: super::ErrorKind::SemanticError(
                    "Interpolation qualifiers can only be used in in/out variables".into(),
                ),
                meta,
            });
        }

        if let Some((_, meta)) = self.sampling {
            errors.push(super::Error {
                kind: super::ErrorKind::SemanticError(
                    "Sampling qualifiers can only be used in in/out variables".into(),
                ),
                meta,
            });
        }

        if let Some((_, meta)) = self.storage_access {
            errors.push(super::Error {
                kind: super::ErrorKind::SemanticError(
                    "Memory qualifiers can only be used in storage variables".into(),
                ),
                meta,
            });
        }

        for &(_, meta) in self.layout_qualifiers.values() {
            errors.push(super::Error {
                kind: super::ErrorKind::SemanticError("Unexpected qualifier".into()),
                meta,
            });
        }
    }

    /// Removes the layout qualifier with `name`, if it exists and adds an error if it isn't
    /// a [`QualifierValue::Uint`]
    pub fn uint_layout_qualifier(
        &mut self,
        name: &'a str,
        errors: &mut Vec<super::Error>,
    ) -> Option<u32> {
        match self
            .layout_qualifiers
            .remove(&QualifierKey::String(name.into()))
        {
            Some((QualifierValue::Uint(v), _)) => Some(v),
            Some((_, meta)) => {
                errors.push(super::Error {
                    kind: super::ErrorKind::SemanticError("Qualifier expects a uint value".into()),
                    meta,
                });
                // Return a dummy value instead of `None` to differentiate from
                // the qualifier not existing, since some parts might require the
                // qualifier to exist and throwing another error that it doesn't
                // exist would be unhelpful
                Some(0)
            }
            _ => None,
        }
    }

    /// Removes the layout qualifier with `name`, if it exists and adds an error if it isn't
    /// a [`QualifierValue::None`]
    pub fn none_layout_qualifier(&mut self, name: &'a str, errors: &mut Vec<super::Error>) -> bool {
        match self
            .layout_qualifiers
            .remove(&QualifierKey::String(name.into()))
        {
            Some((QualifierValue::None, _)) => true,
            Some((_, meta)) => {
                errors.push(super::Error {
                    kind: super::ErrorKind::SemanticError(
                        "Qualifier doesn't expect a value".into(),
                    ),
                    meta,
                });
                // Return a `true` to since the qualifier is defined and adding
                // another error for it not being defined would be unhelpful
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FunctionCallKind {
    TypeConstructor(Handle<Type>),
    Function(String),
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub kind: FunctionCallKind,
    pub args: Vec<Handle<HirExpr>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageQualifier {
    AddressSpace(AddressSpace),
    Input,
    Output,
    Const,
}

impl Default for StorageQualifier {
    fn default() -> Self {
        StorageQualifier::AddressSpace(AddressSpace::Function)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructLayout {
    Std140,
    Std430,
}

// TODO: Encode precision hints in the IR
/// A precision hint used in GLSL declarations.
///
/// Precision hints can be used to either speed up shader execution or control
/// the precision of arithmetic operations.
///
/// To use a precision hint simply add it before the type in the declaration.
/// ```glsl
/// mediump float a;
/// ```
///
/// The default when no precision is declared is `highp` which means that all
/// operations operate with the type defined width.
///
/// For `mediump` and `lowp` operations follow the spir-v
/// [`RelaxedPrecision`][RelaxedPrecision] decoration semantics.
///
/// [RelaxedPrecision]: https://www.khronos.org/registry/SPIR-V/specs/unified1/SPIRV.html#_a_id_relaxedprecisionsection_a_relaxed_precision
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Precision {
    /// `lowp` precision
    Low,
    /// `mediump` precision
    Medium,
    /// `highp` precision
    High,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ParameterQualifier {
    In,
    Out,
    InOut,
    Const,
}

impl ParameterQualifier {
    /// Returns true if the argument should be passed as a lhs expression
    pub const fn is_lhs(&self) -> bool {
        match *self {
            ParameterQualifier::Out | ParameterQualifier::InOut => true,
            _ => false,
        }
    }
}

/// The GLSL profile used by a shader.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Profile {
    /// The `core` profile, default when no profile is specified.
    Core,
}
