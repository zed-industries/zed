extern crate quick_xml;

mod cg;
mod ir;
mod output;
mod parse;

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use cg::{CodeGen, DepInfo};
use ir::ExtInfo;
use output::Output;
use parse::{Event, Parser, Result};

fn is_always(name: &str) -> bool {
    matches!(name, "xproto" | "bigreq" | "xc_misc")
}

fn has_feature(name: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", name.to_ascii_uppercase())).is_ok()
}

fn main() -> io::Result<()> {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let xml_dir = Path::new(&root).join("xml");
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| "./gen/current".to_string());
    let out_dir = Path::new(&out_dir);

    let rustfmt = env::var("RXCB_RUSTFMT").ok().and_then(|var| {
        if var == "1" || var == "y" || var == "Y" {
            find_exe("rustfmt")
        } else {
            None
        }
    });
    let gen_all = env::var("RXCB_GENALL").is_ok();
    let export = env::var("RXCB_EXPORT").ok();

    if rustfmt.is_some() && export.is_some() {
        panic!("RXCB_EXPORT and RXCB_RUSTFMT do not work well together. Please choose one or the other.");
    }
    let dbg_atom_names = has_feature("debug_atom_names");

    let mut dep_info = Vec::new();

    for xml_file in iter_xml(&xml_dir) {
        process_xcb_gen(
            &xml_file,
            out_dir,
            &rustfmt,
            gen_all,
            &mut dep_info,
            dbg_atom_names,
        )
        .unwrap_or_else(|err| {
            panic!(
                "Error during processing of {}: {:?}",
                xml_file.display(),
                err
            )
        });
    }

    if let Some(export) = export {
        let export = Path::new(&export);
        fs::create_dir_all(&export)?;

        for f in iter_out_rs(out_dir) {
            let copy = export.join(f.file_name().unwrap());
            fs::copy(f, copy)?;
        }
    }

    #[cfg(target_os = "freebsd")]
    println!("cargo:rustc-link-search=/usr/local/lib");

    Ok(())
}

fn iter_xml(xml_dir: &Path) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(xml_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| match p.extension() {
            Some(e) => e == "xml",
            _ => false,
        })
}

fn iter_out_rs(out_dir: &Path) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(out_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| match p.extension() {
            Some(e) => e == "rs",
            _ => false,
        })
}

fn find_exe<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

fn recurse_push_depinfo(dep_info: &mut Vec<DepInfo>, xcb_mod: &str, global: &[DepInfo]) {
    if dep_info.iter().any(|di| di.xcb_mod == xcb_mod) {
        return;
    }

    let di = global
        .iter()
        .find(|di| di.xcb_mod == xcb_mod)
        .unwrap_or_else(|| panic!("can't find dependency {}", xcb_mod));

    dep_info.push(di.clone());

    for d in &di.deps {
        recurse_push_depinfo(dep_info, d, global);
    }
}

fn process_xcb_gen(
    xml_file: &Path,
    out_dir: &Path,
    rustfmt: &Option<PathBuf>,
    gen_all: bool,
    dep_info: &mut Vec<DepInfo>,
    dbg_atom_names: bool,
) -> Result<()> {
    let xcb_mod = xml_file.file_stem().unwrap();
    let xcb_mod = xcb_mod.to_str().unwrap().to_string();

    if dep_info.iter().any(|di| di.xcb_mod == xcb_mod) {
        return Ok(());
    }

    if !gen_all && !is_always(&xcb_mod) && !has_feature(&xcb_mod) {
        return Ok(());
    }

    let mut parser = Parser::from_file(xml_file);

    let mut imports = Vec::new();
    let mut mod_info: Option<(String, Option<ExtInfo>)> = None;
    let mut items = Vec::new();

    for e in &mut parser {
        match e? {
            Event::ModuleInfo { name, extinfo } => {
                mod_info = Some((name, extinfo));
            }
            Event::Import(imp) => imports.push(imp),
            Event::Item(item) => items.push(item),
            _ => {}
        }
    }

    let mod_info = mod_info.expect("no xcb protocol opening");

    if xcb_mod != "xproto" && imports.iter().all(|i| i != "xproto") {
        imports.insert(0, "xproto".to_string());
    }

    let deps = {
        let mut deps: Vec<DepInfo> = Vec::new();

        for i in imports.iter() {
            let xml_file = xml_file.with_file_name(&format!("{}.xml", i));

            // panic also from here to have the correct xml_file reported
            process_xcb_gen(
                &xml_file,
                out_dir,
                rustfmt,
                gen_all,
                dep_info,
                dbg_atom_names,
            )
            .unwrap_or_else(|err| {
                panic!(
                    "Error during processing of {}: {:?}",
                    xml_file.display(),
                    err
                )
            });

            recurse_push_depinfo(&mut deps, i, dep_info);
        }

        deps
    };

    let out_file = out_dir.join(&xcb_mod).with_extension("rs");
    let mut out = Output::new(rustfmt, &out_file)
        .unwrap_or_else(|_| panic!("cannot create Rust output file: {}", out_file.display()));

    let mut cg = CodeGen::new(xcb_mod, &mod_info.1, deps, dbg_atom_names);

    for item in &items {
        cg.preregister_item(item);
    }
    for item in &items {
        cg.resolve_type(item);
    }
    for item in items {
        cg.resolve_error_event_request(item);
    }

    cg.emit_prologue(&mut out)?;
    cg.emit_errors(&mut out)?;
    cg.emit_events(&mut out)?;
    cg.emit_types(&mut out)?;
    cg.emit_requests(&mut out)?;

    dep_info.push(cg.into_depinfo());

    Ok(())
}
