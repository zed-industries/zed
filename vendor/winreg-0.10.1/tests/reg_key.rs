extern crate rand;
extern crate tempfile;
extern crate winapi;
extern crate winreg;
#[cfg(feature = "serialization-serde")]
#[macro_use]
extern crate serde_derive;
use self::rand::Rng;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use tempfile::tempdir;
use winapi::shared::winerror;
use winreg::enums::*;
use winreg::types::FromRegValue;
use winreg::{RegKey, RegValue};

#[test]
fn test_raw_handle() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let handle = hklm.raw_handle();
    assert_eq!(HKEY_LOCAL_MACHINE, handle);
}

#[test]
fn test_load_appkey() {
    let val_name = "LoadKeyTest";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("RustLoadAppkeyTest.dat");
    let val1 = "Test123".to_owned();
    {
        let key1 = RegKey::load_app_key(&file_path, true).unwrap();
        key1.set_value(val_name, &val1).unwrap();
        // this fails on Windows 7 with ERROR_ALREADY_EXISTS
        let key_err = RegKey::load_app_key_with_flags(&file_path, KEY_READ, 0).unwrap_err();
        assert_eq!(
            key_err.raw_os_error(),
            Some(winerror::ERROR_SHARING_VIOLATION as i32)
        );
    }
    let val2: String = {
        // this fails on Windows 7 with ERROR_ALREADY_EXISTS
        let key2 = RegKey::load_app_key_with_flags(&file_path, KEY_READ, 1).unwrap();
        key2.get_value(val_name).unwrap()
    };
    assert_eq!(val1, val2);
}

#[test]
fn test_open_subkey_with_flags_query_info() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let win = hklm
        .open_subkey_with_flags("Software\\Microsoft\\Windows", KEY_READ)
        .unwrap();

    let info = win.query_info().unwrap();
    info.get_last_write_time_system();
    #[cfg(feature = "chrono")]
    info.get_last_write_time_chrono();

    assert!(win
        .open_subkey_with_flags("CurrentVersion\\", KEY_READ)
        .is_ok());
    assert!(hklm
        .open_subkey_with_flags("i\\just\\hope\\nobody\\created\\that\\key", KEY_READ)
        .is_err());
}

#[test]
fn test_create_subkey_disposition() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Software\\WinRegRsTestCreateSubkey";
    let (_subkey, disp) = hkcu.create_subkey(path).unwrap();
    assert_eq!(disp, REG_CREATED_NEW_KEY);
    let (_subkey2, disp2) = hkcu.create_subkey(path).unwrap();
    assert_eq!(disp2, REG_OPENED_EXISTING_KEY);
    hkcu.delete_subkey_all(&path).unwrap();
}

macro_rules! with_key {
    ($k:ident, $path:expr => $b:block) => {{
        let mut path = "Software\\WinRegRsTest".to_owned();
        path.push_str($path);
        let ($k, _disp) = RegKey::predef(HKEY_CURRENT_USER)
            .create_subkey(&path).unwrap();
        $b
        RegKey::predef(HKEY_CURRENT_USER)
        .delete_subkey_all(path).unwrap();
    }}
}

#[test]
fn test_delete_subkey() {
    let path = "Software\\WinRegRsTestDeleteSubkey";
    RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(path)
        .unwrap();
    assert!(RegKey::predef(HKEY_CURRENT_USER)
        .delete_subkey(path)
        .is_ok());
}

#[test]
fn test_delete_subkey_with_flags() {
    let path = "Software\\Classes\\WinRegRsTestDeleteSubkeyWithFlags";
    RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey_with_flags(path, KEY_WOW64_32KEY)
        .unwrap();
    assert!(RegKey::predef(HKEY_CURRENT_USER)
        .delete_subkey_with_flags(path, KEY_WOW64_32KEY)
        .is_ok());
}

#[test]
fn test_copy_tree() {
    with_key!(key, "CopyTree" => {
        let (sub_tree, _sub_tree_disp) = key.create_subkey("Src\\Sub\\Tree").unwrap();
        for v in &["one", "two", "three"] {
            sub_tree.set_value(v, v).unwrap();
        }
        let (dst, _dst_disp) = key.create_subkey("Dst").unwrap();
        assert!(key.copy_tree("Src", &dst).is_ok());
    });
}

#[test]
fn test_long_value() {
    with_key!(key, "LongValue" => {
        let name = "RustLongVal";
        let val1 = RegValue { vtype: REG_BINARY, bytes: (0..6000).map(|_| rand::random::<u8>()).collect() };
        key.set_raw_value(name, &val1).unwrap();
        let val2 = key.get_raw_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

macro_rules! test_value_sz {
    ($fname:ident, $kname:expr, $conv:expr => $tout:ty) => {
        #[test]
        fn $fname() {
            with_key!(key, $kname => {
                let name = "RustSzVal";
                let val1 = $conv("Test123 \n$%^&|+-*/\\()");
                key.set_value(name, &val1).unwrap();
                let val2: $tout = key.get_value(name).unwrap();
                assert_eq!(val1, val2);
            });
        }
    }
}

test_value_sz!(test_string_value, "StringValue", str::to_owned => String);
test_value_sz!(test_str_value, "StrValue", |x|x => String);
test_value_sz!(test_os_string_value, "OsStringValue", OsString::from => OsString);
test_value_sz!(test_os_str_value, "OsStrValue", OsStr::new => OsString);

#[test]
fn test_long_string_value() {
    with_key!(key, "LongStringValue" => {
        let name = "RustLongStringVal";
        let val1 : String = rand::thread_rng().gen_ascii_chars().take(7000).collect();
        key.set_value(name, &val1).unwrap();
        let val2: String = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

#[test]
fn test_long_os_string_value() {
    with_key!(key, "LongOsStringValue" => {
        let name = "RustLongOsStringVal";
        let val1 = rand::thread_rng().gen_ascii_chars().take(7000).collect::<String>();
        let val1 = OsStr::new(&val1);
        key.set_value(name, &val1).unwrap();
        let val2: OsString = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

macro_rules! test_value_multi_sz {
    ($fname:ident, $kname:expr, $conv:expr => $tout:ty) => {
        #[test]
        fn $fname() {
            with_key!(key, $kname => {
                let name = "RustMultiSzVal";

                let val1 = vec![
                    $conv("lorem ipsum\ndolor"),
                    $conv("sit amet")
                ];
                key.set_value(name, &val1).unwrap();
                let val2: Vec<$tout> = key.get_value(name).unwrap();

                assert_eq!(val1, val2);
            });
        }
    }
}

test_value_multi_sz!(test_vec_string_value, "StringVectorValue", str::to_owned => String);
test_value_multi_sz!(test_vec_str_value, "StrVectorValue", |x|x => String);
test_value_multi_sz!(test_vec_os_string_value, "OsStringVectorValue", OsString::from => OsString);
test_value_multi_sz!(test_vec_os_str_value, "OsStrVectorValue", OsStr::new => OsString);

#[test]
fn test_u32_value() {
    with_key!(key, "U32Value" => {
        let name = "RustU32Val";
        let val1 = 1_234_567_890u32;
        key.set_value(name, &val1).unwrap();
        let val2: u32 = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

#[test]
fn test_u64_value() {
    with_key!(key, "U64Value" => {
        let name = "RustU64Val";
        let val1 = 1_234_567_891_011_121_314u64;
        key.set_value(name, &val1).unwrap();
        let val2: u64 = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

#[test]
fn test_delete_value() {
    with_key!(key, "DeleteValue" => {
        let name = "WinregRsTestVal";
        key.set_value(name, &"Qwerty123").unwrap();
        assert!(key.delete_value(name).is_ok());
    });
}

#[test]
fn test_enum_keys() {
    with_key!(key, "EnumKeys" => {
        let mut keys1 = vec!("qwerty", "asdf", "1", "2", "3", "5", "8", "йцукен");
        keys1.sort_unstable();
        for i in &keys1 {
            key.create_subkey(i).unwrap();
        }
        let keys2: Vec<_> = key.enum_keys().map(|x| x.unwrap()).collect();
        assert_eq!(keys1, keys2);
    });
}

#[test]
fn test_enum_values() {
    with_key!(key, "EnumValues" => {
        let mut vals1 = vec!("qwerty", "asdf", "1", "2", "3", "5", "8", "йцукен");
        vals1.sort_unstable();
        for i in &vals1 {
            key.set_value(i,i).unwrap();
        }
        let mut vals2: Vec<String> = Vec::with_capacity(vals1.len());
        let mut vals3: Vec<String> = Vec::with_capacity(vals1.len());
        for (name, val) in key.enum_values()
            .map(|x| x.unwrap())
        {
            vals2.push(name);
            vals3.push(String::from_reg_value(&val).unwrap());
        }
        assert_eq!(vals1, vals2);
        assert_eq!(vals1, vals3);
    });
}

#[test]
fn test_enum_long_values() {
    with_key!(key, "EnumLongValues" => {
        let mut vals = HashMap::with_capacity(3);

        for i in &[5500, 9500, 15000] {
            let name: String = format!("val{}", i);
            let val = RegValue { vtype: REG_BINARY, bytes: (0..*i).map(|_| rand::random::<u8>()).collect() };
            vals.insert(name, val);
        }

        for (name, val) in key.enum_values()
                              .map(|x| x.unwrap())
        {
            assert_eq!(val.bytes, vals[&name].bytes);
        }
    });
}

#[cfg(feature = "serialization-serde")]
#[test]
fn test_serialization() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Rectangle {
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Test {
        t_bool: bool,
        t_u8: u8,
        t_u16: u16,
        t_u32: u32,
        t_u64: u64,
        t_usize: usize,
        t_struct: Rectangle,
        t_string: String,
        t_map: HashMap<String, HashMap<String, u32>>,
        t_i8: i8,
        t_i16: i16,
        t_i32: i32,
        t_i64: i64,
        t_isize: isize,
        t_f64: f64,
        t_f32: f32,
        t_char: char,
    }

    let mut k1 = HashMap::new();
    k1.insert("val1".to_owned(), 32);
    k1.insert("val2".to_owned(), 64);
    k1.insert("val3".to_owned(), 128);

    let mut k2 = HashMap::new();
    k2.insert("val1".to_owned(), 256);
    k2.insert("val2".to_owned(), 512);
    k2.insert("val3".to_owned(), 1024);

    let mut map = HashMap::new();
    map.insert("key1".to_owned(), k1);
    map.insert("key2".to_owned(), k2);

    let v1 = Test {
        t_bool: false,
        t_u8: 127,
        t_u16: 32768,
        t_u32: 123_456_789,
        t_u64: 123_456_789_101_112,
        t_usize: 1_234_567_891,
        t_struct: Rectangle {
            x: 55,
            y: 77,
            w: 500,
            h: 300,
        },
        t_map: map,
        t_string: "Test123 \n$%^&|+-*/\\()".to_owned(),
        t_i8: -123,
        t_i16: -2049,
        t_i32: 20100,
        t_i64: -12_345_678_910,
        t_isize: -1_234_567_890,
        t_f64: -0.01,
        t_f32: 3.15,
        t_char: 'a',
    };

    with_key!(key, "Serialization" => {
        key.encode(&v1).unwrap();
        let v2: Test = key.decode().unwrap();
        assert_eq!(v1, v2);
    });
}
