use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

use serde_json::value::Value as Json;

use handlebars::Handlebars;

fn usage() -> ! {
    eprintln!("Usage: handlebars-cli template.hbs '{{\"json\": \"data\"}}'");
    process::exit(1);
}

fn parse_json(text: &str) -> Json {
    let result = if let Some(text) = text.strip_prefix('@') {
        fs::read_to_string(text).unwrap()
    } else {
        text.to_owned()
    };
    match Json::from_str(&result) {
        Ok(json) => json,
        Err(_) => usage(),
    }
}

fn main() {
    let mut args = env::args();
    args.next(); // skip own filename
    let (filename, json) = match (args.next(), args.next()) {
        (Some(filename), Some(json)) => (filename, json),
        _ => usage(),
    };
    let data = parse_json(&json);

    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file(&filename, &filename)
        .ok()
        .unwrap();
    match handlebars.render(&filename, &data) {
        Ok(data) => {
            println!("{}", data);
        }
        Err(e) => {
            println!("Error rendering {}: {}", filename, e);
            process::exit(2);
        }
    }
}
