use crate::syntax::check::Check;
use crate::syntax::{error, Api, Pair};

fn check(cx: &mut Check, name: &Pair) {
    for segment in &name.namespace {
        check_cxx_ident(cx, &segment.to_string());
    }
    check_cxx_ident(cx, &name.cxx.to_string());
    check_rust_ident(cx, &name.rust.to_string());

    fn check_cxx_ident(cx: &mut Check, ident: &str) {
        if ident.starts_with("cxxbridge") {
            cx.error(ident, error::CXXBRIDGE_RESERVED.msg);
        }
        if ident.contains("__") {
            cx.error(ident, error::DOUBLE_UNDERSCORE.msg);
        }
    }

    fn check_rust_ident(cx: &mut Check, ident: &str) {
        if ident.starts_with("cxxbridge") {
            cx.error(ident, error::CXXBRIDGE_RESERVED.msg);
        }
    }
}

pub(crate) fn check_all(cx: &mut Check, apis: &[Api]) {
    for api in apis {
        match api {
            Api::Include(_) | Api::Impl(_) => {}
            Api::Struct(strct) => {
                check(cx, &strct.name);
                for field in &strct.fields {
                    check(cx, &field.name);
                }
            }
            Api::Enum(enm) => {
                check(cx, &enm.name);
                for variant in &enm.variants {
                    check(cx, &variant.name);
                }
            }
            Api::CxxType(ety) | Api::RustType(ety) => {
                check(cx, &ety.name);
            }
            Api::CxxFunction(efn) | Api::RustFunction(efn) => {
                check(cx, &efn.name);
                for arg in &efn.args {
                    check(cx, &arg.name);
                }
            }
            Api::TypeAlias(alias) => {
                check(cx, &alias.name);
            }
        }
    }
}
