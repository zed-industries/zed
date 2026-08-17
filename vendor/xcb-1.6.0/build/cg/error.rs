use super::r#struct::{make_field, ResolvedFields};
use super::{CodeGen, Error};
use crate::cg::{self};
use crate::ir;

use std::io::{self, Write};

impl CodeGen {
    pub(super) fn resolve_error(
        &mut self,
        name: String,
        number: i32,
        fields: Vec<ir::Field>,
        _doc: Option<ir::Doc>,
    ) {
        let fields = {
            let mut ff = vec![
                make_field("response_type".into(), "CARD8".into()),
                make_field("error_code".into(), "CARD8".into()),
                make_field("sequence".into(), "CARD16".into()),
            ];
            for f in fields.into_iter() {
                ff.push(f);
            }
            ff
        };

        let variant = cg::rust_type_name(&name);
        let rs_typ = variant.clone() + "Error";

        let ResolvedFields {
            fields, wire_sz, ..
        } = self.resolv_struct_fields(&rs_typ, "", &fields, None);

        self.rs_typs_need_count.insert(rs_typ.clone(), 1);

        self.errors.push(Error {
            rs_typ,
            copy_from_rs_typ: None,
            variant,
            number,
            fields,
            wire_sz,
        });
    }

    pub(super) fn resolve_errorcopy(&mut self, name: String, number: i32, r#ref: String) {
        let variant = cg::rust_type_name(&name);
        let rs_typ = variant.clone() + "Error";
        let (ref_module, ref_variant) = self.extract_module(&r#ref);
        let ref_variant = cg::rust_variant_name(ref_variant);

        let mut implicit_module = None;

        let error = match &ref_module {
            Some(module) => {
                let di = self
                    .depinfo
                    .iter()
                    .find(|di| di.xcb_mod == *module)
                    .unwrap_or_else(|| panic!("could not find {} dependency", module));
                di.errors
                    .iter()
                    .find(|e| e.variant == ref_variant)
                    .unwrap_or_else(|| panic!("could not find error {}::{}", module, ref_variant))
            }
            None => self
                .errors
                .iter()
                .find(|e| e.variant == ref_variant)
                .or_else(|| {
                    for di in &self.depinfo {
                        for err in &di.errors {
                            if err.variant == ref_variant {
                                implicit_module = Some(di.xcb_mod.clone());
                                return Some(err);
                            }
                        }
                    }
                    None
                })
                .unwrap_or_else(|| {
                    panic!(
                        "{}: cannot find error {} referenced by {}",
                        self.xcb_mod, r#ref, name
                    )
                }),
        }
        .clone();

        self.errors.push(Error {
            rs_typ,
            variant,
            number,
            copy_from_rs_typ: Some(error.rs_typ),
            fields: error.fields,
            wire_sz: error.wire_sz,
        });
    }

    pub(crate) fn emit_errors<O: Write>(&self, out: &mut O) -> io::Result<()> {
        if self.errors.is_empty() {
            return Ok(());
        }

        for error in &self.errors {
            if let Some(copy_from_rs_typ) = &error.copy_from_rs_typ {
                let prefix = if copy_from_rs_typ == "ValueError" && self.xcb_mod != "xproto" {
                    "xproto::"
                } else {
                    ""
                };
                writeln!(out)?;
                writeln!(out, "/// The `{}` error.", error.rs_typ)?;
                writeln!(
                    out,
                    "pub type {} = {}{};",
                    error.rs_typ, prefix, copy_from_rs_typ
                )?;
                continue;
            }

            // Event are struct holding a pointer.
            // They own the data pointed to that must be freed during drop.

            writeln!(out)?;
            writeln!(out, "/// The `{}` error.", error.rs_typ)?;
            writeln!(out, "pub struct {} {{", error.rs_typ)?;
            writeln!(out, "    raw: *mut xcb_generic_error_t,")?;
            writeln!(out, "}}")?;

            writeln!(out)?;
            writeln!(
                out,
                "impl base::Raw<xcb_generic_error_t> for {} {{",
                error.rs_typ
            )?;
            writeln!(
                out,
                "    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self {{ {} {{ raw }} }}",
                error.rs_typ
            )?;
            writeln!(out)?;
            writeln!(out, "    fn as_raw(&self) -> *mut xcb_generic_error_t {{")?;
            writeln!(out, "        self.raw")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;

            writeln!(out)?;
            writeln!(out, "impl base::BaseError for {} {{", error.rs_typ)?;
            if let Some(ext_info) = self.ext_info.as_ref() {
                writeln!(
                    out,
                    "    const EXTENSION: std::option::Option<ext::Extension> = Some(ext::Extension::{});",
                    ext_info.rs_name
                )?;
            } else {
                writeln!(
                    out,
                    "    const EXTENSION: std::option::Option<ext::Extension> = None;"
                )?;
            }
            writeln!(out)?;
            if error.number >= 0 {
                writeln!(out, "    const NUMBER: u32 = {};", error.number)?;
            } else {
                writeln!(out, "    const NUMBER: u32 = u32::MAX;")?;
            }
            writeln!(out, "}}")?;

            writeln!(out)?;
            writeln!(out, "impl {} {{", error.rs_typ)?;
            writeln!(
                out,
                "    fn wire_ptr(&self) -> *const u8 {{ self.raw as *const u8 }}"
            )?;
            writeln!(out)?;
            writeln!(out, "    fn wire_len(&self) -> usize {{ 32 }}")?;
            self.emit_struct_accessors(out, &error.rs_typ, &error.fields)?;
            writeln!(out, "}}")?;

            self.emit_debug_impl(out, &error.rs_typ, &error.fields)?;

            writeln!(out)?;
            writeln!(out, "impl Drop for {} {{", error.rs_typ)?;
            writeln!(out, "    fn drop(&mut self) {{")?;
            writeln!(out, "        unsafe {{ libc::free(self.raw as *mut _); }}")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;

            writeln!(out)?;
            writeln!(out, "unsafe impl Send for {} {{}}", error.rs_typ)?;
            writeln!(out, "unsafe impl Sync for {} {{}}", error.rs_typ)?;
        }

        writeln!(out)?;
        if let Some(ext_info) = &self.ext_info {
            writeln!(
                out,
                "/// Unified error type for the {} extension",
                ext_info.rs_name
            )?;
        } else {
            writeln!(out, "/// Unified error type for the X core protocol")?;
        }
        writeln!(out, "#[derive(Debug)]")?;
        writeln!(out, "pub enum Error {{")?;
        for error in &self.errors {
            writeln!(out, "    {}({}),", error.variant, error.rs_typ)?;
        }
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl Error {{")?;
        writeln!(out, "  pub fn as_raw(&self) -> *mut xcb_generic_error_t {{")?;
        writeln!(out, "    match self {{")?;
        for error in &self.errors {
            writeln!(out, "      Self::{}(e) => e.as_raw(),", error.variant)?;
        }
        writeln!(out, "    }}")?;
        writeln!(out, "  }}")?;
        writeln!(out, "}}")?;

        self.emit_resolve_wire_error(out)?;

        Ok(())
    }

    fn emit_resolve_wire_error<O: Write>(&self, out: &mut O) -> io::Result<()> {
        writeln!(out)?;
        writeln!(out, "impl base::ResolveWireError for Error {{")?;
        writeln!(out, "{}unsafe fn resolve_wire_error(first_error: u8, raw: *mut xcb_generic_error_t) -> std::option::Option<Self> {{", cg::ind(1))?;
        writeln!(out, "{}debug_assert!(!raw.is_null());", cg::ind(2))?;
        writeln!(out, "{}let error_code = (*raw).error_code;", cg::ind(2))?;
        writeln!(out, "{}match error_code - first_error {{", cg::ind(2))?;
        for error in &self.errors {
            if error.number < 0 {
                continue;
            }
            writeln!(
                out,
                "{}{} => std::option::Option::Some(Error::{}({}::from_raw(raw))),",
                cg::ind(3),
                error.number,
                error.variant,
                error.rs_typ
            )?;
        }
        writeln!(out, "{}_ => std::option::Option::None,", cg::ind(3))?;
        writeln!(out, "{}}}", cg::ind(2))?;
        writeln!(out, "{}}}", cg::ind(1))?;
        writeln!(out, "}}")?;

        Ok(())
    }
}
