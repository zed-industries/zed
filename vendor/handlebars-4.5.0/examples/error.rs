extern crate env_logger;
extern crate handlebars;
#[macro_use]
extern crate serde_json;

use std::error::Error as StdError;

use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HelperError {
    #[error("db error")]
    DbError,
    #[error("api error")]
    ApiError,
}

/// A helper that raise error according to parameters
pub fn error_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    _: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for error helper."))?;
    match param.value().as_str() {
        Some("db") => Err(RenderError::from_error(
            "helper error",
            HelperError::DbError,
        )),
        Some("api") => Err(RenderError::from_error(
            "helper error",
            HelperError::ApiError,
        )),
        _ => Ok(()),
    }
}

fn main() -> Result<(), Box<dyn StdError>> {
    env_logger::init();
    let mut handlebars = Handlebars::new();

    // template not found
    println!(
        "{}",
        handlebars
            .register_template_file("notfound", "./examples/error/notfound.hbs")
            .unwrap_err()
    );

    // an invalid templat
    println!(
        "{}",
        handlebars
            .register_template_file("error", "./examples/error/error.hbs")
            .unwrap_err()
    );

    // render error
    let e1 = handlebars
        .render_template("{{#if}}", &json!({}))
        .unwrap_err();
    let be1 = Box::new(e1);
    println!("{}", be1);
    println!("{}", be1.source().unwrap());
    println!("{:?}", be1.source().unwrap().source());

    // process error generated in helper
    handlebars.register_helper("err", Box::new(error_helper));
    let e2 = handlebars
        .render_template("{{err \"db\"}}", &json!({}))
        .unwrap_err();
    // down-casting the error to user defined type
    match e2.source().and_then(|e| e.downcast_ref::<HelperError>()) {
        Some(HelperError::DbError) => {
            println!("Detected error from helper: db error",)
        }
        _ => {}
    }

    Ok(())
}
