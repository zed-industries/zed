use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::{PredicateNode, SettingGroupBuilder};

macro_rules! define_zvl_ext {
    (DEF: $settings:expr, $size:expr) => {{
        let name = concat!("has_zvl", $size, "b");
        let desc = concat!("has extension Zvl", $size, "b?");
        let comment = concat!(
            "Zvl",
            $size,
            "b: Vector register has a minimum of ",
            $size,
            " bits"
        );
        $settings.add_bool(&name, &desc, &comment, false)
    }};
    ($settings:expr, $size:expr $(, $implies:expr)*) => {{
        let has_feature = define_zvl_ext!(DEF: $settings, $size);

        let name = concat!("zvl", $size, "b");
        let desc = concat!("Has a vector register size of at least ", $size, " bits");

        let preset = $settings.add_preset(&name, &desc, preset!(has_feature $( && $implies )*));
        (has_feature, preset)
    }};
}

pub(crate) fn define() -> TargetIsa {
    let mut setting = SettingGroupBuilder::new("riscv64");

    // We target a minimum of riscv64g. That means that we have the following extensions by default:
    //
    // * M (integer multiplication and division)
    // * A (atomic instructions)
    // * F (single-precision floating point)
    // * D (double-precision floating point)
    // * Zicsr (control and status register instructions)
    // * Zifencei (instruction-fetch fence)

    let has_m = setting.add_bool(
        "has_m",
        "has extension M?",
        "Integer multiplication and division",
        true,
    );
    let has_a = setting.add_bool("has_a", "has extension A?", "Atomic instructions", true);
    let has_f = setting.add_bool(
        "has_f",
        "has extension F?",
        "Single-precision floating point",
        true,
    );
    let has_d = setting.add_bool(
        "has_d",
        "has extension D?",
        "Double-precision floating point",
        true,
    );

    let _has_zfa = setting.add_bool(
        "has_zfa",
        "has extension Zfa?",
        "Zfa: Extension for Additional Floating-Point Instructions",
        false,
    );

    let _has_zfhmin = setting.add_bool(
        "has_zfhmin",
        "has extension Zfhmin?",
        "Zfhmin: Minimal Half-Precision Floating-Point",
        false,
    );

    let _has_zfh = setting.add_bool(
        "has_zfh",
        "has extension Zfh?",
        "Zfh: Half-Precision Floating-Point Instructions",
        false,
    );

    let _has_v = setting.add_bool(
        "has_v",
        "has extension V?",
        "Vector instruction support",
        false,
    );

    let _has_zvfh = setting.add_bool(
        "has_zvfh",
        "has extension Zvfh?",
        "Zvfh: Vector Extension for Half-Precision Floating-Point",
        false,
    );

    let has_zca = setting.add_bool(
        "has_zca",
        "has extension Zca?",
        "Zca is the C extension without floating point loads",
        false,
    );
    let has_zcd = setting.add_bool(
        "has_zcd",
        "has extension Zcd?",
        "Zcd contains only the double precision floating point loads from the C extension",
        false,
    );
    setting.add_preset(
        "has_c",
        "Support for compressed instructions",
        preset!(has_zca && has_zcd),
    );

    let _has_zcb = setting.add_bool(
        "has_zcb",
        "has extension Zcb?",
        "Zcb: Extra compressed instructions",
        false,
    );

    let _has_zbkb = setting.add_bool(
        "has_zbkb",
        "has extension zbkb?",
        "Zbkb: Bit-manipulation for Cryptography",
        false,
    );
    let _has_zba = setting.add_bool(
        "has_zba",
        "has extension zba?",
        "Zba: Address Generation",
        false,
    );
    let _has_zbb = setting.add_bool(
        "has_zbb",
        "has extension zbb?",
        "Zbb: Basic bit-manipulation",
        false,
    );
    let _has_zbc = setting.add_bool(
        "has_zbc",
        "has extension zbc?",
        "Zbc: Carry-less multiplication",
        false,
    );
    let _has_zbs = setting.add_bool(
        "has_zbs",
        "has extension zbs?",
        "Zbs: Single-bit instructions",
        false,
    );
    let _has_zicond = setting.add_bool(
        "has_zicond",
        "has extension zicond?",
        "ZiCond: Integer Conditional Operations",
        false,
    );

    let has_zicsr = setting.add_bool(
        "has_zicsr",
        "has extension zicsr?",
        "Zicsr: Control and Status Register (CSR) Instructions",
        true,
    );
    let has_zifencei = setting.add_bool(
        "has_zifencei",
        "has extension zifencei?",
        "Zifencei: Instruction-Fetch Fence",
        true,
    );

    // Zvl*: Minimum Vector Length Standard Extensions
    // These extension specify the minimum number of bits in a vector register.
    // Since it is a minimum, Zvl64b implies Zvl32b, Zvl128b implies Zvl64b, etc.
    // The V extension supports a maximum of 64K bits in a single register.
    //
    // See: https://github.com/riscv/riscv-v-spec/blob/master/v-spec.adoc#181-zvl-minimum-vector-length-standard-extensions
    let (_, zvl32b) = define_zvl_ext!(setting, 32);
    let (_, zvl64b) = define_zvl_ext!(setting, 64, zvl32b);
    let (_, zvl128b) = define_zvl_ext!(setting, 128, zvl64b);
    let (_, zvl256b) = define_zvl_ext!(setting, 256, zvl128b);
    let (_, zvl512b) = define_zvl_ext!(setting, 512, zvl256b);
    let (_, zvl1024b) = define_zvl_ext!(setting, 1024, zvl512b);
    let (_, zvl2048b) = define_zvl_ext!(setting, 2048, zvl1024b);
    let (_, zvl4096b) = define_zvl_ext!(setting, 4096, zvl2048b);
    let (_, zvl8192b) = define_zvl_ext!(setting, 8192, zvl4096b);
    let (_, zvl16384b) = define_zvl_ext!(setting, 16384, zvl8192b);
    let (_, zvl32768b) = define_zvl_ext!(setting, 32768, zvl16384b);
    let (_, _zvl65536b) = define_zvl_ext!(setting, 65536, zvl32768b);

    setting.add_predicate(
        "has_g",
        predicate!(has_m && has_a && has_f && has_d && has_zicsr && has_zifencei),
    );

    TargetIsa::new("riscv64", setting.build())
}
