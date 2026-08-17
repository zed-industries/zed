use crate::{rawvalue::RawValue, Properties, PropertiesSource};

static BASIC_KEYS: [&str; 4] = ["2", "3", "0", "1"];
static ALT_VALUES: [&str; 4] = ["a", "b", "c", "d"];

fn zip_self() -> impl Iterator<Item = (&'static str, &'static str)> {
    BASIC_KEYS.iter().cloned().zip(BASIC_KEYS.iter().cloned())
}

fn zip_alts() -> impl Iterator<Item = (&'static str, &'static str)> {
    BASIC_KEYS.iter().cloned().zip(ALT_VALUES.iter().cloned())
}

fn test_basic_keys(props: &Properties) {
    for s in BASIC_KEYS {
        // Test mapping correctness using get.
        assert_eq!(props.get_raw_for_key(s).into_option(), Some(s))
    }
    // Ensure that they keys are returned in order.
    assert!(props.iter().map(|k| k.0).eq(BASIC_KEYS.iter().cloned()))
}

#[test]
fn from_iter() {
    let props: Properties = zip_self().collect();
    test_basic_keys(&props);
}

#[test]
fn insert() {
    let mut props = Properties::new();
    for s in BASIC_KEYS {
        props.insert_raw_for_key(s, s);
    }
    test_basic_keys(&props);
}

#[test]
fn insert_replacing() {
    let mut props: Properties = zip_alts().collect();
    for (k, v) in zip_alts() {
        let old = props
            .get_raw_for_key(k)
            .into_option()
            .expect("missing pair")
            .to_owned();
        assert_eq!(old, v);
        props.insert_raw_for_key(k, k);
    }
    test_basic_keys(&props);
}

#[test]
fn try_insert() {
    let mut props = Properties::new();
    for s in BASIC_KEYS {
        assert!(props.try_insert_raw_for_key(s, s).is_ok());
    }
    test_basic_keys(&props);
}

#[test]
fn try_insert_replacing() {
    let mut props: Properties = zip_self().collect();
    for (k, v) in zip_alts() {
        assert_eq!(
            props
                .try_insert_raw_for_key(k, k)
                .expect_err("try_insert wrongly returns Ok for same value")
                .into_str(),
            k
        );
        assert_eq!(
            props
                .try_insert_raw_for_key(k, v)
                .expect_err("try_insert wrongly returns Ok for update")
                .into_str(),
            k
        );
    }
}

#[test]
fn apply_empty_to() {
    let mut props = Properties::new();
    props.insert_raw_for_key("foo", "a");
    props.insert_raw_for_key("bar", "b");
    let mut empty_pairs = Properties::new();
    empty_pairs.insert_raw_for_key("bar", "");
    empty_pairs.insert_raw_for_key("baz", "");
    assert_eq!(empty_pairs.len(), 2);
    empty_pairs
        .apply_to(&mut props, "")
        .expect("Properties::apply_to should be infallible");
    assert_eq!(props.len(), 3);
    assert_eq!(props.get_raw_for_key("bar"), &RawValue::from(""));
}
