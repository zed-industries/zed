// font-kit/tests/select-font.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate font_kit;

use font_kit::error::SelectionError;
use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::Properties;
use std::ffi::OsStr;

#[cfg(feature = "source")]
use font_kit::source::SystemSource;

#[cfg(all(feature = "source", any(target_os = "windows", target_os = "macos")))]
macro_rules! match_handle {
    ($handle:expr, $path:expr, $index:expr) => {
        match $handle {
            Handle::Path {
                ref path,
                font_index,
            } => {
                assert_eq!(path, path);
                assert_eq!(
                    font_index, $index,
                    "expecting font index {} not {}",
                    font_index, $index
                );
            }
            Handle::Memory {
                bytes: _,
                font_index,
            } => {
                assert_eq!(
                    font_index, $index,
                    "expecting font index {} not {}",
                    font_index, $index
                );
            }
            Handle::Native { .. } => {}
        }
    };
}

#[inline(always)]
fn check_filename(handle: &Handle, filename: &str) {
    match *handle {
        Handle::Path {
            ref path,
            font_index,
        } => {
            assert_eq!(path.file_name(), Some(OsStr::new(filename)));
            assert_eq!(font_index, 0);
        }
        _ => panic!("Expected path handle!"),
    }
}

#[cfg(all(feature = "source", target_os = "windows"))]
mod test {
    use super::*;

    #[test]
    fn select_best_match_serif() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::Serif], &Properties::default())
            .unwrap();
        match_handle!(handle, "C:\\WINDOWS\\FONTS\\TIMES.TTF", 0);
    }

    #[test]
    fn select_best_match_sans_serif() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::default())
            .unwrap();
        match_handle!(handle, ":\\WINDOWS\\FONTS\\ARIAL.TTF", 0);
    }

    #[test]
    fn select_best_match_monospace() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::Monospace], &Properties::default())
            .unwrap();
        match_handle!(handle, "C:\\WINDOWS\\FONTS\\COUR.TTF", 0);
    }

    #[test]
    fn select_best_match_cursive() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::Cursive], &Properties::default())
            .unwrap();
        match_handle!(handle, "C:\\WINDOWS\\FONTS\\COMIC.TTF", 0);
    }

    // TODO: no such font in a Travis CI instance.
    // https://github.com/servo/font-kit/issues/62
    // #[test]
    // fn select_best_match_fantasy() {
    //     let handle = SystemSource::new()
    //         .select_best_match(&[FamilyName::Fantasy], &Properties::default())
    //         .unwrap();
    //     match_handle!(handle, "C:\\WINDOWS\\FONTS\\IMPACT.TTF", 0);
    // }

    #[test]
    fn select_best_match_bold_sans_serif() {
        let handle = SystemSource::new()
            .select_best_match(
                &[FamilyName::SansSerif],
                &Properties {
                    style: font_kit::properties::Style::Normal,
                    weight: font_kit::properties::Weight::BOLD,
                    stretch: font_kit::properties::Stretch::NORMAL,
                },
            )
            .unwrap();
        match_handle!(handle, "C:\\WINDOWS\\FONTS\\ARIALBD.TTF", 0);
    }

    #[test]
    fn select_best_match_by_name_after_invalid() {
        let handle = SystemSource::new()
            .select_best_match(
                &[
                    FamilyName::Title("Invalid".to_string()),
                    FamilyName::Title("Times New Roman".to_string()),
                ],
                &Properties::default(),
            )
            .unwrap();
        match_handle!(handle, "C:\\WINDOWS\\FONTS\\TIMES.TTF", 0);
    }

    #[test]
    fn select_family_by_name_arial() {
        let family = SystemSource::new().select_family_by_name("Arial").unwrap();
        assert_eq!(family.fonts().len(), 8);
        match_handle!(family.fonts()[0], "C:\\WINDOWS\\FONTS\\ARIAL.TTF", 0);
        match_handle!(family.fonts()[1], "C:\\WINDOWS\\FONTS\\ARIALI.TTF", 0);
        match_handle!(family.fonts()[2], "C:\\WINDOWS\\FONTS\\ARIALBD.TTF", 0);
        match_handle!(family.fonts()[3], "C:\\WINDOWS\\FONTS\\ARIALBI.TTF", 0);
        match_handle!(family.fonts()[4], "C:\\WINDOWS\\FONTS\\ARIBLK.TTF", 0);
        match_handle!(family.fonts()[5], "C:\\WINDOWS\\FONTS\\ARIAL.TTF", 0);
        match_handle!(family.fonts()[6], "C:\\WINDOWS\\FONTS\\ARIALBD.TTF", 0);
        match_handle!(family.fonts()[7], "C:\\WINDOWS\\FONTS\\ARIBLK.TTF", 0);
    }

    #[allow(non_snake_case)]
    #[test]
    fn select_by_postscript_name_ArialMT() {
        let font = SystemSource::new()
            .select_by_postscript_name("ArialMT")
            .unwrap()
            .load()
            .unwrap();

        assert_eq!(font.postscript_name().unwrap(), "ArialMT");
    }

    #[test]
    fn select_by_postscript_name_invalid() {
        match SystemSource::new().select_by_postscript_name("zxhjfgkadsfhg") {
            Err(SelectionError::NotFound) => {}
            other => panic!("unexpected error: {:?}", other),
        }
    }

    // This test fails on TravisCI's Windows environment.
    #[test]
    #[ignore]
    fn select_localized_family_name() {
        let handle = SystemSource::new()
            .select_best_match(
                &[FamilyName::Title("ＭＳ ゴシック".to_string())],
                &Properties::default(),
            )
            .unwrap();
        match_handle!(handle, "C:\\WINDOWS\\FONTS\\msgothic.ttc", 0);
    }
}

#[cfg(all(feature = "source", target_os = "linux"))]
mod test {
    use super::*;

    #[test]
    fn select_best_match_serif() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::Serif], &Properties::default())
            .unwrap();
        check_filename(&handle, "DejaVuSerif.ttf");
    }

    #[test]
    fn select_best_match_sans_serif() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::default())
            .unwrap();
        check_filename(&handle, "DejaVuSans.ttf");
    }

    #[test]
    fn select_best_match_monospace() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::Monospace], &Properties::default())
            .unwrap();
        check_filename(&handle, "DejaVuSansMono.ttf");
    }

    #[test]
    fn select_best_match_bold_sans_serif() {
        let handle = SystemSource::new()
            .select_best_match(
                &[FamilyName::SansSerif],
                &Properties {
                    style: font_kit::properties::Style::Normal,
                    weight: font_kit::properties::Weight::BOLD,
                    stretch: font_kit::properties::Stretch::NORMAL,
                },
            )
            .unwrap();
        check_filename(&handle, "DejaVuSans-Bold.ttf");
    }

    #[test]
    fn select_best_match_by_name_after_invalid() {
        let handle = SystemSource::new()
            .select_best_match(
                &[
                    FamilyName::Title("Invalid".to_string()),
                    FamilyName::Title("DejaVu Sans".to_string()),
                ],
                &Properties::default(),
            )
            .unwrap();
        check_filename(&handle, "DejaVuSans.ttf");
    }

    #[test]
    fn select_family_by_name_dejavu() {
        let family = SystemSource::new()
            .select_family_by_name("DejaVu Sans")
            .unwrap();
        let filenames: Vec<String> = family
            .fonts()
            .iter()
            .map(|handle| match *handle {
                Handle::Path {
                    ref path,
                    font_index,
                } => {
                    assert_eq!(font_index, 0);
                    path.file_name()
                        .expect("Where's the filename?")
                        .to_string_lossy()
                        .into_owned()
                }
                _ => panic!("Expected path handle!"),
            })
            .collect();
        assert!(filenames.iter().any(|name| name == "DejaVuSans-Bold.ttf"));
        assert!(filenames.iter().any(|name| name == "DejaVuSans.ttf"));
    }

    #[allow(non_snake_case)]
    #[test]
    fn select_by_postscript_name_ArialMT() {
        let font = SystemSource::new()
            .select_by_postscript_name("DejaVuSans")
            .unwrap()
            .load()
            .unwrap();

        assert_eq!(font.postscript_name().unwrap(), "DejaVuSans");
    }

    #[test]
    fn select_by_postscript_name_invalid() {
        match SystemSource::new().select_by_postscript_name("zxhjfgkadsfhg") {
            Err(SelectionError::NotFound) => {}
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn select_localized_family_name() {
        if let Ok(handle) = SystemSource::new().select_best_match(
            &[FamilyName::Title("さざなみゴシック".to_string())],
            &Properties::default(),
        ) {
            check_filename(&handle, "sazanami-gothic.ttf");
        }
    }
}

#[cfg(all(feature = "source", target_os = "macos"))]
mod test {
    use super::*;

    #[test]
    fn select_best_match_serif() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::Serif], &Properties::default())
            .unwrap();
        match_handle!(handle, "/Library/Fonts/Times New Roman.ttf", 0);
    }

    #[test]
    fn select_best_match_sans_serif() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::default())
            .unwrap();
        match_handle!(handle, "/Library/Fonts/Arial.ttf", 0);
    }

    #[test]
    fn select_best_match_monospace() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::Monospace], &Properties::default())
            .unwrap();
        match_handle!(handle, "/Library/Fonts/Courier New.ttf", 0);
    }

    #[test]
    fn select_best_match_cursive() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::Cursive], &Properties::default())
            .unwrap();
        match_handle!(handle, "/Library/Fonts/Comic Sans MS.ttf", 0);
    }

    #[test]
    fn select_best_match_fantasy() {
        let handle = SystemSource::new()
            .select_best_match(&[FamilyName::Fantasy], &Properties::default())
            .unwrap();
        match_handle!(handle, "/Library/Fonts/Papyrus.ttc", 1);
    }

    #[test]
    fn select_best_match_bold_sans_serif() {
        let handle = SystemSource::new()
            .select_best_match(
                &[FamilyName::SansSerif],
                &Properties {
                    style: font_kit::properties::Style::Normal,
                    weight: font_kit::properties::Weight::BOLD,
                    stretch: font_kit::properties::Stretch::NORMAL,
                },
            )
            .unwrap();
        match_handle!(handle, "/Library/Fonts/Arial Bold.ttf", 0);
    }

    #[test]
    fn select_best_match_by_name_after_invalid() {
        let handle = SystemSource::new()
            .select_best_match(
                &[
                    FamilyName::Title("Invalid".to_string()),
                    FamilyName::Title("Times New Roman".to_string()),
                ],
                &Properties::default(),
            )
            .unwrap();
        match_handle!(handle, "/Library/Fonts/Times New Roman.ttf", 0);
    }

    #[test]
    fn select_family_by_name_arial() {
        let family = SystemSource::new().select_family_by_name("Arial").unwrap();
        assert_eq!(family.fonts().len(), 4);
        match_handle!(family.fonts()[0], "/Library/Fonts/Arial.ttf", 0);
        match_handle!(family.fonts()[1], "/Library/Fonts/Arial Bold.ttf", 0);
        match_handle!(family.fonts()[2], "/Library/Fonts/Arial Bold Italic.ttf", 0);
        match_handle!(family.fonts()[3], "/Library/Fonts/Arial Italic.ttf", 0);
    }

    #[allow(non_snake_case)]
    #[test]
    fn select_by_postscript_name_ArialMT() {
        let font = SystemSource::new()
            .select_by_postscript_name("ArialMT")
            .unwrap()
            .load()
            .unwrap();

        assert_eq!(font.postscript_name().unwrap(), "ArialMT");
    }

    #[test]
    fn select_by_postscript_name_invalid() {
        match SystemSource::new().select_by_postscript_name("zxhjfgkadsfhg") {
            Err(SelectionError::NotFound) => {}
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn select_localized_family_name() {
        let handle = SystemSource::new()
            .select_best_match(
                &[FamilyName::Title("רעננה".to_string())],
                &Properties::default(),
            )
            .unwrap();
        match_handle!(handle, "/Library/Fonts/Raanana.ttc", 0);
    }
}
