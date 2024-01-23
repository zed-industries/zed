use schemars::{
    schema::{InstanceType, Schema, SchemaObject, SingleOrVec},
    JsonSchema,
};

macro_rules! create_definitions {
    ($($(#[$meta:meta])* ($name:ident, $idx:expr)),* $(,)?) => {

        /// The OpenType features that can be configured for a given font.
        #[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
        pub struct FontFeatures {
            enabled: u64,
            disabled: u64,
        }

        impl FontFeatures {
            $(
                /// Get the current value of the corresponding OpenType feature
                pub fn $name(&self) -> Option<bool> {
                    if (self.enabled & (1 << $idx)) != 0 {
                        Some(true)
                    } else if (self.disabled & (1 << $idx)) != 0 {
                        Some(false)
                    } else {
                        None
                    }
                }
            )*
        }

        impl std::fmt::Debug for FontFeatures {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug = f.debug_struct("FontFeatures");
                $(
                    if let Some(value) = self.$name() {
                        debug.field(stringify!($name), &value);
                    };
                )*
                debug.finish()
            }
        }

        impl<'de> serde::Deserialize<'de> for FontFeatures {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::{MapAccess, Visitor};
                use std::fmt;

                struct FontFeaturesVisitor;

                impl<'de> Visitor<'de> for FontFeaturesVisitor {
                    type Value = FontFeatures;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a map of font features")
                    }

                    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                    where
                        M: MapAccess<'de>,
                    {
                        let mut enabled: u64 = 0;
                        let mut disabled: u64 = 0;

                        while let Some((key, value)) = access.next_entry::<String, Option<bool>>()? {
                            let idx = match key.as_str() {
                                $(stringify!($name) => $idx,)*
                                _ => continue,
                            };
                            match value {
                                Some(true) => enabled |= 1 << idx,
                                Some(false) => disabled |= 1 << idx,
                                None => {}
                            };
                        }
                        Ok(FontFeatures { enabled, disabled })
                    }
                }

                let features = deserializer.deserialize_map(FontFeaturesVisitor)?;
                Ok(features)
            }
        }

        impl serde::Serialize for FontFeatures {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeMap;

                let mut map = serializer.serialize_map(None)?;

                $(
                    let feature = stringify!($name);
                    if let Some(value) = self.$name() {
                        map.serialize_entry(feature, &value)?;
                    }
                )*

                map.end()
            }
        }

        impl JsonSchema for FontFeatures {
            fn schema_name() -> String {
                "FontFeatures".into()
            }

            fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> Schema {
                let mut schema = SchemaObject::default();
                let properties = &mut schema.object().properties;
                let feature_schema = Schema::Object(SchemaObject {
                    instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Boolean))),
                    ..Default::default()
                });

                $(
                    properties.insert(stringify!($name).to_owned(), feature_schema.clone());
                )*

                schema.into()
            }
        }
    };
}

create_definitions!(
    (calt, 0),
    (case, 1),
    (cpsp, 2),
    (frac, 3),
    (liga, 4),
    (onum, 5),
    (ordn, 6),
    (pnum, 7),
    (ss01, 8),
    (ss02, 9),
    (ss03, 10),
    (ss04, 11),
    (ss05, 12),
    (ss06, 13),
    (ss07, 14),
    (ss08, 15),
    (ss09, 16),
    (ss10, 17),
    (ss11, 18),
    (ss12, 19),
    (ss13, 20),
    (ss14, 21),
    (ss15, 22),
    (ss16, 23),
    (ss17, 24),
    (ss18, 25),
    (ss19, 26),
    (ss20, 27),
    (subs, 28),
    (sups, 29),
    (swsh, 30),
    (titl, 31),
    (tnum, 32),
    (zero, 33)
);
