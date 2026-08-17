# plotters examples

* The example projects have been moved to independent git repository under plotters-rs organization, please check the [Example Project](#example-project) section for the links.

To run any example, from within the repo, run `cargo run --example <example_name>` where `<example name>` is the name of the file without the `.rs` extension.

All the examples assumes the directory [plotters-doc-data](https://github.com/38/plotters-doc-data) exists, otherwise those example crashes.

The output of these example files are used to generate the [plotters-doc-data](https://github.com/38/plotters-doc-data) repo that populates the sample images in the main README.
We also rely on the output of examples to detect potential layout changes.
For that reason, **they must be run with `cargo` from within the repo, or you must change the output filename in the example code to a directory that exists.**

The examples that have their own directories and `Cargo.toml` files work differently. They are run the same way you would a standalone project.

## Example Projects

- For WebAssembly sample project, check [plotters-wasm-demo](https://github.com/plotters-rs/plotters-wasm-demo)
- For Frame Buffer, Realtime Readering example, check [plotters-minifb-demo](https://github.com/plotters-rs/plotters-minifb-demo)
- For GTK integration, check [plotters-gtk-demo](https://github.com/plotters-rs/plotters-gtk-demo)
