use crate::cdsl::{isa::TargetIsa, settings::SettingGroupBuilder};

pub(crate) fn define() -> TargetIsa {
    let mut settings = SettingGroupBuilder::new("pulley");
    settings.add_enum(
        "pointer_width",
        "The width of pointers for this Pulley target",
        "Supported values:\n\
         * 'pointer32'\n\
         * 'pointer64'\n",
        vec!["pointer32", "pointer64"],
    );
    settings.add_bool(
        "big_endian",
        "Whether this is a big-endian target",
        "Whether this is a big-endian target",
        false,
    );
    TargetIsa::new("pulley", settings.build())
}
