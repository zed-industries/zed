use svg_fmt::{black, rectangle, red, text, white, BeginSvg, EndSvg, Stroke};

#[test]
fn foo() {
    println!("{}", BeginSvg { w: 800.0, h: 600.0 });
    println!(
        "    {}",
        rectangle(20.0, 50.0, 200.0, 100.0)
            .fill(red())
            .stroke(Stroke::Color(black(), 3.0))
            .border_radius(5.0)
    );
    println!(
        "    {}",
        text(25.0, 100.0, "Foo!").size(42.0).color(white())
    );
    println!("{}", EndSvg);
}
