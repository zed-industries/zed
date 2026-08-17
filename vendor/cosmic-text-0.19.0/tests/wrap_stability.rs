use cosmic_text::{
    fontdb, Align, Attrs, AttrsList, BidiParagraphs, Buffer, Family, FontSystem, Hinting,
    LayoutLine, Metrics, ShapeLine, Shaping, Weight, Wrap,
};

// Test for https://github.com/pop-os/cosmic-text/issues/134
//
// Being able to get the same wrapping when feeding the measured width back into ShapeLine::layout
// as the new width limit is very useful for certain UI layout use cases.
#[test]
fn stable_wrap() {
    let font_size = 18.0;
    let attrs = AttrsList::new(
        &Attrs::new()
            .family(Family::Name("FiraMono"))
            .weight(Weight::MEDIUM),
    );
    let mut font_system =
        FontSystem::new_with_locale_and_db("en-US".into(), fontdb::Database::new());
    let font = std::fs::read("fonts/FiraMono-Medium.ttf").unwrap();
    font_system.db_mut().load_font_data(font);

    let mut check_wrap = |text: &_, wrap, align_opt, start_width_opt| {
        let line = ShapeLine::new(&mut font_system, text, &attrs, Shaping::Advanced, 8);

        let layout_unbounded = line.layout(
            font_size,
            start_width_opt,
            wrap,
            align_opt,
            None,
            Hinting::Disabled,
        );
        let max_width = layout_unbounded.iter().map(|l| l.w).fold(0.0, f32::max);
        let new_limit = match start_width_opt {
            Some(start_width) => f32::min(start_width, max_width),
            None => max_width,
        };

        let layout_bounded = line.layout(
            font_size,
            Some(new_limit),
            wrap,
            align_opt,
            None,
            Hinting::Disabled,
        );
        let bounded_max_width = layout_bounded.iter().map(|l| l.w).fold(0.0, f32::max);

        // For debugging:
        // dbg_layout_lines(text, &layout_unbounded);
        // dbg_layout_lines(text, &layout_bounded);

        assert_eq!(
            (max_width, layout_unbounded.len()),
            (bounded_max_width, layout_bounded.len()),
            "Wrap \"{wrap:?}\" and align \"{align_opt:?}\" with text: \"{text}\"",
        );
        for (u, b) in layout_unbounded[1..].iter().zip(layout_bounded[1..].iter()) {
            assert_eq!(
                u.w, b.w,
                "Wrap {wrap:?} and align \"{align_opt:?}\" with text: \"{text}\"",
            );
        }
    };

    let hello_sample = std::fs::read_to_string("sample/hello.txt").unwrap();
    let cases = [
        "(6)  SomewhatBoringDisplayTransform",
        "",
        " ",
        "  ",
        "   ",
        "       ",
    ]
    .into_iter()
    // This has several cases where the line would wrap when the computed width was used as the
    // width limit.
    .chain(BidiParagraphs::new(&hello_sample));

    for text in cases {
        for wrap in [Wrap::None, Wrap::Glyph, Wrap::Word, Wrap::WordOrGlyph] {
            for align_opt in [
                None,
                Some(Align::Left),
                Some(Align::Right),
                Some(Align::Center),
                //TODO: Align::Justified
                Some(Align::End),
            ] {
                for start_width_opt in [
                    None,
                    Some(f32::MAX),
                    Some(80.0),
                    Some(198.2132),
                    Some(20.0),
                    Some(4.0),
                    Some(300.0),
                ] {
                    check_wrap(text, wrap, align_opt, start_width_opt);
                    let with_spaces = format!("{text}            ");
                    check_wrap(&with_spaces, wrap, align_opt, start_width_opt);
                    let with_spaces_2 = format!("{text}    ");
                    check_wrap(&with_spaces_2, wrap, align_opt, start_width_opt);
                }
            }
        }
    }
}

#[test]
fn wrap_extra_line() {
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(14.0, 20.0);

    let mut buffer = Buffer::new(&mut font_system, metrics);

    let mut buffer = buffer.borrow_with(&mut font_system);

    // Configure wrap and size, then add text
    buffer.set_wrap(Wrap::Word);
    buffer.set_size(Some(50.0), Some(1000.0));
    buffer.set_text("Lorem ipsum dolor sit amet, qui minim labore adipisicing\n\nweeewoooo minim sint cillum sint consectetur cupidatat.", &Attrs::new().family(cosmic_text::Family::Name("Inter")), Shaping::Advanced, None);

    let empty_lines = buffer.layout_runs().filter(|x| x.line_w == 0.).count();
    let overflow_lines = buffer.layout_runs().filter(|x| x.line_w > 50.).count();

    assert_eq!(empty_lines, 1);
    assert_eq!(overflow_lines, 4);
}

#[allow(dead_code)]
fn dbg_layout_lines(text: &str, lines: &[LayoutLine]) {
    for line in lines {
        let mut s = String::new();
        for glyph in line.glyphs.iter() {
            s.push_str(&text[glyph.start..glyph.end]);
        }
        println!("\"{s}\"");
    }
}
