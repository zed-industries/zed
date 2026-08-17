use crate::ir;

use std::collections::HashMap;
use std::io::{self, Write};

use self::util::extract_module;

mod doc;
mod r#enum;
mod error;
mod event;
mod expr;
mod request;
mod r#struct;
mod switch;
mod union;
mod util;
mod xid;

use doc::{Doc, DocField};
use expr::Expr;
use r#struct::ParamsStruct;

/// Information about a type
/// for each variant
///     - module is the module where the type is defined (None for current module)
///     - rs_typ is the rust traduction of the type
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
enum TypeInfo {
    /// simple copyable types such as u32
    Simple {
        rs_typ: String,
        wire_sz: usize,
        has_wire_layout: bool,
    },
    Typedef {
        module: Option<String>,
        rs_typ: String,
        old_module: Option<String>,
        old_typ: String,
        wire_sz: Expr,
        has_wire_layout: bool,
    },
    Xid {
        module: Option<String>,
        rs_typ: String,
    },
    XidUnion {
        module: Option<String>,
        rs_typ: String,
        variants: Vec<UnionVariant>,
    },
    Enum {
        module: Option<String>,
        rs_typ: String,
        items: Vec<(String, u32, Option<String>)>,
        altenum_typ: Option<(Option<String>, String)>,
        doc: Option<Doc>,
    },
    Mask {
        module: Option<String>,
        rs_typ: String,
        items: Vec<(String, u32, Option<String>)>,
        doc: Option<Doc>,
    },
    Struct {
        module: Option<String>,
        rs_typ: String,
        fields: Vec<Field>,
        wire_sz: Expr,
        has_wire_layout: bool,
        params_struct: Option<ParamsStruct>,
        doc: Option<Doc>,
    },
    Union {
        module: Option<String>,
        rs_typ: String,
        variants: Vec<UnionVariant>,
        wire_sz: Expr,
        type_field: Option<UnionTypeField>,
        impl_clone: bool,
        emit: bool,
    },
    Switch {
        module: Option<String>,
        rs_typ: String,
        expr: Expr,
        cases: Vec<SwitchCase>,
        maskenum: (Option<String>, String),
        params_struct: ParamsStruct,
        wire_sz: Expr,
        is_mask: bool,
        emit: bool,
    },
}

#[derive(Clone, Debug)]
struct SwitchCase {
    name: String,
    exprs: Vec<Expr>,
    fields: Vec<Field>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug)]
enum Field {
    Field {
        name: String,
        module: Option<String>,
        rs_typ: String,
        wire_off: Expr,
        wire_sz: Expr,
        struct_style: Option<StructStyle>,
        params_struct: Option<ParamsStruct>,
        r#enum: Option<(Option<String>, String)>,
        mask: Option<(Option<String>, String)>,
        doc: Option<DocField>,
        is_fieldref: bool,
        is_paramref: bool,
        is_copy: bool,
        is_union: bool,
        is_xid: bool,
        is_mask: bool,
        need_compute_offset: bool,
        is_prop_format: bool, // format field for a property
    },
    List {
        name: String,
        module: Option<String>,
        rs_typ: String,
        wire_off: Expr,
        wire_sz: Expr,
        len_expr: Expr,
        struct_style: Option<StructStyle>,
        params_struct: Option<ParamsStruct>,
        r#enum: Option<(Option<String>, String)>,
        mask: Option<(Option<String>, String)>,
        doc: Option<DocField>,
        is_fieldref: bool,
        need_compute_offset: bool,
        is_prop: bool, // property field (resolved with type `void` and a format field is present)
        is_union: bool,
        union_typefield: Option<UnionTypeField>,
    },
    Switch {
        name: String,
        module: Option<String>,
        params_struct: ParamsStruct,
        rs_typ: String,
        expr: Expr,
        wire_off: Expr,
        wire_sz: Expr,
        doc: Option<DocField>,
        is_mask: bool,
        need_compute_offset: bool,
    },
    Expr {
        name: String,
        typ: String,
        wire_sz: Expr,
        expr: Expr,
    },
    Pad {
        wire_sz: Expr,
    },
    AlignPad {
        wire_sz: Expr,
    },
}

#[derive(Copy, Clone, Debug)]
enum StructStyle {
    WireLayout,
    FixBuf,
    DynBuf,
}

#[derive(Clone, Debug)]
struct UnionVariant {
    variant: String,
    module: Option<String>,
    content: UnionVariantContent,
}

#[derive(Clone, Debug)]
struct UnionTypeField {
    offset: usize,
    enu_module: Option<String>,
    enu_typ: String,
}

#[derive(Clone, Debug)]
enum UnionVariantContent {
    RsTyp(String),
    Array(String, usize),
    Struct(Vec<Field>),
}

#[derive(Clone, Debug)]
struct Error {
    rs_typ: String,
    copy_from_rs_typ: Option<String>,
    variant: String,
    number: i32,
    fields: Vec<Field>,
    wire_sz: Expr,
}

#[derive(Clone, Debug)]
struct Reply {
    fields: Vec<Field>,
    doc: Option<Doc>,
}

#[derive(Clone, Debug)]
struct Request {
    rs_typ: String,
    opcode: u32,
    params: Vec<Field>,
    reply: Option<Reply>,
    sends_event: bool,
    doc: Option<Doc>,
}

#[derive(Clone, Debug)]
struct Event {
    rs_typ: String,
    variant: String,
    number: i32,
    fields: Vec<Field>,
    copy_from_rs_typ: Option<String>,
    wire_sz: Expr,
    is_xge: bool,
    doc: Option<Doc>,
}

#[derive(Debug, Clone)]
struct ExtInfo {
    pub rs_name: String,
    pub name: String,
    pub xname: String,
    pub major_version: u32,
    pub minor_version: u32,
}

#[derive(Clone, Debug)]
pub struct DepInfo {
    pub xcb_mod: String,
    pub deps: Vec<String>,
    typinfos: HashMap<String, TypeInfo>,
    errors: Vec<Error>,
    events: Vec<Event>,
}

#[derive(Debug)]
pub struct CodeGen {
    xcb_mod: String,
    ext_info: Option<ExtInfo>,
    depinfo: Vec<DepInfo>,
    typinfos: HashMap<String, TypeInfo>,
    typs: Vec<String>,
    rs_typs: HashMap<String, u32>,
    // struct and typedefs that are only used in unions are defined as variants and removed everywhere else
    // we keep track of whether e.g. struct or typedefs must be emitted through the need_count
    rs_typs_need_count: HashMap<String, i32>,
    errors: Vec<Error>,
    errors_preregistered: bool,
    requests: Vec<Request>,
    events: Vec<Event>,
    mask_exceptions: Vec<RsTypException>,
    switch_exceptions: Vec<RsTypException>,
    dbg_atom_names: bool,
}

impl CodeGen {
    pub fn new(
        xcb_mod: String,
        ext_info: &Option<ir::ExtInfo>,
        depinfo: Vec<DepInfo>,
        dbg_atom_names: bool,
    ) -> CodeGen {
        let simples = &[
            ("CARD8", "u8", 1, true),
            ("CARD16", "u16", 2, true),
            ("CARD32", "u32", 4, true),
            ("CARD64", "u64", 8, true),
            ("INT8", "i8", 1, true),
            ("INT16", "i16", 2, true),
            ("INT32", "i32", 4, true),
            ("BYTE", "u8", 1, true),
            ("BOOL", "bool", 1, false),
            ("BOOL32", "bool", 4, false),
            ("char", "char", 1, false),
            ("float", "f32", 4, true),
            ("double", "f64", 8, true),
            ("FLOAT32", "f32", 4, true),
            ("FLOAT64", "f64", 8, true),
            ("void", "u8", 1, true), // void lists are blob of bytes
            ("fd", "RawFd", 4, true),
        ];

        let typinfos = simples
            .iter()
            .map(|(typ, rs_typ, wire_sz, has_wire_layout)| {
                (
                    typ.to_string(),
                    TypeInfo::Simple {
                        rs_typ: rs_typ.to_string(),
                        wire_sz: *wire_sz,
                        has_wire_layout: *has_wire_layout,
                    },
                )
            })
            .collect();

        let ext_info = ext_info.as_ref().map(|ei| ExtInfo {
            rs_name: rust_variant_name(&ei.name),
            name: ei.name.clone(),
            xname: ei.xname.clone(),
            major_version: ei.major_version,
            minor_version: ei.minor_version,
        });

        CodeGen {
            xcb_mod,
            ext_info,
            depinfo,
            typinfos,
            typs: Vec::new(),
            rs_typs: HashMap::new(),
            rs_typs_need_count: HashMap::new(),
            errors: Vec::new(),
            errors_preregistered: false,
            requests: Vec::new(),
            events: Vec::new(),
            mask_exceptions: mask_exceptions(),
            switch_exceptions: switch_exceptions(),
            dbg_atom_names,
        }
    }

    pub fn into_depinfo(self) -> DepInfo {
        let CodeGen {
            xcb_mod,
            depinfo,
            mut typinfos,
            errors,
            events,
            ..
        } = self;
        let deps = depinfo.into_iter().map(|di| di.xcb_mod).collect();
        for (_, typinfo) in typinfos.iter_mut() {
            match typinfo {
                TypeInfo::Typedef { module, .. }
                | TypeInfo::Xid { module, .. }
                | TypeInfo::XidUnion { module, .. }
                | TypeInfo::Enum { module, .. }
                | TypeInfo::Mask { module, .. }
                | TypeInfo::Struct { module, .. }
                | TypeInfo::Union { module, .. } => *module = Some(xcb_mod.clone()),
                _ => {}
            }
        }
        DepInfo {
            xcb_mod,
            deps,
            typinfos,
            errors,
            events,
        }
    }

    pub fn preregister_item(&mut self, item: &ir::Item) {
        match item {
            ir::Item::XidType { typ }
            | ir::Item::XidUnion { typ, .. }
            | ir::Item::Enum { typ, .. } => {
                let rs_typ = rust_type_name(typ);
                self.preregister_typ(rs_typ);
            }
            ir::Item::Error { .. } | ir::Item::ErrorCopy { .. } => {
                if !self.errors_preregistered {
                    self.preregister_typ("Error".into());
                    self.errors_preregistered = true;
                }
            }
            _ => {}
        }
    }

    pub fn resolve_type(&mut self, item: &ir::Item) {
        match item {
            ir::Item::Typedef { old_typ, new_typ } => {
                self.resolve_typedef(old_typ, new_typ);
            }
            ir::Item::XidType { typ } => {
                self.resolve_xid(typ);
            }
            ir::Item::XidUnion { typ, xidtypes } => {
                self.resolve_xidunion(typ, xidtypes);
            }
            ir::Item::Enum { typ, items, doc } => {
                self.resolve_enum(typ, items, doc);
            }
            ir::Item::Struct { typ, fields, doc } => {
                self.resolve_struct(typ, fields, doc);
            }
            ir::Item::Union { typ, fields, doc } => {
                self.resolve_union(typ, fields, doc);
            }
            _ => {}
        }
    }

    pub fn resolve_error_event_request(&mut self, item: ir::Item) {
        match item {
            ir::Item::Error {
                name,
                number,
                fields,
                doc,
            } => self.resolve_error(name, number, fields, doc),
            ir::Item::ErrorCopy {
                name,
                number,
                r#ref,
            } => self.resolve_errorcopy(name, number, r#ref),

            ir::Item::Request {
                name,
                opcode,
                params,
                reply,
                doc,
            } => self.resolve_request(name, opcode, params, reply, doc),

            ir::Item::Event {
                number,
                name,
                fields,
                xge,
                no_seq_number,
                doc,
            } => self.resolve_event(name, number, fields, xge, no_seq_number, doc),
            ir::Item::EventCopy {
                name,
                number,
                r#ref,
            } => self.resolve_eventcopy(name, number, r#ref),

            ir::Item::EventStruct { typ, selectors } => {
                assert!(!selectors.is_empty());
                self.resolve_event_struct(typ, selectors);
            }

            _ => {}
        }
    }

    pub fn emit_prologue<O: Write>(&self, out: &mut O) -> io::Result<()> {
        writeln!(
            out,
            "// This source file is generated automatically from {}.xml",
            self.xcb_mod
        )?;
        writeln!(out)?;
        writeln!(
            out,
            "use crate::base::{{self, BaseError, BaseEvent, GeEvent, Raw, Reply, WiredIn, WiredOut, Xid}};"
        )?;
        writeln!(out, "use crate::ext;")?;
        writeln!(out, "use crate::ffi::base::*;")?;
        writeln!(out, "use crate::ffi::ext::*;")?;
        writeln!(
            out,
            "use crate::lat1_str::{{Lat1Str, Lat1String, Lat1StrF}};"
        )?;
        for di in &self.depinfo {
            writeln!(out, "use crate::{};", di.xcb_mod)?;
        }
        if self.ext_info.is_some() {
            writeln!(out, "use crate::xproto::PropEl;")?;
        }
        writeln!(out)?;
        writeln!(out, "use bitflags::bitflags;")?;
        writeln!(out, "use libc::{{self, iovec}};")?;
        writeln!(out, "use std::convert::TryInto;")?;
        writeln!(out, "use std::hash::{{Hash, Hasher}};")?;
        writeln!(out, "use std::os::unix::io::RawFd;")?;
        if self.xcb_mod == "xproto" {
            writeln!(out, "use std::cmp::Ordering;")?;
        }

        if let Some(ext_info) = self.ext_info.as_ref() {
            writeln!(out)?;
            writeln!(
                out,
                "/// The official identifier for the `{}` extension.",
                ext_info.name
            )?;
            writeln!(out, "pub const XNAME: &str = \"{}\";", ext_info.xname)?;
            writeln!(
                out,
                "/// The major version of the `{}` extension.",
                ext_info.name
            )?;
            writeln!(
                out,
                "pub const MAJOR_VERSION: u32 = {};",
                ext_info.major_version
            )?;
            writeln!(
                out,
                "/// The minor version of the `{}` extension.",
                ext_info.name
            )?;
            writeln!(
                out,
                "pub const MINOR_VERSION: u32 = {};",
                ext_info.minor_version
            )?;
            writeln!(
                out,
                "/// The version string of the `{}` extension.",
                ext_info.name
            )?;
            writeln!(
                out,
                "pub const VERSION_STRING: &str = \"{}.{}\";",
                ext_info.major_version, ext_info.minor_version
            )?;

            writeln!(out)?;
            writeln!(
                out,
                "pub(crate) static mut FFI_EXT: xcb_extension_t = xcb_extension_t {{"
            )?;
            writeln!(
                out,
                "    name: \"{}\\0\".as_ptr() as *const _,",
                ext_info.xname
            )?;
            writeln!(out, "    global_id: 0,")?;
            writeln!(out, "}};")?;

            writeln!(out)?;
            writeln!(
                out,
                "/// Prefetch server runtime info data of the `{}` extension.",
                ext_info.name
            )?;
            writeln!(
                out,
                "pub fn prefetch_extension_data(conn: &base::Connection) {{"
            )?;
            writeln!(out, "    unsafe {{")?;
            writeln!(
                out,
                "        xcb_prefetch_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));"
            )?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;

            writeln!(out)?;
            writeln!(
                out,
                "/// Fetch server runtime info data of the `{}` extension.",
                ext_info.name
            )?;
            writeln!(out, "///")?;
            writeln!(
                out,
                "/// Might be non-blocking if [prefetch_extension_data] was called before."
            )?;
            writeln!(
                out,
                "/// This function is of seldom use as the extensions are initialized by the"
            )?;
            writeln!(out, "/// [Connection](crate::Connection) constructor.")?;
            writeln!(
                out,
                "pub fn get_extension_data(conn: &base::Connection) -> std::option::Option<ext::ExtensionData> {{"
            )?;
            writeln!(out, "    unsafe {{")?;
            writeln!(
                out,
                "        let reply = xcb_get_extension_data(conn.get_raw_conn(), std::ptr::addr_of_mut!(FFI_EXT));"
            )?;
            writeln!(
                out,
                "        assert!(!reply.is_null(), \"Could not fetch {} extension data\");",
                ext_info.name
            )?;
            writeln!(
                out,
                "        let reply = xproto::QueryExtensionReply::from_raw(reply);"
            )?;
            writeln!(out, "        if !reply.present() {{")?;
            writeln!(out, "            std::mem::forget(reply);")?;
            writeln!(out, "            return None;")?;
            writeln!(out, "        }}")?;
            writeln!(out, "        let res = ext::ExtensionData{{")?;
            writeln!(
                out,
                "            ext: ext::Extension::{},",
                ext_info.rs_name
            )?;
            writeln!(out, "            major_opcode: reply.major_opcode(),")?;
            writeln!(out, "            first_event: reply.first_event(),")?;
            writeln!(out, "            first_error: reply.first_error(),")?;
            writeln!(out, "        }};")?;
            writeln!(out, "        std::mem::forget(reply);")?;
            writeln!(out, "        Some(res)")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }

        Ok(())
    }

    pub fn emit_types<O: Write>(&self, out: &mut O) -> io::Result<()> {
        for typ in &self.typs {
            let typinfo = &self.typinfos[typ];
            match typinfo {
                TypeInfo::Typedef {
                    rs_typ, old_typ, ..
                } => {
                    self.emit_typedef(out, rs_typ, old_typ)?;
                }
                TypeInfo::Xid { rs_typ, .. } => {
                    self.emit_xid(out, rs_typ)?;
                }
                TypeInfo::XidUnion {
                    rs_typ, variants, ..
                } => {
                    self.emit_xidunion(out, rs_typ, variants)?;
                }
                TypeInfo::Enum {
                    rs_typ,
                    items,
                    altenum_typ,
                    doc,
                    ..
                } => {
                    self.emit_enum(out, rs_typ, items, altenum_typ, doc.as_ref())?;
                }
                TypeInfo::Mask {
                    rs_typ, items, doc, ..
                } => {
                    self.emit_mask(out, rs_typ, items, doc.as_ref())?;
                }
                TypeInfo::Struct {
                    rs_typ,
                    fields,
                    wire_sz,
                    has_wire_layout,
                    params_struct,
                    doc,
                    ..
                } => {
                    self.emit_struct(
                        out,
                        rs_typ,
                        fields,
                        wire_sz,
                        *has_wire_layout,
                        params_struct.as_ref(),
                        doc.as_ref(),
                    )?;
                }
                TypeInfo::Union {
                    rs_typ,
                    variants,
                    wire_sz,
                    type_field,
                    impl_clone,
                    emit: true,
                    ..
                } => {
                    self.emit_union(out, rs_typ, variants, wire_sz, type_field, *impl_clone)?;
                }
                TypeInfo::Switch {
                    rs_typ,
                    expr,
                    cases,
                    maskenum,
                    emit: true,
                    is_mask,
                    params_struct,
                    ..
                } => {
                    self.emit_switch(out, rs_typ, expr, cases, maskenum, params_struct, *is_mask)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn resolve_typedef(&mut self, old_typ: &str, new_typ: &str) {
        match new_typ {
            // this one handled has built-in simple type to get a bool interface
            "BOOL32"
            // these two ones are unnecessary in Rust
            | "FLOAT32"
            | "FLOAT64" => { return; }
            _ => {}
        }
        let old_typ = match new_typ {
            "STRING8" => "CARD8",
            _ => old_typ,
        };
        if self.xcb_mod == "xinput" && new_typ == "DeviceId" {
            self.handle_xinput_deviceid_typedef();
            return;
        }
        let rs_typ = rust_type_name(new_typ);
        let (old_module, old_typ) = extract_module(old_typ);
        let old_typinfo = self.find_typinfo(old_module, old_typ);
        let wire_sz = old_typinfo.wire_sz();
        let has_wire_layout = old_typinfo.has_wire_layout();
        let typinfo = TypeInfo::Typedef {
            module: None, // this the where typedef is defined, not where old_typ is defined
            rs_typ,
            old_module: old_module.map(str::to_owned),
            old_typ: old_typ.to_string(),
            wire_sz,
            has_wire_layout,
        };
        self.register_typ(new_typ.to_string(), typinfo);
    }

    fn emit_typedef<O: Write>(&self, out: &mut O, rs_typ: &str, old_typ: &str) -> io::Result<()> {
        if !self.rs_typ_is_needed(rs_typ) {
            return Ok(());
        }
        let old_rs_typ = self.typinfos[old_typ].rs_typ();
        writeln!(out)?;
        writeln!(out, "pub type {} = {};", rs_typ, old_rs_typ)?;
        Ok(())
    }

    fn emit_sizeof_test<O: Write>(&self, out: &mut O, rs_typ: &str, sz: usize) -> io::Result<()> {
        writeln!(out)?;
        writeln!(out, "#[test]")?;
        writeln!(
            out,
            "fn test_sizeof_{}() {{",
            util::tit_split(rs_typ).to_lowercase()
        )?;
        writeln!(
            out,
            "    assert_eq!(std::mem::size_of::<{}>(), {});",
            rs_typ, sz
        )?;
        writeln!(out, "}}")?;
        Ok(())
    }

    fn extract_module<'a>(&self, typ: &'a str) -> (Option<&'a str>, &'a str) {
        let (mut module, typ) = util::extract_module(typ);
        if let Some(m) = module {
            if m == self.xcb_mod {
                module = None;
            }
        }
        (module, typ)
    }

    fn preregister_typ(&mut self, rs_typ: String) {
        if let Some(cnt) = self.rs_typs.get_mut(&rs_typ) {
            *cnt += 1;
        } else {
            self.rs_typs.insert(rs_typ, 1);
        }
    }

    fn register_typ(&mut self, typ: String, info: TypeInfo) {
        self.rs_typs_need_count.insert(info.rs_typ().to_string(), 0);
        self.typinfos.insert(typ.clone(), info);
        self.typs.push(typ.clone());
        // we insert 0 and use the function to handle typedefs
        self.typ_need_add(&typ, 1);
    }

    fn typ_need_add(&mut self, typ: &str, val: i32) {
        let typinfo = &self.typinfos[typ];
        let rs_typ = typinfo.rs_typ();

        if let Some(count) = self.rs_typs_need_count.get_mut(rs_typ) {
            *count += val;
            if let Some((None, old_typ)) = typinfo.typedef_old_mod_typ() {
                let old_typ = old_typ.to_string();
                self.typ_need_add(&old_typ, val);
            }
        }
    }

    fn rs_typ_is_needed(&self, rs_typ: &str) -> bool {
        if let Some(count) = self.rs_typs_need_count.get(rs_typ) {
            *count > 0
        } else {
            false
        }
    }

    fn register_altenum_typ(
        &mut self,
        enum_typ: &str,
        module: Option<&str>,
        new_altenum_typ: &str,
    ) {
        let typinfo = self.find_typinfo_mut(None, enum_typ);
        if let Some(TypeInfo::Enum { altenum_typ, .. }) = typinfo {
            *altenum_typ = Some((module.map(str::to_owned), new_altenum_typ.into()))
        }
    }

    fn get_depinfo(&self, module: &str) -> &DepInfo {
        self.depinfo
            .iter()
            .find(|&di| di.xcb_mod == module)
            .unwrap_or_else(|| panic!("cannot find dependency module {}", module))
    }

    fn find_typinfo(&self, module: Option<&str>, typ: &str) -> &TypeInfo {
        if let Some(module) = module {
            if module == self.xcb_mod {
                &self.typinfos[typ]
            } else {
                &self.get_depinfo(module).typinfos[typ]
            }
        } else if let Some(typinfo) = self.typinfos.get(typ) {
            typinfo
        } else {
            self.depinfo
                .iter()
                .find_map(|di| di.typinfos.get(typ))
                .unwrap_or_else(|| panic!("could not resolve typeinfo {:?}::{}", module, typ))
        }
    }

    fn find_typinfo_mut(&mut self, module: Option<&str>, typ: &str) -> Option<&mut TypeInfo> {
        if let Some(module) = module {
            if module == self.xcb_mod {
                self.typinfos.get_mut(typ)
            } else {
                None
            }
        } else {
            self.typinfos.get_mut(typ)
        }
    }

    /// Same as find_typinfo, but recurse down in case the result is a typedef
    fn find_typinfo_recurse(&self, module: Option<&str>, typ: &str) -> &TypeInfo {
        let typinfo = self.find_typinfo(module, typ);
        match typinfo {
            TypeInfo::Typedef {
                old_module,
                old_typ,
                ..
            } => self.find_typinfo_recurse(old_module.as_ref().map(|m| m.as_str()), old_typ),
            typinfo => typinfo,
        }
    }

    fn handle_xinput_deviceid_typedef(&mut self) {
        let typinfo = TypeInfo::Union {
            module: None,
            rs_typ: "Device".into(),
            variants: Vec::new(),
            wire_sz: Expr::Value(2),
            type_field: None,
            impl_clone: true,
            emit: false,
        };
        self.register_typ("DeviceId".into(), typinfo);
    }

    fn handle_xinput_device_enum(&self) {
        // Do not emit anything
    }
}

/// Describe the exceptions in switch and associated enum naming
#[derive(Copy, Clone, Debug)]
struct RsTypException {
    module: &'static str,
    /// Computed rs_typ
    rs_typ: &'static str,
    /// What is the new name?
    new_module: Option<&'static str>,
    /// What is the new name?
    new_rs_typ: &'static str,
    /// Do we emit it?
    emit: bool,
}

fn switch_exceptions() -> Vec<RsTypException> {
    vec![
        RsTypException {
            module: "xproto",
            rs_typ: "CreateWindowValueList",
            new_module: None,
            new_rs_typ: "Cw",
            emit: true,
        },
        RsTypException {
            module: "xproto",
            rs_typ: "ChangeWindowAttributesValueList",
            new_module: None,
            new_rs_typ: "Cw",
            emit: false,
        },
        RsTypException {
            module: "xproto",
            rs_typ: "ConfigureWindowValueList",
            new_module: None,
            new_rs_typ: "ConfigWindow",
            emit: true,
        },
        RsTypException {
            module: "xproto",
            rs_typ: "CreateGcValueList",
            new_module: None,
            new_rs_typ: "Gc",
            emit: true,
        },
        RsTypException {
            module: "xproto",
            rs_typ: "ChangeGcValueList",
            new_module: None,
            new_rs_typ: "Gc",
            emit: false,
        },
        RsTypException {
            module: "xproto",
            rs_typ: "ChangeKeyboardControlValueList",
            new_module: None,
            new_rs_typ: "Kb",
            emit: true,
        },
        RsTypException {
            module: "screensaver",
            rs_typ: "SetAttributesValueList",
            new_module: Some("xproto"),
            new_rs_typ: "Cw",
            emit: false,
        },
        RsTypException {
            module: "render",
            rs_typ: "CreatePictureValueList",
            new_module: None,
            new_rs_typ: "Cp",
            emit: true,
        },
        RsTypException {
            module: "render",
            rs_typ: "ChangePictureValueList",
            new_module: None,
            new_rs_typ: "Cp",
            emit: false,
        },
        RsTypException {
            module: "sync",
            rs_typ: "CreateAlarmValueList",
            new_module: None,
            new_rs_typ: "Ca",
            emit: true,
        },
        RsTypException {
            module: "sync",
            rs_typ: "ChangeAlarmValueList",
            new_module: None,
            new_rs_typ: "Ca",
            emit: false,
        },
    ]
}

fn mask_exceptions() -> Vec<RsTypException> {
    vec![
        RsTypException {
            module: "xproto",
            rs_typ: "Cw",
            new_module: None,
            new_rs_typ: "CwMask",
            emit: true,
        },
        RsTypException {
            module: "xproto",
            rs_typ: "ConfigWindow",
            new_module: None,
            new_rs_typ: "ConfigWindowMask",
            emit: true,
        },
        RsTypException {
            module: "xproto",
            rs_typ: "Gc",
            new_module: None,
            new_rs_typ: "GcMask",
            emit: true,
        },
        RsTypException {
            module: "xproto",
            rs_typ: "Kb",
            new_module: None,
            new_rs_typ: "KbMask",
            emit: true,
        },
        RsTypException {
            module: "render",
            rs_typ: "Cp",
            new_module: None,
            new_rs_typ: "CpMask",
            emit: true,
        },
        RsTypException {
            module: "sync",
            rs_typ: "Ca",
            new_module: None,
            new_rs_typ: "CaMask",
            emit: true,
        },
    ]
}

fn ind(level: u32) -> &'static str {
    match level {
        0 => "",
        1 => "    ",
        2 => "        ",
        3 => "            ",
        4 => "                ",
        5 => "                    ",
        6 => "                        ",
        _ => unreachable!(),
    }
}

fn rust_type_name(typ: &str) -> String {
    util::tit_cap(typ)
}

fn rust_variant_name(name: &str) -> String {
    util::tit_cap(name)
}

static KEYWORDS: &[&str] = &["type", "match"];

fn rust_field_name(name: &str) -> String {
    let mut name = util::tit_split(name).to_lowercase();
    if KEYWORDS.contains(&name.as_str()) {
        name.insert_str(0, "r#");
    }
    name
}

fn get_struct_style(has_wire_layout: bool, wire_sz: &Expr) -> StructStyle {
    match (has_wire_layout, wire_sz) {
        (true, Expr::Value(_)) => StructStyle::WireLayout,
        (false, Expr::Value(_)) => StructStyle::FixBuf,
        (true, _) => unreachable!(),
        _ => StructStyle::DynBuf,
    }
}

impl TypeInfo {
    fn module(&self) -> Option<&str> {
        match self {
            TypeInfo::Simple { .. } => None,
            TypeInfo::Typedef { module, .. }
            | TypeInfo::Xid { module, .. }
            | TypeInfo::XidUnion { module, .. }
            | TypeInfo::Enum { module, .. }
            | TypeInfo::Mask { module, .. }
            | TypeInfo::Struct { module, .. }
            | TypeInfo::Union { module, .. }
            | TypeInfo::Switch { module, .. } => module.as_ref().map(|m| m.as_str()),
        }
    }
    fn typedef_old_mod_typ(&self) -> Option<(Option<&str>, &str)> {
        match self {
            TypeInfo::Typedef {
                old_module,
                old_typ,
                ..
            } => Some((old_module.as_ref().map(|m| m.as_str()), old_typ)),
            _ => None,
        }
    }
    fn struct_style(&self) -> Option<StructStyle> {
        match self {
            TypeInfo::Struct {
                has_wire_layout,
                wire_sz,
                ..
            } => Some(get_struct_style(*has_wire_layout, wire_sz)),
            _ => None,
        }
    }
}

type ModRsTyp<'a> = (Option<&'a str>, &'a str);

trait RsTyp {
    fn rs_typ(&self) -> &str;
}

trait QualifiedRsTyp {
    fn qualified_rs_typ(&self) -> String;
}

trait AsModRsTyp {
    fn as_mod_rs_typ(&self) -> ModRsTyp<'_>;
}

trait WireSz {
    fn wire_sz(&self) -> Expr;
}

trait HasWireLayout {
    fn has_wire_layout(&self) -> bool;
}

impl RsTyp for TypeInfo {
    fn rs_typ(&self) -> &str {
        match self {
            TypeInfo::Simple { rs_typ, .. }
            | TypeInfo::Typedef { rs_typ, .. }
            | TypeInfo::Xid { rs_typ, .. }
            | TypeInfo::XidUnion { rs_typ, .. }
            | TypeInfo::Enum { rs_typ, .. }
            | TypeInfo::Mask { rs_typ, .. }
            | TypeInfo::Struct { rs_typ, .. }
            | TypeInfo::Union { rs_typ, .. }
            | TypeInfo::Switch { rs_typ, .. } => rs_typ,
        }
    }
}

impl RsTyp for ModRsTyp<'_> {
    fn rs_typ(&self) -> &str {
        self.1
    }
}

impl QualifiedRsTyp for ModRsTyp<'_> {
    fn qualified_rs_typ(&self) -> String {
        match &self.0 {
            Some(module) => module.to_string() + "::" + self.1,
            None => self.1.to_string(),
        }
    }
}

impl<T: AsModRsTyp> QualifiedRsTyp for T {
    fn qualified_rs_typ(&self) -> String {
        self.as_mod_rs_typ().qualified_rs_typ()
    }
}

impl<'a> AsModRsTyp for (&'a Option<String>, &String) {
    fn as_mod_rs_typ(&self) -> ModRsTyp<'_> {
        (self.0.as_ref().map(|m| m.as_str()), self.1)
    }
}

impl AsModRsTyp for TypeInfo {
    fn as_mod_rs_typ(&self) -> ModRsTyp<'_> {
        (self.module(), self.rs_typ())
    }
}

// impl AsModRsTyp for UnionVariant {
//     fn as_mod_rs_typ(&self) -> ModRsTyp<'_> {
//         (self.module.as_ref().map(|m| m.as_str()), &self.rs_typ)
//     }
// }

impl WireSz for Field {
    fn wire_sz(&self) -> Expr {
        match self {
            Field::Field { wire_sz, .. }
            | Field::List { wire_sz, .. }
            | Field::Switch { wire_sz, .. }
            | Field::Expr { wire_sz, .. }
            | Field::Pad { wire_sz, .. }
            | Field::AlignPad { wire_sz, .. } => wire_sz.clone(),
        }
    }
}

impl WireSz for TypeInfo {
    fn wire_sz(&self) -> Expr {
        match self {
            TypeInfo::Simple { wire_sz, .. } => Expr::Value(*wire_sz),
            TypeInfo::Typedef { wire_sz, .. }
            | TypeInfo::Struct { wire_sz, .. }
            | TypeInfo::Union { wire_sz, .. }
            | TypeInfo::Switch { wire_sz, .. } => wire_sz.clone(),
            TypeInfo::Xid { .. }
            | TypeInfo::XidUnion { .. }
            | TypeInfo::Enum { .. }
            | TypeInfo::Mask { .. } => Expr::Value(4),
        }
    }
}

impl HasWireLayout for TypeInfo {
    fn has_wire_layout(&self) -> bool {
        match self {
            TypeInfo::Simple {
                has_wire_layout, ..
            }
            | TypeInfo::Typedef {
                has_wire_layout, ..
            }
            | TypeInfo::Struct {
                has_wire_layout, ..
            } => *has_wire_layout,
            TypeInfo::Xid { .. } => true,
            _ => false,
        }
    }
}
