use emojis::{SkinTone, UnicodeVersion};

#[test]
fn get_variation() {
    assert_eq!(emojis::get("☹"), emojis::get("☹️"));
}

#[test]
fn iter_only_default_skin_tones() {
    assert!(emojis::iter().all(|emoji| matches!(emoji.skin_tone(), Some(SkinTone::Default) | None)));
    assert_ne!(
        emojis::iter()
            .filter(|emoji| matches!(emoji.skin_tone(), Some(SkinTone::Default)))
            .count(),
        0
    );
}

#[test]
fn unicode_version_partial_ord() {
    assert!(UnicodeVersion::new(13, 0) >= UnicodeVersion::new(12, 0));
    assert!(UnicodeVersion::new(12, 1) >= UnicodeVersion::new(12, 0));
    assert!(UnicodeVersion::new(12, 0) >= UnicodeVersion::new(12, 0));
    assert!(UnicodeVersion::new(12, 0) < UnicodeVersion::new(12, 1));
    assert!(UnicodeVersion::new(11, 0) < UnicodeVersion::new(12, 1));
    assert!(UnicodeVersion::new(11, 0) < UnicodeVersion::new(12, 1));
}

#[test]
fn emoji_partial_eq_str() {
    assert_eq!(emojis::get("😀").unwrap(), "😀");
}

#[test]
fn emoji_display() {
    let s = emojis::get("😀").unwrap().to_string();
    assert_eq!(s, "😀");
}

#[test]
fn emoji_skin_tones() {
    let skin_tones = [
        SkinTone::Default,
        SkinTone::Light,
        SkinTone::MediumLight,
        SkinTone::Medium,
        SkinTone::MediumDark,
        SkinTone::Dark,
        SkinTone::LightAndMediumLight,
        SkinTone::LightAndMedium,
        SkinTone::LightAndMediumDark,
        SkinTone::LightAndDark,
        SkinTone::MediumLightAndLight,
        SkinTone::MediumLightAndMedium,
        SkinTone::MediumLightAndMediumDark,
        SkinTone::MediumLightAndDark,
        SkinTone::MediumAndLight,
        SkinTone::MediumAndMediumLight,
        SkinTone::MediumAndMediumDark,
        SkinTone::MediumAndDark,
        SkinTone::MediumDarkAndLight,
        SkinTone::MediumDarkAndMediumLight,
        SkinTone::MediumDarkAndMedium,
        SkinTone::MediumDarkAndDark,
        SkinTone::DarkAndLight,
        SkinTone::DarkAndMediumLight,
        SkinTone::DarkAndMedium,
        SkinTone::DarkAndMediumDark,
    ];

    for emoji in emojis::iter() {
        match emoji.skin_tone() {
            Some(_) => {
                let emojis: Vec<_> = emoji.skin_tones().unwrap().collect();
                assert!(emojis.len() == 6 || emojis.len() == 26);
                let default = emojis[0];
                for (emoji, skin_tone) in emojis
                    .iter()
                    .zip(skin_tones.iter().copied().take(emojis.len()))
                {
                    assert_eq!(emoji.skin_tone().unwrap(), skin_tone, "{emojis:#?}");
                    assert_eq!(default.with_skin_tone(skin_tone).unwrap(), *emoji);
                    assert_eq!(emoji.with_skin_tone(SkinTone::Default).unwrap(), default);
                }
            }
            None => {
                assert!(emoji.skin_tones().is_none());
            }
        }
    }
}

#[test]
fn emoji_shortcodes() {
    for emoji in emojis::iter() {
        assert_eq!(emoji.shortcodes().next(), emoji.shortcode());
    }
}

#[test]
fn group_iter_and_emojis() {
    let left: Vec<_> = emojis::Group::iter().flat_map(|g| g.emojis()).collect();
    let right: Vec<_> = emojis::iter().collect();
    assert_eq!(left, right);
}
