//! The [feature name](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6feat.html) table.

include!("../../generated/generated_feat.rs");

impl Feat<'_> {
    /// Returns the name for the given feature code.
    pub fn find(&self, feature: u16) -> Option<FeatureName> {
        let names = self.names();
        let ix = names
            .binary_search_by(|name| name.feature().cmp(&feature))
            .ok()?;
        names.get(ix).copied()
    }
}

impl FeatureName {
    /// Returns true if the feature settings are mutually exclusive.
    pub fn is_exclusive(&self) -> bool {
        // Bit 31 signifies a mutually exclusive feature
        self.feature_flags() & 0x8000 != 0
    }

    /// Returns the index of the default setting for the feature.
    pub fn default_setting_index(&self) -> u16 {
        // If bit 30 is set, the default setting index is in the low byte
        if self.feature_flags() & 0x4000 != 0 {
            self.feature_flags() & 0xFF
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::*;

    #[test]
    fn feat_example() {
        let feat_data = build_feat_example();
        let feat = Feat::read(feat_data.data().into()).unwrap();
        let names = feat.names();
        #[rustfmt::skip]
        let expected_name_fields = [
            // (feature, n_settings, flags, name, exclusive, default_index)
            (0, 1, 0, NameId::new(260), false, 0),
            (1, 1, 0, NameId::new(256), false, 0),
            (3, 3, 0x8000, NameId::new(262), true, 0),
            (6, 2, 0xC001, NameId::new(258), true, 1),
        ];
        let name_fields = names
            .iter()
            .map(|name| {
                (
                    name.feature(),
                    name.n_settings(),
                    name.feature_flags(),
                    name.name_index(),
                    name.is_exclusive(),
                    name.default_setting_index(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(name_fields, expected_name_fields);
        #[rustfmt::skip]
        let expected_setting_names: [&[(u16, NameId)]; 4] = [
            &[(0, NameId::new(261))],
            &[(2, NameId::new(257))],
            &[(0, NameId::new(268)), (3, NameId::new(264)), (4, NameId::new(265))],
            &[(0, NameId::new(259)), (1, NameId::new(260))],
        ];
        let setting_names = names
            .iter()
            .map(|name| {
                let settings = name.setting_table(feat.offset_data()).unwrap();
                settings
                    .settings()
                    .iter()
                    .map(|setting| (setting.setting(), setting.name_index()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert!(expected_setting_names.iter().eq(setting_names.iter()));
    }

    #[test]
    fn feat_find() {
        let feat_data = build_feat_example();
        let feat = Feat::read(feat_data.data().into()).unwrap();
        // List of available feature types
        let valid_features = [0, 1, 3, 6];
        for i in 0..10 {
            let is_valid = valid_features.contains(&i);
            let name = feat.find(i);
            if is_valid {
                assert_eq!(name.unwrap().feature(), i);
            } else {
                assert!(name.is_none());
            }
        }
    }

    fn build_feat_example() -> BeBuffer {
        // Example taken from bottom of <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6feat.html>
        let mut buf = BeBuffer::new();
        // header
        buf = buf.push(0x00010000u32).extend([4u16, 0, 0, 0]);
        // feature name array
        buf = buf.extend([0u16, 1]).push(60u32).extend([0u16, 260]);
        buf = buf.extend([1u16, 1]).push(64u32).extend([0u16, 256]);
        buf = buf.extend([3u16, 3]).push(68u32).extend([0x8000u16, 262]);
        buf = buf.extend([6u16, 2]).push(80u32).extend([0xC001u16, 258]);
        // The setting name array
        buf.extend([0u16, 261, 2, 257, 0, 268, 3, 264, 4, 265, 0, 259, 1, 260])
    }
}
