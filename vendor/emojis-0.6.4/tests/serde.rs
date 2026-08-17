#![cfg(feature = "serde")]

use emojis::{Emoji, Group, SkinTone, UnicodeVersion};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Test {
    skin_tone: SkinTone,
    group: Group,
    emoji: &'static Emoji,
    version: UnicodeVersion,
}

#[test]
fn test_serialize_roundtrip_json() {
    let test = Test {
        emoji: emojis::get("🚀").unwrap(),
        version: UnicodeVersion::new(13, 0),
        skin_tone: SkinTone::Default,
        group: Group::Activities,
    };
    let serialized = serde_json::to_string_pretty(&test).unwrap();
    assert_eq!(
        serialized,
        r#"{
  "skin_tone": "Default",
  "group": "Activities",
  "emoji": "🚀",
  "version": {
    "major": 13,
    "minor": 0
  }
}"#
    );
    let deserialized: Test = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized, test);
}

#[test]
fn test_serialize_roundtrip_toml() {
    let test = Test {
        emoji: emojis::get("🚀").unwrap(),
        skin_tone: SkinTone::Default,
        group: Group::Activities,
        version: UnicodeVersion::new(13, 0),
    };
    let serialized = toml::to_string(&test).unwrap();
    assert_eq!(
        serialized,
        r#"skin_tone = "Default"
group = "Activities"
emoji = "🚀"

[version]
major = 13
minor = 0
"#
    );

    let deserialized: Test = toml::from_str(&serialized).unwrap();

    assert_eq!(deserialized, test);
}

#[test]
fn emoji_deserialize_invalid() {
    let err = serde_json::from_str::<Test>(r#"{"emoji":"invalid"}"#).unwrap_err();
    assert_eq!(err.to_string(), "invalid emoji at line 1 column 18");
}
