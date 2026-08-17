## nbformat - Jupyter Notebook Format in Rust

This crate provides functionality to parse and work with Jupyter Notebook files in Rust.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nbformat = "0.1.0"
```

Here's a basic example of how to use `parse_notebook`:

```rust
use nbformat::{parse_notebook, Notebook};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the notebook file
    let notebook_json = fs::read_to_string("Untitled1337.ipynb")?;

    // Parse the notebook
    let notebook = parse_notebook(&notebook_json)?;

    // Work with the parsed notebook
    match notebook {
        Notebook::V4(nb) => {
            println!("Notebook version: {}.{}", nb.nbformat, nb.nbformat_minor);
            println!("Number of cells: {}", nb.cells.len());
            // Access other notebook properties...
        }
        Notebook::Legacy(nb) => {
            println!("Legacy notebook version: {}.{}", nb.nbformat, nb.nbformat_minor);
            println!("Number of cells: {}", nb.cells.len());
            // Access other notebook properties...
        }
    }

    Ok(())
}
```

At present, this crate supports v4.5 notebooks via `Notebook::V4`, v4.1-v4.4 via `Notebook::Legacy` and v3 via `Notebook::V3`. v4.5 have some more hard constraints on CellIDs being required, only allowing certain characters, and not having duplicates. Converting from a v4.1-v4.4 notebook to a v4.5 notebook requires modifying the notebook to include Cell IDs.


## ROADMAP

* [x] Serialize and Deserialize v4.1-v4.5 notebooks into rust structures
* [x] Test operations on a suite of notebooks from Python nbformat
* [x] Add support for upconverting v3 notebooks to v4
* [x] Add support for upconverting v4.1-v4.4 notebooks to v4.5
* [x] Break out types to be shared between runtimelib's Media setup and the notebook crate
* [ ] Add support for v2 and v1 notebook upconversion
