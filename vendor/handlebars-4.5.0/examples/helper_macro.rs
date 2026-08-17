use std::error::Error;

use handlebars::{handlebars_helper, Handlebars, JsonRender};
use serde_json::{json, Value};
use time::format_description::parse;
use time::OffsetDateTime;

// define a helper using helper
// a date format helper accept an `OffsetDateTime` as parameter
handlebars_helper!(date: |dt: OffsetDateTime| dt.format(&parse("[year]-[month]-[day]").unwrap()).unwrap());

// a helper returns number of provided parameters
handlebars_helper!(nargs: |*args| args.len());

// a helper joins all values, using both hash and parameters
handlebars_helper!(join: |{sep:str=","}, *args|
                   args.iter().map(|a| a.render()).collect::<Vec<String>>().join(sep)
);

handlebars_helper!(isdefined: |v: Value| !v.is_null());

// a helper provides format
handlebars_helper!(date2: |dt: OffsetDateTime, {fmt:str = "[year]-[month]-[day]"}|
    dt.format(&parse(fmt).unwrap()).unwrap()
);

fn main() -> Result<(), Box<dyn Error>> {
    // create the handlebars registry
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("date", Box::new(date));
    handlebars.register_helper("date2", Box::new(date2));
    handlebars.register_helper("nargs", Box::new(nargs));
    handlebars.register_helper("join", Box::new(join));
    handlebars.register_helper("isdefined", Box::new(isdefined));

    let data = OffsetDateTime::now_utc();

    println!("{}", handlebars.render_template("{{date this}}", &data)?);
    println!("{}", handlebars.render_template("{{date2 this}}", &data)?);
    println!(
        "{}",
        handlebars.render_template("{{date2 this fmt=\"[day]/[month]/[year]\"}}", &data)?
    );

    println!("{}", handlebars.render_template("{{nargs 1 2 3 4}}", &())?);

    println!(
        "{}",
        handlebars.render_template("{{join 1 2 3 4 sep=\"|\" }}", &())?
    );

    println!(
        "{}",
        handlebars.render_template(
            r#"{{isdefined a}} {{isdefined b}}
{{#if (isdefined a)}}a{{/if}} {{#if (isdefined b)}}b{{/if}}"#,
            &json!({"a": 1})
        )?
    );

    Ok(())
}
