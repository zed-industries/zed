use std::collections::HashMap;

use crate::cdsl::typevar::TypeVar;

/// An instruction operand can be an *immediate*, an *SSA value*, or an *entity reference*. The
/// type of the operand is one of:
///
/// 1. A `ValueType` instance indicates an SSA value operand with a concrete type.
///
/// 2. A `TypeVar` instance indicates an SSA value operand, and the instruction is polymorphic over
///    the possible concrete types that the type variable can assume.
///
/// 3. An `ImmediateKind` instance indicates an immediate operand whose value is encoded in the
///    instruction itself rather than being passed as an SSA value.
///
/// 4. An `EntityRefKind` instance indicates an operand that references another entity in the
///    function, typically something declared in the function preamble.
#[derive(Clone, Debug)]
pub(crate) struct Operand {
    /// Name of the operand variable, as it appears in function parameters, legalizations, etc.
    pub name: &'static str,

    /// Type of the operand.
    pub kind: OperandKind,

    doc: Option<&'static str>,
}

impl Operand {
    pub fn new(name: &'static str, kind: impl Into<OperandKind>) -> Self {
        Self {
            name,
            doc: None,
            kind: kind.into(),
        }
    }
    pub fn with_doc(mut self, doc: &'static str) -> Self {
        self.doc = Some(doc);
        self
    }

    pub fn doc(&self) -> &str {
        if let Some(doc) = &self.doc {
            return doc;
        }
        match &self.kind.fields {
            OperandKindFields::TypeVar(tvar) => &tvar.doc,
            _ => self.kind.doc(),
        }
    }

    pub fn is_value(&self) -> bool {
        matches!(self.kind.fields, OperandKindFields::TypeVar(_))
    }

    pub fn type_var(&self) -> Option<&TypeVar> {
        match &self.kind.fields {
            OperandKindFields::TypeVar(typevar) => Some(typevar),
            _ => None,
        }
    }

    pub fn is_varargs(&self) -> bool {
        matches!(self.kind.fields, OperandKindFields::VariableArgs)
    }

    /// Returns true if the operand has an immediate kind or is an EntityRef.
    pub fn is_immediate_or_entityref(&self) -> bool {
        matches!(
            self.kind.fields,
            OperandKindFields::ImmEnum(_)
                | OperandKindFields::ImmValue
                | OperandKindFields::EntityRef
        )
    }

    /// Returns true if the operand has an immediate kind.
    pub fn is_immediate(&self) -> bool {
        matches!(
            self.kind.fields,
            OperandKindFields::ImmEnum(_) | OperandKindFields::ImmValue
        )
    }
}

pub type EnumValues = HashMap<&'static str, &'static str>;

#[derive(Clone, Debug)]
pub(crate) enum OperandKindFields {
    EntityRef,
    VariableArgs,
    ImmValue,
    ImmEnum(EnumValues),
    TypeVar(TypeVar),
}

impl OperandKindFields {
    /// Return the [EnumValues] for this field if it is an `enum`.
    pub(crate) fn enum_values(&self) -> Option<&EnumValues> {
        match self {
            OperandKindFields::ImmEnum(map) => Some(map),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct OperandKind {
    /// String representation of the Rust type mapping to this OperandKind.
    pub rust_type: &'static str,

    /// Name of this OperandKind in the format's member field.
    pub rust_field_name: &'static str,

    /// Type-specific fields for this OperandKind.
    pub fields: OperandKindFields,

    doc: Option<&'static str>,
}

impl OperandKind {
    pub fn new(
        rust_field_name: &'static str,
        rust_type: &'static str,
        fields: OperandKindFields,
        doc: &'static str,
    ) -> Self {
        Self {
            rust_field_name,
            rust_type,
            fields,
            doc: Some(doc),
        }
    }
    fn doc(&self) -> &str {
        if let Some(doc) = &self.doc {
            return doc;
        }
        match &self.fields {
            OperandKindFields::TypeVar(type_var) => &type_var.doc,
            // The only method to create an OperandKind with `doc` set to None is using a TypeVar,
            // so all other options are unreachable here.
            OperandKindFields::ImmEnum(_)
            | OperandKindFields::ImmValue
            | OperandKindFields::EntityRef
            | OperandKindFields::VariableArgs => unreachable!(),
        }
    }

    pub(crate) fn is_block(&self) -> bool {
        self.rust_type == "ir::BlockCall"
    }
}

impl From<&TypeVar> for OperandKind {
    fn from(type_var: &TypeVar) -> Self {
        OperandKind {
            rust_field_name: "value",
            rust_type: "ir::Value",
            fields: OperandKindFields::TypeVar(type_var.into()),
            doc: None,
        }
    }
}
impl From<&OperandKind> for OperandKind {
    fn from(kind: &OperandKind) -> Self {
        kind.clone()
    }
}
