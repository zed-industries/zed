use std::sync::Arc;

use handlebars::Handlebars;
use serde_json::json;
use tiny_http::{Response, Server};

fn handlebars() -> Handlebars<'static> {
    let mut reg = Handlebars::new();
    // enable dev mode for template reloading
    reg.set_dev_mode(true);
    // register a template from the file
    // modified the file after the server starts to see things changing
    reg.register_template_file("tpl", "./examples/dev_mode/template.hbs")
        .unwrap();

    reg
}

fn main() {
    let hbs = Arc::new(handlebars());

    let server = Server::http("127.0.0.1:3030").expect("Failed to start demo server.");
    println!("Edit ./examples/dev_mode/template.hbs and request http://localhost:3030 to see the change on the fly.");

    for req in server.incoming_requests() {
        let result = hbs
            .render("tpl", &json!({"model": "t14s", "brand": "Thinkpad"}))
            .unwrap_or_else(|e| e.to_string());
        req.respond(Response::from_string(result)).unwrap();
    }
}
