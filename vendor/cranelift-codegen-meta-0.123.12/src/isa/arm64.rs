use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::SettingGroupBuilder;

pub(crate) fn define() -> TargetIsa {
    let mut settings = SettingGroupBuilder::new("arm64");

    settings.add_bool(
        "has_lse",
        "Has Large System Extensions (FEAT_LSE) support.",
        "",
        false,
    );
    settings.add_bool(
        "has_pauth",
        "Has Pointer authentication (FEAT_PAuth) support; enables the use of \
         non-HINT instructions, but does not have an effect on code generation \
         by itself.",
        "",
        false,
    );
    settings.add_bool(
        "has_fp16",
        "Use half-precision floating point (FEAT_FP16) instructions.",
        "",
        false,
    );
    settings.add_bool(
        "sign_return_address_all",
        "If function return address signing is enabled, then apply it to all \
        functions; does not have an effect on code generation by itself.",
        "",
        false,
    );
    settings.add_bool(
        "sign_return_address",
        "Use pointer authentication instructions to sign function return \
         addresses; HINT-space instructions using the A key are generated \
         and simple functions that do not use the stack are not affected \
         unless overridden by other settings.",
        "",
        false,
    );
    settings.add_bool(
        "sign_return_address_with_bkey",
        "Use the B key with pointer authentication instructions instead of \
        the default A key; does not have an effect on code generation by \
        itself. Some platform ABIs may require this, for example.",
        "",
        false,
    );
    settings.add_bool(
        "use_bti",
        "Use Branch Target Identification (FEAT_BTI) instructions.",
        "",
        false,
    );

    TargetIsa::new("arm64", settings.build())
}
