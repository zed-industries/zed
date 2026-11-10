use super::*;
use anyhow::{Context as _, ensure};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedInlineReview {
    pub version: u32,
    pub discussion_id: String,
    pub submission_id: String,
    pub path: std::path::PathBuf,
    pub start_line: u32,
    pub end_line: u32,
    pub excerpt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<InlineReviewDiff>,
    pub before: String,
    pub after: String,
    pub comment: String,
    pub created_at: i64,
    pub outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl SavedInlineReview {
    fn status(&self) -> String {
        match self.outcome.as_str() {
            "pending" => {
                "Interrupted or still running elsewhere — check pi's transcript; not resent".into()
            }
            "answered" => "Answered".into(),
            "cancelled" => "Cancelled".into(),
            "not_sent" => "Not sent".into(),
            "failed" => format!(
                "Failed: {}",
                self.error.as_deref().unwrap_or("see pi's transcript")
            ),
            other => format!("Unknown stored outcome: {other}"),
        }
    }
}

pub(super) fn locate_excerpt(text: &str, review: &SavedInlineReview) -> Option<Range<usize>> {
    let context = format!("{}{}{}", review.before, review.excerpt, review.after);
    let mut found = None;
    let mut offset = 0;
    while let Some(relative) = text.get(offset..)?.find(&context) {
        let start = offset + relative;
        let end = start + context.len();
        if (start == 0 || text.as_bytes().get(start - 1) == Some(&b'\n'))
            && (context.ends_with('\n')
                || end == text.len()
                || text.as_bytes().get(end) == Some(&b'\n'))
        {
            if found.is_some() {
                return None;
            }
            found = Some(
                start + review.before.len()..start + review.before.len() + review.excerpt.len(),
            );
        }
        match text.get(start..)?.chars().next() {
            Some(character) => offset = start + character.len_utf8(),
            None => break,
        }
    }
    found
}

impl Editor {
    pub fn inline_review_file_path(&self, cx: &App) -> Result<std::path::PathBuf> {
        Ok(self
            .inline_review_source(self.inline_review_selection(cx)?, cx)?
            .path)
    }

    pub fn restore_inline_review(
        &mut self,
        review: SavedInlineReview,
        on_submit: ReviewSubmitHandler,
        replace_submit_handler: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        ensure!(
            (review.version == 1 && review.diff.is_none())
                || (review.version == 2 && review.diff.is_some()),
            "Saved review belongs to another file or storage version"
        );
        let current_source = self.inline_review_source(self.inline_review_selection(cx)?, cx)?;
        ensure!(
            current_source.path == review.path,
            "Saved review belongs to another file or storage version"
        );
        ensure!(
            review.start_line < review.end_line,
            "Invalid saved review lines"
        );
        ensure!(
            (review.outcome == "answered") == review.reply.is_some(),
            "Invalid saved reply outcome"
        );
        if current_source.diff.as_ref().map(|diff| diff.comparison)
            != review.diff.as_ref().map(|diff| diff.comparison)
        {
            return Ok(());
        }
        if let Some(diff) = &review.diff {
            if !self.inline_review_side_visible(diff.side, cx)? {
                return Ok(());
            }
        }
        let status = review.status();
        uuid::Uuid::parse_str(&review.submission_id)?;
        uuid::Uuid::parse_str(&review.discussion_id)?;
        if let Some(comment) = self
            .stored_review_comments
            .iter_mut()
            .flat_map(|(_, comments)| comments)
            .find(|comment| comment.submission_id.as_ref() == Some(&review.submission_id))
        {
            let id = comment.id;
            if comment.restored_pending && review.outcome != "pending" {
                comment.restored_pending = false;
                comment.submission_status = Some(status);
                if let Some(reply) = review.reply {
                    self.set_inline_review_reply(id, "Pi".into(), reply, cx);
                }
                cx.notify();
            }
            return self.show_saved_review_overlay(
                id,
                on_submit,
                replace_submit_handler,
                window,
                cx,
            );
        }
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let anchors = self.saved_review_range(&review, cx)?;
        let Range { start, end } = anchors.to_point(&snapshot);
        ensure!(
            start.column == 0 && end.column == snapshot.line_len(MultiBufferRow(end.row)),
            "Saved review does not cover complete current lines"
        );
        let key = Self::review_hunk_key(&snapshot, start);
        ensure!(
            self.comments_for_hunk(&key, &snapshot)
                .iter()
                .all(|comment| comment.discussion_id.as_ref() == Some(&review.discussion_id)),
            "Another discussion occupies this line; saved discussion remains in remote storage"
        );
        ensure!(
            self.diff_review_overlays
                .iter()
                .filter(|overlay| Self::hunk_keys_match(&overlay.hunk_key, &key, &snapshot))
                .all(|overlay| overlay.on_submit.is_some()
                    && overlay.anchor_range.to_point(&snapshot) == (start..end)),
            "Another review range occupies this line; saved discussion remains in remote storage"
        );
        let id = self.add_review_comment(key, review.comment, anchors, cx);
        if let Some(comment) = self
            .stored_review_comments
            .iter_mut()
            .flat_map(|(_, comments)| comments)
            .find(|comment| comment.id == id)
        {
            comment.original_excerpt = Some(review.excerpt);
            comment.diff = review.diff;
            comment.discussion_id = Some(review.discussion_id);
            comment.submission_id = Some(review.submission_id);
            comment.submission_status = Some(status);
            comment.restored_pending = review.outcome == "pending";
        }
        if let Some(reply) = review.reply {
            self.set_inline_review_reply(id, "Pi".into(), reply, cx);
        }
        self.show_saved_review_overlay(id, on_submit, replace_submit_handler, window, cx)
    }

    fn show_saved_review_overlay(
        &mut self,
        id: usize,
        on_submit: ReviewSubmitHandler,
        replace_submit_handler: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let (key, range) = self
            .stored_review_comments
            .iter()
            .find_map(|(key, comments)| {
                comments
                    .iter()
                    .find(|comment| comment.id == id)
                    .map(|comment| (key.clone(), comment.range.clone()))
            })
            .context("Saved review was removed")?;
        let snapshot = self.buffer.read(cx).snapshot(cx);
        if let Some(overlay) = self
            .diff_review_overlays
            .iter_mut()
            .find(|overlay| Self::hunk_keys_match(&overlay.hunk_key, &key, &snapshot))
        {
            if replace_submit_handler {
                overlay.on_submit = Some(on_submit);
            }
            return Ok(());
        }
        ensure!(
            range.start.is_valid(&snapshot) && range.end.is_valid(&snapshot),
            "Review anchor is no longer valid; saved discussion remains in storage"
        );
        self.show_review_overlay_for_anchor_range(range, Some(on_submit), window, cx);
        self.set_review_overlay_collapsed(&key, true, window, cx);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn saved() -> SavedInlineReview {
        SavedInlineReview {
            version: 1,
            discussion_id: uuid::Uuid::new_v4().to_string(),
            submission_id: uuid::Uuid::new_v4().to_string(),
            path: "/project/file".into(),
            start_line: 1,
            end_line: 2,
            excerpt: "target".into(),
            diff: None,
            before: "before\n".into(),
            after: "\nafter\n".into(),
            comment: "Why?".into(),
            created_at: 1,
            outcome: "answered".into(),
            reply: Some("Because".into()),
            error: None,
        }
    }

    #[test]
    fn restore_requires_unique_complete_context_including_overlapping_matches() {
        let mut review = saved();
        assert_eq!(
            locate_excerpt("before\ntarget\nafter\n", &review),
            Some(7..13)
        );
        assert_eq!(
            locate_excerpt("new\nbefore\ntarget\nafter\n", &review),
            Some(11..17)
        );
        assert_eq!(locate_excerpt("before\nchanged\nafter\n", &review), None);
        assert_eq!(
            locate_excerpt("before\ntarget\nafter\nbefore\ntarget\nafter\n", &review),
            None
        );
        review.before = "x\n".into();
        review.excerpt = "x".into();
        review.after = "\nx\n".into();
        assert_eq!(locate_excerpt("x\nx\nx\nx\n", &review), None);
        assert_eq!(locate_excerpt("prefixx\nx\nx\n", &review), None);
    }
}
