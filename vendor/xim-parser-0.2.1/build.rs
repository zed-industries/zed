fn main() {
    #[cfg(feature = "bootstrap")]
    xim_gen::write_format(include_str!("xim-format.yaml"), "./src/parser.rs").unwrap();
}
