use wit_parser::abi::{AbiVariant, WasmType};
use wit_parser::{Function, Resolve, TypeDefKind, TypeId, WorldId, WorldItem};

/// Generate a dummy implementation core Wasm module for a given WIT document
pub fn dummy_module(resolve: &Resolve, world: WorldId) -> Vec<u8> {
    let world = &resolve.worlds[world];
    let mut wat = String::new();
    wat.push_str("(module\n");
    for (name, import) in world.imports.iter() {
        match import {
            WorldItem::Function(func) => {
                let sig = resolve.wasm_signature(AbiVariant::GuestImport, func);

                wat.push_str(&format!("(import \"$root\" \"{}\" (func", func.name));
                push_tys(&mut wat, "param", &sig.params);
                push_tys(&mut wat, "result", &sig.results);
                wat.push_str("))\n");
            }
            WorldItem::Interface(import) => {
                let name = resolve.name_world_key(name);
                for (_, func) in resolve.interfaces[*import].functions.iter() {
                    let sig = resolve.wasm_signature(AbiVariant::GuestImport, func);

                    wat.push_str(&format!("(import \"{name}\" \"{}\" (func", func.name));
                    push_tys(&mut wat, "param", &sig.params);
                    push_tys(&mut wat, "result", &sig.results);
                    wat.push_str("))\n");
                }
                for (_, ty) in resolve.interfaces[*import].types.iter() {
                    push_resource_func_imports(&mut wat, resolve, &name, *ty);
                }
            }
            WorldItem::Type(id) => {
                push_resource_func_imports(&mut wat, resolve, "$root", *id);
            }
        }
    }

    // Import any resource-related functions for exports.
    for (name, export) in world.exports.iter() {
        let export = match export {
            WorldItem::Interface(export) => *export,
            _ => continue,
        };
        let module = format!("[export]{}", resolve.name_world_key(name));
        for (name, ty) in resolve.interfaces[export].types.iter() {
            let ty = &resolve.types[*ty];
            match ty.kind {
                TypeDefKind::Resource => {}
                _ => continue,
            }
            wat.push_str(&format!(
                "\
(import \"{module}\" \"[resource-drop]{name}\" (func (param i32)))
(import \"{module}\" \"[resource-new]{name}\" (func (param i32) (result i32)))
(import \"{module}\" \"[resource-rep]{name}\" (func (param i32) (result i32)))
                "
            ));
        }
    }

    for (name, export) in world.exports.iter() {
        match export {
            WorldItem::Function(func) => {
                push_func(&mut wat, &func.name, resolve, func);
            }
            WorldItem::Interface(export) => {
                let name = resolve.name_world_key(name);
                for (_, func) in resolve.interfaces[*export].functions.iter() {
                    let name = func.core_export_name(Some(&name));
                    push_func(&mut wat, &name, resolve, func);
                }

                // Feign destructors for any resource that this interface
                // exports
                for (resource_name, ty) in resolve.interfaces[*export].types.iter() {
                    let ty = &resolve.types[*ty];
                    match ty.kind {
                        TypeDefKind::Resource => {}
                        _ => continue,
                    }
                    wat.push_str(&format!(
                        "(func (export \"{name}#[dtor]{resource_name}\") (param i32))"
                    ));
                }
            }
            WorldItem::Type(_) => {}
        }
    }

    wat.push_str("(memory (export \"memory\") 0)\n");
    wat.push_str(
        "(func (export \"cabi_realloc\") (param i32 i32 i32 i32) (result i32) unreachable)\n",
    );
    wat.push_str(")\n");

    return wat::parse_str(&wat).unwrap();

    fn push_resource_func_imports(wat: &mut String, resolve: &Resolve, module: &str, ty: TypeId) {
        let ty = &resolve.types[ty];
        match ty.kind {
            TypeDefKind::Resource => {}
            _ => return,
        }
        let name = ty.name.as_ref().unwrap();
        wat.push_str(&format!("(import \"{module}\" \"[resource-drop]{name}\""));
        wat.push_str(" (func (param i32)))\n");
    }

    fn push_func(wat: &mut String, name: &str, resolve: &Resolve, func: &Function) {
        let sig = resolve.wasm_signature(AbiVariant::GuestExport, func);
        wat.push_str(&format!("(func (export \"{name}\")"));
        push_tys(wat, "param", &sig.params);
        push_tys(wat, "result", &sig.results);
        wat.push_str(" unreachable)\n");
    }

    fn push_tys(dst: &mut String, desc: &str, params: &[WasmType]) {
        if params.is_empty() {
            return;
        }
        dst.push_str(" (");
        dst.push_str(desc);
        for ty in params {
            dst.push(' ');
            match ty {
                WasmType::I32 => dst.push_str("i32"),
                WasmType::I64 => dst.push_str("i64"),
                WasmType::F32 => dst.push_str("f32"),
                WasmType::F64 => dst.push_str("f64"),
                WasmType::Pointer => dst.push_str("i32"),
                WasmType::PointerOrI64 => dst.push_str("i64"),
                WasmType::Length => dst.push_str("i32"),
            }
        }
        dst.push(')');
    }
}
