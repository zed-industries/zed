use wit_parser::abi::WasmType;
use wit_parser::{
    Function, LiftLowerAbi, ManglingAndAbi, Resolve, ResourceIntrinsic, TypeDefKind, TypeId,
    WasmExport, WasmExportKind, WasmImport, WorldId, WorldItem, WorldKey,
};

/// Generate a dummy implementation core Wasm module for a given WIT document
pub fn dummy_module(resolve: &Resolve, world: WorldId, mangling: ManglingAndAbi) -> Vec<u8> {
    let world = &resolve.worlds[world];
    let mut wat = String::new();
    wat.push_str("(module\n");
    for (name, import) in world.imports.iter() {
        match import {
            WorldItem::Function(func) => {
                push_imported_func(&mut wat, resolve, None, func, mangling);
            }
            WorldItem::Interface { id: import, .. } => {
                for (_, func) in resolve.interfaces[*import].functions.iter() {
                    push_imported_func(&mut wat, resolve, Some(name), func, mangling);
                }
                for (_, ty) in resolve.interfaces[*import].types.iter() {
                    push_imported_type_intrinsics(&mut wat, resolve, Some(name), *ty, mangling);
                }
            }
            WorldItem::Type(id) => {
                push_imported_type_intrinsics(&mut wat, resolve, None, *id, mangling);
            }
        }
    }

    if mangling.is_async() {
        push_root_async_intrinsics(&mut wat);
    }

    // Append any intrinsics which are imported but used in exported items
    // (e.g. resources)
    for (name, export) in world.exports.iter() {
        match export {
            WorldItem::Function(func) => {
                push_exported_func_intrinsics(&mut wat, resolve, None, func, mangling);
            }
            WorldItem::Interface { id: export, .. } => {
                for (_, func) in resolve.interfaces[*export].functions.iter() {
                    push_exported_func_intrinsics(&mut wat, resolve, Some(name), func, mangling);
                }
                for (_, ty) in resolve.interfaces[*export].types.iter() {
                    push_exported_type_intrinsics(&mut wat, resolve, Some(name), *ty, mangling);
                }
            }
            WorldItem::Type(_) => {}
        }
    }

    for (name, export) in world.exports.iter() {
        match export {
            WorldItem::Function(func) => {
                push_func_export(&mut wat, resolve, None, func, mangling);
            }
            WorldItem::Interface { id: export, .. } => {
                for (_, func) in resolve.interfaces[*export].functions.iter() {
                    push_func_export(&mut wat, resolve, Some(name), func, mangling);
                }
                for (_, ty) in resolve.interfaces[*export].types.iter() {
                    push_exported_resource_functions(&mut wat, resolve, name, *ty, mangling);
                }
            }
            WorldItem::Type(_) => {}
        }
    }

    let memory = resolve.wasm_export_name(mangling, WasmExport::Memory);
    wat.push_str(&format!("(memory (export {memory:?}) 0)\n"));
    let realloc = resolve.wasm_export_name(mangling, WasmExport::Realloc);
    wat.push_str(&format!(
        "(func (export {realloc:?}) (param i32 i32 i32 i32) (result i32) unreachable)\n"
    ));

    let initialize = resolve.wasm_export_name(mangling, WasmExport::Initialize);
    wat.push_str(&format!("(func (export {initialize:?}))"));
    wat.push_str(")\n");

    return wat::parse_str(&wat).unwrap();
}

fn push_imported_func(
    wat: &mut String,
    resolve: &Resolve,
    interface: Option<&WorldKey>,
    func: &Function,
    mangling: ManglingAndAbi,
) {
    let sig = resolve.wasm_signature(mangling.import_variant(), func);

    let (module, name) = resolve.wasm_import_name(mangling, WasmImport::Func { interface, func });
    wat.push_str(&format!("(import {module:?} {name:?} (func"));
    push_tys(wat, "param", &sig.params);
    push_tys(wat, "result", &sig.results);
    wat.push_str("))\n");

    if mangling.is_async() {
        push_imported_future_and_stream_intrinsics(wat, resolve, "", interface, func);
    }
}

fn push_imported_type_intrinsics(
    wat: &mut String,
    resolve: &Resolve,
    interface: Option<&WorldKey>,
    resource: TypeId,
    mangling: ManglingAndAbi,
) {
    let ty = &resolve.types[resource];
    match ty.kind {
        TypeDefKind::Resource => {
            let (module, name) = resolve.wasm_import_name(
                // Force using a sync ABI here at this time as support for async
                // resource drop isn't implemented yet.
                mangling.sync(),
                WasmImport::ResourceIntrinsic {
                    interface,
                    resource,
                    intrinsic: ResourceIntrinsic::ImportedDrop,
                },
            );
            wat.push_str(&format!("(import {module:?} {name:?} (func (param i32)))"));

            if mangling.is_async() {
                // TODO: when wit-component supports async resource drop,
                // implement it here too.
                // let name = format!("[async-lower]{name}");
                // wat.push_str(&format!("(import {module:?} {name:?} (func (param i32)))"));
            }
        }

        // No other types with intrinsics at this time (futures/streams are
        // relative to where they show up in function types.
        _ => {}
    }
}

fn push_exported_func_intrinsics(
    wat: &mut String,
    resolve: &Resolve,
    interface: Option<&WorldKey>,
    func: &Function,
    mangling: ManglingAndAbi,
) {
    if !mangling.is_async() {
        return;
    }

    // For exported async functions, generate a `task.return` intrinsic.
    let (module, name, sig) = func.task_return_import(resolve, interface, mangling.mangling());
    wat.push_str(&format!("(import {module:?} {name:?} (func"));
    push_tys(wat, "param", &sig.params);
    push_tys(wat, "result", &sig.results);
    wat.push_str("))\n");

    push_imported_future_and_stream_intrinsics(wat, resolve, "[export]", interface, func);
}

fn push_imported_future_and_stream_intrinsics(
    wat: &mut String,
    resolve: &Resolve,
    module_prefix: &str,
    interface: Option<&WorldKey>,
    func: &Function,
) {
    let module = match interface {
        Some(key) => format!("{module_prefix}{}", resolve.name_world_key(key)),
        None => format!("{module_prefix}$root"),
    };
    let name = &func.name;

    for (i, id) in func
        .find_futures_and_streams(resolve)
        .into_iter()
        .enumerate()
    {
        match &resolve.types[id].kind {
            TypeDefKind::Future(_) => {
                wat.push_str(&format!(
                    r#"
(import {module:?} "[future-new-{i}]{name}" (func (result i64)))
(import {module:?} "[future-read-{i}]{name}" (func (param i32 i32) (result i32)))
(import {module:?} "[future-write-{i}]{name}" (func (param i32 i32) (result i32)))
(import {module:?} "[future-cancel-read-{i}]{name}" (func (param i32) (result i32)))
(import {module:?} "[future-cancel-write-{i}]{name}" (func (param i32) (result i32)))
(import {module:?} "[future-drop-readable-{i}]{name}" (func (param i32)))
(import {module:?} "[future-drop-writable-{i}]{name}" (func (param i32)))
(import {module:?} "[async-lower][future-read-{i}]{name}" (func (param i32 i32) (result i32)))
(import {module:?} "[async-lower][future-write-{i}]{name}" (func (param i32 i32) (result i32)))

;; deferred behind 🚝
;;(import {module:?} "[async-lower][future-cancel-read-{i}]{name}" (func (param i32) (result i32)))
;;(import {module:?} "[async-lower][future-cancel-write-{i}]{name}" (func (param i32) (result i32)))
"#
                ));
            }
            TypeDefKind::Stream(_) => {
                wat.push_str(&format!(
                    r#"
(import {module:?} "[stream-new-{i}]{name}" (func (result i64)))
(import {module:?} "[stream-read-{i}]{name}" (func (param i32 i32 i32) (result i32)))
(import {module:?} "[stream-write-{i}]{name}" (func (param i32 i32 i32) (result i32)))
(import {module:?} "[stream-cancel-read-{i}]{name}" (func (param i32) (result i32)))
(import {module:?} "[stream-cancel-write-{i}]{name}" (func (param i32) (result i32)))
(import {module:?} "[stream-drop-readable-{i}]{name}" (func (param i32)))
(import {module:?} "[stream-drop-writable-{i}]{name}" (func (param i32)))
(import {module:?} "[async-lower][stream-read-{i}]{name}" (func (param i32 i32 i32) (result i32)))
(import {module:?} "[async-lower][stream-write-{i}]{name}" (func (param i32 i32 i32) (result i32)))

;; deferred behind 🚝
;;(import {module:?} "[async-lower][stream-cancel-read-{i}]{name}" (func (param i32) (result i32)))
;;(import {module:?} "[async-lower][stream-cancel-write-{i}]{name}" (func (param i32) (result i32)))
"#
                ));
            }
            _ => unreachable!(),
        }
    }
}

fn push_exported_type_intrinsics(
    wat: &mut String,
    resolve: &Resolve,
    interface: Option<&WorldKey>,
    resource: TypeId,
    mangling: ManglingAndAbi,
) {
    let ty = &resolve.types[resource];
    match ty.kind {
        TypeDefKind::Resource => {
            let intrinsics = [
                (ResourceIntrinsic::ExportedDrop, "(func (param i32))"),
                (
                    ResourceIntrinsic::ExportedNew,
                    "(func (param i32) (result i32))",
                ),
                (
                    ResourceIntrinsic::ExportedRep,
                    "(func (param i32) (result i32))",
                ),
            ];
            for (intrinsic, sig) in intrinsics {
                let (module, name) = resolve.wasm_import_name(
                    mangling.sync(),
                    WasmImport::ResourceIntrinsic {
                        interface,
                        resource,
                        intrinsic,
                    },
                );
                wat.push_str(&format!("(import {module:?} {name:?} {sig})\n"));
            }
        }

        // No other types with intrinsics at this time (futures/streams
        // relative to where they are in a function).
        _ => {}
    }
}

fn push_exported_resource_functions(
    wat: &mut String,
    resolve: &Resolve,
    interface: &WorldKey,
    resource: TypeId,
    mangling: ManglingAndAbi,
) {
    let ty = &resolve.types[resource];
    match ty.kind {
        TypeDefKind::Resource => {}
        _ => return,
    }
    // Feign destructors for any resource that this interface
    // exports
    let name = resolve.wasm_export_name(
        mangling,
        WasmExport::ResourceDtor {
            interface,
            resource,
        },
    );
    wat.push_str(&format!("(func (export {name:?}) (param i32))"));
}

fn push_func_export(
    wat: &mut String,
    resolve: &Resolve,
    interface: Option<&WorldKey>,
    func: &Function,
    mangling: ManglingAndAbi,
) {
    let sig = resolve.wasm_signature(mangling.export_variant(), func);
    let name = resolve.wasm_export_name(
        mangling,
        WasmExport::Func {
            interface,
            func,
            kind: WasmExportKind::Normal,
        },
    );
    wat.push_str(&format!("(func (export \"{name}\")"));
    push_tys(wat, "param", &sig.params);
    push_tys(wat, "result", &sig.results);
    wat.push_str(" unreachable)\n");

    match mangling {
        ManglingAndAbi::Standard32 | ManglingAndAbi::Legacy(LiftLowerAbi::Sync) => {
            let name = resolve.wasm_export_name(
                mangling,
                WasmExport::Func {
                    interface,
                    func,
                    kind: WasmExportKind::PostReturn,
                },
            );
            wat.push_str(&format!("(func (export \"{name}\")"));
            push_tys(wat, "param", &sig.results);
            wat.push_str(")\n");
        }
        ManglingAndAbi::Legacy(LiftLowerAbi::AsyncCallback) => {
            let name = resolve.wasm_export_name(
                mangling,
                WasmExport::Func {
                    interface,
                    func,
                    kind: WasmExportKind::Callback,
                },
            );
            wat.push_str(&format!(
                "(func (export \"{name}\") (param i32 i32 i32) (result i32) unreachable)\n"
            ));
        }
        ManglingAndAbi::Legacy(LiftLowerAbi::AsyncStackful) => {}
    }
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

fn push_root_async_intrinsics(dst: &mut String) {
    dst.push_str(
        r#"
(import "[export]$root" "[task-cancel]" (func))
(import "$root" "[backpressure-inc]" (func))
(import "$root" "[backpressure-dec]" (func))
(import "$root" "[waitable-set-new]" (func (result i32)))
(import "$root" "[waitable-set-wait]" (func (param i32 i32) (result i32)))
(import "$root" "[waitable-set-poll]" (func (param i32 i32) (result i32)))
(import "$root" "[waitable-set-drop]" (func (param i32)))
(import "$root" "[waitable-join]" (func (param i32 i32)))
(import "$root" "[thread-yield]" (func (result i32)))
(import "$root" "[subtask-drop]" (func (param i32)))
(import "$root" "[subtask-cancel]" (func (param i32) (result i32)))
(import "$root" "[context-get-0]" (func (result i32)))
(import "$root" "[context-set-0]" (func (param i32)))

;; deferred behind 🧵 upstream
;;(import "$root" "[cancellable][waitable-set-wait]" (func (param i32 i32) (result i32)))
;;(import "$root" "[cancellable][waitable-set-poll]" (func (param i32 i32) (result i32)))
;;(import "$root" "[cancellable][thread-yield]" (func (result i32)))
;;(import "$root" "[context-get-1]" (func (result i32)))
;;(import "$root" "[context-set-1]" (func (param i32)))

;; deferred behind 📝 upstream
;;(import "$root" "[error-context-new-utf8]" (func (param i32 i32) (result i32)))
;;(import "$root" "[error-context-new-utf16]" (func (param i32 i32) (result i32)))
;;(import "$root" "[error-context-new-latin1+utf16]" (func (param i32 i32) (result i32)))
;;(import "$root" "[error-context-debug-message-utf8]" (func (param i32 i32)))
;;(import "$root" "[error-context-debug-message-utf16]" (func (param i32 i32)))
;;(import "$root" "[error-context-debug-message-latin1+utf16]" (func (param i32 i32)))
;;(import "$root" "[error-context-drop]" (func (param i32)))
"#,
    );
}
