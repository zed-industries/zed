#![cfg(all(test, unix, not(target_os = "macos")))]

use dbus::arg::messageitem::MessageItem as Item;

use self::Hint;
use super::{Urgency::*, *};

static _INIT: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
    color_backtrace::install();
});

#[test]
fn hint_to_item() {
    let category = &Hint::Category("test-me".to_owned());
    let (k, v) = category.into();

    let test_k = Item::Str("category".into());
    let test_v = Item::Variant(Box::new(Item::Str("test-me".into())));

    assert_eq!(k, test_k);
    assert_eq!(v, test_v);
}

#[test]
fn urgency() {
    let low = &Hint::Urgency(Low);
    let (k, v) = low.into();

    let test_k = Item::Str("urgency".into());
    let test_v = Item::Variant(Box::new(Item::Byte(0)));

    assert_eq!(k, test_k);
    assert_eq!(v, test_v);
}

#[test]
fn simple_hint_to_item() {
    let old_hint = &Hint::Custom("foo".into(), "bar".into());

    let (k, v) = old_hint.into();
    let hint: Hint = (&k, &v).into();

    assert_eq!(old_hint, &hint);
}

#[test]
#[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
fn imagedata_hint_to_item() {
    let hint = &Hint::ImageData(Image::from_rgb(1, 1, vec![0, 0, 0]).unwrap());
    let item: MessageItem = hint.into();
    let test_item = Item::DictEntry(
        Box::new(Item::Str(image_spec(*::SPEC_VERSION))),
        Box::new(Item::Variant(Box::new(Item::Struct(vec![
            Item::Int32(1),
            Item::Int32(1),
            Item::Int32(3),
            Item::Bool(false),
            Item::Int32(8),
            Item::Int32(3),
            Item::Array(
                dbus::MessageItemArray::new(
                    vec![Item::Byte(0), Item::Byte(0), Item::Byte(0)],
                    "ay".into(),
                )
                .unwrap(),
            ),
        ])))),
    );
    assert_eq!(item, test_item);
}

#[test]
#[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
fn imagedata_hint_to_item_with_spec() {
    let key = image_spec(Version::new(1, 0));
    assert_eq!(key, String::from("icon_data"));

    let key = image_spec(Version::new(1, 1));
    assert_eq!(key, String::from("image_data"));

    let key = image_spec(Version::new(1, 2));
    assert_eq!(key, String::from("image-data"));
}
