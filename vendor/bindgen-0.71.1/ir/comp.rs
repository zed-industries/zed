//! Compound types (unions and structs) in our intermediate representation.

use itertools::Itertools;

use super::analysis::Sizedness;
use super::annotations::Annotations;
use super::context::{BindgenContext, FunctionId, ItemId, TypeId, VarId};
use super::dot::DotAttributes;
use super::item::{IsOpaque, Item};
use super::layout::Layout;
use super::template::TemplateParameters;
use super::traversal::{EdgeKind, Trace, Tracer};
use super::ty::RUST_DERIVE_IN_ARRAY_LIMIT;
use crate::clang;
use crate::codegen::struct_layout::{align_to, bytes_from_bits_pow2};
use crate::ir::derive::CanDeriveCopy;
use crate::parse::ParseError;
use crate::HashMap;
use crate::NonCopyUnionStyle;
use std::cmp;
use std::io;
use std::mem;

/// The kind of compound type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum CompKind {
    /// A struct.
    Struct,
    /// A union.
    Union,
}

/// The kind of C++ method.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum MethodKind {
    /// A constructor. We represent it as method for convenience, to avoid code
    /// duplication.
    Constructor,
    /// A destructor.
    Destructor,
    /// A virtual destructor.
    VirtualDestructor {
        /// Whether it's pure virtual.
        pure_virtual: bool,
    },
    /// A static method.
    Static,
    /// A normal method.
    Normal,
    /// A virtual method.
    Virtual {
        /// Whether it's pure virtual.
        pure_virtual: bool,
    },
}

impl MethodKind {
    /// Is this a destructor method?
    pub(crate) fn is_destructor(&self) -> bool {
        matches!(
            *self,
            MethodKind::Destructor | MethodKind::VirtualDestructor { .. }
        )
    }

    /// Is this a pure virtual method?
    pub(crate) fn is_pure_virtual(&self) -> bool {
        match *self {
            MethodKind::Virtual { pure_virtual } |
            MethodKind::VirtualDestructor { pure_virtual } => pure_virtual,
            _ => false,
        }
    }
}

/// A struct representing a C++ method, either static, normal, or virtual.
#[derive(Debug)]
pub(crate) struct Method {
    kind: MethodKind,
    /// The signature of the method. Take into account this is not a `Type`
    /// item, but a `Function` one.
    ///
    /// This is tricky and probably this field should be renamed.
    signature: FunctionId,
    is_const: bool,
}

impl Method {
    /// Construct a new `Method`.
    pub(crate) fn new(
        kind: MethodKind,
        signature: FunctionId,
        is_const: bool,
    ) -> Self {
        Method {
            kind,
            signature,
            is_const,
        }
    }

    /// What kind of method is this?
    pub(crate) fn kind(&self) -> MethodKind {
        self.kind
    }

    /// Is this a constructor?
    pub(crate) fn is_constructor(&self) -> bool {
        self.kind == MethodKind::Constructor
    }

    /// Is this a virtual method?
    pub(crate) fn is_virtual(&self) -> bool {
        matches!(
            self.kind,
            MethodKind::Virtual { .. } | MethodKind::VirtualDestructor { .. }
        )
    }

    /// Is this a static method?
    pub(crate) fn is_static(&self) -> bool {
        self.kind == MethodKind::Static
    }

    /// Get the ID for the `Function` signature for this method.
    pub(crate) fn signature(&self) -> FunctionId {
        self.signature
    }

    /// Is this a const qualified method?
    pub(crate) fn is_const(&self) -> bool {
        self.is_const
    }
}

/// Methods common to the various field types.
pub(crate) trait FieldMethods {
    /// Get the name of this field.
    fn name(&self) -> Option<&str>;

    /// Get the type of this field.
    fn ty(&self) -> TypeId;

    /// Get the comment for this field.
    fn comment(&self) -> Option<&str>;

    /// If this is a bitfield, how many bits does it need?
    fn bitfield_width(&self) -> Option<u32>;

    /// Is this field declared public?
    fn is_public(&self) -> bool;

    /// Get the annotations for this field.
    fn annotations(&self) -> &Annotations;

    /// The offset of the field (in bits)
    fn offset(&self) -> Option<usize>;
}

/// A contiguous set of logical bitfields that live within the same physical
/// allocation unit. See 9.2.4 [class.bit] in the C++ standard and [section
/// 2.4.II.1 in the Itanium C++
/// ABI](http://itanium-cxx-abi.github.io/cxx-abi/abi.html#class-types).
#[derive(Debug)]
pub(crate) struct BitfieldUnit {
    nth: usize,
    layout: Layout,
    bitfields: Vec<Bitfield>,
}

impl BitfieldUnit {
    /// Get the 1-based index of this bitfield unit within its containing
    /// struct. Useful for generating a Rust struct's field name for this unit
    /// of bitfields.
    pub(crate) fn nth(&self) -> usize {
        self.nth
    }

    /// Get the layout within which these bitfields reside.
    pub(crate) fn layout(&self) -> Layout {
        self.layout
    }

    /// Get the bitfields within this unit.
    pub(crate) fn bitfields(&self) -> &[Bitfield] {
        &self.bitfields
    }
}

/// A struct representing a C++ field.
#[derive(Debug)]
pub(crate) enum Field {
    /// A normal data member.
    DataMember(FieldData),

    /// A physical allocation unit containing many logical bitfields.
    Bitfields(BitfieldUnit),
}

impl Field {
    /// Get this field's layout.
    pub(crate) fn layout(&self, ctx: &BindgenContext) -> Option<Layout> {
        match *self {
            Field::Bitfields(BitfieldUnit { layout, .. }) => Some(layout),
            Field::DataMember(ref data) => {
                ctx.resolve_type(data.ty).layout(ctx)
            }
        }
    }
}

impl Trace for Field {
    type Extra = ();

    fn trace<T>(&self, _: &BindgenContext, tracer: &mut T, _: &())
    where
        T: Tracer,
    {
        match *self {
            Field::DataMember(ref data) => {
                tracer.visit_kind(data.ty.into(), EdgeKind::Field);
            }
            Field::Bitfields(BitfieldUnit { ref bitfields, .. }) => {
                for bf in bitfields {
                    tracer.visit_kind(bf.ty().into(), EdgeKind::Field);
                }
            }
        }
    }
}

impl DotAttributes for Field {
    fn dot_attributes<W>(
        &self,
        ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        match *self {
            Field::DataMember(ref data) => data.dot_attributes(ctx, out),
            Field::Bitfields(BitfieldUnit {
                layout,
                ref bitfields,
                ..
            }) => {
                writeln!(
                    out,
                    r#"<tr>
                              <td>bitfield unit</td>
                              <td>
                                <table border="0">
                                  <tr>
                                    <td>unit.size</td><td>{}</td>
                                  </tr>
                                  <tr>
                                    <td>unit.align</td><td>{}</td>
                                  </tr>
                         "#,
                    layout.size, layout.align
                )?;
                for bf in bitfields {
                    bf.dot_attributes(ctx, out)?;
                }
                writeln!(out, "</table></td></tr>")
            }
        }
    }
}

impl DotAttributes for FieldData {
    fn dot_attributes<W>(
        &self,
        _ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(
            out,
            "<tr><td>{}</td><td>{:?}</td></tr>",
            self.name().unwrap_or("(anonymous)"),
            self.ty()
        )
    }
}

impl DotAttributes for Bitfield {
    fn dot_attributes<W>(
        &self,
        _ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(
            out,
            "<tr><td>{} : {}</td><td>{:?}</td></tr>",
            self.name().unwrap_or("(anonymous)"),
            self.width(),
            self.ty()
        )
    }
}

/// A logical bitfield within some physical bitfield allocation unit.
#[derive(Debug)]
pub(crate) struct Bitfield {
    /// Index of the bit within this bitfield's allocation unit where this
    /// bitfield's bits begin.
    offset_into_unit: usize,

    /// The field data for this bitfield.
    data: FieldData,

    /// Name of the generated Rust getter for this bitfield.
    ///
    /// Should be assigned before codegen.
    getter_name: Option<String>,

    /// Name of the generated Rust setter for this bitfield.
    ///
    /// Should be assigned before codegen.
    setter_name: Option<String>,
}

impl Bitfield {
    /// Construct a new bitfield.
    fn new(offset_into_unit: usize, raw: RawField) -> Bitfield {
        assert!(raw.bitfield_width().is_some());

        Bitfield {
            offset_into_unit,
            data: raw.0,
            getter_name: None,
            setter_name: None,
        }
    }

    /// Get the index of the bit within this bitfield's allocation unit where
    /// this bitfield begins.
    pub(crate) fn offset_into_unit(&self) -> usize {
        self.offset_into_unit
    }

    /// Get the bit width of this bitfield.
    pub(crate) fn width(&self) -> u32 {
        self.data.bitfield_width().unwrap()
    }

    /// Name of the generated Rust getter for this bitfield.
    ///
    /// Panics if called before assigning bitfield accessor names or if
    /// this bitfield have no name.
    pub(crate) fn getter_name(&self) -> &str {
        assert!(
            self.name().is_some(),
            "`Bitfield::getter_name` called on anonymous field"
        );
        self.getter_name.as_ref().expect(
            "`Bitfield::getter_name` should only be called after\
             assigning bitfield accessor names",
        )
    }

    /// Name of the generated Rust setter for this bitfield.
    ///
    /// Panics if called before assigning bitfield accessor names or if
    /// this bitfield have no name.
    pub(crate) fn setter_name(&self) -> &str {
        assert!(
            self.name().is_some(),
            "`Bitfield::setter_name` called on anonymous field"
        );
        self.setter_name.as_ref().expect(
            "`Bitfield::setter_name` should only be called\
             after assigning bitfield accessor names",
        )
    }
}

impl FieldMethods for Bitfield {
    fn name(&self) -> Option<&str> {
        self.data.name()
    }

    fn ty(&self) -> TypeId {
        self.data.ty()
    }

    fn comment(&self) -> Option<&str> {
        self.data.comment()
    }

    fn bitfield_width(&self) -> Option<u32> {
        self.data.bitfield_width()
    }

    fn is_public(&self) -> bool {
        self.data.is_public()
    }

    fn annotations(&self) -> &Annotations {
        self.data.annotations()
    }

    fn offset(&self) -> Option<usize> {
        self.data.offset()
    }
}

/// A raw field might be either of a plain data member or a bitfield within a
/// bitfield allocation unit, but we haven't processed it and determined which
/// yet (which would involve allocating it into a bitfield unit if it is a
/// bitfield).
#[derive(Debug)]
struct RawField(FieldData);

impl RawField {
    /// Construct a new `RawField`.
    fn new(
        name: Option<String>,
        ty: TypeId,
        comment: Option<String>,
        annotations: Option<Annotations>,
        bitfield_width: Option<u32>,
        public: bool,
        offset: Option<usize>,
    ) -> RawField {
        RawField(FieldData {
            name,
            ty,
            comment,
            annotations: annotations.unwrap_or_default(),
            bitfield_width,
            public,
            offset,
        })
    }
}

impl FieldMethods for RawField {
    fn name(&self) -> Option<&str> {
        self.0.name()
    }

    fn ty(&self) -> TypeId {
        self.0.ty()
    }

    fn comment(&self) -> Option<&str> {
        self.0.comment()
    }

    fn bitfield_width(&self) -> Option<u32> {
        self.0.bitfield_width()
    }

    fn is_public(&self) -> bool {
        self.0.is_public()
    }

    fn annotations(&self) -> &Annotations {
        self.0.annotations()
    }

    fn offset(&self) -> Option<usize> {
        self.0.offset()
    }
}

/// Convert the given ordered set of raw fields into a list of either plain data
/// members, and/or bitfield units containing multiple bitfields.
///
/// If we do not have the layout for a bitfield's type, then we can't reliably
/// compute its allocation unit. In such cases, we return an error.
fn raw_fields_to_fields_and_bitfield_units<I>(
    ctx: &BindgenContext,
    raw_fields: I,
    packed: bool,
) -> Result<(Vec<Field>, bool), ()>
where
    I: IntoIterator<Item = RawField>,
{
    let mut raw_fields = raw_fields.into_iter().fuse().peekable();
    let mut fields = vec![];
    let mut bitfield_unit_count = 0;

    loop {
        // While we have plain old data members, just keep adding them to our
        // resulting fields. We introduce a scope here so that we can use
        // `raw_fields` again after the `by_ref` iterator adaptor is dropped.
        {
            let non_bitfields = raw_fields
                .by_ref()
                .peeking_take_while(|f| f.bitfield_width().is_none())
                .map(|f| Field::DataMember(f.0));
            fields.extend(non_bitfields);
        }

        // Now gather all the consecutive bitfields. Only consecutive bitfields
        // may potentially share a bitfield allocation unit with each other in
        // the Itanium C++ ABI.
        let mut bitfields = raw_fields
            .by_ref()
            .peeking_take_while(|f| f.bitfield_width().is_some())
            .peekable();

        if bitfields.peek().is_none() {
            break;
        }

        bitfields_to_allocation_units(
            ctx,
            &mut bitfield_unit_count,
            &mut fields,
            bitfields,
            packed,
        )?;
    }

    assert!(
        raw_fields.next().is_none(),
        "The above loop should consume all items in `raw_fields`"
    );

    Ok((fields, bitfield_unit_count != 0))
}

/// Given a set of contiguous raw bitfields, group and allocate them into
/// (potentially multiple) bitfield units.
fn bitfields_to_allocation_units<E, I>(
    ctx: &BindgenContext,
    bitfield_unit_count: &mut usize,
    fields: &mut E,
    raw_bitfields: I,
    packed: bool,
) -> Result<(), ()>
where
    E: Extend<Field>,
    I: IntoIterator<Item = RawField>,
{
    assert!(ctx.collected_typerefs());

    // NOTE: What follows is reverse-engineered from LLVM's
    // lib/AST/RecordLayoutBuilder.cpp
    //
    // FIXME(emilio): There are some differences between Microsoft and the
    // Itanium ABI, but we'll ignore those and stick to Itanium for now.
    //
    // Also, we need to handle packed bitfields and stuff.
    //
    // TODO(emilio): Take into account C++'s wide bitfields, and
    // packing, sigh.

    fn flush_allocation_unit<E>(
        fields: &mut E,
        bitfield_unit_count: &mut usize,
        unit_size_in_bits: usize,
        unit_align_in_bits: usize,
        bitfields: Vec<Bitfield>,
        packed: bool,
    ) where
        E: Extend<Field>,
    {
        *bitfield_unit_count += 1;
        let align = if packed {
            1
        } else {
            bytes_from_bits_pow2(unit_align_in_bits)
        };
        let size = align_to(unit_size_in_bits, 8) / 8;
        let layout = Layout::new(size, align);
        fields.extend(Some(Field::Bitfields(BitfieldUnit {
            nth: *bitfield_unit_count,
            layout,
            bitfields,
        })));
    }

    let mut max_align = 0;
    let mut unfilled_bits_in_unit = 0;
    let mut unit_size_in_bits = 0;
    let mut unit_align = 0;
    let mut bitfields_in_unit = vec![];

    // TODO(emilio): Determine this from attributes or pragma ms_struct
    // directives. Also, perhaps we should check if the target is MSVC?
    const is_ms_struct: bool = false;

    for bitfield in raw_bitfields {
        let bitfield_width = bitfield.bitfield_width().unwrap() as usize;
        let bitfield_layout =
            ctx.resolve_type(bitfield.ty()).layout(ctx).ok_or(())?;
        let bitfield_size = bitfield_layout.size;
        let bitfield_align = bitfield_layout.align;

        let mut offset = unit_size_in_bits;
        if !packed {
            if is_ms_struct {
                if unit_size_in_bits != 0 &&
                    (bitfield_width == 0 ||
                        bitfield_width > unfilled_bits_in_unit)
                {
                    // We've reached the end of this allocation unit, so flush it
                    // and its bitfields.
                    unit_size_in_bits =
                        align_to(unit_size_in_bits, unit_align * 8);
                    flush_allocation_unit(
                        fields,
                        bitfield_unit_count,
                        unit_size_in_bits,
                        unit_align,
                        mem::take(&mut bitfields_in_unit),
                        packed,
                    );

                    // Now we're working on a fresh bitfield allocation unit, so reset
                    // the current unit size and alignment.
                    offset = 0;
                    unit_align = 0;
                }
            } else if offset != 0 &&
                (bitfield_width == 0 ||
                    (offset & (bitfield_align * 8 - 1)) + bitfield_width >
                        bitfield_size * 8)
            {
                offset = align_to(offset, bitfield_align * 8);
            }
        }

        // According to the x86[-64] ABI spec: "Unnamed bit-fields’ types do not
        // affect the alignment of a structure or union". This makes sense: such
        // bit-fields are only used for padding, and we can't perform an
        // un-aligned read of something we can't read because we can't even name
        // it.
        if bitfield.name().is_some() {
            max_align = cmp::max(max_align, bitfield_align);

            // NB: The `bitfield_width` here is completely, absolutely
            // intentional.  Alignment of the allocation unit is based on the
            // maximum bitfield width, not (directly) on the bitfields' types'
            // alignment.
            unit_align = cmp::max(unit_align, bitfield_width);
        }

        // Always keep all bitfields around. While unnamed bitifields are used
        // for padding (and usually not needed hereafter), large unnamed
        // bitfields over their types size cause weird allocation size behavior from clang.
        // Therefore, all bitfields needed to be kept around in order to check for this
        // and make the struct opaque in this case
        bitfields_in_unit.push(Bitfield::new(offset, bitfield));

        unit_size_in_bits = offset + bitfield_width;

        // Compute what the physical unit's final size would be given what we
        // have seen so far, and use that to compute how many bits are still
        // available in the unit.
        let data_size = align_to(unit_size_in_bits, bitfield_align * 8);
        unfilled_bits_in_unit = data_size - unit_size_in_bits;
    }

    if unit_size_in_bits != 0 {
        // Flush the last allocation unit and its bitfields.
        flush_allocation_unit(
            fields,
            bitfield_unit_count,
            unit_size_in_bits,
            unit_align,
            bitfields_in_unit,
            packed,
        );
    }

    Ok(())
}

/// A compound structure's fields are initially raw, and have bitfields that
/// have not been grouped into allocation units. During this time, the fields
/// are mutable and we build them up during parsing.
///
/// Then, once resolving typerefs is completed, we compute all structs' fields'
/// bitfield allocation units, and they remain frozen and immutable forever
/// after.
#[derive(Debug)]
enum CompFields {
    Before(Vec<RawField>),
    After {
        fields: Vec<Field>,
        has_bitfield_units: bool,
    },
    Error,
}

impl Default for CompFields {
    fn default() -> CompFields {
        CompFields::Before(vec![])
    }
}

impl CompFields {
    fn append_raw_field(&mut self, raw: RawField) {
        match *self {
            CompFields::Before(ref mut raws) => {
                raws.push(raw);
            }
            _ => {
                panic!(
                    "Must not append new fields after computing bitfield allocation units"
                );
            }
        }
    }

    fn compute_bitfield_units(&mut self, ctx: &BindgenContext, packed: bool) {
        let raws = match *self {
            CompFields::Before(ref mut raws) => mem::take(raws),
            _ => {
                panic!("Already computed bitfield units");
            }
        };

        let result = raw_fields_to_fields_and_bitfield_units(ctx, raws, packed);

        match result {
            Ok((fields, has_bitfield_units)) => {
                *self = CompFields::After {
                    fields,
                    has_bitfield_units,
                };
            }
            Err(()) => {
                *self = CompFields::Error;
            }
        }
    }

    fn deanonymize_fields(&mut self, ctx: &BindgenContext, methods: &[Method]) {
        let fields = match *self {
            CompFields::After { ref mut fields, .. } => fields,
            // Nothing to do here.
            CompFields::Error => return,
            CompFields::Before(_) => {
                panic!("Not yet computed bitfield units.");
            }
        };

        fn has_method(
            methods: &[Method],
            ctx: &BindgenContext,
            name: &str,
        ) -> bool {
            methods.iter().any(|method| {
                let method_name = ctx.resolve_func(method.signature()).name();
                method_name == name || ctx.rust_mangle(method_name) == name
            })
        }

        struct AccessorNamesPair {
            getter: String,
            setter: String,
        }

        let mut accessor_names: HashMap<String, AccessorNamesPair> = fields
            .iter()
            .flat_map(|field| match *field {
                Field::Bitfields(ref bu) => &*bu.bitfields,
                Field::DataMember(_) => &[],
            })
            .filter_map(|bitfield| bitfield.name())
            .map(|bitfield_name| {
                let bitfield_name = bitfield_name.to_string();
                let getter = {
                    let mut getter =
                        ctx.rust_mangle(&bitfield_name).to_string();
                    if has_method(methods, ctx, &getter) {
                        getter.push_str("_bindgen_bitfield");
                    }
                    getter
                };
                let setter = {
                    let setter = format!("set_{bitfield_name}");
                    let mut setter = ctx.rust_mangle(&setter).to_string();
                    if has_method(methods, ctx, &setter) {
                        setter.push_str("_bindgen_bitfield");
                    }
                    setter
                };
                (bitfield_name, AccessorNamesPair { getter, setter })
            })
            .collect();

        let mut anon_field_counter = 0;
        for field in fields.iter_mut() {
            match *field {
                Field::DataMember(FieldData { ref mut name, .. }) => {
                    if name.is_some() {
                        continue;
                    }

                    anon_field_counter += 1;
                    *name = Some(format!(
                        "{}{anon_field_counter}",
                        ctx.options().anon_fields_prefix,
                    ));
                }
                Field::Bitfields(ref mut bu) => {
                    for bitfield in &mut bu.bitfields {
                        if bitfield.name().is_none() {
                            continue;
                        }

                        if let Some(AccessorNamesPair { getter, setter }) =
                            accessor_names.remove(bitfield.name().unwrap())
                        {
                            bitfield.getter_name = Some(getter);
                            bitfield.setter_name = Some(setter);
                        }
                    }
                }
            }
        }
    }

    /// Return the flex array member for the struct/class, if any.
    fn flex_array_member(&self, ctx: &BindgenContext) -> Option<TypeId> {
        let fields = match self {
            CompFields::Before(_) => panic!("raw fields"),
            CompFields::After { fields, .. } => fields,
            CompFields::Error => return None, // panic?
        };

        match fields.last()? {
            Field::Bitfields(..) => None,
            Field::DataMember(FieldData { ty, .. }) => ctx
                .resolve_type(*ty)
                .is_incomplete_array(ctx)
                .map(|item| item.expect_type_id(ctx)),
        }
    }
}

impl Trace for CompFields {
    type Extra = ();

    fn trace<T>(&self, context: &BindgenContext, tracer: &mut T, _: &())
    where
        T: Tracer,
    {
        match *self {
            CompFields::Error => {}
            CompFields::Before(ref fields) => {
                for f in fields {
                    tracer.visit_kind(f.ty().into(), EdgeKind::Field);
                }
            }
            CompFields::After { ref fields, .. } => {
                for f in fields {
                    f.trace(context, tracer, &());
                }
            }
        }
    }
}

/// Common data shared across different field types.
#[derive(Clone, Debug)]
pub(crate) struct FieldData {
    /// The name of the field, empty if it's an unnamed bitfield width.
    name: Option<String>,

    /// The inner type.
    ty: TypeId,

    /// The doc comment on the field if any.
    comment: Option<String>,

    /// Annotations for this field, or the default.
    annotations: Annotations,

    /// If this field is a bitfield, and how many bits does it contain if it is.
    bitfield_width: Option<u32>,

    /// If the C++ field is declared `public`
    public: bool,

    /// The offset of the field (in bits)
    offset: Option<usize>,
}

impl FieldMethods for FieldData {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn ty(&self) -> TypeId {
        self.ty
    }

    fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    fn bitfield_width(&self) -> Option<u32> {
        self.bitfield_width
    }

    fn is_public(&self) -> bool {
        self.public
    }

    fn annotations(&self) -> &Annotations {
        &self.annotations
    }

    fn offset(&self) -> Option<usize> {
        self.offset
    }
}

/// The kind of inheritance a base class is using.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum BaseKind {
    /// Normal inheritance, like:
    ///
    /// ```cpp
    /// class A : public B {};
    /// ```
    Normal,
    /// Virtual inheritance, like:
    ///
    /// ```cpp
    /// class A: public virtual B {};
    /// ```
    Virtual,
}

/// A base class.
#[derive(Clone, Debug)]
pub(crate) struct Base {
    /// The type of this base class.
    pub(crate) ty: TypeId,
    /// The kind of inheritance we're doing.
    pub(crate) kind: BaseKind,
    /// Name of the field in which this base should be stored.
    pub(crate) field_name: String,
    /// Whether this base is inherited from publicly.
    pub(crate) is_pub: bool,
}

impl Base {
    /// Whether this base class is inheriting virtually.
    pub(crate) fn is_virtual(&self) -> bool {
        self.kind == BaseKind::Virtual
    }

    /// Whether this base class should have it's own field for storage.
    pub(crate) fn requires_storage(&self, ctx: &BindgenContext) -> bool {
        // Virtual bases are already taken into account by the vtable
        // pointer.
        //
        // FIXME(emilio): Is this always right?
        if self.is_virtual() {
            return false;
        }

        // NB: We won't include zero-sized types in our base chain because they
        // would contribute to our size given the dummy field we insert for
        // zero-sized types.
        if self.ty.is_zero_sized(ctx) {
            return false;
        }

        true
    }

    /// Whether this base is inherited from publicly.
    pub(crate) fn is_public(&self) -> bool {
        self.is_pub
    }
}

/// A compound type.
///
/// Either a struct or union, a compound type is built up from the combination
/// of fields which also are associated with their own (potentially compound)
/// type.
#[derive(Debug)]
pub(crate) struct CompInfo {
    /// Whether this is a struct or a union.
    kind: CompKind,

    /// The members of this struct or union.
    fields: CompFields,

    /// The abstract template parameters of this class. Note that these are NOT
    /// concrete template arguments, and should always be a
    /// `Type(TypeKind::TypeParam(name))`. For concrete template arguments, see
    /// `TypeKind::TemplateInstantiation`.
    template_params: Vec<TypeId>,

    /// The method declarations inside this class, if in C++ mode.
    methods: Vec<Method>,

    /// The different constructors this struct or class contains.
    constructors: Vec<FunctionId>,

    /// The destructor of this type. The bool represents whether this destructor
    /// is virtual.
    destructor: Option<(MethodKind, FunctionId)>,

    /// Vector of classes this one inherits from.
    base_members: Vec<Base>,

    /// The inner types that were declared inside this class, in something like:
    ///
    /// ```c++
    /// class Foo {
    ///     typedef int FooTy;
    ///     struct Bar {
    ///         int baz;
    ///     };
    /// }
    ///
    /// static Foo::Bar const = {3};
    /// ```
    inner_types: Vec<TypeId>,

    /// Set of static constants declared inside this class.
    inner_vars: Vec<VarId>,

    /// Whether this type should generate an vtable (TODO: Should be able to
    /// look at the virtual methods and ditch this field).
    has_own_virtual_method: bool,

    /// Whether this type has destructor.
    has_destructor: bool,

    /// Whether this type has a base type with more than one member.
    ///
    /// TODO: We should be able to compute this.
    has_nonempty_base: bool,

    /// If this type has a template parameter which is not a type (e.g.: a
    /// `size_t`)
    has_non_type_template_params: bool,

    /// Whether this type has a bit field member whose width couldn't be
    /// evaluated (e.g. if it depends on a template parameter). We generate an
    /// opaque type in this case.
    has_unevaluable_bit_field_width: bool,

    /// Whether we saw `__attribute__((packed))` on or within this type.
    packed_attr: bool,

    /// Used to know if we've found an opaque attribute that could cause us to
    /// generate a type with invalid layout. This is explicitly used to avoid us
    /// generating bad alignments when parsing types like `max_align_t`.
    ///
    /// It's not clear what the behavior should be here, if generating the item
    /// and pray, or behave as an opaque type.
    found_unknown_attr: bool,

    /// Used to indicate when a struct has been forward declared. Usually used
    /// in headers so that APIs can't modify them directly.
    is_forward_declaration: bool,
}

impl CompInfo {
    /// Construct a new compound type.
    pub(crate) fn new(kind: CompKind) -> Self {
        CompInfo {
            kind,
            fields: CompFields::default(),
            template_params: vec![],
            methods: vec![],
            constructors: vec![],
            destructor: None,
            base_members: vec![],
            inner_types: vec![],
            inner_vars: vec![],
            has_own_virtual_method: false,
            has_destructor: false,
            has_nonempty_base: false,
            has_non_type_template_params: false,
            has_unevaluable_bit_field_width: false,
            packed_attr: false,
            found_unknown_attr: false,
            is_forward_declaration: false,
        }
    }

    /// Compute the layout of this type.
    ///
    /// This is called as a fallback under some circumstances where LLVM doesn't
    /// give us the correct layout.
    ///
    /// If we're a union without known layout, we try to compute it from our
    /// members. This is not ideal, but clang fails to report the size for these
    /// kind of unions, see `test/headers/template_union.hpp`
    pub(crate) fn layout(&self, ctx: &BindgenContext) -> Option<Layout> {
        // We can't do better than clang here, sorry.
        if self.kind == CompKind::Struct {
            return None;
        }

        // By definition, we don't have the right layout information here if
        // we're a forward declaration.
        if self.is_forward_declaration() {
            return None;
        }

        // empty union case
        if !self.has_fields() {
            return None;
        }

        let mut max_size = 0;
        // Don't allow align(0)
        let mut max_align = 1;
        self.each_known_field_layout(ctx, |layout| {
            max_size = cmp::max(max_size, layout.size);
            max_align = cmp::max(max_align, layout.align);
        });

        Some(Layout::new(max_size, max_align))
    }

    /// Get this type's set of fields.
    pub(crate) fn fields(&self) -> &[Field] {
        match self.fields {
            CompFields::Error => &[],
            CompFields::After { ref fields, .. } => fields,
            CompFields::Before(..) => {
                panic!("Should always have computed bitfield units first");
            }
        }
    }

    /// Return the flex array member and its element type if any
    pub(crate) fn flex_array_member(
        &self,
        ctx: &BindgenContext,
    ) -> Option<TypeId> {
        self.fields.flex_array_member(ctx)
    }

    fn has_fields(&self) -> bool {
        match self.fields {
            CompFields::Error => false,
            CompFields::After { ref fields, .. } => !fields.is_empty(),
            CompFields::Before(ref raw_fields) => !raw_fields.is_empty(),
        }
    }

    fn each_known_field_layout(
        &self,
        ctx: &BindgenContext,
        mut callback: impl FnMut(Layout),
    ) {
        match self.fields {
            CompFields::Error => {}
            CompFields::After { ref fields, .. } => {
                for field in fields {
                    if let Some(layout) = field.layout(ctx) {
                        callback(layout);
                    }
                }
            }
            CompFields::Before(ref raw_fields) => {
                for field in raw_fields {
                    let field_ty = ctx.resolve_type(field.0.ty);
                    if let Some(layout) = field_ty.layout(ctx) {
                        callback(layout);
                    }
                }
            }
        }
    }

    fn has_bitfields(&self) -> bool {
        match self.fields {
            CompFields::Error => false,
            CompFields::After {
                has_bitfield_units, ..
            } => has_bitfield_units,
            CompFields::Before(_) => {
                panic!("Should always have computed bitfield units first");
            }
        }
    }

    /// Returns whether we have a too large bitfield unit, in which case we may
    /// not be able to derive some of the things we should be able to normally
    /// derive.
    pub(crate) fn has_too_large_bitfield_unit(&self) -> bool {
        if !self.has_bitfields() {
            return false;
        }
        self.fields().iter().any(|field| match *field {
            Field::DataMember(..) => false,
            Field::Bitfields(ref unit) => {
                unit.layout.size > RUST_DERIVE_IN_ARRAY_LIMIT
            }
        })
    }

    /// Does this type have any template parameters that aren't types
    /// (e.g. int)?
    pub(crate) fn has_non_type_template_params(&self) -> bool {
        self.has_non_type_template_params
    }

    /// Do we see a virtual function during parsing?
    /// Get the `has_own_virtual_method` boolean.
    pub(crate) fn has_own_virtual_method(&self) -> bool {
        self.has_own_virtual_method
    }

    /// Did we see a destructor when parsing this type?
    pub(crate) fn has_own_destructor(&self) -> bool {
        self.has_destructor
    }

    /// Get this type's set of methods.
    pub(crate) fn methods(&self) -> &[Method] {
        &self.methods
    }

    /// Get this type's set of constructors.
    pub(crate) fn constructors(&self) -> &[FunctionId] {
        &self.constructors
    }

    /// Get this type's destructor.
    pub(crate) fn destructor(&self) -> Option<(MethodKind, FunctionId)> {
        self.destructor
    }

    /// What kind of compound type is this?
    pub(crate) fn kind(&self) -> CompKind {
        self.kind
    }

    /// Is this a union?
    pub(crate) fn is_union(&self) -> bool {
        self.kind() == CompKind::Union
    }

    /// The set of types that this one inherits from.
    pub(crate) fn base_members(&self) -> &[Base] {
        &self.base_members
    }

    /// Construct a new compound type from a Clang type.
    pub(crate) fn from_ty(
        potential_id: ItemId,
        ty: &clang::Type,
        location: Option<clang::Cursor>,
        ctx: &mut BindgenContext,
    ) -> Result<Self, ParseError> {
        use clang_sys::*;
        assert!(
            ty.template_args().is_none(),
            "We handle template instantiations elsewhere"
        );

        let mut cursor = ty.declaration();
        let mut kind = Self::kind_from_cursor(&cursor);
        if kind.is_err() {
            if let Some(location) = location {
                kind = Self::kind_from_cursor(&location);
                cursor = location;
            }
        }

        let kind = kind?;

        debug!("CompInfo::from_ty({kind:?}, {cursor:?})");

        let mut ci = CompInfo::new(kind);
        ci.is_forward_declaration =
            location.map_or(true, |cur| match cur.kind() {
                CXCursor_ParmDecl => true,
                CXCursor_StructDecl | CXCursor_UnionDecl |
                CXCursor_ClassDecl => !cur.is_definition(),
                _ => false,
            });

        let mut maybe_anonymous_struct_field = None;
        cursor.visit(|cur| {
            if cur.kind() != CXCursor_FieldDecl {
                if let Some((ty, clang_ty, public, offset)) =
                    maybe_anonymous_struct_field.take()
                {
                    if cur.kind() == CXCursor_TypedefDecl &&
                        cur.typedef_type().unwrap().canonical_type() ==
                            clang_ty
                    {
                        // Typedefs of anonymous structs appear later in the ast
                        // than the struct itself, that would otherwise be an
                        // anonymous field. Detect that case here, and do
                        // nothing.
                    } else {
                        let field = RawField::new(
                            None, ty, None, None, None, public, offset,
                        );
                        ci.fields.append_raw_field(field);
                    }
                }
            }

            match cur.kind() {
                CXCursor_FieldDecl => {
                    if let Some((ty, clang_ty, public, offset)) =
                        maybe_anonymous_struct_field.take()
                    {
                        let mut used = false;
                        cur.visit(|child| {
                            if child.cur_type() == clang_ty {
                                used = true;
                            }
                            CXChildVisit_Continue
                        });

                        if !used {
                            let field = RawField::new(
                                None, ty, None, None, None, public, offset,
                            );
                            ci.fields.append_raw_field(field);
                        }
                    }

                    let bit_width = if cur.is_bit_field() {
                        let width = cur.bit_width();

                        // Make opaque type if the bit width couldn't be
                        // evaluated.
                        if width.is_none() {
                            ci.has_unevaluable_bit_field_width = true;
                            return CXChildVisit_Break;
                        }

                        width
                    } else {
                        None
                    };

                    let field_type = Item::from_ty_or_ref(
                        cur.cur_type(),
                        cur,
                        Some(potential_id),
                        ctx,
                    );

                    let comment = cur.raw_comment();
                    let annotations = Annotations::new(&cur);
                    let name = cur.spelling();
                    let is_public = cur.public_accessible();
                    let offset = cur.offset_of_field().ok();

                    // Name can be empty if there are bitfields, for example,
                    // see tests/headers/struct_with_bitfields.h
                    assert!(
                        !name.is_empty() || bit_width.is_some(),
                        "Empty field name?"
                    );

                    let name = if name.is_empty() { None } else { Some(name) };

                    let field = RawField::new(
                        name,
                        field_type,
                        comment,
                        annotations,
                        bit_width,
                        is_public,
                        offset,
                    );
                    ci.fields.append_raw_field(field);

                    // No we look for things like attributes and stuff.
                    cur.visit(|cur| {
                        if cur.kind() == CXCursor_UnexposedAttr {
                            ci.found_unknown_attr = true;
                        }
                        CXChildVisit_Continue
                    });
                }
                CXCursor_UnexposedAttr => {
                    ci.found_unknown_attr = true;
                }
                CXCursor_EnumDecl |
                CXCursor_TypeAliasDecl |
                CXCursor_TypeAliasTemplateDecl |
                CXCursor_TypedefDecl |
                CXCursor_StructDecl |
                CXCursor_UnionDecl |
                CXCursor_ClassTemplate |
                CXCursor_ClassDecl => {
                    // We can find non-semantic children here, clang uses a
                    // StructDecl to note incomplete structs that haven't been
                    // forward-declared before, see [1].
                    //
                    // Also, clang seems to scope struct definitions inside
                    // unions, and other named struct definitions inside other
                    // structs to the whole translation unit.
                    //
                    // Let's just assume that if the cursor we've found is a
                    // definition, it's a valid inner type.
                    //
                    // [1]: https://github.com/rust-lang/rust-bindgen/issues/482
                    let is_inner_struct =
                        cur.semantic_parent() == cursor || cur.is_definition();
                    if !is_inner_struct {
                        return CXChildVisit_Continue;
                    }

                    // Even if this is a definition, we may not be the semantic
                    // parent, see #1281.
                    let inner = Item::parse(cur, Some(potential_id), ctx)
                        .expect("Inner ClassDecl");

                    // If we avoided recursion parsing this type (in
                    // `Item::from_ty_with_id()`), then this might not be a
                    // valid type ID, so check and gracefully handle this.
                    if ctx.resolve_item_fallible(inner).is_some() {
                        let inner = inner.expect_type_id(ctx);

                        ci.inner_types.push(inner);

                        // A declaration of an union or a struct without name
                        // could also be an unnamed field, unfortunately.
                        if cur.is_anonymous() && cur.kind() != CXCursor_EnumDecl
                        {
                            let ty = cur.cur_type();
                            let public = cur.public_accessible();
                            let offset = cur.offset_of_field().ok();

                            maybe_anonymous_struct_field =
                                Some((inner, ty, public, offset));
                        }
                    }
                }
                CXCursor_PackedAttr => {
                    ci.packed_attr = true;
                }
                CXCursor_TemplateTypeParameter => {
                    let param = Item::type_param(None, cur, ctx).expect(
                        "Item::type_param shouldn't fail when pointing \
                         at a TemplateTypeParameter",
                    );
                    ci.template_params.push(param);
                }
                CXCursor_CXXBaseSpecifier => {
                    let is_virtual_base = cur.is_virtual_base();
                    ci.has_own_virtual_method |= is_virtual_base;

                    let kind = if is_virtual_base {
                        BaseKind::Virtual
                    } else {
                        BaseKind::Normal
                    };

                    let field_name = match ci.base_members.len() {
                        0 => "_base".into(),
                        n => format!("_base_{n}"),
                    };
                    let type_id =
                        Item::from_ty_or_ref(cur.cur_type(), cur, None, ctx);
                    ci.base_members.push(Base {
                        ty: type_id,
                        kind,
                        field_name,
                        is_pub: cur.access_specifier() == CX_CXXPublic,
                    });
                }
                CXCursor_Constructor | CXCursor_Destructor |
                CXCursor_CXXMethod => {
                    let is_virtual = cur.method_is_virtual();
                    let is_static = cur.method_is_static();
                    debug_assert!(!(is_static && is_virtual), "How?");

                    ci.has_destructor |= cur.kind() == CXCursor_Destructor;
                    ci.has_own_virtual_method |= is_virtual;

                    // This used to not be here, but then I tried generating
                    // stylo bindings with this (without path filters), and
                    // cried a lot with a method in gfx/Point.h
                    // (ToUnknownPoint), that somehow was causing the same type
                    // to be inserted in the map two times.
                    //
                    // I couldn't make a reduced test case, but anyway...
                    // Methods of template functions not only used to be inlined,
                    // but also instantiated, and we wouldn't be able to call
                    // them, so just bail out.
                    if !ci.template_params.is_empty() {
                        return CXChildVisit_Continue;
                    }

                    // NB: This gets us an owned `Function`, not a
                    // `FunctionSig`.
                    let signature =
                        match Item::parse(cur, Some(potential_id), ctx) {
                            Ok(item)
                                if ctx
                                    .resolve_item(item)
                                    .kind()
                                    .is_function() =>
                            {
                                item
                            }
                            _ => return CXChildVisit_Continue,
                        };

                    let signature = signature.expect_function_id(ctx);

                    match cur.kind() {
                        CXCursor_Constructor => {
                            ci.constructors.push(signature);
                        }
                        CXCursor_Destructor => {
                            let kind = if is_virtual {
                                MethodKind::VirtualDestructor {
                                    pure_virtual: cur.method_is_pure_virtual(),
                                }
                            } else {
                                MethodKind::Destructor
                            };
                            ci.destructor = Some((kind, signature));
                        }
                        CXCursor_CXXMethod => {
                            let is_const = cur.method_is_const();
                            let method_kind = if is_static {
                                MethodKind::Static
                            } else if is_virtual {
                                MethodKind::Virtual {
                                    pure_virtual: cur.method_is_pure_virtual(),
                                }
                            } else {
                                MethodKind::Normal
                            };

                            let method =
                                Method::new(method_kind, signature, is_const);

                            ci.methods.push(method);
                        }
                        _ => unreachable!("How can we see this here?"),
                    }
                }
                CXCursor_NonTypeTemplateParameter => {
                    ci.has_non_type_template_params = true;
                }
                CXCursor_VarDecl => {
                    let linkage = cur.linkage();
                    if linkage != CXLinkage_External &&
                        linkage != CXLinkage_UniqueExternal
                    {
                        return CXChildVisit_Continue;
                    }

                    let visibility = cur.visibility();
                    if visibility != CXVisibility_Default {
                        return CXChildVisit_Continue;
                    }

                    if let Ok(item) = Item::parse(cur, Some(potential_id), ctx)
                    {
                        ci.inner_vars.push(item.as_var_id_unchecked());
                    }
                }
                // Intentionally not handled
                CXCursor_CXXAccessSpecifier |
                CXCursor_CXXFinalAttr |
                CXCursor_FunctionTemplate |
                CXCursor_ConversionFunction => {}
                _ => {
                    warn!(
                        "unhandled comp member `{}` (kind {:?}) in `{}` ({})",
                        cur.spelling(),
                        clang::kind_to_str(cur.kind()),
                        cursor.spelling(),
                        cur.location()
                    );
                }
            }
            CXChildVisit_Continue
        });

        if let Some((ty, _, public, offset)) = maybe_anonymous_struct_field {
            let field =
                RawField::new(None, ty, None, None, None, public, offset);
            ci.fields.append_raw_field(field);
        }

        Ok(ci)
    }

    fn kind_from_cursor(
        cursor: &clang::Cursor,
    ) -> Result<CompKind, ParseError> {
        use clang_sys::*;
        Ok(match cursor.kind() {
            CXCursor_UnionDecl => CompKind::Union,
            CXCursor_ClassDecl | CXCursor_StructDecl => CompKind::Struct,
            CXCursor_CXXBaseSpecifier |
            CXCursor_ClassTemplatePartialSpecialization |
            CXCursor_ClassTemplate => match cursor.template_kind() {
                CXCursor_UnionDecl => CompKind::Union,
                _ => CompKind::Struct,
            },
            _ => {
                warn!("Unknown kind for comp type: {cursor:?}");
                return Err(ParseError::Continue);
            }
        })
    }

    /// Get the set of types that were declared within this compound type
    /// (e.g. nested class definitions).
    pub(crate) fn inner_types(&self) -> &[TypeId] {
        &self.inner_types
    }

    /// Get the set of static variables declared within this compound type.
    pub(crate) fn inner_vars(&self) -> &[VarId] {
        &self.inner_vars
    }

    /// Have we found a field with an opaque type that could potentially mess up
    /// the layout of this compound type?
    pub(crate) fn found_unknown_attr(&self) -> bool {
        self.found_unknown_attr
    }

    /// Is this compound type packed?
    pub(crate) fn is_packed(
        &self,
        ctx: &BindgenContext,
        layout: Option<&Layout>,
    ) -> bool {
        if self.packed_attr {
            return true;
        }

        // Even though `libclang` doesn't expose `#pragma packed(...)`, we can
        // detect it through its effects.
        if let Some(parent_layout) = layout {
            let mut packed = false;
            self.each_known_field_layout(ctx, |layout| {
                packed = packed || layout.align > parent_layout.align;
            });
            if packed {
                info!("Found a struct that was defined within `#pragma packed(...)`");
                return true;
            }

            if self.has_own_virtual_method && parent_layout.align == 1 {
                return true;
            }
        }

        false
    }

    /// Return true if a compound type is "naturally packed". This means we can exclude the
    /// "packed" attribute without changing the layout.
    /// This is useful for types that need an "align(N)" attribute since rustc won't compile
    /// structs that have both of those attributes.
    pub(crate) fn already_packed(&self, ctx: &BindgenContext) -> Option<bool> {
        let mut total_size: usize = 0;

        for field in self.fields() {
            let layout = field.layout(ctx)?;

            if layout.align != 0 && total_size % layout.align != 0 {
                return Some(false);
            }

            total_size += layout.size;
        }

        Some(true)
    }

    /// Returns true if compound type has been forward declared
    pub(crate) fn is_forward_declaration(&self) -> bool {
        self.is_forward_declaration
    }

    /// Compute this compound structure's bitfield allocation units.
    pub(crate) fn compute_bitfield_units(
        &mut self,
        ctx: &BindgenContext,
        layout: Option<&Layout>,
    ) {
        let packed = self.is_packed(ctx, layout);
        self.fields.compute_bitfield_units(ctx, packed);
    }

    /// Assign for each anonymous field a generated name.
    pub(crate) fn deanonymize_fields(&mut self, ctx: &BindgenContext) {
        self.fields.deanonymize_fields(ctx, &self.methods);
    }

    /// Returns whether the current union can be represented as a Rust `union`
    ///
    /// Requirements:
    ///     1. Current `RustTarget` allows for `untagged_union`
    ///     2. Each field can derive `Copy` or we use `ManuallyDrop`.
    ///     3. It's not zero-sized.
    ///
    /// Second boolean returns whether all fields can be copied (and thus
    /// `ManuallyDrop` is not needed).
    pub(crate) fn is_rust_union(
        &self,
        ctx: &BindgenContext,
        layout: Option<&Layout>,
        name: &str,
    ) -> (bool, bool) {
        if !self.is_union() {
            return (false, false);
        }

        if !ctx.options().untagged_union {
            return (false, false);
        }

        if self.is_forward_declaration() {
            return (false, false);
        }

        let union_style = if ctx.options().bindgen_wrapper_union.matches(name) {
            NonCopyUnionStyle::BindgenWrapper
        } else if ctx.options().manually_drop_union.matches(name) {
            NonCopyUnionStyle::ManuallyDrop
        } else {
            ctx.options().default_non_copy_union_style
        };

        let all_can_copy = self.fields().iter().all(|f| match *f {
            Field::DataMember(ref field_data) => {
                field_data.ty().can_derive_copy(ctx)
            }
            Field::Bitfields(_) => true,
        });

        if !all_can_copy && union_style == NonCopyUnionStyle::BindgenWrapper {
            return (false, false);
        }

        if layout.is_some_and(|l| l.size == 0) {
            return (false, false);
        }

        (true, all_can_copy)
    }
}

impl DotAttributes for CompInfo {
    fn dot_attributes<W>(
        &self,
        ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(out, "<tr><td>CompKind</td><td>{:?}</td></tr>", self.kind)?;

        if self.has_own_virtual_method {
            writeln!(out, "<tr><td>has_vtable</td><td>true</td></tr>")?;
        }

        if self.has_destructor {
            writeln!(out, "<tr><td>has_destructor</td><td>true</td></tr>")?;
        }

        if self.has_nonempty_base {
            writeln!(out, "<tr><td>has_nonempty_base</td><td>true</td></tr>")?;
        }

        if self.has_non_type_template_params {
            writeln!(
                out,
                "<tr><td>has_non_type_template_params</td><td>true</td></tr>"
            )?;
        }

        if self.packed_attr {
            writeln!(out, "<tr><td>packed_attr</td><td>true</td></tr>")?;
        }

        if self.is_forward_declaration {
            writeln!(
                out,
                "<tr><td>is_forward_declaration</td><td>true</td></tr>"
            )?;
        }

        if !self.fields().is_empty() {
            writeln!(out, r#"<tr><td>fields</td><td><table border="0">"#)?;
            for field in self.fields() {
                field.dot_attributes(ctx, out)?;
            }
            writeln!(out, "</table></td></tr>")?;
        }

        Ok(())
    }
}

impl IsOpaque for CompInfo {
    type Extra = Option<Layout>;

    fn is_opaque(&self, ctx: &BindgenContext, layout: &Option<Layout>) -> bool {
        if self.has_non_type_template_params ||
            self.has_unevaluable_bit_field_width
        {
            return true;
        }

        // When we do not have the layout for a bitfield's type (for example, it
        // is a type parameter), then we can't compute bitfield units. We are
        // left with no choice but to make the whole struct opaque, or else we
        // might generate structs with incorrect sizes and alignments.
        if let CompFields::Error = self.fields {
            return true;
        }

        // Bitfields with a width that is larger than their unit's width have
        // some strange things going on, and the best we can do is make the
        // whole struct opaque.
        if self.fields().iter().any(|f| match *f {
            Field::DataMember(_) => false,
            Field::Bitfields(ref unit) => unit.bitfields().iter().any(|bf| {
                let bitfield_layout = ctx
                    .resolve_type(bf.ty())
                    .layout(ctx)
                    .expect("Bitfield without layout? Gah!");
                bf.width() / 8 > bitfield_layout.size as u32
            }),
        }) {
            return true;
        }

        if !ctx.options().rust_features().repr_packed_n {
            // If we don't have `#[repr(packed(N)]`, the best we can
            // do is make this struct opaque.
            //
            // See https://github.com/rust-lang/rust-bindgen/issues/537 and
            // https://github.com/rust-lang/rust/issues/33158
            if self.is_packed(ctx, layout.as_ref()) &&
                layout.map_or(false, |l| l.align > 1)
            {
                warn!("Found a type that is both packed and aligned to greater than \
                       1; Rust before version 1.33 doesn't have `#[repr(packed(N))]`, so we \
                       are treating it as opaque. You may wish to set bindgen's rust target \
                       version to 1.33 or later to enable `#[repr(packed(N))]` support.");
                return true;
            }
        }

        false
    }
}

impl TemplateParameters for CompInfo {
    fn self_template_params(&self, _ctx: &BindgenContext) -> Vec<TypeId> {
        self.template_params.clone()
    }
}

impl Trace for CompInfo {
    type Extra = Item;

    fn trace<T>(&self, context: &BindgenContext, tracer: &mut T, item: &Item)
    where
        T: Tracer,
    {
        for p in item.all_template_params(context) {
            tracer.visit_kind(p.into(), EdgeKind::TemplateParameterDefinition);
        }

        for ty in self.inner_types() {
            tracer.visit_kind(ty.into(), EdgeKind::InnerType);
        }

        for &var in self.inner_vars() {
            tracer.visit_kind(var.into(), EdgeKind::InnerVar);
        }

        for method in self.methods() {
            tracer.visit_kind(method.signature.into(), EdgeKind::Method);
        }

        if let Some((_kind, signature)) = self.destructor() {
            tracer.visit_kind(signature.into(), EdgeKind::Destructor);
        }

        for ctor in self.constructors() {
            tracer.visit_kind(ctor.into(), EdgeKind::Constructor);
        }

        // Base members and fields are not generated for opaque types (but all
        // of the above things are) so stop here.
        if item.is_opaque(context, &()) {
            return;
        }

        for base in self.base_members() {
            tracer.visit_kind(base.ty.into(), EdgeKind::BaseMember);
        }

        self.fields.trace(context, tracer, &());
    }
}
