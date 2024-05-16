use std::sync::Arc;

use schemars::schema::{InstanceType, SchemaObject};

/// The OpenType features that can be configured for a given font.
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FontFeatures(pub Option<Arc<Vec<(String, u32)>>>);

impl FontFeatures {
    /// Get the tag name list of the font OpenType features
    /// only enabled or disabled features are returned
    pub fn tag_value_list(&self) -> Vec<(String, u32)> {
        let mut result = Vec::new();
        if let Some(ref feature_list) = self.0 {
            for (tag, value) in feature_list.iter() {
                result.push((tag.clone(), *value));
            }
        }
        result
    }
}

impl std::fmt::Debug for FontFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("FontFeatures");
        if let Some(ref feature_list) = self.0 {
            for (tag, value) in feature_list.iter() {
                debug.field(tag, value);
            }
        }
        debug.finish()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum FeatureValue {
    Bool(bool),
    Number(serde_json::Number),
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
                let mut feature_list = Vec::new();

                while let Some((key, value)) =
                    access.next_entry::<String, Option<FeatureValue>>()?
                {
                    if key.len() != 4 && !key.is_ascii() {
                        log::error!("Incorrect font feature tag: {}", key);
                        continue;
                    }
                    if let Some(value) = value {
                        match value {
                            FeatureValue::Bool(enable) => {
                                if enable {
                                    feature_list.push((key, 1));
                                } else {
                                    feature_list.push((key, 0));
                                }
                            }
                            FeatureValue::Number(value) => {
                                if value.is_u64() {
                                    feature_list.push((key, value.as_u64().unwrap() as u32));
                                } else {
                                    log::error!(
                                        "Incorrect font feature value {} for feature tag {}",
                                        value,
                                        key
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                }
                if feature_list.is_empty() {
                    Ok(FontFeatures(None))
                } else {
                    Ok(FontFeatures(Some(Arc::new(feature_list))))
                }
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
        if let Some(ref feature_list) = self.0 {
            for (tag, value) in feature_list.iter() {
                map.serialize_entry(tag, value)?;
            }
        }
        map.end()
    }
}

impl schemars::JsonSchema for FontFeatures {
    fn schema_name() -> String {
        "FontFeatures".into()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut schema = SchemaObject::default();
        schema.instance_type = Some(schemars::schema::SingleOrVec::Single(Box::new(
            InstanceType::Object,
        )));
        {
            let mut property = SchemaObject::default();
            property.instance_type = Some(schemars::schema::SingleOrVec::Single(Box::new(
                InstanceType::Boolean,
            )));
            schema
                .object()
                .pattern_properties
                .insert("[0-9a-zA-Z]{4}$".into(), property.into());
        }
        {
            let mut property = SchemaObject::default();
            property.instance_type = Some(schemars::schema::SingleOrVec::Single(Box::new(
                InstanceType::Integer,
            )));
            property.number().minimum = Some(0.0);
            schema
                .object()
                .pattern_properties
                .insert("[0-9a-zA-Z]{4}$".into(), property.into());
        }
        schema.into()
    }
}
