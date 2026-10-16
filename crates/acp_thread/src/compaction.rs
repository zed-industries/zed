use agent_client_protocol::schema::{MaybeUndefined, v1 as acp_v1, v2 as acp_v2};
use anyhow::{Result, bail};

pub fn update_from_v1(update: acp_v1::CompactionUpdate) -> Result<acp_v2::CompactionUpdate> {
    let status = match update.status {
        acp_v1::CompactionStatus::InProgress => acp_v2::CompactionStatus::InProgress,
        acp_v1::CompactionStatus::Completed => acp_v2::CompactionStatus::Completed,
        acp_v1::CompactionStatus::Failed => acp_v2::CompactionStatus::Failed,
        acp_v1::CompactionStatus::Cancelled => acp_v2::CompactionStatus::Cancelled,
        acp_v1::CompactionStatus::Other(status) => acp_v2::CompactionStatus::Other(status),
        _ => bail!("unsupported v1 compaction status variant"),
    };
    let summary = match update.summary {
        MaybeUndefined::Undefined => MaybeUndefined::Undefined,
        MaybeUndefined::Null => MaybeUndefined::Null,
        MaybeUndefined::Value(blocks) => MaybeUndefined::Value(
            blocks
                .into_iter()
                .map(crate::content::from_v1)
                .collect::<Result<Vec<_>>>()?,
        ),
    };
    Ok(
        acp_v2::CompactionUpdate::new(acp_v2::CompactionId::new(update.compaction_id.0), status)
            .summary(summary)
            .error(update.error)
            .meta(update.meta),
    )
}

pub fn chunk_from_v1(
    chunk: acp_v1::CompactionSummaryChunk,
) -> Result<acp_v2::CompactionSummaryChunk> {
    Ok(acp_v2::CompactionSummaryChunk::new(
        acp_v2::CompactionId::new(chunk.compaction_id.0),
        crate::content::from_v1(chunk.content)?,
    )
    .meta(chunk.meta))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::Arc;

    #[test]
    fn legacy_compaction_preserves_statuses_patches_and_metadata_scopes() -> Result<()> {
        let opaque_id = "  compaction/\0e\u{301}雪  ";
        let content = json!({
            "type": "resource",
            "resource": {
                "uri": "file:///summary/\0雪",
                "text": "retained \0 summary",
                "mimeType": "not/a valid mime ; 😀",
                "_meta": {"resource": {"nested": [null, "", {}]}}
            },
            "annotations": {
                "audience": ["assistant", "user"],
                "lastModified": "not a date",
                "priority": -0.25,
                "_meta": {"annotation": {"nested": [false, 0]}}
            },
            "_meta": {"content": {"nested": [null, [], true]}}
        });
        for (wire, status) in [
            (
                json!({"compactionId": opaque_id, "status": "in_progress"}),
                acp_v2::CompactionStatus::InProgress,
            ),
            (
                json!({
                    "compactionId": opaque_id, "status": "completed",
                    "summary": null, "error": null, "_meta": null
                }),
                acp_v2::CompactionStatus::Completed,
            ),
            (
                json!({
                    "compactionId": opaque_id, "status": "failed",
                    "summary": [], "error": "", "_meta": {}
                }),
                acp_v2::CompactionStatus::Failed,
            ),
            (
                json!({
                    "compactionId": opaque_id, "status": "cancelled",
                    "summary": [content], "error": "retained \0 error",
                    "_meta": {"compaction": {"nested": [null, {}, "雪"]}}
                }),
                acp_v2::CompactionStatus::Cancelled,
            ),
            (
                json!({"compactionId": opaque_id, "status": "_custom \0 雪"}),
                acp_v2::CompactionStatus::Other("_custom \0 雪".into()),
            ),
            (
                json!({"compactionId": opaque_id, "status": "future_status"}),
                acp_v2::CompactionStatus::Other("future_status".into()),
            ),
        ] {
            let update: acp_v1::CompactionUpdate = serde_json::from_value(wire.clone())?;
            let original_id = update.compaction_id.0.clone();
            let converted = update_from_v1(update)?;
            assert!(Arc::ptr_eq(&converted.compaction_id.0, &original_id));
            assert_eq!(converted.status, status);
            assert_eq!(serde_json::to_value(converted)?, wire);
        }

        for meta in [
            None,
            Some(acp_v1::Meta::new()),
            Some(acp_v1::Meta::from_iter([(
                "delivery".into(),
                json!({"nested": [null, false, {"sequence": 0}]}),
            )])),
        ] {
            let original_id: Arc<str> = opaque_id.into();
            let chunk = acp_v1::CompactionSummaryChunk::new(
                acp_v1::CompactionId::new(original_id.clone()),
                serde_json::from_value(content.clone())?,
            )
            .meta(meta.clone());
            let converted = chunk_from_v1(chunk)?;
            assert!(Arc::ptr_eq(&converted.compaction_id.0, &original_id));
            assert_eq!(converted.meta, meta);
            assert_eq!(serde_json::to_value(converted.content)?, content);
        }
        Ok(())
    }
}
