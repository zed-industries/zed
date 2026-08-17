use super::r#enum::{self};
use super::r#struct::{ParamsStruct, ResolvedFields};
use super::{CodeGen, Expr, Field, SwitchCase, TypeInfo};
use crate::cg::doc::Doc;
use crate::cg::r#struct::enum_mask_qualified_rs_typ;
use crate::cg::request::{fieldref_get_value, request_fieldref_emitted};
use crate::cg::{self, util, QualifiedRsTyp, StructStyle};
use crate::ir;

use std::io::{self, Write};

impl CodeGen {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn resolve_switch(
        &mut self,
        parent_rs_typ: &str,
        parent_switch: &str,
        name: &str,
        expr: &Expr,
        wire_off: &Expr,
        cases: &[ir::SwitchCase],
        prev_fields: &mut [Field],
        need_compute_offset: bool,
        doc: Option<&Doc>,
    ) -> Field {
        // switch types are made-up, so we create the typ (same as intended rs_typ)
        let typ = parent_rs_typ.to_string() + &util::tit_cap(name);
        let mut switch_rs_typ = typ.clone();

        let mut module = None;

        let mut emit = true;
        for ex in &self.switch_exceptions {
            if ex.module == self.xcb_mod && ex.rs_typ == switch_rs_typ {
                switch_rs_typ = ex.new_rs_typ.to_string();
                module = ex.new_module.map(str::to_owned);
                emit = ex.emit;
                break;
            }
        }

        let doc = self.doc_lookup_field(doc, name);

        let is_mask = {
            let check1 = cases.iter().all(|sc| sc.bit);
            let check2 = cases.iter().any(|sc| sc.bit);
            assert_eq!(check1, check2, "switch cannot mix case and bitcase");
            check1
        };

        let name = util::to_snake_case(name);

        let switch_name = if parent_switch.is_empty() {
            name.clone()
        } else {
            format!("{}_{}", parent_switch, name)
        };

        let mut param_fields = Vec::new();

        let cases: Vec<SwitchCase> = cases
            .iter()
            .map(|sc| {
                let ir::SwitchCase {
                    name,
                    fields,
                    exprs,
                    ..
                } = sc;
                let name = name
                    .as_ref()
                    .map(|n| cg::rust_type_name(n))
                    .unwrap_or_else(|| match &exprs[0] {
                        ir::Expr::EnumRef { item, .. } => r#enum::map_enum_item_name(item),
                        _ => unreachable!("switch expressions not enum ref"),
                    });

                let exprs = exprs
                    .iter()
                    .map(|e| self.resolve_expr(e))
                    .collect::<Vec<_>>();
                let ResolvedFields {
                    fields,
                    unresolved_fieldrefs: mut uf,
                    ..
                } = self.resolv_struct_fields(&switch_rs_typ, &switch_name, fields, None);
                param_fields.append(&mut uf);
                SwitchCase {
                    name,
                    fields,
                    exprs,
                }
            })
            .collect();

        let expr_fields = expr.fieldrefs();
        assert!(
            !expr_fields.is_empty(),
            "switch {}::{} has no fieldref expr: {:?}",
            parent_rs_typ,
            name,
            expr,
        );

        let maskenum = {
            let (r#enum, mask) = prev_fields
                .iter()
                .find_map(|f| match f {
                    Field::Field {
                        name, r#enum, mask, ..
                    } if name == expr_fields[0] => Some((r#enum, mask)),
                    _ => None,
                })
                .unwrap_or_else(|| {
                    panic!(
                        "cannot find field {} for switch {}::{}",
                        expr_fields[0], parent_rs_typ, name
                    )
                });

            assert!(r#enum.is_some() && !is_mask || mask.is_some() && is_mask);

            let maskenum = r#enum.as_ref().unwrap_or_else(|| mask.as_ref().unwrap());
            maskenum.clone()
        };

        // {
        //     let enumtyp = self.typinfos.get_mut(&maskenum.1);
        //     if let Some(TypeInfo::Bitflags { ref mut rs_typ, .. }) = enumtyp {
        //         switch_rs_typ = rs_typ.clone() + "Mask";
        //         mem::swap(&mut switch_rs_typ, rs_typ);
        //     }
        // }

        param_fields.sort();
        param_fields.dedup();

        let params = expr_fields
            .into_iter()
            .map(str::to_owned)
            .chain(param_fields.into_iter())
            .collect();

        let params_struct = ParamsStruct {
            rs_typ: switch_rs_typ.clone() + "Params",
            params,
        };

        let typinfo = TypeInfo::Switch {
            module: module.clone(),
            rs_typ: switch_rs_typ.clone(),
            wire_sz: Expr::Unknown("switch len".into()), // FIXME switch size expr (may be a method?)
            maskenum,
            params_struct: params_struct.clone(),
            expr: expr.clone(),
            cases,
            is_mask,
            emit,
        };
        let field = Field::Switch {
            name,
            module,
            rs_typ: switch_rs_typ,
            wire_sz: Expr::Unknown("switch len".to_string()),
            wire_off: wire_off.clone(),
            params_struct,
            expr: expr.clone(),
            is_mask,
            need_compute_offset,
            doc,
        };

        self.register_typ(typ, typinfo);

        field
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn emit_switch<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        expr: &Expr,
        cases: &[SwitchCase],
        maskenum: &(Option<String>, String),
        params_struct: &ParamsStruct,
        is_mask: bool,
    ) -> io::Result<()> {
        params_struct.emit(out)?;

        writeln!(out)?;
        // implementing Eq and Ord only for xproto switches which are simple enough
        if is_mask && self.xcb_mod == "xproto" {
            writeln!(out, "#[derive(Clone, Debug, PartialEq, Eq)]")?;
        } else {
            writeln!(out, "#[derive(Clone, Debug)]")?;
        }
        writeln!(out, "pub enum {} {{", rs_typ)?;
        for c in cases {
            if visible_fields_len(&c.fields) == 1 {
                let is_input_info_info = rs_typ == "InputInfoInfo";

                assert!(!c.fields.is_empty());

                match &c.fields[0] {
                    Field::Field {
                        r#enum: Some(r#enum),
                        ..
                    } => {
                        let q_rs_typ = (&r#enum.0, &r#enum.1).qualified_rs_typ();
                        writeln!(out, "    {}({}),", c.name, q_rs_typ)?;
                    }
                    Field::Field {
                        mask: Some(mask), ..
                    } => {
                        let q_rs_typ = (&mask.0, &mask.1).qualified_rs_typ();
                        writeln!(out, "    {}({}),", c.name, q_rs_typ)?;
                    }
                    Field::Field { module, rs_typ, .. } => {
                        let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                        if is_input_info_info && c.name == "Button" {
                            writeln!(out, "    /// The value is the number of buttons")?;
                        }
                        writeln!(out, "    {}({}),", c.name, q_rs_typ)?;
                    }
                    Field::List { rs_typ, .. } if rs_typ == "char" => {
                        writeln!(out, "    {}(Lat1String),", c.name)?;
                    }
                    Field::List {
                        r#enum: Some(r#enum),
                        ..
                    } => {
                        let q_rs_typ = (&r#enum.0, &r#enum.1).qualified_rs_typ();
                        writeln!(out, "    {}(Vec<{}>),", c.name, q_rs_typ)?;
                    }
                    Field::List {
                        mask: Some(mask), ..
                    } => {
                        let q_rs_typ = (&mask.0, &mask.1).qualified_rs_typ();
                        writeln!(out, "    {}(Vec<{}>),", c.name, q_rs_typ)?;
                    }
                    Field::List {
                        module,
                        rs_typ,
                        struct_style,
                        ..
                    } => {
                        let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                        let buf = if matches!(struct_style, Some(StructStyle::DynBuf)) {
                            "Buf"
                        } else {
                            ""
                        };
                        writeln!(out, "    {}(Vec<{}{}>),", c.name, q_rs_typ, buf)?;
                    }
                    _ => unreachable!(),
                };
            } else {
                writeln!(out, "    {}{{", c.name)?;
                for f in &c.fields {
                    match f {
                        Field::Field {
                            name,
                            r#enum: Some(r#enum),
                            ..
                        } => {
                            let q_rs_typ = (&r#enum.0, &r#enum.1).qualified_rs_typ();
                            writeln!(out, "        {}: {},", name, q_rs_typ)?;
                        }
                        Field::Field {
                            name,
                            mask: Some(mask),
                            is_fieldref,
                            ..
                        } => {
                            if !*is_fieldref || request_fieldref_emitted(name, &c.fields, false) {
                                let q_rs_typ = (&mask.0, &mask.1).qualified_rs_typ();
                                writeln!(out, "        {}: {},", name, q_rs_typ)?;
                            }
                        }
                        Field::Field {
                            module,
                            name,
                            rs_typ,
                            struct_style,
                            is_fieldref,
                            ..
                        } => {
                            if !*is_fieldref || request_fieldref_emitted(name, &c.fields, false) {
                                let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                                let buf = if matches!(struct_style, Some(StructStyle::DynBuf)) {
                                    "Buf"
                                } else {
                                    ""
                                };
                                writeln!(out, "        {}: {}{},", name, q_rs_typ, buf)?;
                            }
                        }
                        Field::List { name, rs_typ, .. } if rs_typ == "char" => {
                            writeln!(out, "        {}: Lat1String,", name)?;
                        }
                        Field::List {
                            name,
                            r#enum: Some(r#enum),
                            ..
                        } => {
                            let q_rs_typ = (&r#enum.0, &r#enum.1).qualified_rs_typ();
                            writeln!(out, "{}{}: Vec<{}>,", cg::ind(2), name, q_rs_typ)?;
                        }
                        Field::List {
                            name,
                            mask: Some(mask),
                            ..
                        } => {
                            let q_rs_typ = (&mask.0, &mask.1).qualified_rs_typ();
                            writeln!(out, "{}{}: Vec<{}>,", cg::ind(2), name, q_rs_typ)?;
                        }
                        Field::List {
                            module,
                            name,
                            rs_typ,
                            ..
                        } => {
                            let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                            writeln!(out, "        {}: Vec<{}>,", name, q_rs_typ)?;
                        }
                        Field::Switch {
                            name,
                            rs_typ,
                            is_mask: true,
                            ..
                        } => {
                            writeln!(out, "        {}: Vec<{}>,", name, rs_typ)?;
                        }
                        Field::Pad { .. } => {}
                        Field::AlignPad { .. } => {}
                        _ => unreachable!("{:#?}", f),
                    }
                }
                writeln!(out, "    }},")?;
            }
        }
        writeln!(out, "}}")?;

        if is_mask {
            self.emit_mask_switch_impl(out, rs_typ, maskenum, cases)?;
        } else {
            self.emit_enum_switch_impl(out, rs_typ, maskenum, cases)?;
        }

        writeln!(out)?;
        if is_mask {
            writeln!(out, "impl base::WiredOut for &[{}] {{", rs_typ)?;
        } else {
            writeln!(out, "impl base::WiredOut for {} {{", rs_typ)?;
        }
        self.emit_switch_wire_len(out, rs_typ, cases, is_mask)?;
        writeln!(out)?;
        self.emit_switch_serialize(out, rs_typ, cases, is_mask)?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        if is_mask {
            writeln!(out, "impl base::WiredIn for Vec<{}> {{", rs_typ)?;
        } else {
            writeln!(out, "impl base::WiredIn for {} {{", rs_typ)?;
        }
        writeln!(out, "    type Params = {};", params_struct.rs_typ)?;
        self.emit_switch_compute_wire_len(out, expr, rs_typ, cases, params_struct, is_mask)?;

        self.emit_switch_unserialize(out, expr, rs_typ, is_mask, cases, params_struct)?;

        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_switch_unserialize<O: Write>(
        &self,
        out: &mut O,
        expr: &Expr,
        rs_typ: &str,
        is_mask: bool,
        cases: &[SwitchCase],
        params_struct: &ParamsStruct,
    ) -> io::Result<()> {
        let return_typ = if is_mask {
            format!("Vec<{}>", rs_typ)
        } else {
            rs_typ.to_string()
        };
        writeln!(out)?;
        writeln!(out, "    #[allow(unused_assignments)]")?;
        writeln!(out,
            "    unsafe fn unserialize(wire_data: *const u8, params: {}, out_offset: &mut usize) -> {} {{",
            params_struct.rs_typ, return_typ)?;

        writeln!(out, "{}let {}{{", cg::ind(2), params_struct.rs_typ)?;
        for p in &params_struct.params {
            writeln!(out, "{}{},", cg::ind(3), p)?;
        }
        writeln!(out, "{}}} = params;", cg::ind(2))?;

        writeln!(
            out,
            "{}let expr = {};",
            cg::ind(2),
            self.build_rs_expr(expr, "", "", &[])
        )?;

        if is_mask {
            writeln!(out, "{}let mut result = Vec::new();", cg::ind(2))?;
        }

        for sc in cases {
            let exprs = sc.exprs.iter().map(|e| self.build_rs_expr(e, "", "", &[]));

            let expr: String = if is_mask {
                format!("expr & {} != 0", exprs.collect::<Vec<_>>().join(" & "))
            } else {
                exprs
                    .map(|e| format!("expr == {}", e))
                    .collect::<Vec<_>>()
                    .join(" || ")
            };

            writeln!(out, "{}if {} {{", cg::ind(2), expr)?;

            writeln!(out, "{}let mut offset = 0usize;", cg::ind(3))?;

            for f in &sc.fields {
                // TODO: remove fieldrefs
                match f {
                    Field::Field {
                        name,
                        rs_typ,
                        wire_sz: Expr::Value(sz),
                        r#enum: Some(r#enum),
                        is_fieldref: false,
                        ..
                    } => {
                        let q_rs_typ = (&r#enum.0, &r#enum.1).qualified_rs_typ();
                        writeln!(out, "{}let {} = std::mem::transmute::<_, {}>(*(wire_data.add(offset) as *const {}) as u32);", cg::ind(3), name, q_rs_typ, rs_typ)?;
                        writeln!(out, "{}offset += {};", cg::ind(3), sz)?;
                    }
                    Field::Field {
                        name,
                        module,
                        rs_typ,
                        struct_style: Some(StructStyle::DynBuf),
                        params_struct,
                        ..
                    } => {
                        let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                        let params_expr = self.build_params_expr(
                            params_struct.as_ref(),
                            module.as_deref(),
                            "",
                            "",
                        );
                        writeln!(
                            out,
                            "{}let {} = {}Buf::unserialize(wire_data.add(offset), {}, &mut offset);",
                            cg::ind(3),
                            name,
                            q_rs_typ,
                            params_expr
                        )?;
                    }
                    Field::Field {
                        name,
                        rs_typ,
                        wire_sz: Expr::Value(sz),
                        ..
                    } if rs_typ == "bool" => {
                        let wire_typ = format!("u{}", *sz * 8);
                        writeln!(
                            out,
                            "{}let {} = *(wire_data.add(offset) as *const {}) != 0;",
                            cg::ind(3),
                            name,
                            wire_typ
                        )?;
                        writeln!(out, "{}offset += {};", cg::ind(3), sz)?;
                    }
                    Field::Field {
                        name,
                        module,
                        rs_typ,
                        ..
                    } => {
                        let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                        writeln!(
                            out,
                            "{}let {} = *(wire_data.add(offset) as *const {});",
                            cg::ind(3),
                            name,
                            q_rs_typ
                        )?;
                        writeln!(
                            out,
                            "{}offset += std::mem::size_of::<{}>();",
                            cg::ind(3),
                            q_rs_typ
                        )?;
                    }
                    Field::List {
                        name,
                        rs_typ,
                        len_expr,
                        wire_sz,
                        ..
                    } if rs_typ == "char" => {
                        let len_expr = self.build_rs_expr(len_expr, "", "", &sc.fields);
                        assert_eq!(len_expr, self.build_rs_expr(wire_sz, "", "", &sc.fields));

                        writeln!(
                            out,
                            "{}let {}_ptr = wire_data.add(offset);",
                            cg::ind(3),
                            name
                        )?;
                        writeln!(
                            out,
                            "{}let {}_bytes = std::slice::from_raw_parts({}_ptr, {});",
                            cg::ind(3),
                            name,
                            name,
                            len_expr,
                        )?;
                        writeln!(
                            out,
                            "{}let {} = Lat1String::from_bytes({}_bytes);",
                            cg::ind(3),
                            name,
                            name
                        )?;
                        writeln!(out, "{}offset += {}.len();", cg::ind(3), name)?;
                    }
                    Field::List {
                        name,
                        module,
                        rs_typ,
                        struct_style: None | Some(StructStyle::WireLayout | StructStyle::FixBuf),
                        is_union: false,
                        len_expr,
                        mask,
                        r#enum,
                        ..
                    } if rs_typ == "u32" || (r#enum.is_none() || mask.is_none()) => {
                        let q_rs_typ = enum_mask_qualified_rs_typ(module, rs_typ, r#enum, mask);
                        writeln!(out, "{}let {} = {{", cg::ind(3), name)?;
                        writeln!(
                            out,
                            "{}    let ptr = wire_data.add(offset) as *const {};",
                            cg::ind(3),
                            q_rs_typ
                        )?;
                        writeln!(
                            out,
                            "{}    let len = {};",
                            cg::ind(3),
                            self.build_rs_expr(len_expr, "", "", &[])
                        )?;
                        writeln!(
                            out,
                            "{}    let data = std::slice::from_raw_parts(ptr, len);",
                            cg::ind(3)
                        )?;
                        writeln!(
                            out,
                            "{}offset += len * std::mem::size_of::<{}>();",
                            cg::ind(4),
                            q_rs_typ
                        )?;
                        writeln!(out, "{}    data.to_vec()", cg::ind(3))?;
                        writeln!(out, "{}}};", cg::ind(3))?;
                    }
                    Field::List {
                        name,
                        rs_typ,
                        len_expr,
                        mask: Some(mask),
                        ..
                    } => {
                        let q_rs_typ = (&mask.0, &mask.1).qualified_rs_typ();
                        writeln!(out, "{}let mut {} = Vec::new();", cg::ind(3), name)?;
                        writeln!(
                            out,
                            "{}for i in 0..{} {{",
                            cg::ind(3),
                            self.build_rs_expr(len_expr, "", "", &[])
                        )?;
                        writeln!(out,
                            "{}{}.push({}::from_bits(*(wire_data.add(offset) as *const {}) as u32).unwrap());",
                            cg::ind(4), name, q_rs_typ, rs_typ)?;
                        writeln!(
                            out,
                            "{}offset += std::mem::size_of::<{}>();",
                            cg::ind(4),
                            rs_typ
                        )?;
                        writeln!(out, "{}}}", cg::ind(3))?;
                    }
                    Field::List {
                        name,
                        module,
                        rs_typ,
                        len_expr,
                        struct_style,
                        params_struct,
                        is_union,
                        union_typefield,
                        ..
                    } if matches!(struct_style, Some(StructStyle::DynBuf)) || *is_union => {
                        let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                        writeln!(out, "{}let mut {} = Vec::new();", cg::ind(3), name)?;
                        writeln!(
                            out,
                            "{}let {}_params = {};",
                            cg::ind(3),
                            name,
                            self.build_params_expr(
                                params_struct.as_ref(),
                                module.as_deref(),
                                "",
                                ""
                            )
                        )?;
                        writeln!(
                            out,
                            "{}for i in 0..{} {{",
                            cg::ind(3),
                            self.build_rs_expr(len_expr, "", "", &[])
                        )?;
                        let rs_typ_suff = if *is_union {
                            assert!(
                                union_typefield.is_some(),
                                "cannot build a union here without type field"
                            );
                            ""
                        } else {
                            "Buf"
                        };
                        writeln!(
                                out,
                                "{}let el = {}{}::unserialize(wire_data.add(offset), {}_params, &mut offset);",
                                cg::ind(4),
                                q_rs_typ,
                                rs_typ_suff,name,
                            )?;
                        writeln!(out, "{}{}.push(el);", cg::ind(4), name,)?;
                        writeln!(out, "{}}}", cg::ind(3))?;
                    }
                    Field::Switch {
                        name,
                        module,
                        rs_typ,
                        params_struct,
                        is_mask,
                        ..
                    } => {
                        let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                        let params_expr =
                            self.build_params_expr(Some(params_struct), module.as_deref(), "", "");
                        // do not supply fields because the masks are defined as integer here
                        let impl_type = if *is_mask {
                            format!("<Vec<{}>>", q_rs_typ)
                        } else {
                            q_rs_typ.clone()
                        };
                        writeln!(out, "{}let {}_params = {};", cg::ind(3), name, params_expr)?;
                        writeln!(out, "{}let {} = {}::unserialize(wire_data.add(offset), {}_params, &mut offset);", cg::ind(3), name, impl_type, name)?;
                    }
                    Field::Pad {
                        wire_sz: Expr::Value(sz),
                        ..
                    } => {
                        writeln!(out, "{}offset += {}; // pad", cg::ind(3), sz)?;
                    }
                    Field::AlignPad {
                        wire_sz: Expr::AlignPad(sz, _),
                        ..
                    } => {
                        writeln!(
                            out,
                            "{}offset += base::align_pad(offset, {});",
                            cg::ind(3),
                            sz
                        )?;
                    }
                    f => unreachable!("{:#?}", f),
                }
            }

            writeln!(out, "{}*out_offset += offset;", cg::ind(3))?;

            let (open, close) = if visible_fields_len(&sc.fields) == 1 {
                ("(", ")")
            } else {
                ("{", "}")
            };
            let stmt = if is_mask { "result.push(" } else { "return " };
            writeln!(out, "{}{}{}::{}{}", cg::ind(3), stmt, rs_typ, sc.name, open)?;
            for f in &sc.fields {
                match f {
                    Field::Field {
                        name,
                        mask,
                        is_fieldref,
                        ..
                    } => {
                        if !*is_fieldref || request_fieldref_emitted(name, &sc.fields, false) {
                            if let Some((module, mask)) = mask {
                                let pref_name = if visible_fields_len(&sc.fields) != 1 {
                                    format!("{}: ", name)
                                } else {
                                    "".to_string()
                                };
                                let q_rs_typ = (module, mask).qualified_rs_typ();
                                writeln!(
                                    out,
                                    "{}{}{}::from_bits({} as u32).unwrap(),",
                                    cg::ind(4),
                                    pref_name,
                                    q_rs_typ,
                                    name
                                )?;
                            } else {
                                writeln!(out, "{}{},", cg::ind(4), name)?;
                            }
                        }
                    }
                    Field::List { name, .. } => {
                        writeln!(out, "{}{},", cg::ind(4), name)?;
                    }
                    Field::Switch { name, .. } => {
                        writeln!(out, "{}{},", cg::ind(4), name)?;
                    }
                    Field::Pad { .. } => {}
                    Field::AlignPad { .. } => {}
                    f => unreachable!("{:#?}", f),
                }
            }
            writeln!(
                out,
                "{}{}{};",
                cg::ind(3),
                close,
                if is_mask { ")" } else { "" }
            )?;
            writeln!(out, "{}}}", cg::ind(2))?;
        }
        if is_mask {
            writeln!(out, "{}result", cg::ind(2))?;
        } else {
            writeln!(
                out,
                "{}unreachable!(\"Could not match any expression for {}\");",
                cg::ind(2),
                rs_typ
            )?;
        }
        writeln!(out, "    }}")?;

        Ok(())
    }

    pub(super) fn emit_switch_compute_wire_len<O: Write>(
        &self,
        out: &mut O,
        expr: &Expr,
        rs_typ: &str,
        cases: &[SwitchCase],
        params_struct: &ParamsStruct,
        is_mask: bool,
    ) -> io::Result<()> {
        writeln!(out)?;
        writeln!(
            out,
            "    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize {{"
        )?;
        writeln!(out, "{}let {} {{", cg::ind(2), params_struct.rs_typ)?;
        for p in &params_struct.params {
            writeln!(out, "{}{},", cg::ind(3), p)?;
        }
        writeln!(out, "{}}} = params;", cg::ind(2))?;

        writeln!(
            out,
            "{}let expr = {};",
            cg::ind(2),
            self.build_rs_expr(expr, "", "", &[])
        )?;

        if is_mask {
            writeln!(out, "{}let mut sz = 0usize;", cg::ind(2))?;
        }

        for c in cases {
            let exprs: Vec<String> = c
                .exprs
                .iter()
                .map(|e| self.build_rs_expr(e, "", "", &[]))
                .collect();
            let exprs = exprs.join(if is_mask { " | " } else { " || " });
            if is_mask {
                writeln!(out, "        if expr & {} != 0 {{", exprs)?;
            } else {
                writeln!(out, "        if expr == {} {{", exprs)?;
                writeln!(out, "            let mut sz = 0usize;")?;
            }
            for cf in &c.fields {
                let mut stmts = Vec::new();
                self.field_compute_len_stmts(cf, &mut stmts, &Expr::Value(0));
                for s in stmts {
                    writeln!(out, "            {}", s)?;
                }
            }
            if !is_mask {
                writeln!(out, "            return sz;")?;
            }
            writeln!(out, "        }}")?;
        }

        if is_mask {
            writeln!(out, "{}sz", cg::ind(2))?;
        } else {
            writeln!(
                out,
                "        unreachable!(\"could not match switch expression in {}::{}::compute_wire_len\")",
                self.xcb_mod, rs_typ
            )?;
        }
        writeln!(out, "    }}")?;
        Ok(())
    }

    fn emit_switch_wire_len<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        cases: &[SwitchCase],
        is_mask: bool,
    ) -> io::Result<()> {
        writeln!(out)?;
        writeln!(out, "    fn wire_len(&self) -> usize {{")?;
        writeln!(out, "{}let mut sz = 0usize;", cg::ind(2))?;
        let ind = if is_mask {
            writeln!(out, "{}for el in self.iter() {{", cg::ind(2))?;
            writeln!(out, "{}match el {{", cg::ind(3))?;
            3
        } else {
            writeln!(out, "{}match self {{", cg::ind(2))?;
            2
        };

        for sc in cases {
            let (open, close) = if visible_fields_len(&sc.fields) == 1 {
                ("(", ")")
            } else {
                ("{", "}")
            };

            writeln!(out, "{}{}::{}{}", cg::ind(ind + 1), rs_typ, sc.name, open)?;

            for f in &sc.fields {
                match f {
                    Field::Field {
                        name,
                        is_fieldref: true,
                        ..
                    } => {
                        if request_fieldref_emitted(name, &sc.fields, false) {
                            writeln!(out, "{}{},", cg::ind(ind + 2), name)?;
                        }
                    }
                    Field::Field {
                        wire_sz: Expr::Value(_),
                        ..
                    }
                    | Field::List {
                        wire_sz: Expr::Value(_),
                        ..
                    }
                    | Field::Pad {
                        wire_sz: Expr::Value(_),
                        ..
                    } => {}
                    Field::Field { name, .. }
                    | Field::List { name, .. }
                    | Field::Switch { name, .. } => writeln!(out, "{}{},", cg::ind(ind + 2), name)?,
                    Field::Expr { .. } => unreachable!(),
                    Field::Pad { .. } => {}
                    Field::AlignPad { .. } => {}
                }
            }
            writeln!(out, "{}..", cg::ind(ind + 2))?;

            writeln!(out, "{}{} => {{", cg::ind(ind + 1), close)?;
            for f in &sc.fields {
                match f {
                    Field::Field {
                        wire_sz: Expr::Value(sz),
                        ..
                    }
                    | Field::List {
                        wire_sz: Expr::Value(sz),
                        ..
                    }
                    | Field::Pad {
                        wire_sz: Expr::Value(sz),
                        ..
                    } => {
                        writeln!(out, "{}sz += {};", cg::ind(ind + 2), sz)?;
                    }
                    Field::Field {
                        mask,
                        r#enum,
                        rs_typ,
                        ..
                    } if mask.is_some() || r#enum.is_some() => {
                        writeln!(
                            out,
                            "{}sz += std::mem::size_of::<{}>();",
                            cg::ind(ind + 2),
                            rs_typ
                        )?;
                    }
                    Field::Field { name, .. } => {
                        writeln!(out, "{}sz += {}.wire_len();", cg::ind(ind + 2), name)?;
                    }
                    Field::List { name, rs_typ, .. } if rs_typ == "char" => {
                        writeln!(out, "{}sz += {}.len();", cg::ind(ind + 2), name)?;
                    }
                    Field::List {
                        name,
                        mask,
                        r#enum,
                        rs_typ,
                        ..
                    } if mask.is_some() || r#enum.is_some() => {
                        writeln!(
                            out,
                            "{}sz += {}.len() * std::mem::size_of::<{}>();",
                            cg::ind(ind + 2),
                            name,
                            rs_typ
                        )?;
                    }
                    Field::List { name, .. } => {
                        writeln!(out, "{}for el in {} {{", cg::ind(ind + 2), name)?;
                        writeln!(out, "{}sz += el.wire_len();", cg::ind(ind + 3))?;
                        writeln!(out, "{}}}", cg::ind(ind + 2))?;
                    }
                    Field::Switch { name, is_mask, .. } => {
                        let acc = if *is_mask { ".as_slice()" } else { "" };
                        writeln!(out, "{}sz += {}{}.wire_len();", cg::ind(ind + 2), name, acc)?;
                    }
                    Field::AlignPad {
                        wire_sz: Expr::AlignPad(sz, _),
                        ..
                    } => {
                        writeln!(
                            out,
                            "{}sz += base::align_pad(sz, {});",
                            cg::ind(ind + 2),
                            sz
                        )?;
                    }
                    _ => unreachable!("{:#?}", f),
                }
            }
            writeln!(out, "{}}}", cg::ind(ind + 1))?;
        }

        writeln!(out, "{}}}", cg::ind(ind))?;
        if is_mask {
            writeln!(out, "{}}}", cg::ind(2))?;
        }
        writeln!(out, "{}sz", cg::ind(2))?;
        writeln!(out, "    }}")?;
        Ok(())
    }

    fn emit_switch_serialize<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        cases: &[SwitchCase],
        is_mask: bool,
    ) -> io::Result<()> {
        writeln!(
            out,
            "    fn serialize(&self, wire_buf: &mut [u8]) -> usize {{"
        )?;
        writeln!(out, "{}let mut offset = 0usize;", cg::ind(2))?;
        let ind = if is_mask {
            writeln!(out, "{}for el in self.iter() {{", cg::ind(2))?;
            writeln!(out, "{}match el {{", cg::ind(3))?;
            3
        } else {
            writeln!(out, "{}match self {{", cg::ind(2))?;
            2
        };

        for sc in cases {
            let (open, close) = if visible_fields_len(&sc.fields) == 1 {
                ("(", ")")
            } else {
                ("{", "}")
            };

            writeln!(out, "{}{}::{}{}", cg::ind(ind + 1), rs_typ, sc.name, open)?;

            for f in &sc.fields {
                match f {
                    Field::Field {
                        name,
                        is_fieldref: true,
                        ..
                    } => {
                        if request_fieldref_emitted(name, &sc.fields, false) {
                            writeln!(out, "{}{},", cg::ind(ind + 2), name)?;
                        }
                    }
                    Field::Field { name, .. }
                    | Field::List { name, .. }
                    | Field::Switch { name, .. } => writeln!(out, "{}{},", cg::ind(ind + 2), name)?,
                    Field::Expr { .. } => unreachable!(),
                    Field::Pad { .. } => {}
                    Field::AlignPad { .. } => {}
                }
            }
            writeln!(out, "{}..", cg::ind(ind + 2))?;

            writeln!(out, "{}{} => {{", cg::ind(ind + 1), close)?;
            for f in &sc.fields {
                match f {
                    Field::Field {
                        name,
                        rs_typ,
                        wire_sz: Expr::Value(sz),
                        ..
                    } if rs_typ == "bool" => {
                        // have to take into account "BOOL32"
                        let typ = if *sz == 4 {
                            "u32"
                        } else if *sz == 1 {
                            "u8"
                        } else {
                            unreachable!("unknown sized bool")
                        };
                        writeln!(
                            out,
                            "{}let {}: {} = if *{} {{ 1 }} else {{ 0 }};",
                            cg::ind(ind + 2),
                            name,
                            typ,
                            name
                        )?;
                        writeln!(
                            out,
                            "{}offset += {}.serialize(&mut wire_buf[offset..]);",
                            cg::ind(ind + 2),
                            name
                        )?;
                    }
                    Field::Field {
                        name,
                        r#enum: Some(_),
                        rs_typ,
                        ..
                    } => {
                        writeln!(out,
                            "{}offset += (unsafe {{ std::mem::transmute::<_, u32>(*{}) }} as {}).serialize(&mut wire_buf[offset..]);",
                            cg::ind(ind+2), name, rs_typ)?;
                    }
                    Field::Field {
                        name,
                        mask: Some(_),
                        is_fieldref,
                        rs_typ,
                        ..
                    } => {
                        let fieldref_value = if *is_fieldref {
                            fieldref_get_value(name, &sc.fields, false, "")
                        } else {
                            None
                        };
                        let expr = if let Some(fieldref_value) = fieldref_value {
                            fieldref_value
                        } else {
                            name.clone() + ".bits()"
                        };
                        writeln!(
                            out,
                            "{}offset += ({} as {}).serialize(&mut wire_buf[offset..]);",
                            cg::ind(ind + 2),
                            expr,
                            rs_typ
                        )?;
                    }
                    Field::Field {
                        name,
                        rs_typ,
                        is_fieldref,
                        ..
                    } => {
                        let fieldref_value = if *is_fieldref {
                            fieldref_get_value(name, &sc.fields, false, "")
                        } else {
                            None
                        };
                        if let Some(fieldref_value) = fieldref_value {
                            writeln!(
                                out,
                                "{}offset += ({} as {}).serialize(&mut wire_buf[offset..]);",
                                cg::ind(ind + 2),
                                fieldref_value,
                                rs_typ
                            )?;
                        } else {
                            writeln!(
                                out,
                                "{}offset += {}.serialize(&mut wire_buf[offset..]);",
                                cg::ind(ind + 2),
                                name
                            )?;
                        }
                    }
                    Field::List { name, rs_typ, .. } if rs_typ == "char" => {
                        writeln!(
                            out,
                            "{}wire_buf[offset..offset+{}.len()].copy_from_slice({}.as_bytes());",
                            cg::ind(ind + 2),
                            name,
                            name
                        )?;
                        writeln!(out, "{}offset += {}.len();", cg::ind(ind + 2), name)?;
                    }
                    Field::List {
                        name,
                        rs_typ,
                        mask: Some(_),
                        ..
                    } => {
                        writeln!(out, "{}for el in {} {{", cg::ind(ind + 2), name)?;
                        writeln!(
                            out,
                            "{}offset += (el.bits() as {}).serialize(&mut wire_buf[offset..]);",
                            cg::ind(ind + 3),
                            rs_typ,
                        )?;
                        writeln!(out, "{}}}", cg::ind(ind + 2))?;
                    }
                    Field::List { name, .. } => {
                        writeln!(out, "{}for el in {} {{", cg::ind(ind + 2), name)?;
                        writeln!(
                            out,
                            "{}offset += el.serialize(&mut wire_buf[offset..]);",
                            cg::ind(ind + 3)
                        )?;
                        writeln!(out, "{}}}", cg::ind(ind + 2))?;
                    }
                    Field::Switch { name, is_mask, .. } => {
                        let acc = if *is_mask { ".as_slice()" } else { "" };
                        writeln!(
                            out,
                            "{}offset += {}{}.serialize(&mut wire_buf[offset..]);",
                            cg::ind(ind + 2),
                            name,
                            acc
                        )?;
                    }
                    Field::Pad {
                        wire_sz: Expr::Value(sz),
                        ..
                    } => {
                        writeln!(out, "{}offset += {};", cg::ind(ind + 2), sz)?;
                    }
                    Field::AlignPad {
                        wire_sz: Expr::AlignPad(sz, _),
                        ..
                    } => {
                        writeln!(
                            out,
                            "{}offset += base::align_pad(offset, {});",
                            cg::ind(ind + 2),
                            sz
                        )?;
                    }
                    _ => unreachable!("{:#?}", f),
                }
            }
            writeln!(out, "{}}}", cg::ind(ind + 1))?;
        }

        writeln!(out, "{}}}", cg::ind(ind))?;
        if is_mask {
            writeln!(out, "{}}}", cg::ind(2))?;
        }
        writeln!(out, "{}offset", cg::ind(2))?;
        writeln!(out, "    }}")?;
        Ok(())
    }

    fn emit_mask_switch_impl<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        mask: &(Option<String>, String),
        cases: &[SwitchCase],
    ) -> io::Result<()> {
        let mask = (&mask.0, &mask.1).qualified_rs_typ();
        writeln!(out)?;
        writeln!(out, "impl {} {{", rs_typ)?;
        writeln!(
            out,
            "    pub(crate) fn get_mask(slice: &[{}]) -> {} {{",
            rs_typ, mask
        )?;
        writeln!(out, "        let mut res = {}::empty();", mask)?;
        writeln!(out, "        for el in slice {{")?;
        writeln!(out, "            match el {{")?;
        for c in cases {
            let indent = "                ";
            writeln!(out, "{}{}::{}{{..}} => {{", indent, rs_typ, c.name)?;
            for expr in &c.exprs {
                if let Expr::MaskRef { item, .. } = expr {
                    writeln!(out, "{}    res |= {}::{};", indent, mask, item)?;
                } else {
                    unreachable!();
                }
            }
            writeln!(out, "{}}}", indent)?;
        }
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "        res")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
        writeln!(
            out,
            "    pub(crate) fn is_sorted_distinct(slice: &[{}]) -> bool {{",
            rs_typ
        )?;
        writeln!(out, "        if slice.len() <= 1 {{")?;
        writeln!(out, "            true")?;
        writeln!(out, "        }} else {{")?;
        writeln!(out, "            let mut last = slice[0].get_ord();")?;
        writeln!(
            out,
            "            slice[1..].iter().map(|el| el.get_ord()).all(|o| {{"
        )?;
        writeln!(out, "                let lasto = last;")?;
        writeln!(out, "                last = o;")?;
        writeln!(out, "                lasto < o")?;
        writeln!(out, "            }})")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
        writeln!(out, "    fn get_ord(&self) -> u32 {{")?;
        writeln!(out, "        match self {{")?;
        for (i, c) in cases.iter().enumerate() {
            writeln!(out, "            {}::{}{{..}} => {},", rs_typ, c.name, i)?;
        }
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        // implementing Ord only for xproto switches which are simple enough
        if self.xcb_mod == "xproto" {
            writeln!(out)?;
            writeln!(out, "impl Ord for {} {{", rs_typ)?;
            writeln!(out, "    fn cmp(&self, other: &Self) -> Ordering {{")?;
            writeln!(out, "        let o = self.get_ord().cmp(&other.get_ord());")?;
            writeln!(out, "        match o {{")?;
            writeln!(out, "            Ordering::Less | Ordering::Greater => o,")?;
            writeln!(out, "            Ordering::Equal => {{")?;
            writeln!(out, "                match (self, other) {{")?;
            for c in cases.iter() {
                writeln!(
                    out,
                    "{}({}::{}(val), {}::{}(oval)) => val.cmp(oval),",
                    cg::ind(5),
                    rs_typ,
                    c.name,
                    rs_typ,
                    c.name
                )?;
            }
            writeln!(
                out,
                "                    _ => unreachable!(\"Bug: o should not be Ordering::Equal\"),"
            )?;
            writeln!(out, "                }}")?;
            writeln!(out, "            }}")?;
            writeln!(out, "        }}")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
            writeln!(out)?;
            writeln!(out, "impl PartialOrd for {} {{", rs_typ)?;
            writeln!(
                out,
                "    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {{"
            )?;
            writeln!(out, "        Some(self.cmp(other))")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }
        Ok(())
    }

    fn emit_enum_switch_impl<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        r#enum: &(Option<String>, String),
        cases: &[SwitchCase],
    ) -> io::Result<()> {
        let r#enum = (&r#enum.0, &r#enum.1).qualified_rs_typ();
        writeln!(out)?;
        writeln!(out, "impl {} {{", rs_typ)?;
        writeln!(out, "    pub(crate) fn get_enum(&self) -> {} {{", r#enum)?;
        writeln!(out, "        match self {{")?;
        for c in cases {
            let indent = "        ";
            writeln!(out, "{}{}::{}{{..}} => {{", indent, rs_typ, c.name)?;
            for expr in &c.exprs {
                if let Expr::EnumRef { item, .. } = expr {
                    writeln!(out, "{}    {}::{}", indent, r#enum, item)?;
                } else {
                    unreachable!();
                }
            }
            writeln!(out, "{}}}", indent)?;
        }
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        Ok(())
    }
}

fn visible_fields_len(fields: &[Field]) -> usize {
    let mut len = 0;
    for f in fields {
        match f {
            Field::Pad { .. } => {}
            Field::AlignPad { .. } => {}
            _ => {
                len += 1;
            }
        }
    }
    len
}
