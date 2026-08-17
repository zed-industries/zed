use super::{CodeGen, QualifiedRsTyp, RsTyp, TypeInfo, UnionVariantContent};
use crate::cg;

use std::io::{self, Write};

use super::{util, UnionVariant};

impl CodeGen {
    pub(super) fn resolve_xid(&mut self, typ: &str) {
        let rs_typ = match (self.xcb_mod.as_str(), typ) {
            ("present", "EVENT") => "EventXid".to_string(),
            (_, typ) => cg::rust_type_name(typ),
        };
        let info = TypeInfo::Xid {
            module: None,
            rs_typ,
        };
        self.register_typ(typ.to_string(), info);
    }

    pub(super) fn resolve_xidunion(&mut self, typ: &str, xidtypes: &[String]) {
        let rs_typ = cg::rust_type_name(typ);
        let variants: Vec<_> = xidtypes
            .iter()
            .map(|typ| {
                let (module, typ) = self.extract_module(typ);
                let rs_typ = self.find_typinfo(module, typ).rs_typ().to_string();

                let variant = match module {
                    Some(module) => util::tit_cap(module) + &rs_typ,
                    None => rs_typ.clone(),
                };
                UnionVariant {
                    variant,
                    module: module.map(str::to_owned),
                    content: UnionVariantContent::RsTyp(rs_typ),
                }
            })
            .collect();

        let info = TypeInfo::XidUnion {
            module: None,
            rs_typ,
            variants,
        };
        self.register_typ(typ.to_string(), info);
    }

    pub(super) fn emit_xid<O: Write>(&self, out: &mut O, rs_typ: &str) -> io::Result<()> {
        let dbg = if self.dbg_atom_names && self.xcb_mod == "xproto" && rs_typ == "Atom" {
            ""
        } else {
            "Debug, "
        };

        writeln!(out)?;
        writeln!(
            out,
            "#[derive(Copy, Clone, {}PartialEq, Eq, Hash, PartialOrd, Ord)]",
            dbg
        )?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "pub struct {} {{", rs_typ)?;
        writeln!(out, "    res_id: u32,")?;
        writeln!(out, "}}")?;
        writeln!(out)?;
        writeln!(out, "impl base::Xid for {} {{", rs_typ)?;
        writeln!(
            out,
            "    fn none() -> Self {{ {} {{ res_id: 0 }} }}",
            rs_typ
        )?;
        writeln!(out, "    fn resource_id(&self) -> u32 {{ self.res_id }}")?;
        writeln!(out, "}}")?;
        writeln!(out)?;
        writeln!(out, "impl base::XidNew for {} {{", rs_typ)?;
        writeln!(
            out,
            "    unsafe fn new(res_id: u32) -> Self {{ {} {{ res_id }} }}",
            rs_typ
        )?;
        writeln!(out, "}}")?;

        self.emit_sizeof_test(out, rs_typ, 4)?;

        Ok(())
    }

    pub(super) fn emit_xidunion<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        variants: &[UnionVariant],
    ) -> io::Result<()> {
        let has_unknown = matches!(
            (self.xcb_mod.as_str(), rs_typ),
            ("xproto", "Drawable") | ("glx", "Drawable")
        );

        writeln!(out)?;
        writeln!(out, "#[derive(Copy, Clone, Debug)]")?;
        writeln!(out, "pub enum {} {{", rs_typ)?;
        writeln!(out, "    None,")?;
        if has_unknown {
            writeln!(out, "    /// Whether the drawable is a `Window` or a `Pixmap` is only known to the user context")?;
            writeln!(out, "    Unknown(u32),")?;
        }
        for v in variants {
            if let UnionVariantContent::RsTyp(rs_typ) = &v.content {
                let mod_rs_typ = (&v.module, rs_typ);
                writeln!(out, "    {}({}),", v.variant, mod_rs_typ.qualified_rs_typ())?;
            } else {
                unreachable!();
            }
        }
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl base::Xid for {} {{", rs_typ)?;
        writeln!(out, "    fn none() -> Self {{ {}::None }}", rs_typ)?;
        writeln!(out)?;
        writeln!(out, "    fn resource_id(&self) -> u32 {{")?;
        writeln!(out, "        match self {{")?;
        writeln!(out, "            {}::None => 0,", rs_typ)?;
        if has_unknown {
            writeln!(out, "{}Drawable::Unknown(id) => *id,", cg::ind(3))?;
        }
        for v in variants {
            writeln!(
                out,
                "            {}::{}(xid) => xid.resource_id(),",
                rs_typ, v.variant
            )?;
        }
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl PartialEq for {} {{", rs_typ)?;
        writeln!(out, "    fn eq(&self, rhs: &{}) -> bool {{", rs_typ)?;
        writeln!(out, "        self.resource_id() == rhs.resource_id()")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out)?;
        writeln!(out, "impl Eq for {} {{}}", rs_typ)?;
        writeln!(out)?;
        writeln!(out, "impl Hash for {} {{", rs_typ)?;
        writeln!(out, "    fn hash<H: Hasher>(&self, state: &mut H) {{")?;
        writeln!(out, "        self.resource_id().hash(state);")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        for v in variants {
            let variant = if v.variant == "XprotoWindow" {
                "xproto::Window"
            } else {
                &v.variant
            };

            writeln!(out)?;
            writeln!(out, "impl PartialEq<{}> for {} {{", variant, rs_typ)?;
            writeln!(out, "    fn eq(&self, rhs: &{}) -> bool {{", variant)?;
            writeln!(out, "        self.resource_id() == rhs.resource_id()")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
            writeln!(out)?;
            writeln!(out, "impl PartialEq<{}> for {} {{", rs_typ, variant)?;
            writeln!(out, "    fn eq(&self, rhs: &{}) -> bool {{", rs_typ)?;
            writeln!(out, "        self.resource_id() == rhs.resource_id()")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }
        Ok(())
    }
}
