use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <notebook.ipynb>", args[0]);
        eprintln!("Parses and upgrades a notebook to v4.5 format");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);

    if !path.exists() {
        eprintln!("Error: File not found: {:?}", path);
        std::process::exit(1);
    }

    let notebook_json = fs::read_to_string(path).expect("Failed to read notebook file");

    match nbformat::parse_notebook(&notebook_json) {
        Ok(nbformat::Notebook::V3(v3_notebook)) => {
            match nbformat::upgrade_v3_notebook(v3_notebook) {
                Ok(v4_notebook) => {
                    let output = nbformat::serialize_notebook(&nbformat::Notebook::V4(v4_notebook))
                        .expect("Failed to serialize upgraded notebook");

                    println!("{}", output);
                }
                Err(e) => {
                    eprintln!("Error upgrading notebook: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Ok(nbformat::Notebook::V4(v4_notebook)) => {
            let output = nbformat::serialize_notebook(&nbformat::Notebook::V4(v4_notebook))
                .expect("Failed to serialize notebook");
            println!("{}", output);
        }
        Ok(nbformat::Notebook::Legacy(legacy_notebook)) => {
            match nbformat::upgrade_legacy_notebook(legacy_notebook) {
                Ok(v4_notebook) => {
                    let output = nbformat::serialize_notebook(&nbformat::Notebook::V4(v4_notebook))
                        .expect("Failed to serialize upgraded notebook");

                    println!("{}", output);
                }
                Err(e) => {
                    eprintln!("Error upgrading notebook: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing notebook: {}", e);
            std::process::exit(1);
        }
    }
}
