//! Helpers for code generation that need struct layout

use super::helpers;

use crate::ir::comp::CompInfo;
use crate::ir::context::BindgenContext;
use crate::ir::layout::Layout;
use crate::ir::ty::{Type, TypeKind};
use crate::FieldVisibilityKind;
use proc_macro2::{Ident, Span};
use std::cmp;

const MAX_GUARANTEED_ALIGN: usize = 8;

/// Trace the layout of struct.
#[derive(Debug)]
pub(crate) struct StructLayoutTracker<'a> {
    name: &'a str,
    ctx: &'a BindgenContext,
    comp: &'a CompInfo,
    is_packed: bool,
    known_type_layout: Option<Layout>,
    is_rust_union: bool,
    can_copy_union_fields: bool,
    latest_offset: usize,
    padding_count: usize,
    latest_field_layout: Option<Layout>,
    max_field_align: usize,
    last_field_was_bitfield: bool,
    visibility: FieldVisibilityKind,
    last_field_was_flexible_array: bool,
}

/// Returns a size aligned to a given value.
pub(crate) fn align_to(size: usize, align: usize) -> usize {
    if align == 0 {
        return size;
    }

    let rem = size % align;
    if rem == 0 {
        return size;
    }

    size + align - rem
}

/// Returns the lower power of two byte count that can hold at most n bits.
pub(crate) fn bytes_from_bits_pow2(mut n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    if n <= 8 {
        return 1;
    }

    if !n.is_power_of_two() {
        n = n.next_power_of_two();
    }

    n / 8
}

#[test]
fn test_align_to() {
    assert_eq!(align_to(1, 1), 1);
    assert_eq!(align_to(1, 2), 2);
    assert_eq!(align_to(1, 4), 4);
    assert_eq!(align_to(5, 1), 5);
    assert_eq!(align_to(17, 4), 20);
}

#[test]
fn test_bytes_from_bits_pow2() {
    assert_eq!(bytes_from_bits_pow2(0), 0);
    for i in 1..9 {
        assert_eq!(bytes_from_bits_pow2(i), 1);
    }
    for i in 9..17 {
        assert_eq!(bytes_from_bits_pow2(i), 2);
    }
    for i in 17..33 {
        assert_eq!(bytes_from_bits_pow2(i), 4);
    }
}

impl<'a> StructLayoutTracker<'a> {
    pub(crate) fn new(
        ctx: &'a BindgenContext,
        comp: &'a CompInfo,
        ty: &'a Type,
        name: &'a str,
        visibility: FieldVisibilityKind,
        is_packed: bool,
    ) -> Self {
        let known_type_layout = ty.layout(ctx);
        let (is_rust_union, can_copy_union_fields) =
            comp.is_rust_union(ctx, known_type_layout.as_ref(), name);
        StructLayoutTracker {
            name,
            ctx,
            comp,
            visibility,
            is_packed,
            known_type_layout,
            is_rust_union,
            can_copy_union_fields,
            latest_offset: 0,
            padding_count: 0,
            latest_field_layout: None,
            max_field_align: 0,
            last_field_was_bitfield: false,
            last_field_was_flexible_array: false,
        }
    }

    pub(crate) fn can_copy_union_fields(&self) -> bool {
        self.can_copy_union_fields
    }

    pub(crate) fn is_rust_union(&self) -> bool {
        self.is_rust_union
    }

    pub(crate) fn saw_flexible_array(&mut self) {
        self.last_field_was_flexible_array = true;
    }

    pub(crate) fn saw_vtable(&mut self) {
        debug!("saw vtable for {}", self.name);

        let ptr_size = self.ctx.target_pointer_size();
        self.latest_offset += ptr_size;
        self.latest_field_layout = Some(Layout::new(ptr_size, ptr_size));
        self.max_field_align = ptr_size;
    }

    pub(crate) fn saw_base(&mut self, base_ty: &Type) {
        debug!("saw base for {}", self.name);
        if let Some(layout) = base_ty.layout(self.ctx) {
            self.align_to_latest_field(layout);

            self.latest_offset += self.padding_bytes(layout) + layout.size;
            self.latest_field_layout = Some(layout);
            self.max_field_align = cmp::max(self.max_field_align, layout.align);
        }
    }

    pub(crate) fn saw_bitfield_unit(&mut self, layout: Layout) {
        debug!("saw bitfield unit for {}: {layout:?}", self.name);

        self.align_to_latest_field(layout);

        self.latest_offset += layout.size;

        debug!(
            "Offset: <bitfield>: {} -> {}",
            self.latest_offset - layout.size,
            self.latest_offset
        );

        self.latest_field_layout = Some(layout);
        self.last_field_was_bitfield = true;
        self.max_field_align = cmp::max(self.max_field_align, layout.align);
    }

    /// Returns a padding field if necessary for a given new field _before_
    /// adding that field.
    pub(crate) fn saw_field(
        &mut self,
        field_name: &str,
        field_ty: &Type,
        field_offset: Option<usize>,
    ) -> Option<proc_macro2::TokenStream> {
        let mut field_layout = field_ty.layout(self.ctx)?;

        if let TypeKind::Array(inner, len) =
            *field_ty.canonical_type(self.ctx).kind()
        {
            // FIXME(emilio): As an _ultra_ hack, we correct the layout returned
            // by arrays of structs that have a bigger alignment than what we
            // can support.
            //
            // This means that the structs in the array are super-unsafe to
            // access, since they won't be properly aligned, but there's not too
            // much we can do about it.
            if let Some(layout) = self.ctx.resolve_type(inner).layout(self.ctx)
            {
                if layout.align > MAX_GUARANTEED_ALIGN {
                    field_layout.size =
                        align_to(layout.size, layout.align) * len;
                    field_layout.align = MAX_GUARANTEED_ALIGN;
                }
            }
        }
        self.saw_field_with_layout(field_name, field_layout, field_offset)
    }

    pub(crate) fn saw_field_with_layout(
        &mut self,
        field_name: &str,
        field_layout: Layout,
        field_offset: Option<usize>,
    ) -> Option<proc_macro2::TokenStream> {
        let will_merge_with_bitfield = self.align_to_latest_field(field_layout);

        let is_union = self.comp.is_union();
        let padding_bytes = match field_offset {
            Some(offset) if offset / 8 > self.latest_offset => {
                offset / 8 - self.latest_offset
            }
            _ => {
                if will_merge_with_bitfield ||
                    field_layout.align == 0 ||
                    is_union
                {
                    0
                } else if !self.is_packed {
                    self.padding_bytes(field_layout)
                } else if let Some(mut l) = self.known_type_layout {
                    if field_layout.align < l.align {
                        l.align = field_layout.align;
                    }
                    self.padding_bytes(l)
                } else {
                    0
                }
            }
        };

        self.latest_offset += padding_bytes;

        let padding_layout = if self.is_packed || is_union {
            None
        } else {
            let force_padding = self.ctx.options().force_explicit_padding;

            // Otherwise the padding is useless.
            let need_padding = force_padding ||
                padding_bytes >= field_layout.align ||
                field_layout.align > MAX_GUARANTEED_ALIGN;

            debug!(
                "Offset: <padding>: {} -> {}",
                self.latest_offset - padding_bytes,
                self.latest_offset
            );

            debug!(
                "align field {field_name} to {}/{} with {padding_bytes} padding bytes {field_layout:?}",
                self.latest_offset,
                field_offset.unwrap_or(0) / 8,
            );

            let padding_align = if force_padding {
                1
            } else {
                cmp::min(field_layout.align, MAX_GUARANTEED_ALIGN)
            };

            if need_padding && padding_bytes != 0 {
                Some(Layout::new(padding_bytes, padding_align))
            } else {
                None
            }
        };

        self.latest_offset += field_layout.size;
        self.latest_field_layout = Some(field_layout);
        self.max_field_align =
            cmp::max(self.max_field_align, field_layout.align);
        self.last_field_was_bitfield = false;

        debug!(
            "Offset: {field_name}: {} -> {}",
            self.latest_offset - field_layout.size,
            self.latest_offset
        );

        padding_layout.map(|layout| self.padding_field(layout))
    }

    pub(crate) fn add_tail_padding(
        &mut self,
        comp_name: &str,
        comp_layout: Layout,
    ) -> Option<proc_macro2::TokenStream> {
        // Only emit an padding field at the end of a struct if the
        // user configures explicit padding.
        if !self.ctx.options().force_explicit_padding {
            return None;
        }

        // Padding doesn't make sense for rust unions.
        if self.is_rust_union {
            return None;
        }

        // Also doesn't make sense for structs with flexible array members
        if self.last_field_was_flexible_array {
            return None;
        }

        if self.latest_offset == comp_layout.size {
            // This struct does not contain tail padding.
            return None;
        }

        trace!(
            "need a tail padding field for {comp_name}: offset {} -> size {}",
            self.latest_offset,
            comp_layout.size
        );
        let size = comp_layout.size - self.latest_offset;
        Some(self.padding_field(Layout::new(size, 0)))
    }

    pub(crate) fn pad_struct(
        &mut self,
        layout: Layout,
    ) -> Option<proc_macro2::TokenStream> {
        debug!("pad_struct:\n\tself = {self:#?}\n\tlayout = {layout:#?}");

        if layout.size < self.latest_offset {
            warn!(
                "Calculated wrong layout for {}, too more {} bytes",
                self.name,
                self.latest_offset - layout.size
            );
            return None;
        }

        let padding_bytes = layout.size - self.latest_offset;
        if padding_bytes == 0 {
            return None;
        }

        let repr_align = true;

        // We always pad to get to the correct size if the struct is one of
        // those we can't align properly.
        //
        // Note that if the last field we saw was a bitfield, we may need to pad
        // regardless, because bitfields don't respect alignment as strictly as
        // other fields.
        if padding_bytes >= layout.align ||
            (self.last_field_was_bitfield &&
                padding_bytes >= self.latest_field_layout.unwrap().align) ||
            (!repr_align && layout.align > MAX_GUARANTEED_ALIGN)
        {
            let layout = if self.is_packed {
                Layout::new(padding_bytes, 1)
            } else if self.last_field_was_bitfield ||
                layout.align > MAX_GUARANTEED_ALIGN
            {
                // We've already given up on alignment here.
                Layout::for_size(self.ctx, padding_bytes)
            } else {
                Layout::new(padding_bytes, layout.align)
            };

            debug!("pad bytes to struct {}, {layout:?}", self.name);

            Some(self.padding_field(layout))
        } else {
            None
        }
    }

    pub(crate) fn requires_explicit_align(&self, layout: Layout) -> bool {
        let repr_align = true;

        // Always force explicit repr(align) for stuff more than 16-byte aligned
        // to work-around https://github.com/rust-lang/rust/issues/54341.
        //
        // Worst-case this just generates redundant alignment attributes.
        if repr_align && self.max_field_align >= 16 {
            return true;
        }

        if self.max_field_align >= layout.align {
            return false;
        }

        // We can only generate up-to a 8-bytes of alignment unless we support
        // repr(align).
        repr_align || layout.align <= MAX_GUARANTEED_ALIGN
    }

    fn padding_bytes(&self, layout: Layout) -> usize {
        align_to(self.latest_offset, layout.align) - self.latest_offset
    }

    fn padding_field(&mut self, layout: Layout) -> proc_macro2::TokenStream {
        let ty = helpers::blob(self.ctx, layout, false);
        let padding_count = self.padding_count;

        self.padding_count += 1;

        let padding_field_name = Ident::new(
            &format!("__bindgen_padding_{padding_count}"),
            Span::call_site(),
        );

        self.max_field_align = cmp::max(self.max_field_align, layout.align);

        let vis = super::access_specifier(self.visibility);

        quote! {
            #vis #padding_field_name : #ty ,
        }
    }

    /// Returns whether the new field is known to merge with a bitfield.
    ///
    /// This is just to avoid doing the same check also in `pad_field`.
    fn align_to_latest_field(&mut self, new_field_layout: Layout) -> bool {
        if self.is_packed {
            // Skip to align fields when packed.
            return false;
        }

        let Some(layout) = self.latest_field_layout else {
            return false;
        };

        // If it was, we may or may not need to align, depending on what the
        // current field alignment and the bitfield size and alignment are.
        debug!(
            "align_to_bitfield? {}: {layout:?} {new_field_layout:?}",
            self.last_field_was_bitfield,
        );

        // Avoid divide-by-zero errors if align is 0.
        let align = cmp::max(1, layout.align);

        if self.last_field_was_bitfield &&
            new_field_layout.align <= layout.size % align &&
            new_field_layout.size <= layout.size % align
        {
            // The new field will be coalesced into some of the remaining bits.
            //
            // FIXME(emilio): I think this may not catch everything?
            debug!("Will merge with bitfield");
            return true;
        }

        // Else, just align the obvious way.
        self.latest_offset += self.padding_bytes(layout);
        false
    }
}
