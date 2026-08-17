extern crate env_logger;
extern crate handlebars;

use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("template", "./examples/partials/template2.hbs")?;

    handlebars.register_template_file("base0", "./examples/partials/base0.hbs")?;
    handlebars.register_template_file("base1", "./examples/partials/base1.hbs")?;

    let data0 = json!({
        "title": "example 0",
        "parent": "base0"
    });
    let data1 = json!({
        "title": "example 1",
        "parent": "base1"
    });

    println!("Page 0");
    println!("{}", handlebars.render("template", &data0)?);
    println!("=======================================================");

    println!("Page 1");
    println!("{}", handlebars.render("template", &data1)?);

    Ok(())
}
