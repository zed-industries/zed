use fontconfig_parser::FontConfig;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example dump -- <conf file path>");
        return;
    }

    let mut config = FontConfig::default();
    config.merge_config(&args[1]).unwrap();

    println!("{:#?}", config);
}
