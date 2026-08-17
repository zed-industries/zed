use super::r#struct::{self, make_field, ResolvedFields};
use super::{CodeGen, Expr, Field, Request};
use crate::cg::{self, Doc, QualifiedRsTyp, Reply, StructStyle};
use crate::ir::{self};

use std::borrow::Cow;
use std::io::{self, Write};

impl CodeGen {
    pub(super) fn resolve_request(
        &mut self,
        name: String,
        opcode: u32,
        mut params: Vec<ir::Field>,
        reply: Option<ir::Reply>,
        doc: Option<ir::Doc>,
    ) {
        // special case for the request that send events
        let sends_event = matches!(
            (self.xcb_mod.as_str(), name.as_str()),
            ("xproto", "SendEvent") | ("xevie", "Send")
        );

        // let event_is_list = sends_event
        //     && params
        //         .iter()
        //         .any(|p| matches!(p, ir::Field::List{name, ..} if name == "Event"));

        let rs_typ = cg::rust_type_name(&name);
        let doc = self.resolve_doc(doc);

        let params = {
            let mut fields = vec![
                make_field("major_opcode".into(), "CARD8".into()),
                if self.xcb_mod == "xproto" {
                    if params.is_empty() {
                        ir::Field::Pad(1)
                    } else {
                        params.remove(0)
                    }
                } else {
                    make_field("minor_opcode".into(), "CARD8".into())
                },
                make_field("length".into(), "CARD16".into()),
            ];
            fields.append(&mut params);
            fields
        };

        let ResolvedFields {
            fields: mut params, ..
        } = self.resolv_struct_fields(&rs_typ, "", &params, doc.as_ref());

        let req_rs_typ = &rs_typ;

        for param in &mut params {
            if let Field::Field {
                name,
                ref mut rs_typ,
                ..
            } = param
            {
                if self.xcb_mod == "xproto"
                    && req_rs_typ == "SendEvent"
                    && name == "destination"
                    && rs_typ == "Window"
                {
                    *rs_typ = "SendEventDest".to_string();
                }
            }
        }

        let reply = reply.map(|mut r| {
            let rs_typ = rs_typ.clone() + "Reply";
            let doc = self.resolve_doc(r.doc);
            let mut fields = vec![
                make_field("response_type".into(), "CARD8".into()),
                if r.fields.is_empty() {
                    ir::Field::Pad(1usize)
                } else {
                    r.fields.remove(0)
                },
                make_field("sequence".into(), "CARD16".into()),
                make_field("length".into(), "CARD32".into()),
            ];
            fields.append(&mut r.fields);
            let ResolvedFields { fields, .. } =
                self.resolv_struct_fields(&rs_typ, "", &fields, doc.as_ref());

            Reply { fields, doc }
        });
        self.requests.push(Request {
            rs_typ,
            opcode,
            params,
            reply,
            sends_event,
            doc,
            // event_is_list,
        });
    }

    pub fn emit_requests<O: Write>(&self, out: &mut O) -> io::Result<()> {
        writeln!(out)?;
        writeln!(
            out,
            "pub(crate) fn request_name(opcode: u16) -> std::option::Option<&'static str> {{"
        )?;
        writeln!(out, "{}match opcode {{", cg::ind(1))?;
        let module = if self.ext_info.is_some() {
            self.xcb_mod.as_str()
        } else {
            "x"
        };
        for r in &self.requests {
            writeln!(
                out,
                "{}{} => Some(\"{}::{}\"),",
                cg::ind(2),
                r.opcode,
                module,
                r.rs_typ
            )?;
        }
        writeln!(out, "{}_ => None,", cg::ind(2))?;
        writeln!(out, "{}}}", cg::ind(1))?;
        writeln!(out, "}}")?;

        for r in &self.requests {
            let Request {
                rs_typ,
                opcode,
                params,
                reply,
                sends_event,
                doc,
            } = r;

            let info = self.query_request_info(rs_typ, params, *opcode, reply.as_ref());

            if let Some(reply) = reply {
                self.emit_cookie_reply(out, reply, &info)?;
            }

            self.emit_request_struct(out, rs_typ, params, &info, doc.as_ref(), *sends_event)?;

            self.emit_request_traits(out, rs_typ, params, &info, *sends_event)?;
        }
        Ok(())
    }

    fn query_request_info<'a>(
        &self,
        rs_typ: &str,
        params: &'a [Field],
        opcode: u32,
        reply: Option<&Reply>,
    ) -> RequestInfo<'a> {
        let mut last_fixed = true;
        let mut has_lt = false;
        let mut sections = Vec::new();
        let mut start = 0;
        let mut end = 0;

        let mut field = |is_fixed: bool, is_align_pad: bool| {
            if !is_fixed && last_fixed {
                // we passed the end of a fixed section
                sections.push(SerializeSection::Fixed(&params[start..end]));
                start = end;
            }
            end += 1;
            if !is_fixed && !is_align_pad {
                // we reach a variable section
                sections.push(SerializeSection::Var(&params[start]));
            }
            if is_align_pad {
                // each section is followed by an align section, so align pads don't need any treatment
                assert!(!last_fixed);
            }
            if !is_fixed {
                // fixed fields are grouped together, so we advance the start pointer only when we reach a var field
                start = end;
            }
            last_fixed = is_fixed;
        };

        for p in params {
            match p {
                Field::Field {
                    struct_style: Some(StructStyle::DynBuf),
                    ..
                } => {
                    field(false, false);
                    has_lt = true;
                }
                Field::Field { .. } => {
                    field(true, false);
                }
                Field::List {
                    struct_style: Some(StructStyle::DynBuf),
                    len_expr: Expr::Value(_),
                    ..
                } => {
                    field(false, false);
                    has_lt = true;
                }
                Field::List { is_union: true, .. } => {
                    field(false, false);
                    has_lt = true;
                }
                Field::List {
                    len_expr: Expr::Value(_),
                    ..
                } => {
                    field(true, false);
                }
                Field::List { .. } => {
                    field(false, false);
                    has_lt = true;
                }
                Field::Switch { is_mask, .. } => {
                    field(false, false);
                    if *is_mask {
                        has_lt = true;
                    }
                }
                Field::Expr { .. } => {
                    field(true, false);
                }
                Field::Pad { .. } => {
                    field(true, false);
                }
                Field::AlignPad { .. } => {
                    //assert!(last_fixed);
                    field(false, true);
                }
            }
        }

        if last_fixed && start < end {
            sections.push(SerializeSection::Fixed(&params[start..end]));
            start = end;
        }

        assert!(start == end);

        let (cookie_rs_typ, reply_rs_typ) = if reply.is_none() {
            ("base::VoidCookie".to_string(), "()".to_string())
        } else {
            (rs_typ.to_string() + "Cookie", rs_typ.to_string() + "Reply")
        };

        let has_prop_field = params.iter().any(|p| {
            matches!(
                p,
                Field::Field {
                    is_prop_format: true,
                    ..
                }
            )
        });

        let has_fd = params.iter().any(field_is_fd);

        let reply_has_fd = if let Some(reply) = reply {
            reply.fields.iter().any(field_is_fd)
        } else {
            false
        };

        RequestInfo {
            rs_typ: rs_typ.to_string(),
            cookie_rs_typ,
            reply_rs_typ,
            sections,
            has_lifetime: has_lt,
            opcode,
            is_void: reply.is_none(),
            has_prop_field,
            has_fd,
            reply_has_fd,
        }
    }

    fn emit_cookie_reply<O: Write>(
        &self,
        out: &mut O,
        reply: &Reply,
        info: &RequestInfo,
    ) -> io::Result<()> {
        let RequestInfo {
            rs_typ: req_name,
            cookie_rs_typ,
            reply_rs_typ,
            ..
        } = info;

        writeln!(out)?;
        if let Some(doc) = &reply.doc {
            doc.emit(out, 0)?;
        } else {
            writeln!(out, "/// Reply type for [{}].", req_name)?;
            writeln!(out, "///")?;
            writeln!(out,
                "/// Can be obtained from a [{}] with [Connection::wait_for_reply](crate::Connection::wait_for_reply)",
                cookie_rs_typ)?;
            writeln!(out,
                "/// or from a [{}Unchecked] with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)",
                cookie_rs_typ)?;
        }
        writeln!(out, "pub struct {} {{", reply_rs_typ)?;
        writeln!(out, "    raw: *const u8,")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl {} {{", reply_rs_typ)?;
        writeln!(out)?;
        writeln!(out, "    fn wire_ptr(&self) -> *const u8 {{")?;
        writeln!(out, "        self.raw")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
        // reply length field is expressed in 4 bytes units and start after the 32 bytes reply body
        writeln!(out, "    fn wire_len(&self) -> usize {{")?;
        writeln!(out, "        (32 + self.length() * 4) as _")?;
        writeln!(out, "    }}")?;

        for f in &reply.fields {
            if let Field::Field {
                struct_style: Some(StructStyle::DynBuf),
                ..
            }
            | Field::List { .. } = f
            {
                let len_stmts =
                    self.emit_compute_offset_and_get_stmts(out, reply_rs_typ, &reply.fields, None)?;
                self.emit_compute_func(out, "compute_len", None, &len_stmts)?;
                break;
            }
        }
        self.emit_struct_accessors(out, reply_rs_typ, &reply.fields)?;

        self.emit_reply_fds(out, reply_rs_typ, &reply.fields)?;

        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl base::Reply for {} {{", reply_rs_typ)?;
        writeln!(out, "    unsafe fn from_raw(raw: *const u8) -> Self {{")?;
        writeln!(out, "        Self {{ raw }}")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
        writeln!(out, "    unsafe fn into_raw(self) -> *const u8 {{")?;
        writeln!(out, "        let raw = self.raw;")?;
        writeln!(out, "        std::mem::forget(self);")?;
        writeln!(out, "        raw")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
        writeln!(out, "    unsafe fn as_raw(&self) -> *const u8 {{")?;
        writeln!(out, "        self.raw")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        self.emit_debug_impl(out, reply_rs_typ, &reply.fields)?;

        writeln!(out)?;
        writeln!(out, "impl Drop for {} {{", reply_rs_typ)?;
        writeln!(out, "    fn drop(&mut self) {{")?;
        writeln!(out, "        unsafe {{ libc::free(self.raw as *mut _); }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(
            out,
            "unsafe impl std::marker::Send for {} {{}}",
            reply_rs_typ
        )?;
        writeln!(
            out,
            "unsafe impl std::marker::Sync for {} {{}}",
            reply_rs_typ
        )?;

        writeln!(out)?;
        writeln!(out, "/// Cookie type for [{}].", req_name)?;
        writeln!(out, "///")?;
        writeln!(
            out,
            "/// This cookie can be used to get a [{}]",
            reply_rs_typ
        )?;
        writeln!(
            out,
            "/// with [Connection::wait_for_reply](crate::Connection::wait_for_reply)"
        )?;
        writeln!(out, "#[derive(Debug)]")?;
        writeln!(out, "pub struct {} {{", cookie_rs_typ)?;
        writeln!(out, "    seq: u64,")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "#[derive(Debug)]")?;
        writeln!(out, "/// Unchecked cookie type for [{}].", req_name)?;
        writeln!(out, "///")?;
        writeln!(
            out,
            "/// This cookie can be used to get a [{}]",
            reply_rs_typ
        )?;
        writeln!(out, "/// with [Connection::wait_for_reply_unchecked](crate::Connection::wait_for_reply_unchecked)")?;
        writeln!(out, "pub struct {}Unchecked {{", cookie_rs_typ)?;
        writeln!(out, "    seq: u64,")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl base::Cookie for {} {{", cookie_rs_typ)?;
        writeln!(out, "    unsafe fn from_sequence(seq: u64) -> Self {{")?;
        writeln!(out, "        {} {{ seq }}", cookie_rs_typ)?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
        writeln!(out, "    fn sequence(&self) -> u64 {{")?;
        writeln!(out, "        self.seq")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(
            out,
            "unsafe impl base::CookieChecked for {} {{",
            cookie_rs_typ
        )?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(
            out,
            "unsafe impl base::CookieWithReplyChecked for {} {{",
            cookie_rs_typ
        )?;
        writeln!(out, "    type Reply = {};", reply_rs_typ)?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl base::Cookie for {}Unchecked {{", cookie_rs_typ)?;
        writeln!(out, "    unsafe fn from_sequence(seq: u64) -> Self {{")?;
        writeln!(out, "        {}Unchecked {{ seq }}", cookie_rs_typ)?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
        writeln!(out, "    fn sequence(&self) -> u64 {{")?;
        writeln!(out, "        self.seq")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(
            out,
            "unsafe impl base::CookieWithReplyUnchecked for {}Unchecked {{",
            cookie_rs_typ
        )?;
        writeln!(out, "    type Reply = {};", reply_rs_typ)?;
        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_reply_fds<O: Write>(
        &self,
        out: &mut O,
        reply_rs_typ: &str,
        fields: &[Field],
    ) -> io::Result<()> {
        // We emit the reply fds.
        // libxcb store them after the wire body.
        for f in fields {
            match f {
                Field::Field { name, rs_typ, .. } if rs_typ == "RawFd" => {
                    writeln!(out)?;
                    writeln!(out, "{}pub fn {}(&self) -> RawFd {{", cg::ind(1), name)?;
                    writeln!(out, "{}unsafe {{", cg::ind(2))?;
                    writeln!(
                        out,
                        "{}assert!(self.nfd() == 1, \"Expected a single Fd for {}::{}\");",
                        cg::ind(3),
                        reply_rs_typ,
                        name
                    )?;
                    writeln!(
                        out,
                        "{}*(self.wire_ptr().add(self.wire_len()) as *const RawFd)",
                        cg::ind(3)
                    )?;
                    writeln!(out, "{}}}", cg::ind(2))?;
                    writeln!(out, "{}}}", cg::ind(1))?;
                }
                Field::List { name, rs_typ, .. } if rs_typ == "RawFd" => {
                    writeln!(out)?;
                    writeln!(out, "{}pub fn {}(&self) -> &[RawFd] {{", cg::ind(1), name)?;
                    writeln!(out, "{}unsafe {{", cg::ind(2))?;
                    writeln!(out, "{}let len = self.nfd() as usize;", cg::ind(3))?;
                    writeln!(
                        out,
                        "{}let ptr = self.wire_ptr().add(self.wire_len()) as *const RawFd;",
                        cg::ind(3)
                    )?;
                    writeln!(out, "{}std::slice::from_raw_parts(ptr, len)", cg::ind(3))?;
                    writeln!(out, "{}}}", cg::ind(2))?;
                    writeln!(out, "{}}}", cg::ind(1))?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn emit_request_struct<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        params: &[Field],
        info: &RequestInfo,
        doc: Option<&Doc>,
        sends_event: bool,
    ) -> io::Result<()> {
        let (generic_decl, _) = self.generic_decl_and_use(sends_event, info);

        writeln!(out)?;
        if let Some(doc) = doc {
            doc.emit(out, 0)?;
        } else {
            writeln!(out, "/// The `{}` request.", rs_typ)?;
        }
        writeln!(out, "///")?;
        if info.is_void {
            writeln!(out, "/// This request has no reply.")?;
            writeln!(out, "///")?;
            writeln!(out,
                "/// Associated cookie types are [VoidCookie](crate::VoidCookie) and [VoidCookieChecked](crate::VoidCookieChecked).")?;
        } else {
            writeln!(out, "/// This request replies [{}].", info.reply_rs_typ)?;
            writeln!(out, "///")?;
            writeln!(
                out,
                "/// Associated cookie types are [{}] and [{}Unchecked].",
                info.cookie_rs_typ, info.cookie_rs_typ
            )?;
        }
        if rs_typ == "InternAtom" {
            writeln!(out, "///")?;
            writeln!(
                out,
                "/// See also [`xcb::atoms_struct`](crate::atoms_struct)."
            )?;
        }
        writeln!(out, "#[derive(Clone, Debug)]")?;
        writeln!(out, "pub struct {}{} {{", rs_typ, generic_decl)?;
        for p in params {
            match p {
                Field::Field { name, doc, .. } | Field::List { name, doc, .. }
                    if sends_event && name == "event" =>
                {
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "{}pub event: &'a E,", cg::ind(1))?;
                }
                Field::Field {
                    name,
                    module,
                    rs_typ,
                    is_fieldref,
                    struct_style: Some(StructStyle::DynBuf),
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    if !is_fieldref {
                        if let Some(doc) = doc {
                            doc.emit(out, 1)?;
                        }
                        writeln!(out, "    pub {}: &'a {},", name, q_rs_typ)?;
                    }
                }
                Field::Field {
                    name,
                    module,
                    rs_typ,
                    is_fieldref,
                    r#enum,
                    mask,
                    is_prop_format,
                    doc,
                    ..
                } => {
                    if name == "major_opcode" || name == "minor_opcode" || name == "length" {
                        // These fields are written on the wire, but not needed as parameters.
                        // They are filled in by XCB directly
                        continue;
                    }
                    if *is_prop_format {
                        // format field is inferred from the property data field
                        continue;
                    }

                    let q_rs_typ =
                        r#struct::enum_mask_qualified_rs_typ(module, rs_typ, r#enum, mask);
                    if !is_fieldref || request_fieldref_emitted(name, params, info.has_prop_field) {
                        if let Some(doc) = doc {
                            doc.emit(out, 1)?;
                        }
                        writeln!(out, "    pub {}: {},", name, q_rs_typ)?;
                    }
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    struct_style: Some(StructStyle::DynBuf),
                    len_expr: Expr::Value(len),
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    pub {}: [{}Buf; {}],", name, q_rs_typ, len)?;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    len_expr: Expr::Value(len),
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    pub {}: [{}; {}],", name, q_rs_typ, len)?;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    struct_style: Some(StructStyle::DynBuf),
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    pub {}: &'a [{}Buf],", name, q_rs_typ)?;
                }
                Field::List {
                    name, rs_typ, doc, ..
                } if rs_typ == "char" => {
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    pub {}: &'a [u8],", name)?;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    is_prop,
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    let typ = if *is_prop { "P" } else { &q_rs_typ };
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    pub {}: &'a [{}],", name, typ)?;
                }
                Field::Switch {
                    name,
                    module,
                    rs_typ,
                    is_mask,
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    if *is_mask {
                        writeln!(out, "    pub {}: &'a [{}],", name, q_rs_typ)?;
                    } else {
                        writeln!(out, "    pub {}: {},", name, q_rs_typ)?;
                    }
                }
                _ => {}
            }
        }
        writeln!(out, "}}")?;
        Ok(())
    }

    fn emit_request_traits<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        params: &[Field],
        info: &RequestInfo,
        sends_event: bool,
    ) -> io::Result<()> {
        self.emit_raw_request(out, rs_typ, params, info, sends_event)?;

        let (generic_decl, generic_use) = self.generic_decl_and_use(sends_event, info);

        let RequestInfo {
            is_void,
            cookie_rs_typ,
            reply_rs_typ,
            ..
        } = info;

        writeln!(out)?;
        writeln!(
            out,
            "impl{} base::Request for {}{} {{",
            generic_decl, rs_typ, generic_use
        )?;
        writeln!(out, "    type Cookie = {};", cookie_rs_typ)?;
        writeln!(out, "    const IS_VOID: bool = {};", is_void)?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(
            out,
            "impl{} base::Request{}Reply for {}{} {{",
            generic_decl,
            if *is_void { "Without" } else { "With" },
            rs_typ,
            generic_use
        )?;

        if !*is_void {
            writeln!(out, "    type Reply = {};", reply_rs_typ)?;
            writeln!(out, "    type Cookie = {};", cookie_rs_typ)?;
            writeln!(
                out,
                "    type CookieUnchecked = {}Unchecked;",
                cookie_rs_typ
            )?;
        }

        writeln!(out, "}}")?;

        Ok(())
    }

    fn generic_decl_and_use(&self, sends_event: bool, info: &RequestInfo) -> (&str, &str) {
        if sends_event {
            ("<'a, E: base::BaseEvent>", "<'a, E>")
        } else if info.has_prop_field {
            ("<'a, P: PropEl>", "<'a, P>")
        } else if info.has_lifetime {
            ("<'a>", "<'a>")
        } else {
            ("", "")
        }
    }

    fn emit_raw_request<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        params: &[Field],
        info: &RequestInfo,
        sends_event: bool,
    ) -> io::Result<()> {
        let (generic_decl, generic_use) = self.generic_decl_and_use(sends_event, info);

        let RequestInfo {
            sections,
            opcode,
            is_void,
            ..
        } = info;

        let parts_count = 2 * sections.len();
        // the C implementation always start with two iovec untouched (not sure why)
        let iovec_count = parts_count + 2;

        writeln!(out)?;
        writeln!(
            out,
            "unsafe impl{} base::RawRequest for {}{} {{",
            generic_decl, rs_typ, generic_use
        )?;
        writeln!(
            out,
            "    fn raw_request(&self, c: &base::Connection, checked: bool) -> u64 {{ unsafe {{",
        )?;
        self.emit_req_assertions(out, rs_typ, params)?;
        writeln!(
            out,
            "{}let mut protocol_request = xcb_protocol_request_t {{",
            cg::ind(2),
        )?;
        writeln!(out, "{}count: {},", cg::ind(3), parts_count)?;
        if self.xcb_mod == "xproto" {
            writeln!(out, "{}ext: std::ptr::null_mut(),", cg::ind(3))?;
        } else {
            writeln!(out, "{}ext: std::ptr::addr_of_mut!(FFI_EXT),", cg::ind(3))?;
        }
        writeln!(out, "{}opcode: {},", cg::ind(3), opcode)?;
        writeln!(
            out,
            "{}isvoid: {},",
            cg::ind(3),
            if *is_void { 1 } else { 0 }
        )?;
        writeln!(out, "{}}};", cg::ind(2))?;
        writeln!(out)?;
        writeln!(
            out,
            "{}let mut sections: [iovec; {}] = [iovec {{",
            cg::ind(2),
            iovec_count
        )?;
        writeln!(out, "{}    iov_base: std::ptr::null_mut(),", cg::ind(2))?;
        writeln!(out, "{}    iov_len: 0,", cg::ind(2))?;
        writeln!(out, "{}}}; {}];", cg::ind(2), iovec_count)?;

        for p in params {
            // there are no cases where more than one field is fd, so we don't need to accumulate a length here
            match p {
                Field::Field { name, rs_typ, .. } if rs_typ == "RawFd" => {
                    writeln!(out, "{}let fds: [RawFd; 1] = [self.{}];", cg::ind(2), name)?;
                }
                Field::List { name, rs_typ, .. } if rs_typ == "RawFd" => {
                    writeln!(out, "{}let fds: &[RawFd] = self.{};", cg::ind(2), name)?;
                }
                _ => {}
            }
        }

        for (num, sect) in info.sections.iter().enumerate() {
            writeln!(out)?;
            match sect {
                SerializeSection::Fixed(fields) => {
                    self.emit_req_fixed_buf(out, num, fields, params, info, sends_event)?;
                }
                SerializeSection::Var(field) => {
                    self.emit_req_var_buf(out, num, field)?;
                }
            }
            writeln!(
                out,
                "{}sections[{}].iov_len = base::align_pad(sections[{}].iov_len, 4);",
                cg::ind(2),
                num * 2 + 3,
                num * 2 + 2
            )?;
        }

        let (unchecked_f, checked_f) = ("base::RequestFlags::NONE", "base::RequestFlags::CHECKED");
        let reply_fd_f = if info.reply_has_fd {
            " | base::RequestFlags::REPLY_FDS"
        } else {
            ""
        };

        writeln!(out)?;
        writeln!(
            out,
            "{}let flags = if checked {{ {}{} }} else {{ {}{} }};",
            cg::ind(2),
            checked_f,
            reply_fd_f,
            unchecked_f,
            reply_fd_f,
        )?;

        let func = if info.has_fd {
            "xcb_send_request_with_fds64"
        } else {
            "xcb_send_request64"
        };
        writeln!(out)?;
        writeln!(out, "{}{}(", cg::ind(2), func)?;
        writeln!(out, "{}    c.get_raw_conn(),", cg::ind(2))?;
        writeln!(out, "{}    flags.bits() as _,", cg::ind(2))?;
        writeln!(out, "{}    sections.as_mut_ptr().add(2),", cg::ind(2))?;
        writeln!(out, "{}    &mut protocol_request as *mut _,", cg::ind(2))?;
        if info.has_fd {
            writeln!(out, "{}fds.len() as _,", cg::ind(3))?;
            // xcb will not modify the content of fds, the cast is therefore safe
            writeln!(out, "{}fds.as_ptr() as *mut _,", cg::ind(3))?;
        }
        writeln!(out, "{})", cg::ind(2))?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}}}")?;

        Ok(())
    }

    fn emit_req_assertions<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        params: &[Field],
    ) -> io::Result<()> {
        for p in params {
            if let Field::Switch {
                name,
                module,
                rs_typ: f_rs_typ,
                is_mask: true,
                ..
            } = p
            {
                let q_rs_typ = (module, f_rs_typ).qualified_rs_typ();
                writeln!(
                    out,
                    "{}assert!({}::is_sorted_distinct(self.{}), \"{}::{} must be sorted!\");",
                    cg::ind(2),
                    q_rs_typ,
                    name,
                    rs_typ,
                    name
                )?;
            }
        }
        Ok(())
    }

    fn emit_req_fixed_buf<O: Write>(
        &self,
        out: &mut O,
        num: usize,
        fields: &[Field],
        params: &[Field],
        info: &RequestInfo,
        sends_event: bool,
    ) -> io::Result<()> {
        let sz = self.request_fixed_fields_size(fields);
        writeln!(
            out,
            "{}let buf{}: &mut [u8] = &mut [0; {}];",
            cg::ind(2),
            num,
            sz,
        )?;
        let mut offset = 0;
        for f in fields {
            if field_is_fd(f) {
                continue;
            }
            match f {
                Field::Field { name, .. } | Field::List { name, .. }
                    if sends_event && name == "event" =>
                {
                    writeln!(
                        out,
                        "{}let raw_{}_slice = std::slice::from_raw_parts(self.{}.as_raw() as *const u8, 32);",
                        cg::ind(2),
                        name,
                        name
                    )?;
                    writeln!(out, "{}std::slice::from_raw_parts_mut(", cg::ind(2))?;
                    writeln!(
                        out,
                        "{}    buf{}.as_mut_ptr().add({}), 32",
                        cg::ind(2),
                        num,
                        offset,
                    )?;
                    writeln!(out, "{}).copy_from_slice(raw_{}_slice);", cg::ind(2), name)?;
                    offset += 32;
                }
                Field::Field {
                    name,
                    wire_sz: Expr::Value(sz),
                    ..
                } if name == "major_opcode" || name == "minor_opcode" || name == "length" => {
                    offset += sz;
                }
                Field::Field {
                    name,
                    module,
                    rs_typ,
                    wire_sz: Expr::Value(sz),
                    r#enum,
                    mask,
                    is_fieldref,
                    is_prop_format,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    let fieldref_value = if *is_fieldref {
                        fieldref_get_value(name, params, info.has_prop_field, "self.")
                    } else {
                        None
                    };

                    let value_expr;

                    if let Some(value) = fieldref_value {
                        value_expr = format!("({} as {})", value, q_rs_typ);
                    } else if *is_prop_format {
                        value_expr = "P::FORMAT".to_string();
                    } else if mask.is_some() && rs_typ == "u32" {
                        value_expr = format!("self.{}.bits()", name);
                    } else if mask.is_some() {
                        value_expr = format!("(self.{}.bits() as {})", name, q_rs_typ);
                    } else if r#enum.is_some() && rs_typ == "u32" {
                        value_expr = format!("std::mem::transmute::<_, u32>(self.{})", name);
                    } else if r#enum.is_some() && rs_typ == "bool" {
                        value_expr = format!(
                            "(std::mem::transmute::<_, u32>(self.{}) as u{})",
                            name,
                            8 * sz
                        );
                    } else if r#enum.is_some() {
                        value_expr = format!(
                            "(std::mem::transmute::<_, u32>(self.{}) as {})",
                            name, q_rs_typ
                        );
                    } else if rs_typ == "bool" {
                        value_expr = format!(
                            "(if self.{} {{ 1u{} }} else {{ 0u{} }})",
                            name,
                            8 * sz,
                            8 * sz
                        );
                    } else {
                        value_expr = format!("self.{}", name);
                    }

                    writeln!(
                        out,
                        "{}{}.serialize(&mut buf{}[{} .. ]);",
                        cg::ind(2),
                        value_expr,
                        num,
                        offset
                    )?;

                    offset += sz;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    wire_sz: Expr::Value(sz),
                    len_expr: Expr::Value(len),
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    writeln!(out, "{}std::slice::from_raw_parts_mut(", cg::ind(2))?;
                    writeln!(
                        out,
                        "{}    buf{}.as_mut_ptr().add({}) as *mut {}, {}",
                        cg::ind(2),
                        num,
                        offset,
                        q_rs_typ,
                        len
                    )?;
                    writeln!(out, "{}).copy_from_slice(&self.{});", cg::ind(2), name)?;

                    offset += sz;
                }
                Field::Expr {
                    name,
                    typ,
                    expr,
                    wire_sz: Expr::Value(sz),
                    ..
                } if typ == "BOOL" => {
                    writeln!(out, "{}// {} expression", cg::ind(2), name)?;
                    writeln!(
                        out,
                        "{}*(buf{}.as_mut_ptr().add({}) as *mut u8) = {} as u8;",
                        cg::ind(2),
                        num,
                        offset,
                        // bad hack, but only one occurence of exprfield in the complete library
                        // no need to push further
                        self.build_rs_expr(expr, "self.", "", &[])
                            .replace("_len", ".len()"),
                    )?;
                    offset += sz;
                }
                Field::Pad {
                    wire_sz: Expr::Value(sz),
                    ..
                } => {
                    offset += sz;
                }
                _ => unreachable!("{:#?}", f),
            }
        }
        writeln!(
            out,
            "{}sections[{}].iov_base = buf{}.as_mut_ptr() as *mut _;",
            cg::ind(2),
            num * 2 + 2,
            num
        )?;
        writeln!(
            out,
            "{}sections[{}].iov_len = {};",
            cg::ind(2),
            num * 2 + 2,
            sz
        )?;
        Ok(())
    }

    fn emit_req_var_buf<O: Write>(&self, out: &mut O, num: usize, field: &Field) -> io::Result<()> {
        match field {
            Field::Field {
                name,
                struct_style: Some(StructStyle::DynBuf),
                ..
            } => {
                writeln!(
                    out,
                    "{}sections[{}].iov_base = self.{}.wire_ptr() as *mut _;",
                    cg::ind(2),
                    num * 2 + 2,
                    name,
                )?;
                writeln!(
                    out,
                    "{}sections[{}].iov_len = self.{}.wire_len();",
                    cg::ind(2),
                    num * 2 + 2,
                    name,
                )?;
            }
            Field::List {
                name,
                struct_style: Some(StructStyle::DynBuf),
                ..
            } => {
                writeln!(
                    out,
                    "{}let len{}: usize = self.{}.iter().map(|el| el.wire_len()).sum();",
                    cg::ind(2),
                    num,
                    name
                )?;
                writeln!(
                    out,
                    "{}let mut buf{} = vec![0u8; len{}];",
                    cg::ind(2),
                    num,
                    num
                )?;
                writeln!(out, "{}let mut offset = 0usize;", cg::ind(2))?;
                writeln!(out, "{}for el in self.{} {{", cg::ind(2), name)?;
                writeln!(
                    out,
                    "{}    offset += el.serialize(&mut buf{}[offset..]);",
                    cg::ind(2),
                    num
                )?;
                writeln!(out, "{}}}", cg::ind(2))?;
                writeln!(
                    out,
                    "{}sections[{}].iov_base = buf{}.as_ptr() as *mut _;",
                    cg::ind(2),
                    num * 2 + 2,
                    num
                )?;
                writeln!(
                    out,
                    "{}sections[{}].iov_len = buf{}.len();",
                    cg::ind(2),
                    num * 2 + 2,
                    num
                )?;
            }
            Field::List { name, rs_typ, .. } if rs_typ == "char" => {
                writeln!(
                    out,
                    "{}sections[{}].iov_base = self.{}.as_ptr() as *mut _;",
                    cg::ind(2),
                    num * 2 + 2,
                    name
                )?;
                writeln!(
                    out,
                    "{}sections[{}].iov_len = self.{}.len();",
                    cg::ind(2),
                    num * 2 + 2,
                    name
                )?;
            }
            Field::List {
                name,
                is_union: true,
                ..
            } => {
                writeln!(
                    out,
                    "{}let len{} = self.{}.iter().map(|el| el.wire_len()).sum::<usize>();",
                    cg::ind(2),
                    num,
                    name
                )?;
                writeln!(
                    out,
                    "{}let mut buf{} = vec![0u8; len{}];",
                    cg::ind(2),
                    num,
                    num
                )?;
                writeln!(out, "{}let mut offset{} = 0usize;", cg::ind(2), num)?;
                writeln!(out, "{}for el in self.{} {{", cg::ind(2), name)?;
                writeln!(
                    out,
                    "{}offset{} += el.serialize(&mut buf{}[offset{} ..]);",
                    cg::ind(3),
                    num,
                    num,
                    num
                )?;
                writeln!(out, "{}}}", cg::ind(2))?;
                writeln!(
                    out,
                    "{}sections[{}].iov_base = buf{}.as_ptr() as *mut _;",
                    cg::ind(2),
                    num * 2 + 2,
                    num
                )?;
                writeln!(
                    out,
                    "{}sections[{}].iov_len = buf{}.len();",
                    cg::ind(2),
                    num * 2 + 2,
                    num
                )?;
            }
            Field::List {
                name,
                module,
                rs_typ,
                is_prop,
                ..
            } => {
                writeln!(
                    out,
                    "{}sections[{}].iov_base = self.{}.as_ptr() as *mut _;",
                    cg::ind(2),
                    num * 2 + 2,
                    name,
                )?;
                let typ_sz: Cow<str> = if *is_prop {
                    Cow::Borrowed("P")
                } else {
                    Cow::Owned((module, rs_typ).qualified_rs_typ())
                };
                writeln!(
                    out,
                    "{}sections[{}].iov_len = self.{}.len() * std::mem::size_of::<{}>();",
                    cg::ind(2),
                    num * 2 + 2,
                    name,
                    typ_sz
                )?;
            }
            Field::Switch { name, .. } => {
                writeln!(
                    out,
                    "{}let len{} = self.{}.wire_len();",
                    cg::ind(2),
                    num,
                    name,
                )?;
                writeln!(
                    out,
                    "{}let mut buf{} = vec![0u8; len{}];",
                    cg::ind(2),
                    num,
                    num
                )?;
                writeln!(
                    out,
                    "{}self.{}.serialize(&mut buf{});",
                    cg::ind(2),
                    name,
                    num
                )?;
                writeln!(
                    out,
                    "{}sections[{}].iov_base = buf{}.as_ptr() as *mut _;",
                    cg::ind(2),
                    num * 2 + 2,
                    num
                )?;
                writeln!(
                    out,
                    "{}sections[{}].iov_len = buf{}.len();",
                    cg::ind(2),
                    num * 2 + 2,
                    num
                )?;
            }
            _ => unreachable!("{:#?}", field),
        }
        Ok(())
    }

    fn request_fixed_fields_size(&self, fields: &[Field]) -> usize {
        let mut sz = 0;
        for f in fields {
            if field_is_fd(f) {
                continue;
            }
            match f {
                Field::Field {
                    wire_sz: Expr::Value(s),
                    ..
                } => {
                    sz += s;
                }
                Field::List {
                    wire_sz: Expr::Value(s),
                    ..
                } => {
                    sz += s;
                }
                Field::Expr {
                    wire_sz: Expr::Value(s),
                    ..
                } => {
                    sz += s;
                }
                Field::Pad {
                    wire_sz: Expr::Value(s),
                    ..
                } => {
                    sz += s;
                }
                _ => unreachable!("{:#?}", f),
            }
        }
        sz
    }
}

#[derive(Debug)]
struct RequestInfo<'a> {
    rs_typ: String,
    cookie_rs_typ: String,
    reply_rs_typ: String,
    sections: Vec<SerializeSection<'a>>,
    has_lifetime: bool,
    opcode: u32,
    is_void: bool,
    has_prop_field: bool,
    has_fd: bool,
    reply_has_fd: bool,
}

#[derive(Debug)]
enum SerializeSection<'a> {
    Fixed(&'a [Field]),
    Var(&'a Field),
}

// When we encounter a field that is referred in eg a list len or switch mask
// we try to infer its value from the list or the switch.
// But we can manage this only for the simplest cases:
//    - a single fieldref
//    - a fieldref multiplied by format divided by 8 (for properties only)
// For other cases, we put the field as public paramater and require user to supply a value.
pub(super) fn request_fieldref_emitted(name: &str, fields: &[Field], has_prop_field: bool) -> bool {
    for f in fields {
        match f {
            Field::List {
                len_expr: Expr::FieldRef(fr),
                ..
            } if fr == name => {
                return false;
            }
            Field::List {
                len_expr: Expr::Op(op, lhs, rhs),
                ..
            } if has_prop_field && op == "*" && is_prop_field_mult(name, lhs, rhs) => {
                return false;
            }
            Field::Switch {
                expr: Expr::FieldRef(fr),
                ..
            } if fr == name => {
                return false;
            }
            _ => {}
        }
    }
    true
}

fn is_prop_field_mult<'a>(name: &'a str, lhs: &Expr, rhs: &Expr) -> bool {
    // we are in the first level of the multiplication `length * (format / 8)`
    // must reach 4 conditions:
    // - lhs is fieldref to name
    // - rhs is a division
    // - rhs::lhs is fieldref format
    // - rhs::rhs is value 8
    // box pattern matching would really be helpful, but not stabilized at time of coding

    if !matches!(lhs, Expr::FieldRef(f) if f == name) {
        return false;
    }

    match rhs {
        Expr::Op(op, lhs, rhs) if op == "/" => {
            if !matches!(&**lhs, Expr::FieldRef(f) if f == "format") {
                return false;
            }
            if !matches!(&**rhs, Expr::Value(8)) {
                return false;
            }
        }
        _ => return false,
    }

    true
}

pub(super) fn fieldref_get_value(
    name: &str,
    fields: &[Field],
    has_prop_field: bool,
    acc: &str,
) -> Option<String> {
    for f in fields {
        match f {
            Field::List {
                name: fname,
                len_expr: Expr::FieldRef(fr),
                ..
            } if fr == name => {
                return Some(format!("{}{}.len()", acc, fname));
            }
            Field::List {
                name: fname,
                len_expr: Expr::Op(op, lhs, rhs),
                ..
            } if has_prop_field && op == "*" && is_prop_field_mult(name, lhs, rhs) => {
                return Some(format!("{}{}.len()", acc, fname));
            }
            Field::Switch {
                name: fname,
                module,
                rs_typ,
                expr: Expr::FieldRef(fr),
                is_mask,
                ..
            } if fr == name => {
                let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                if *is_mask {
                    return Some(format!("{}::get_mask({}{}).bits()", q_rs_typ, acc, fname));
                } else {
                    return Some(format!(
                        "std::mem::transmute::<_, u32>({}{}.get_enum())",
                        acc, fname
                    ));
                }
            }
            _ => {}
        }
    }
    None
}

pub(super) fn field_is_fd(field: &Field) -> bool {
    matches!(
        field,
        Field::Field{rs_typ, ..} | Field::List{rs_typ, ..}
            if rs_typ == "RawFd"
    )
}
