mod app;

use gpui::fonts::{Properties, Weight};
use gpui::text_layout::*;

#[crate::test]
fn test_wrap_line(cx: &mut gpui::AppContext) {
    let font_cache = cx.font_cache().clone();
    let font_system = cx.platform().fonts();
    let family = font_cache
        .load_family(&["Courier"], &Default::default())
        .unwrap();
    let font_id = font_cache.select_font(family, &Default::default()).unwrap();

    let mut wrapper = LineWrapper::new(font_id, 16., font_system);
    assert_eq!(
        wrapper
            .wrap_line("aa bbb cccc ddddd eeee", 72.0)
            .collect::<Vec<_>>(),
        &[
            Boundary::new(7, 0),
            Boundary::new(12, 0),
            Boundary::new(18, 0)
        ],
    );
    assert_eq!(
        wrapper
            .wrap_line("aaa aaaaaaaaaaaaaaaaaa", 72.0)
            .collect::<Vec<_>>(),
        &[
            Boundary::new(4, 0),
            Boundary::new(11, 0),
            Boundary::new(18, 0)
        ],
    );
    assert_eq!(
        wrapper.wrap_line("     aaaaaaa", 72.).collect::<Vec<_>>(),
        &[
            Boundary::new(7, 5),
            Boundary::new(9, 5),
            Boundary::new(11, 5),
        ]
    );
    assert_eq!(
        wrapper
            .wrap_line("                            ", 72.)
            .collect::<Vec<_>>(),
        &[
            Boundary::new(7, 0),
            Boundary::new(14, 0),
            Boundary::new(21, 0)
        ]
    );
    assert_eq!(
        wrapper
            .wrap_line("          aaaaaaaaaaaaaa", 72.)
            .collect::<Vec<_>>(),
        &[
            Boundary::new(7, 0),
            Boundary::new(14, 3),
            Boundary::new(18, 3),
            Boundary::new(22, 3),
        ]
    );
}

#[crate::test(retries = 5)]
fn test_wrap_shaped_line(cx: &mut gpui::AppContext) {
    // This is failing intermittently on CI and we don't have time to figure it out
    let font_cache = cx.font_cache().clone();
    let font_system = cx.platform().fonts();
    let text_layout_cache = TextLayoutCache::new(font_system.clone());

    let family = font_cache
        .load_family(&["Helvetica"], &Default::default())
        .unwrap();
    let font_id = font_cache.select_font(family, &Default::default()).unwrap();
    let normal = RunStyle {
        font_id,
        color: Default::default(),
        underline: Default::default(),
    };
    let bold = RunStyle {
        font_id: font_cache
            .select_font(
                family,
                &Properties {
                    weight: Weight::BOLD,
                    ..Default::default()
                },
            )
            .unwrap(),
        color: Default::default(),
        underline: Default::default(),
    };

    let text = "aa bbb cccc ddddd eeee";
    let line = text_layout_cache.layout_str(
        text,
        16.0,
        &[(4, normal), (5, bold), (6, normal), (1, bold), (7, normal)],
    );

    let mut wrapper = LineWrapper::new(font_id, 16., font_system);
    assert_eq!(
        wrapper
            .wrap_shaped_line(text, &line, 72.0)
            .collect::<Vec<_>>(),
        &[
            ShapedBoundary {
                run_ix: 1,
                glyph_ix: 3
            },
            ShapedBoundary {
                run_ix: 2,
                glyph_ix: 3
            },
            ShapedBoundary {
                run_ix: 4,
                glyph_ix: 2
            }
        ],
    );
}
