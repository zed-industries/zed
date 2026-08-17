#![cfg(feature = "emoji")]

#[test]
fn all_ascii_are_either_nonemoji_or_emojiother() {
    use unicode_properties::EmojiStatus;
    use unicode_properties::UnicodeEmoji;
    for i in 0u8..=255u8 {
        let c = i as char;
        let s = c.emoji_status();
        assert!(matches!(
            s,
            EmojiStatus::NonEmoji
                | EmojiStatus::EmojiOther
                | EmojiStatus::EmojiOtherAndEmojiComponent
        ))
    }
}

#[test]
fn emoji_test() {
    use std::ops::Not;
    use unicode_properties::EmojiStatus;
    use unicode_properties::UnicodeEmoji;
    assert_eq!('🦀'.emoji_status(), EmojiStatus::EmojiPresentation);
    assert!('🦀'.is_emoji_char());
    assert!('🦀'.is_emoji_component().not());
    assert!('🦀'.is_emoji_char_or_emoji_component());
}
