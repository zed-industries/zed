use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::SettingGroupBuilder;

pub(crate) fn define() -> TargetIsa {
    let mut settings = SettingGroupBuilder::new("s390x");

    // The baseline architecture for cranelift is z14 (arch12),
    // so we list only facilities of later processors here.

    // z15 (arch13) facilities
    let has_mie3 = settings.add_bool(
        "has_mie3",
        "Has Miscellaneous-Instruction-Extensions Facility 3 support.",
        "",
        false,
    );
    let has_vxrs_ext2 = settings.add_bool(
        "has_vxrs_ext2",
        "Has Vector-Enhancements Facility 2 support.",
        "",
        false,
    );

    // z16 (arch14) has no new facilities that can be exploited by cranelift

    // z17 (arch15) facilities
    let has_mie4 = settings.add_bool(
        "has_mie4",
        "Has Miscellaneous-Instruction-Extensions Facility 4 support.",
        "",
        false,
    );
    let has_vxrs_ext3 = settings.add_bool(
        "has_vxrs_ext3",
        "Has Vector-Enhancements Facility 3 support.",
        "",
        false,
    );

    // Architecture level presets
    settings.add_preset(
        "arch13",
        "Thirteenth Edition of the z/Architecture.",
        preset!(has_mie3 && has_vxrs_ext2),
    );
    settings.add_preset(
        "arch14",
        "Fourteenth Edition of the z/Architecture.",
        preset!(has_mie3 && has_vxrs_ext2),
    );
    settings.add_preset(
        "arch15",
        "Fifteenth Edition of the z/Architecture.",
        preset!(has_mie3 && has_mie4 && has_vxrs_ext2 && has_vxrs_ext3),
    );

    // Processor presets
    settings.add_preset(
        "z15",
        "IBM z15 processor.",
        preset!(has_mie3 && has_vxrs_ext2),
    );
    settings.add_preset(
        "z16",
        "IBM z16 processor.",
        preset!(has_mie3 && has_vxrs_ext2),
    );
    settings.add_preset(
        "z17",
        "IBM z17 processor.",
        preset!(has_mie3 && has_mie4 && has_vxrs_ext2 && has_vxrs_ext3),
    );

    TargetIsa::new("s390x", settings.build())
}
