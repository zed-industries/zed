use anyhow::Result;
use std::env;
use wasmparser::{Parser, Payload};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} in.wasm", args[0]);
        return Ok(());
    }

    let buf: Vec<u8> = std::fs::read(&args[1])?;
    for payload in Parser::new(0).parse_all(&buf) {
        match payload? {
            Payload::Version { .. } => {
                println!("====== Module");
            }
            Payload::ExportSection(s) => {
                for export in s {
                    let export = export?;
                    println!("  Export {} {:?}", export.name, export.kind);
                }
            }
            Payload::ImportSection(s) => {
                for import in s {
                    let import = import?;
                    println!("  Import {}::{}", import.module, import.name);
                }
            }
            _other => {
                // println!("found payload {:?}", _other);
            }
        }
    }

    Ok(())
}
