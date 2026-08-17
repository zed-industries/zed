/* Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/ */

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tld {
    CentralWindows,
    Cyrillic,
    Western,
    Greek,
    TurkishAzeri,
    Hebrew,
    Arabic,
    Baltic,
    Vietnamese,
    Thai,
    Simplified,
    Traditional,
    Japanese,
    Korean,
    SimplifiedTraditional,
    TraditionalSimplified,
    CentralIso,
    IcelandicFaroese,
    WesternCyrillic,
    CentralCyrillic,
    WesternArabic,
    Generic,
    Eu,
}

pub fn classify_tld(tld: &[u8]) -> Tld {
    if tld.len() == 2 {
        let key = [tld[0], tld[1]];
        if let Ok(i) = TWO_LETTER_KEYS.binary_search(&key) {
            TWO_LETTER_VALUES[i]
        } else {
            Tld::Western
        }
    } else if tld.len() == 3 {
        match tld {
            b"edu" | b"gov" | b"mil" => Tld::Western,
            _ => Tld::Generic,
        }
    } else if tld.starts_with(b"xn--") && tld.len() >= 8 {
        // It's unclear if including the IDNs here is a good idea.
        // Clearly, they are an anachronism relative to the era
        // of legacy encodings. The idea, consistent with previous
        // approach in Firefox is to address the case where one
        // of these TLDs is configured as an alternative name for
        // a server that also serves the same content from a
        // two-ASCII-letter TLD. This makes the detection result
        // the same either way even though otherwise this thing
        // does not make much sense.
        if let Ok(i) = PUNYCODE_KEYS.binary_search(&&tld[4..]) {
            PUNYCODE_VALUES[i]
        } else {
            Tld::Generic
        }
    } else {
        Tld::Generic
    }
}

static TWO_LETTER_VALUES: [Tld; 87] = [
    Tld::Generic,               // ac
    Tld::Arabic,                // ae
    Tld::Arabic,                // af
    Tld::Generic,               // ai
    Tld::WesternCyrillic,       // am
    Tld::TurkishAzeri,          // az
    Tld::CentralCyrillic,       // ba
    Tld::Cyrillic,              // bg
    Tld::Arabic,                // bh
    Tld::Cyrillic,              // by
    Tld::Generic,               // bz
    Tld::Generic,               // cb
    Tld::Generic,               // cc
    Tld::Generic,               // cd
    Tld::Simplified,            // cn
    Tld::Generic,               // cx
    Tld::Greek,                 // cy
    Tld::CentralWindows,        // cz
    Tld::Generic,               // dj
    Tld::Arabic,                // dz
    Tld::Arabic,                // eg
    Tld::Eu,                    // eu
    Tld::Generic,               // fm
    Tld::IcelandicFaroese,      // fo
    Tld::WesternCyrillic,       // ge
    Tld::Greek,                 // gr
    Tld::TraditionalSimplified, // hk
    Tld::CentralWindows,        // hr
    Tld::CentralIso,            // hu
    Tld::Hebrew,                // il
    Tld::Generic,               // in
    Tld::Arabic,                // iq
    Tld::Arabic,                // ir
    Tld::IcelandicFaroese,      // is
    Tld::Arabic,                // jo
    Tld::Japanese,              // jp
    Tld::Cyrillic,              // kg
    Tld::Korean,                // kp
    Tld::Korean,                // kr
    Tld::Arabic,                // kw
    Tld::Cyrillic,              // kz
    Tld::Generic,               // la
    Tld::Arabic,                // lb
    Tld::Baltic,                // lt
    Tld::Baltic,                // lv
    Tld::Arabic,                // ly
    Tld::Arabic,                // ma
    Tld::Cyrillic,              // md
    Tld::Generic,               // me
    Tld::Cyrillic,              // mk
    Tld::Cyrillic,              // mn
    Tld::TraditionalSimplified, // mo
    Tld::Arabic,                // mr
    Tld::Generic,               // ms
    Tld::WesternArabic,         // my
    Tld::Generic,               // nu
    Tld::Arabic,                // om
    Tld::Arabic,                // pk
    Tld::CentralIso,            // pl
    Tld::Arabic,                // ps
    Tld::Arabic,                // qa
    Tld::CentralWindows,        // ro
    Tld::Cyrillic,              // rs
    Tld::Cyrillic,              // ru
    Tld::Arabic,                // sa
    Tld::Arabic,                // sd
    Tld::SimplifiedTraditional, // sg
    Tld::CentralIso,            // si
    Tld::CentralWindows,        // sk
    Tld::Generic,               // st
    Tld::Cyrillic,              // su
    Tld::Arabic,                // sy
    Tld::Thai,                  // th
    Tld::Cyrillic,              // tj
    Tld::Generic,               // tk
    Tld::Cyrillic,              // tm
    Tld::Arabic,                // tn
    Tld::Generic,               // to
    Tld::TurkishAzeri,          // tr
    Tld::Generic,               // tv
    Tld::Traditional,           // tw
    Tld::Cyrillic,              // ua
    Tld::Cyrillic,              // uz
    Tld::Generic,               // vc
    Tld::Vietnamese,            // vn
    Tld::Generic,               // vu
    Tld::Arabic,                // ye
];

static TWO_LETTER_KEYS: [[u8; 2]; 87] = [
    [b'a', b'c'], // Generic
    [b'a', b'e'], // Arabic
    [b'a', b'f'], // Arabic
    [b'a', b'i'], // Generic
    [b'a', b'm'], // WesternCyrillic
    [b'a', b'z'], // TurkishAzeri
    [b'b', b'a'], // CentralCyrillic
    [b'b', b'g'], // Cyrillic
    [b'b', b'h'], // Arabic
    [b'b', b'y'], // Cyrillic
    [b'b', b'z'], // Generic
    [b'c', b'b'], // Generic
    [b'c', b'c'], // Generic
    [b'c', b'd'], // Generic
    [b'c', b'n'], // Simplified
    [b'c', b'x'], // Generic
    [b'c', b'y'], // Greek
    [b'c', b'z'], // CentralWindows
    [b'd', b'j'], // Generic
    [b'd', b'z'], // Arabic
    [b'e', b'g'], // Arabic
    [b'e', b'u'], // Eu
    [b'f', b'm'], // Generic
    [b'f', b'o'], // IcelandicFaroese
    [b'g', b'e'], // WesternCyrillic
    [b'g', b'r'], // Greek
    [b'h', b'k'], // TraditionalSimplified
    [b'h', b'r'], // CentralWindows
    [b'h', b'u'], // CentralIso
    [b'i', b'l'], // Hebrew
    [b'i', b'n'], // Generic
    [b'i', b'q'], // Arabic
    [b'i', b'r'], // Arabic
    [b'i', b's'], // IcelandicFaroese
    [b'j', b'o'], // Arabic
    [b'j', b'p'], // Japanese
    [b'k', b'g'], // Cyrillic
    [b'k', b'p'], // Korean
    [b'k', b'r'], // Korean
    [b'k', b'w'], // Arabic
    [b'k', b'z'], // Cyrillic
    [b'l', b'a'], // Generic
    [b'l', b'b'], // Arabic
    [b'l', b't'], // Baltic
    [b'l', b'v'], // Baltic
    [b'l', b'y'], // Arabic
    [b'm', b'a'], // Arabic
    [b'm', b'd'], // Cyrillic
    [b'm', b'e'], // Generic
    [b'm', b'k'], // Cyrillic
    [b'm', b'n'], // Cyrillic
    [b'm', b'o'], // TraditionalSimplified
    [b'm', b'r'], // Arabic
    [b'm', b's'], // Generic
    [b'm', b'y'], // WesternArabic
    [b'n', b'u'], // Generic
    [b'o', b'm'], // Arabic
    [b'p', b'k'], // Arabic
    [b'p', b'l'], // CentralIso
    [b'p', b's'], // Arabic
    [b'q', b'a'], // Arabic
    [b'r', b'o'], // CentralWindows
    [b'r', b's'], // Cyrillic
    [b'r', b'u'], // Cyrillic
    [b's', b'a'], // Arabic
    [b's', b'd'], // Arabic
    [b's', b'g'], // SimplifiedTraditional
    [b's', b'i'], // CentralIso
    [b's', b'k'], // CentralWindows
    [b's', b't'], // Generic
    [b's', b'u'], // Cyrillic
    [b's', b'y'], // Arabic
    [b't', b'h'], // Thai
    [b't', b'j'], // Cyrillic
    [b't', b'k'], // Generic
    [b't', b'm'], // Cyrillic
    [b't', b'n'], // Arabic
    [b't', b'o'], // Generic
    [b't', b'r'], // TurkishAzeri
    [b't', b'v'], // Generic
    [b't', b'w'], // Traditional
    [b'u', b'a'], // Cyrillic
    [b'u', b'z'], // Cyrillic
    [b'v', b'c'], // Generic
    [b'v', b'n'], // Vietnamese
    [b'v', b'u'], // Generic
    [b'y', b'e'], // Arabic
];

static PUNYCODE_KEYS: [&'static [u8]; 46] = [
    b"3e0b707e",           // Korean
    b"54b7fta0cc",         // Western
    b"80ao21a",            // Cyrillic
    b"90a3ac",             // Cyrillic
    b"90ae",               // Cyrillic
    b"90ais",              // Cyrillic
    b"clchc0ea0b2g2a9gcd", // SimplifiedTraditional
    b"d1alf",              // Cyrillic
    b"e1a4c",              // Eu
    b"fiqs8S",             // Simplified
    b"fiqz9S",             // Simplified
    b"fzc2c9e2c",          // Western
    b"j1amh",              // Cyrillic
    b"j6w193g",            // TraditionalSimplified
    b"kprw13d",            // Traditional
    b"kpry57d",            // Traditional
    b"l1acc",              // Cyrillic
    b"lgbbat1ad8j",        // Arabic
    b"mgb2ddes",           // Arabic
    b"mgb9awbf",           // Arabic
    b"mgba3a4f16a",        // Arabic
    b"mgbaam7a8h",         // Arabic
    b"mgbah1a3hjkrd",      // Arabic
    b"mgbai9azgqp6j",      // Arabic
    b"mgbayh7gpa",         // Arabic
    b"mgbc0a9azcg",        // Arabic
    b"mgbcpq6gpa1a",       // Arabic
    b"mgberp4a5d4ar",      // Arabic
    b"mgbpl2fh",           // Arabic
    b"mgbtx2b",            // Arabic
    b"mgbx4cd0ab",         // WesternArabic
    b"mix891f",            // TraditionalSimplified
    b"node",               // WesternCyrillic
    b"o3cw4h",             // Thai
    b"ogbpf8fl",           // Arabic
    b"p1ai",               // Cyrillic
    b"pgbs0dh",            // Arabic
    b"q7ce6a",             // Arabic
    b"qxa6a",              // Eu
    b"qxam",               // Greek
    b"wgbh1c",             // Arabic
    b"wgbl6a",             // Arabic
    b"xkc2al3hye2a",       // Western
    b"y9a3aq",             // WesternCyrillic
    b"yfro4i67o",          // SimplifiedTraditional
    b"ygbi2ammx",          // Arabic
];

static PUNYCODE_VALUES: [Tld; 46] = [
    Tld::Korean,                // 3e0b707e
    Tld::Western,               // 54b7fta0cc
    Tld::Cyrillic,              // 80ao21a
    Tld::Cyrillic,              // 90a3ac
    Tld::Cyrillic,              // 90ae
    Tld::Cyrillic,              // 90ais
    Tld::SimplifiedTraditional, // clchc0ea0b2g2a9gcd
    Tld::Cyrillic,              // d1alf
    Tld::Eu,                    // e1a4c
    Tld::Simplified,            // fiqs8S
    Tld::Simplified,            // fiqz9S
    Tld::Western,               // fzc2c9e2c
    Tld::Cyrillic,              // j1amh
    Tld::TraditionalSimplified, // j6w193g
    Tld::Traditional,           // kprw13d
    Tld::Traditional,           // kpry57d
    Tld::Cyrillic,              // l1acc
    Tld::Arabic,                // lgbbat1ad8j
    Tld::Arabic,                // mgb2ddes
    Tld::Arabic,                // mgb9awbf
    Tld::Arabic,                // mgba3a4f16a
    Tld::Arabic,                // mgbaam7a8h
    Tld::Arabic,                // mgbah1a3hjkrd
    Tld::Arabic,                // mgbai9azgqp6j
    Tld::Arabic,                // mgbayh7gpa
    Tld::Arabic,                // mgbc0a9azcg
    Tld::Arabic,                // mgbcpq6gpa1a
    Tld::Arabic,                // mgberp4a5d4ar
    Tld::Arabic,                // mgbpl2fh
    Tld::Arabic,                // mgbtx2b
    Tld::WesternArabic,         // mgbx4cd0ab
    Tld::TraditionalSimplified, // mix891f
    Tld::WesternCyrillic,       // node
    Tld::Thai,                  // o3cw4h
    Tld::Arabic,                // ogbpf8fl
    Tld::Cyrillic,              // p1ai
    Tld::Arabic,                // pgbs0dh
    Tld::Arabic,                // q7ce6a
    Tld::Eu,                    // qxa6a
    Tld::Greek,                 // qxam
    Tld::Arabic,                // wgbh1c
    Tld::Arabic,                // wgbl6a
    Tld::Western,               // xkc2al3hye2a
    Tld::WesternCyrillic,       // y9a3aq
    Tld::SimplifiedTraditional, // yfro4i67o
    Tld::Arabic,                // ygbi2ammx
];
