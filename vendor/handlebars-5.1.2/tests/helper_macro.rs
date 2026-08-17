#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;
use time::format_description::{parse, well_known::Rfc2822};
use time::OffsetDateTime;

handlebars_helper!(lower: |s: str| s.to_lowercase());
handlebars_helper!(upper: |s: str| s.to_uppercase());
handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));
handlebars_helper!(money: |v: i64, {cur: str="$"}| format!("{}{}.00", cur, v));
handlebars_helper!(all_hash: |{cur: str="$"}| cur);
handlebars_helper!(nargs: |*args| args.len());
handlebars_helper!(has_a: |{a:i64 = 99}, **kwargs|
                   format!("{}, {}", a, kwargs.get("a").is_some()));
handlebars_helper!(tag: |t: str| format!("<{}>", t));
handlebars_helper!(date: |dt: OffsetDateTime| dt.format(&parse("[year]-[month]-[day]").unwrap()).unwrap());

#[test]
fn test_macro_helper() {
    let mut hbs = Handlebars::new();

    hbs.register_helper("lower", Box::new(lower));
    hbs.register_helper("upper", Box::new(upper));
    hbs.register_helper("hex", Box::new(hex));
    hbs.register_helper("money", Box::new(money));
    hbs.register_helper("nargs", Box::new(nargs));
    hbs.register_helper("has_a", Box::new(has_a));
    hbs.register_helper("tag", Box::new(tag));
    hbs.register_helper("date", Box::new(date));

    let data = json!("Teixeira");

    assert_eq!(
        hbs.render_template("{{lower this}}", &data).unwrap(),
        "teixeira"
    );
    assert_eq!(
        hbs.render_template("{{upper this}}", &data).unwrap(),
        "TEIXEIRA"
    );
    assert_eq!(hbs.render_template("{{hex 16}}", &()).unwrap(), "0x10");

    assert_eq!(
        hbs.render_template("{{money 5000}}", &()).unwrap(),
        "$5000.00"
    );
    assert_eq!(
        hbs.render_template("{{money 5000 cur=\"£\"}}", &())
            .unwrap(),
        "£5000.00"
    );
    assert_eq!(
        hbs.render_template("{{nargs 1 1 1 1 1}}", &()).unwrap(),
        "5"
    );
    assert_eq!(hbs.render_template("{{nargs}}", &()).unwrap(), "0");

    assert_eq!(
        hbs.render_template("{{has_a a=1 b=2}}", &()).unwrap(),
        "1, true"
    );

    assert_eq!(
        hbs.render_template("{{has_a x=1 b=2}}", &()).unwrap(),
        "99, false"
    );

    assert_eq!(
        hbs.render_template("{{tag \"html\"}}", &()).unwrap(),
        "&lt;html&gt;"
    );

    assert_eq!(
        hbs.render_template("{{{tag \"html\"}}}", &()).unwrap(),
        "<html>"
    );

    assert_eq!(
        hbs.render_template(
            "{{date this}}",
            &OffsetDateTime::parse("Wed, 18 Feb 2015 23:16:09 GMT", &Rfc2822).unwrap()
        )
        .unwrap(),
        "2015-02-18"
    );

    assert_eq!(
        hbs.render_template("{{eq image.link null}}", &json!({"image": {"link": null}}))
            .unwrap(),
        "true"
    );

    assert_eq!(
        hbs.render_template(
            "{{eq image.link null}}",
            &json!({"image": {"link": "https://url"}})
        )
        .unwrap(),
        "false"
    );

    assert_eq!(
        hbs.render_template("{{tag 'html'}}", &()).unwrap(),
        "&lt;html&gt;"
    );
}
