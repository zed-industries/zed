#[allow(clippy::unusual_byte_groupings)]
pub fn get(openssl_version: Option<u64>, libressl_version: Option<u64>) -> Vec<&'static str> {
    let mut cfgs = vec![];

    if let Some(libressl_version) = libressl_version {
        cfgs.push("libressl");

        cfgs.push("libressl251");
        cfgs.push("libressl252");
        cfgs.push("libressl261");
        cfgs.push("libressl270");
        cfgs.push("libressl271");
        cfgs.push("libressl273");
        cfgs.push("libressl280");
        cfgs.push("libressl281");
        cfgs.push("libressl291");
        cfgs.push("libressl310");
        cfgs.push("libressl321");
        cfgs.push("libressl332");
        cfgs.push("libressl340");
        cfgs.push("libressl350");

        if libressl_version >= 0x3_06_00_00_0 {
            cfgs.push("libressl360");
        }
        if libressl_version >= 0x3_07_00_00_0 {
            cfgs.push("libressl370");
        }
        if libressl_version >= 0x3_08_00_00_0 {
            cfgs.push("libressl380");
        }
        if libressl_version >= 0x3_08_01_00_0 {
            cfgs.push("libressl381");
        }
        if libressl_version >= 0x3_08_02_00_0 {
            cfgs.push("libressl382");
        }
        if libressl_version >= 0x3_09_00_00_0 {
            cfgs.push("libressl390");
        }
        if libressl_version >= 0x4_00_00_00_0 {
            cfgs.push("libressl400");
        }
        if libressl_version >= 0x4_01_00_00_0 {
            cfgs.push("libressl410");
        }
        if libressl_version >= 0x4_02_00_00_0 {
            cfgs.push("libressl420");
        }
        if libressl_version >= 0x4_03_00_00_0 {
            cfgs.push("libressl430");
        }
    } else {
        let openssl_version = openssl_version.unwrap();
        cfgs.push("ossl101");
        cfgs.push("ossl102");
        cfgs.push("ossl102f");
        cfgs.push("ossl102h");
        cfgs.push("ossl110");
        if openssl_version >= 0x1_01_00_06_0 {
            cfgs.push("ossl110f");
        }
        if openssl_version >= 0x1_01_00_07_0 {
            cfgs.push("ossl110g");
        }
        if openssl_version >= 0x1_01_00_08_0 {
            cfgs.push("ossl110h");
        }
        if openssl_version >= 0x1_01_01_00_0 {
            cfgs.push("ossl111");
        }
        if openssl_version >= 0x1_01_01_02_0 {
            cfgs.push("ossl111b");
        }
        if openssl_version >= 0x1_01_01_03_0 {
            cfgs.push("ossl111c");
        }
        if openssl_version >= 0x1_01_01_04_0 {
            cfgs.push("ossl111d");
        }
        if openssl_version >= 0x3_00_00_00_0 {
            cfgs.push("ossl300");
        }
        if openssl_version >= 0x3_02_00_00_0 {
            cfgs.push("ossl320");
        }
        if openssl_version >= 0x3_03_00_00_0 {
            cfgs.push("ossl330");
        }
        if openssl_version >= 0x3_04_00_00_0 {
            cfgs.push("ossl340");
        }
        if openssl_version >= 0x3_05_00_00_0 {
            cfgs.push("ossl350");
        }
        if openssl_version >= 0x4_00_00_00_0 {
            cfgs.push("ossl400");
        }
    }

    cfgs
}
