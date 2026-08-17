#![cfg_attr(not(feature = "enabled"), allow(dead_code))]

failspot::failspot_name! {
    pub enum FailSpotName {
        One,
        Two,
        Three,
        Four,
    }
}

mod inner {
    failspot::failspot_name! {
        pub enum FailSpotName2 {
            Four,
            Five,
            Six,
            Seven,
        }
    }
}

fn stop_process_or_fail() -> Result<(), Box<dyn std::error::Error>> {
    failspot::failspot!(One bail(std::io::Error::other("fail")));
    failspot::failspot!(<inner::FailSpotName2>::Four bail(std::io::Error::other("fail")));
    Ok(())
}

fn fill_missing_auxv_or_panic() {
    failspot::failspot!(Two panic!());
    failspot::failspot!(<inner::FailSpotName2>::Five panic!());
}

fn get_thread_name_or_none() -> Option<String> {
    if failspot::failspot!(Three) {
        return None;
    }
    if failspot::failspot!(<inner::FailSpotName2>::Six) {
        return None;
    }
    Some("mythread".to_string())
}

fn common() -> Result<Option<String>, Box<dyn std::error::Error>> {
    stop_process_or_fail()?;
    fill_missing_auxv_or_panic();
    let name = get_thread_name_or_none();

    let name = failspot::failspot!(if Four {
        name.map(|_| "deleted".to_string())
    } else {
        name
    });

    let name = name.map(|name| {
        failspot::failspot!(if <inner::FailSpotName2>::Seven {
            "deleted".to_string()
        } else {
            name
        })
    });

    Ok(name)
}

#[cfg(feature = "enabled")]
#[test]
fn no_fails() -> Result<(), Box<dyn std::error::Error>> {
    let _client = FailSpotName::testing_client();
    let _client2 = inner::FailSpotName2::testing_client();
    let thread_name = common()?;
    assert_eq!(thread_name.as_deref(), Some("mythread"));
    Ok(())
}

#[cfg(feature = "enabled")]
#[test]
#[should_panic]
fn panic_fail() {
    let mut client = FailSpotName::testing_client();
    let _client2 = inner::FailSpotName2::testing_client();
    client.set_enabled(FailSpotName::Two, true);
    let _ = common();
}

#[cfg(feature = "enabled")]
#[test]
#[should_panic]
fn panic_fail2() {
    let mut _client = FailSpotName::testing_client();
    let mut client2 = inner::FailSpotName2::testing_client();
    client2.set_enabled(inner::FailSpotName2::Five, true);
    let _ = common();
}

#[cfg(feature = "enabled")]
#[test]
fn test_error() {
    let mut client = FailSpotName::testing_client();
    let mut _client2 = inner::FailSpotName2::testing_client();
    client.set_enabled(FailSpotName::One, true);
    common().unwrap_err();
}

#[cfg(feature = "enabled")]
#[test]
fn test_error2() {
    let mut _client = FailSpotName::testing_client();
    let mut client2 = inner::FailSpotName2::testing_client();
    client2.set_enabled(inner::FailSpotName2::Four, true);
    common().unwrap_err();
}

#[cfg(feature = "enabled")]
#[test]
fn expression_fail() {
    let mut client = FailSpotName::testing_client();
    let mut _client2 = inner::FailSpotName2::testing_client();
    client.set_enabled(FailSpotName::Three, true);
    let thread_name = common().unwrap();
    assert!(thread_name.is_none());
}

#[cfg(feature = "enabled")]
#[test]
fn expression_fail2() {
    let mut _client = FailSpotName::testing_client();
    let mut client2 = inner::FailSpotName2::testing_client();
    client2.set_enabled(inner::FailSpotName2::Six, true);
    let thread_name = common().unwrap();
    assert!(thread_name.is_none());
}

#[cfg(feature = "enabled")]
#[test]
fn if_statement_fail() {
    let mut client = FailSpotName::testing_client();
    let mut _client2 = inner::FailSpotName2::testing_client();
    client.set_enabled(FailSpotName::Four, true);
    let thread_name = common().unwrap();
    assert!(thread_name.as_deref() == Some("deleted"));
}

#[cfg(feature = "enabled")]
#[test]
fn if_statement_fail2() {
    let mut _client = FailSpotName::testing_client();
    let mut client2 = inner::FailSpotName2::testing_client();
    client2.set_enabled(inner::FailSpotName2::Seven, true);
    let thread_name = common().unwrap();
    assert!(thread_name.as_deref() == Some("deleted"));
}
