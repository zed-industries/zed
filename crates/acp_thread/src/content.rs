use agent_client_protocol::schema::{v1 as acp_v1, v2 as acp_v2};
use anyhow::{Result, bail};

pub fn from_v1(block: acp_v1::ContentBlock) -> Result<acp_v2::ContentBlock> {
    Ok(match block {
        acp_v1::ContentBlock::Text(content) => acp_v2::ContentBlock::Text(
            acp_v2::TextContent::new(content.text)
                .annotations(annotations_from_v1(content.annotations)?)
                .meta(content.meta),
        ),
        acp_v1::ContentBlock::Image(content) => acp_v2::ContentBlock::Image(
            acp_v2::ImageContent::new(content.data, content.mime_type)
                .uri(content.uri)
                .annotations(annotations_from_v1(content.annotations)?)
                .meta(content.meta),
        ),
        acp_v1::ContentBlock::Audio(content) => acp_v2::ContentBlock::Audio(
            acp_v2::AudioContent::new(content.data, content.mime_type)
                .annotations(annotations_from_v1(content.annotations)?)
                .meta(content.meta),
        ),
        acp_v1::ContentBlock::ResourceLink(content) => acp_v2::ContentBlock::ResourceLink(
            acp_v2::ResourceLink::new(content.name, content.uri)
                .title(content.title)
                .description(content.description)
                .mime_type(content.mime_type.map(acp_v2::MediaType::new))
                .size(content.size)
                .annotations(annotations_from_v1(content.annotations)?)
                .meta(content.meta),
        ),
        acp_v1::ContentBlock::Resource(content) => {
            let resource = match content.resource {
                acp_v1::EmbeddedResourceResource::TextResourceContents(resource) => {
                    acp_v2::EmbeddedResourceResource::TextResourceContents(
                        acp_v2::TextResourceContents::new(resource.text, resource.uri)
                            .mime_type(resource.mime_type.map(acp_v2::MediaType::new))
                            .meta(resource.meta),
                    )
                }
                acp_v1::EmbeddedResourceResource::BlobResourceContents(resource) => {
                    acp_v2::EmbeddedResourceResource::BlobResourceContents(
                        acp_v2::BlobResourceContents::new(resource.blob, resource.uri)
                            .mime_type(resource.mime_type.map(acp_v2::MediaType::new))
                            .meta(resource.meta),
                    )
                }
                _ => bail!("unsupported v1 embedded resource variant"),
            };
            acp_v2::ContentBlock::Resource(
                acp_v2::EmbeddedResource::new(resource)
                    .annotations(annotations_from_v1(content.annotations)?)
                    .meta(content.meta),
            )
        }
        _ => bail!("unsupported v1 content block variant"),
    })
}

/// An empty icon list normalizes to absence; other unrepresentable fields are rejected.
pub fn to_v1(block: acp_v2::ContentBlock) -> Result<acp_v1::ContentBlock> {
    Ok(match block {
        acp_v2::ContentBlock::Text(content) => acp_v1::ContentBlock::Text(
            acp_v1::TextContent::new(content.text)
                .annotations(annotations_to_v1(content.annotations)?)
                .meta(content.meta),
        ),
        acp_v2::ContentBlock::Image(content) => acp_v1::ContentBlock::Image(
            acp_v1::ImageContent::new(content.data, content.mime_type.0.to_string())
                .uri(content.uri)
                .annotations(annotations_to_v1(content.annotations)?)
                .meta(content.meta),
        ),
        acp_v2::ContentBlock::Audio(content) => acp_v1::ContentBlock::Audio(
            acp_v1::AudioContent::new(content.data, content.mime_type.0.to_string())
                .annotations(annotations_to_v1(content.annotations)?)
                .meta(content.meta),
        ),
        acp_v2::ContentBlock::ResourceLink(content) => {
            if content
                .icons
                .as_ref()
                .is_some_and(|icons| !icons.is_empty())
            {
                bail!("v2 resource link icons cannot be represented in v1");
            }
            acp_v1::ContentBlock::ResourceLink(
                acp_v1::ResourceLink::new(content.name, content.uri)
                    .title(content.title)
                    .description(content.description)
                    .mime_type(content.mime_type.map(|mime_type| mime_type.0.to_string()))
                    .size(content.size)
                    .annotations(annotations_to_v1(content.annotations)?)
                    .meta(content.meta),
            )
        }
        acp_v2::ContentBlock::Resource(content) => {
            let resource = match content.resource {
                acp_v2::EmbeddedResourceResource::TextResourceContents(resource) => {
                    acp_v1::EmbeddedResourceResource::TextResourceContents(
                        acp_v1::TextResourceContents::new(resource.text, resource.uri)
                            .mime_type(resource.mime_type.map(|mime_type| mime_type.0.to_string()))
                            .meta(resource.meta),
                    )
                }
                acp_v2::EmbeddedResourceResource::BlobResourceContents(resource) => {
                    acp_v1::EmbeddedResourceResource::BlobResourceContents(
                        acp_v1::BlobResourceContents::new(resource.blob, resource.uri)
                            .mime_type(resource.mime_type.map(|mime_type| mime_type.0.to_string()))
                            .meta(resource.meta),
                    )
                }
                _ => bail!("unsupported v2 embedded resource variant"),
            };
            acp_v1::ContentBlock::Resource(
                acp_v1::EmbeddedResource::new(resource)
                    .annotations(annotations_to_v1(content.annotations)?)
                    .meta(content.meta),
            )
        }
        acp_v2::ContentBlock::Other(_) => {
            bail!("v2 custom content block cannot be represented in v1")
        }
        _ => bail!("unsupported v2 content block variant"),
    })
}

pub fn can_convert_to_v1(block: &acp_v2::ContentBlock) -> bool {
    let annotations = match block {
        acp_v2::ContentBlock::Text(content) => &content.annotations,
        acp_v2::ContentBlock::Image(content) => &content.annotations,
        acp_v2::ContentBlock::Audio(content) => &content.annotations,
        acp_v2::ContentBlock::ResourceLink(content)
            if content.icons.as_ref().is_none_or(Vec::is_empty) =>
        {
            &content.annotations
        }
        acp_v2::ContentBlock::Resource(content)
            if matches!(
                content.resource,
                acp_v2::EmbeddedResourceResource::TextResourceContents(_)
                    | acp_v2::EmbeddedResourceResource::BlobResourceContents(_)
            ) =>
        {
            &content.annotations
        }
        _ => return false,
    };
    annotations
        .as_ref()
        .and_then(|annotations| annotations.audience.as_ref())
        .is_none_or(|audience| {
            audience
                .iter()
                .all(|role| matches!(role, acp_v2::Role::Assistant | acp_v2::Role::User))
        })
}

fn annotations_from_v1(
    annotations: Option<acp_v1::Annotations>,
) -> Result<Option<acp_v2::Annotations>> {
    annotations
        .map(|annotations| {
            let audience = annotations
                .audience
                .map(|audience| {
                    audience
                        .into_iter()
                        .map(|role| match role {
                            acp_v1::Role::Assistant => Ok(acp_v2::Role::Assistant),
                            acp_v1::Role::User => Ok(acp_v2::Role::User),
                            _ => bail!("unsupported v1 annotation role"),
                        })
                        .collect::<Result<Vec<_>>>()
                })
                .transpose()?;
            Ok(acp_v2::Annotations::new()
                .audience(audience)
                .last_modified(annotations.last_modified)
                .priority(annotations.priority)
                .meta(annotations.meta))
        })
        .transpose()
}

fn annotations_to_v1(
    annotations: Option<acp_v2::Annotations>,
) -> Result<Option<acp_v1::Annotations>> {
    annotations
        .map(|annotations| {
            let audience = annotations
                .audience
                .map(|audience| {
                    audience
                        .into_iter()
                        .map(|role| match role {
                            acp_v2::Role::Assistant => Ok(acp_v1::Role::Assistant),
                            acp_v2::Role::User => Ok(acp_v1::Role::User),
                            acp_v2::Role::Other(_) => {
                                bail!("v2 annotation role cannot be represented in v1")
                            }
                            _ => bail!("unsupported v2 annotation role"),
                        })
                        .collect::<Result<Vec<_>>>()
                })
                .transpose()?;
            Ok(acp_v1::Annotations::new()
                .audience(audience)
                .last_modified(annotations.last_modified)
                .priority(annotations.priority)
                .meta(annotations.meta))
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::collections::BTreeMap;

    fn meta() -> serde_json::Map<String, Value> {
        serde_json::Map::from_iter([("origin".to_owned(), json!({"nested": [0, null, "value"]}))])
    }

    fn annotations() -> acp_v1::Annotations {
        acp_v1::Annotations::new()
            .audience(vec![acp_v1::Role::User, acp_v1::Role::Assistant])
            .last_modified("2025-01-02T03:04:05Z")
            .priority(0.75)
            .meta(meta())
    }

    fn round_trip(block: acp_v1::ContentBlock) {
        let original = block.clone();
        let upgraded = from_v1(block).expect("v1 block should convert to v2");
        assert!(can_convert_to_v1(&upgraded));
        assert_eq!(
            serde_json::to_value(&upgraded).expect("serialize v2 block"),
            serde_json::to_value(&original).expect("serialize original block")
        );
        let restored = to_v1(upgraded).expect("converted block should convert back to v1");
        assert_eq!(restored, original);
    }

    fn rejects(block: acp_v2::ContentBlock) {
        assert!(!can_convert_to_v1(&block));
        assert!(to_v1(block).is_err());
    }

    #[test]
    fn preserves_all_v1_content_variants_and_metadata() {
        round_trip(acp_v1::ContentBlock::Text(
            acp_v1::TextContent::new("Hello \0 世界")
                .annotations(annotations())
                .meta(meta()),
        ));
        round_trip(acp_v1::ContentBlock::Image(
            acp_v1::ImageContent::new("AAECA/8=", "not/a valid mime ; 😀")
                .uri("")
                .annotations(annotations())
                .meta(meta()),
        ));
        round_trip(acp_v1::ContentBlock::Audio(
            acp_v1::AudioContent::new("///+", "")
                .annotations(annotations())
                .meta(meta()),
        ));
        round_trip(acp_v1::ContentBlock::ResourceLink(
            acp_v1::ResourceLink::new("name", "file:///some/resource")
                .title("")
                .description("description")
                .mime_type("??")
                .size(0)
                .annotations(annotations())
                .meta(meta()),
        ));
        round_trip(acp_v1::ContentBlock::Resource(
            acp_v1::EmbeddedResource::new(acp_v1::EmbeddedResourceResource::TextResourceContents(
                acp_v1::TextResourceContents::new("source text", "file:///text")
                    .mime_type("")
                    .meta(meta()),
            ))
            .annotations(annotations())
            .meta(meta()),
        ));
        round_trip(acp_v1::ContentBlock::Resource(
            acp_v1::EmbeddedResource::new(acp_v1::EmbeddedResourceResource::BlobResourceContents(
                acp_v1::BlobResourceContents::new("AAECA/8=", "file:///blob")
                    .mime_type("bad mime")
                    .meta(meta()),
            ))
            .annotations(annotations())
            .meta(meta()),
        ));
    }

    #[test]
    fn preserves_absent_and_empty_optional_fields() {
        round_trip(acp_v1::ContentBlock::Text(acp_v1::TextContent::new("")));
        round_trip(acp_v1::ContentBlock::Text(
            acp_v1::TextContent::new("")
                .annotations(acp_v1::Annotations::new().audience(Vec::new())),
        ));
        round_trip(acp_v1::ContentBlock::Text(
            acp_v1::TextContent::new("")
                .annotations(acp_v1::Annotations::new())
                .meta(acp_v1::Meta::new()),
        ));
        round_trip(acp_v1::ContentBlock::Resource(
            acp_v1::EmbeddedResource::new(acp_v1::EmbeddedResourceResource::TextResourceContents(
                acp_v1::TextResourceContents::new("", "").meta(acp_v1::Meta::new()),
            ))
            .annotations(acp_v1::Annotations::new().meta(acp_v1::Meta::new()))
            .meta(acp_v1::Meta::new()),
        ));
        round_trip(acp_v1::ContentBlock::Image(acp_v1::ImageContent::new(
            "", "",
        )));
        round_trip(acp_v1::ContentBlock::Audio(acp_v1::AudioContent::new(
            "", "",
        )));
        round_trip(acp_v1::ContentBlock::ResourceLink(
            acp_v1::ResourceLink::new("", ""),
        ));
        round_trip(acp_v1::ContentBlock::Resource(
            acp_v1::EmbeddedResource::new(acp_v1::EmbeddedResourceResource::TextResourceContents(
                acp_v1::TextResourceContents::new("", ""),
            )),
        ));
        round_trip(acp_v1::ContentBlock::Resource(
            acp_v1::EmbeddedResource::new(acp_v1::EmbeddedResourceResource::BlobResourceContents(
                acp_v1::BlobResourceContents::new("", ""),
            )),
        ));
    }

    #[test]
    fn accepts_empty_icons_without_discarding_resource_data() {
        let link = acp_v2::ResourceLink::new("Reference", "https://example.com/resource")
            .title("Resource title")
            .description("Resource description")
            .mime_type("custom/type")
            .size(0)
            .annotations(
                acp_v2::Annotations::new()
                    .audience(vec![acp_v2::Role::User])
                    .meta(meta()),
            )
            .meta(meta());
        let original = acp_v2::ContentBlock::ResourceLink(link.clone().icons(Vec::new()));
        let expected = acp_v2::ContentBlock::ResourceLink(link);

        assert!(can_convert_to_v1(&original));
        assert_eq!(
            serde_json::to_value(&original).expect("serialize original")["icons"],
            json!([])
        );
        let downgraded = to_v1(original).expect("empty icon list should not prevent conversion");
        assert_eq!(
            serde_json::to_value(&downgraded).expect("serialize v1"),
            serde_json::to_value(&expected).expect("serialize normalized v2"),
        );
        assert_eq!(from_v1(downgraded).expect("restore v2"), expected);
    }

    #[test]
    fn rejects_v2_only_content_and_annotations() {
        rejects(acp_v2::ContentBlock::Other(acp_v2::OtherContentBlock::new(
            "_custom",
            BTreeMap::from([("payload".to_owned(), json!({"opaque": [1, 2]}))]),
        )));
        rejects(acp_v2::ContentBlock::Image(
            acp_v2::ImageContent::new("AAE=", "image/png").annotations(
                acp_v2::Annotations::new().audience(vec![acp_v2::Role::Other("_custom".into())]),
            ),
        ));
        rejects(acp_v2::ContentBlock::ResourceLink(
            acp_v2::ResourceLink::new("name", "uri")
                .icons(Vec::new())
                .annotations(
                    acp_v2::Annotations::new()
                        .audience(vec![acp_v2::Role::Other("_custom".into())]),
                ),
        ));
        rejects(acp_v2::ContentBlock::ResourceLink(
            acp_v2::ResourceLink::new("name", "uri").icons(vec![acp_v2::Icon::new("file:///icon")]),
        ));
    }

    fn round_trip_from_v2(block: acp_v2::ContentBlock) {
        assert!(can_convert_to_v1(&block), "{block:?}");
        let original_wire = serde_json::to_value(&block).expect("serialize original v2 block");
        let downgraded = to_v1(block.clone()).expect("representable v2 block converts to v1");
        assert_eq!(
            serde_json::to_value(&downgraded).expect("serialize intermediate v1 block"),
            original_wire
        );
        assert_eq!(
            from_v1(downgraded).expect("v1 block converts back to v2"),
            block
        );
    }

    #[test]
    fn preserves_independently_constructed_v2_content() {
        for (annotations, outer_meta, resource_meta) in [
            (None, None, None),
            (
                Some(
                    acp_v2::Annotations::new()
                        .audience(Vec::new())
                        .meta(serde_json::Map::new()),
                ),
                Some(serde_json::Map::new()),
                Some(serde_json::Map::new()),
            ),
            (
                Some(
                    acp_v2::Annotations::new()
                        .audience(vec![
                            acp_v2::Role::User,
                            acp_v2::Role::Assistant,
                            acp_v2::Role::User,
                        ])
                        .last_modified("not a date \0 雪")
                        .priority(-0.25)
                        .meta(serde_json::Map::from_iter([(
                            "hint".to_owned(),
                            json!({"items": [false, null, "é"]}),
                        )])),
                ),
                Some(serde_json::Map::from_iter([(
                    "outer".to_owned(),
                    json!({"key": [1, "\n"]}),
                )])),
                Some(serde_json::Map::from_iter([(
                    "resource".to_owned(),
                    json!({"nested": {"value": 2}}),
                )])),
            ),
        ] {
            let blocks = [
                acp_v2::ContentBlock::Text(
                    acp_v2::TextContent::new("Hello \0 世界\n")
                        .annotations(annotations.clone())
                        .meta(outer_meta.clone()),
                ),
                acp_v2::ContentBlock::Image(
                    acp_v2::ImageContent::new("AAECA/8=", "not/a valid mime ; 😀")
                        .uri("")
                        .annotations(annotations.clone())
                        .meta(outer_meta.clone()),
                ),
                acp_v2::ContentBlock::Audio(
                    acp_v2::AudioContent::new("///+", "")
                        .annotations(annotations.clone())
                        .meta(outer_meta.clone()),
                ),
                acp_v2::ContentBlock::ResourceLink(
                    acp_v2::ResourceLink::new("名\n", "file:///a")
                        .title("")
                        .description("description \0")
                        .mime_type("??")
                        .size(0)
                        .annotations(annotations.clone())
                        .meta(outer_meta.clone()),
                ),
                acp_v2::ContentBlock::Resource(
                    acp_v2::EmbeddedResource::new(
                        acp_v2::EmbeddedResourceResource::TextResourceContents(
                            acp_v2::TextResourceContents::new("source \0 text", "file:///text")
                                .mime_type("")
                                .meta(resource_meta.clone()),
                        ),
                    )
                    .annotations(annotations.clone())
                    .meta(outer_meta.clone()),
                ),
                acp_v2::ContentBlock::Resource(
                    acp_v2::EmbeddedResource::new(
                        acp_v2::EmbeddedResourceResource::BlobResourceContents(
                            acp_v2::BlobResourceContents::new("AAECA/8=", "file:///blob")
                                .mime_type("bad mime")
                                .meta(resource_meta),
                        ),
                    )
                    .annotations(annotations)
                    .meta(outer_meta),
                ),
            ];
            for block in blocks {
                round_trip_from_v2(block);
            }
        }

        for block in [
            acp_v2::ContentBlock::Text(acp_v2::TextContent::new("")),
            acp_v2::ContentBlock::Image(acp_v2::ImageContent::new("", "")),
            acp_v2::ContentBlock::Audio(acp_v2::AudioContent::new("", "")),
            acp_v2::ContentBlock::ResourceLink(
                acp_v2::ResourceLink::new("", "")
                    .size(i64::MAX)
                    .annotations(acp_v2::Annotations::new().priority(0.125)),
            ),
            acp_v2::ContentBlock::ResourceLink(
                acp_v2::ResourceLink::new("negative size", "file:///negative").size(-1),
            ),
            acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                acp_v2::EmbeddedResourceResource::TextResourceContents(
                    acp_v2::TextResourceContents::new("", ""),
                ),
            )),
            acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                acp_v2::EmbeddedResourceResource::BlobResourceContents(
                    acp_v2::BlobResourceContents::new("", ""),
                ),
            )),
        ] {
            round_trip_from_v2(block);
        }
    }

    #[test]
    fn v1_sdk_wire_values_round_trip_after_sdk_normalization() {
        for wire in [
            json!({
                "type": "text",
                "text": "wire \0 世界",
                "annotations": {
                    "audience": ["user", 42, "_future", "assistant", "user"],
                    "lastModified": false,
                    "priority": "invalid",
                    "_meta": {"valid": [null, 1]}
                },
                "_meta": {"outer": {"nested": true}}
            }),
            json!({
                "type": "image",
                "data": "",
                "mimeType": "not/a valid mime ; 😀",
                "uri": 123,
                "annotations": {"audience": [], "priority": 0.25},
                "_meta": false
            }),
            json!({
                "type": "resource_link",
                "name": "link",
                "uri": "file:///link",
                "title": 7,
                "description": "",
                "mimeType": "??",
                "size": "large",
                "annotations": {"audience": ["assistant"]},
                "_meta": {}
            }),
        ] {
            let decoded: acp_v1::ContentBlock =
                serde_json::from_value(wire.clone()).expect("v1 SDK accepts wire fixture");
            let normalized = serde_json::to_value(&decoded).expect("serialize normalized v1");
            assert_ne!(
                normalized, wire,
                "v1 SDK normalizes malformed optional hints"
            );
            let upgraded = from_v1(decoded.clone()).expect("adapter accepts decoded v1 value");
            assert!(can_convert_to_v1(&upgraded));
            assert_eq!(
                serde_json::to_value(&upgraded).expect("serialize upgraded v2"),
                normalized
            );
            let restored = to_v1(upgraded).expect("adapter restores decoded v1 value");
            assert_eq!(restored, decoded);
            assert_eq!(
                serde_json::to_value(restored).expect("serialize restored v1"),
                normalized
            );
        }
    }
}
