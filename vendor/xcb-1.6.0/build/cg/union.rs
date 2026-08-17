use crate::{cg, cg::util, ir};

use super::{
    AsModRsTyp, CodeGen, Expr, Field, QualifiedRsTyp, RsTyp, TypeInfo, UnionTypeField,
    UnionVariant, UnionVariantContent, WireSz,
};

use std::io::{self, Write};

impl CodeGen {
    pub(super) fn resolve_union(
        &mut self,
        typ: &str,
        fields: &[ir::Field],
        _doc: &Option<ir::Doc>,
    ) {
        let rs_typ = cg::rust_type_name(typ);

        let has_type_field = fields
            .iter()
            .any(|f| matches!(f, ir::Field::Field{name, ..} if name == "type"));

        let (variants, wire_sz, type_field) = if has_type_field {
            self.resolve_type_field_union_variants(&rs_typ, fields)
        } else {
            self.resolve_union_variants(&rs_typ, fields)
        };

        let typ_info = TypeInfo::Union {
            module: None,
            rs_typ,
            variants,
            wire_sz,
            type_field,
            impl_clone: true,
            emit: true,
        };
        self.register_typ(typ.to_string(), typ_info);
    }

    fn resolve_union_variants(
        &mut self,
        _union_rs_typ: &str,
        fields: &[ir::Field],
    ) -> (Vec<UnionVariant>, Expr, Option<UnionTypeField>) {
        let mut vec = Vec::new();
        let mut wire_sz = Expr::Value(0usize);

        for f in fields {
            match f {
                ir::Field::Field { name, typ, .. } => {
                    let (module, typ) = util::extract_module(typ);
                    let typinfo = self.find_typinfo(module, typ);
                    let rs_typ = typinfo.rs_typ();
                    let f_wire_sz = typinfo.wire_sz();

                    let content = UnionVariantContent::RsTyp(rs_typ.to_string());

                    if let (Expr::Value(wire_sz), Expr::Value(f_wire_sz)) =
                        (&mut wire_sz, &f_wire_sz)
                    {
                        *wire_sz = (*wire_sz).max(*f_wire_sz);
                    }

                    let variant = util::tit_cap(name);

                    vec.push(UnionVariant {
                        variant,
                        module: module.map(str::to_owned),
                        content,
                    });
                }
                ir::Field::List {
                    name,
                    typ,
                    len_expr: ir::Expr::Value(sz),
                    ..
                } => {
                    let variant = util::tit_cap(name);
                    let (module, typ) = util::extract_module(typ);
                    let typinfo = self.find_typinfo(module, typ);
                    let rs_typ = typinfo.rs_typ();
                    let f_wire_sz = typinfo.wire_sz();
                    let f_wire_sz = Expr::Op(
                        "*".to_string(),
                        Box::new(Expr::Value(*sz)),
                        Box::new(f_wire_sz),
                    );
                    let f_wire_sz = f_wire_sz.reduce();

                    if let (Expr::Value(wire_sz), Expr::Value(f_wire_sz)) =
                        (&mut wire_sz, &f_wire_sz)
                    {
                        *wire_sz = (*wire_sz).max(*f_wire_sz);
                    }

                    vec.push(UnionVariant {
                        variant,
                        module: module.map(str::to_owned),
                        content: UnionVariantContent::Array(rs_typ.to_string(), *sz),
                    });
                }
                ir::Field::Pad(..) => unreachable!("pad in union??"),
                f => unreachable!("{:#?}", f),
            }
        }
        (vec, wire_sz, None)
    }

    fn resolve_type_field_union_variants(
        &mut self,
        union_rs_typ: &str,
        fields: &[ir::Field],
    ) -> (Vec<UnionVariant>, Expr, Option<UnionTypeField>) {
        let mut vec = Vec::new();
        let mut wire_sz = Expr::Value(0usize);
        let mut type_field = None;

        for f in fields {
            match f {
                ir::Field::Field {
                    name, typ, r#enum, ..
                } => {
                    if name == "type" {
                        assert_eq!(typ.as_str(), "CARD8");
                        let (enu_module, enu_typ) = r#enum
                            .as_ref()
                            .map(|typ| {
                                let (module, typ) = util::extract_module(typ);
                                (module.map(str::to_owned), typ.to_string())
                            })
                            .unwrap_or_else(|| (None, union_rs_typ.to_string() + "Type"));
                        type_field = Some(UnionTypeField {
                            offset: 0,
                            enu_module,
                            enu_typ,
                        });
                        continue;
                    }

                    let (module, typ) = util::extract_module(typ);
                    if name == "common" {
                        if module.is_none() {
                            self.typ_need_add(typ, -1);
                        }
                        continue;
                    }
                    let typinfo = self.find_typinfo(module, typ);
                    let rs_typ = typinfo.rs_typ();
                    let f_wire_sz = typinfo.wire_sz();

                    let final_typinfo = self.find_typinfo_recurse(module, typ);

                    let content = match final_typinfo {
                        TypeInfo::Struct { fields, .. } => {
                            UnionVariantContent::Struct(fields.clone())
                        }
                        _ => UnionVariantContent::RsTyp(rs_typ.to_string()),
                    };

                    if let (Expr::Value(wire_sz), Expr::Value(f_wire_sz)) =
                        (&mut wire_sz, &f_wire_sz)
                    {
                        *wire_sz = (*wire_sz).max(*f_wire_sz);
                    }

                    let variant = util::tit_cap(name);

                    vec.push(UnionVariant {
                        variant,
                        module: module.map(str::to_owned),
                        content,
                    });

                    if module.is_none() {
                        self.typ_need_add(typ, -1);
                    }
                }
                ir::Field::Pad(..) => unreachable!("pad in union??"),
                _ => unreachable!(),
            }
        }
        (vec, wire_sz, type_field)
    }

    pub(super) fn emit_union<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        variants: &[UnionVariant],
        wire_sz: &Expr,
        type_field: &Option<UnionTypeField>,
        impl_clone: bool,
    ) -> io::Result<()> {
        writeln!(out)?;
        writeln!(
            out,
            "#[derive({}Debug)]",
            if impl_clone { "Clone, " } else { "" }
        )?;
        writeln!(out, "pub enum {} {{", rs_typ)?;
        for v in variants {
            match &v.content {
                UnionVariantContent::RsTyp(rs_typ) => {
                    writeln!(out, "    {}({}),", v.variant, rs_typ)?;
                }
                UnionVariantContent::Array(rs_typ, sz) => {
                    writeln!(out, "    {}([{}; {}]),", v.variant, rs_typ, sz)?;
                }
                UnionVariantContent::Struct(fields) => {
                    if fields.is_empty() {
                        writeln!(out, "    {},", v.variant)?;
                    } else {
                        writeln!(out, "    {}{{", v.variant)?;
                        for f in fields {
                            if is_type_field(f) {
                                continue;
                            }
                            match f {
                                Field::Field {
                                    name,
                                    module,
                                    rs_typ,
                                    ..
                                } => {
                                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                                    writeln!(out, "        {}: {},", name, q_rs_typ)?;
                                }
                                Field::List {
                                    name,
                                    module,
                                    rs_typ,
                                    len_expr: Expr::Value(sz),
                                    ..
                                } => {
                                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                                    writeln!(out, "        {}: [{}; {}],", name, q_rs_typ, sz)?;
                                }
                                _ => {}
                            }
                        }
                        writeln!(out, "    }},")?;
                    }
                }
            }
        }
        writeln!(out, "}}")?;

        if let Some(type_field) = type_field {
            self.emit_union_ctor_type_field(out, rs_typ, variants, type_field)?;
        }

        let wire_sz = match wire_sz {
            Expr::Value(wire_sz) => wire_sz,
            _ => unreachable!("{:#?}", wire_sz),
        };

        writeln!(out)?;
        writeln!(out, "impl base::WiredOut for {} {{", rs_typ)?;
        writeln!(out, "    fn wire_len(&self) -> usize {{ {} }}", wire_sz)?;
        writeln!(out)?;
        writeln!(
            out,
            "    fn serialize(&self, wire_buf: &mut [u8]) -> usize {{"
        )?;
        writeln!(out, "        match self {{")?;
        for uv in variants {
            match &uv.content {
                UnionVariantContent::RsTyp(_rs_typ) => {
                    assert!(type_field.is_none());
                    let name = cg::rust_field_name(&uv.variant);
                    writeln!(
                        out,
                        "{}{}::{}({}) => {{",
                        cg::ind(3),
                        rs_typ,
                        uv.variant,
                        name
                    )?;
                    writeln!(out, "{}{}.serialize(wire_buf);", cg::ind(4), name)?;
                    writeln!(out, "{}}}", cg::ind(3))?;
                }
                UnionVariantContent::Array(_rs_typ, _sz) => {
                    assert!(type_field.is_none());
                    let name = cg::rust_field_name(&uv.variant);
                    writeln!(
                        out,
                        "{}{}::{}({}) => {{",
                        cg::ind(3),
                        rs_typ,
                        uv.variant,
                        name
                    )?;
                    writeln!(
                        out,
                        "{}let me = unsafe {{ std::slice::from_raw_parts(",
                        cg::ind(4)
                    )?;
                    writeln!(
                        out,
                        "{}{}.as_ptr() as *const u8, {}",
                        cg::ind(5),
                        name,
                        wire_sz
                    )?;
                    writeln!(out, "{})}};", cg::ind(4))?;
                    writeln!(
                        out,
                        "{}let wire_buf = &mut wire_buf[..{}];",
                        cg::ind(4),
                        wire_sz
                    )?;
                    writeln!(out, "{}wire_buf.copy_from_slice(me);", cg::ind(4))?;
                    writeln!(out, "{}}}", cg::ind(3))?;
                }
                UnionVariantContent::Struct(fields) => {
                    writeln!(out, "{}{}::{}{{", cg::ind(3), rs_typ, uv.variant)?;
                    for f in fields {
                        if is_type_field(f) {
                            continue;
                        }
                        match f {
                            Field::Field { name, .. } | Field::List { name, .. } => {
                                writeln!(out, "{}{},", cg::ind(4), name)?;
                            }
                            _ => {}
                        }
                    }
                    writeln!(out, "{}}} => {{", cg::ind(3))?;
                    for f in fields.iter() {
                        if is_type_field(f) {
                            let type_field = type_field.as_ref().unwrap();
                            let enum_typ = self.find_typinfo(
                                type_field.enu_module.as_deref(),
                                &type_field.enu_typ,
                            );
                            let enum_expr = enum_typ.qualified_rs_typ() + "::" + &uv.variant;
                            writeln!(
                                out,
                                "{}wire_buf[0] += unsafe {{ std::mem::transmute::<_, u32>({}) }} as u8;",
                                cg::ind(4),
                                enum_expr,
                            )?;
                            continue;
                        }
                        match f {
                            Field::Field {
                                name,
                                wire_off: Expr::Value(off),
                                ..
                            } => {
                                writeln!(
                                    out,
                                    "{}{}.serialize(&mut wire_buf[{}..]);",
                                    cg::ind(4),
                                    name,
                                    off
                                )?;
                            }
                            Field::List {
                                name,
                                wire_off: Expr::Value(off),
                                wire_sz: Expr::Value(sz),
                                ..
                            } => {
                                writeln!(
                                    out,
                                    "{}wire_buf[{}..{}].copy_from_slice({});",
                                    cg::ind(4),
                                    off,
                                    *off + *sz,
                                    name,
                                )?;
                            }
                            Field::Pad { .. } => {}
                            _ => unreachable!(),
                        }
                    }
                    writeln!(out, "{}}}", cg::ind(3))?;
                }
            }
        }
        writeln!(out, "        }}")?;
        writeln!(out, "        {}", wire_sz)?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        if type_field.is_some() {
            writeln!(out)?;
            writeln!(out, "impl base::WiredIn for {} {{", rs_typ)?;
            writeln!(out, "    type Params = ();")?;
            writeln!(out)?;
            writeln!(
            out,
            "    unsafe fn compute_wire_len(_ptr: *const u8, _params: Self::Params) -> usize {{ {} }}", wire_sz
        )?;
            writeln!(out, "    unsafe fn unserialize(ptr: *const u8, _params: (), offset: &mut usize) -> Self {{")?;
            writeln!(out, "        let sz = Self::compute_wire_len(ptr, ());")?;
            writeln!(out, "        *offset += sz;")?;
            writeln!(
                out,
                "        Self::from_data(std::slice::from_raw_parts(ptr, sz))"
            )?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }

        Ok(())
    }

    fn emit_union_ctor_type_field<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        variants: &[UnionVariant],
        type_field: &UnionTypeField,
    ) -> io::Result<()> {
        let UnionTypeField {
            enu_module,
            enu_typ,
            offset,
        } = type_field;

        let (items, _) = {
            let mod_rs_typ = (enu_module, enu_typ);
            let mod_rs_typ = mod_rs_typ.as_mod_rs_typ();
            let typ_info = self.find_typinfo_recurse(mod_rs_typ.0, mod_rs_typ.1);

            if let TypeInfo::Enum { items, .. } = typ_info {
                (
                    //map_type_field_enum_items(&self.xcb_mod, rs_typ, items),
                    items,
                    typ_info.qualified_rs_typ(),
                )
            } else {
                unreachable!()
            }
        };

        writeln!(out)?;
        writeln!(out, "impl {} {{", rs_typ)?;
        writeln!(
            out,
            "    unsafe fn from_data(wire_data: &[u8]) -> {} {{",
            rs_typ
        )?;
        writeln!(out, "        let r#type = wire_data[{}] as u32;", offset)?;
        writeln!(out, "        match r#type {{")?;
        for item in items {
            let variant = variants.iter().find(|v| v.variant == item.0).unwrap();
            match &variant.content {
                UnionVariantContent::RsTyp(..) => unreachable!(),
                UnionVariantContent::Array(..) => unreachable!(),
                UnionVariantContent::Struct(fields) => {
                    if fields.is_empty() {
                        writeln!(
                            out,
                            "            {} => {}::{},",
                            item.1, rs_typ, variant.variant
                        )?;
                    } else {
                        writeln!(
                            out,
                            "            {} => {}::{}{{",
                            item.1, rs_typ, variant.variant
                        )?;

                        for f in fields {
                            if is_type_field(f) {
                                continue;
                            }
                            match f {
                                Field::Field {
                                    name,
                                    wire_off: Expr::Value(off),
                                    module,
                                    rs_typ,
                                    ..
                                } => {
                                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                                    writeln!(
                                        out,
                                        "                {}: *(wire_data.as_ptr().add({}) as *const {}),",
                                        name,
                                        off,
                                        q_rs_typ,
                                    )?;
                                }
                                Field::List {
                                    name,
                                    wire_off: Expr::Value(off),
                                    wire_sz: Expr::Value(sz),
                                    rs_typ,
                                    ..
                                } if rs_typ == "u8" => {
                                    writeln!(
                                        out,
                                        "{}{}: wire_data[{}..{}].try_into().unwrap(),",
                                        cg::ind(4),
                                        name,
                                        *off,
                                        *off + *sz
                                    )?;
                                }
                                Field::Pad { .. } => {}
                                _ => unreachable!(),
                            }
                        }
                        writeln!(out, "            }},")?;
                    }
                }
            }
        }
        writeln!(
            out,
            "            _ => unreachable!(\"unexpected type value for {}::{}\"),",
            self.xcb_mod, rs_typ
        )?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }
}

fn is_type_field(f: &Field) -> bool {
    match f {
        Field::Field { name, .. } => name == "r#type",
        _ => false,
    }
}
