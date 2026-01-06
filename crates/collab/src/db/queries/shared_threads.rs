use chrono::Utc;

use super::*;
use crate::db::tables::shared_thread;

impl Database {
    /// Upsert a shared thread. If a thread with this ID already exists and belongs
    /// to the same user, update it. Otherwise, create a new one.
    /// Returns the SharedThreadId.
    pub async fn upsert_shared_thread(
        &self,
        id: SharedThreadId,
        user_id: UserId,
        title: &str,
        data: Vec<u8>,
    ) -> Result<()> {
        let title = title.to_string();
        self.transaction(|tx| {
            let title = title.clone();
            let data = data.clone();
            async move {
                let now = Utc::now().naive_utc();

                // Check if thread already exists.
                let existing = shared_thread::Entity::find_by_id(id).one(&*tx).await?;

                match existing {
                    Some(existing) => {
                        // Only allow update if same user owns it.
                        if existing.user_id != user_id {
                            Err(anyhow!("Cannot update shared thread owned by another user"))?;
                        }

                        // Update existing record.
                        let mut active: shared_thread::ActiveModel = existing.into();
                        active.title = ActiveValue::Set(title);
                        active.data = ActiveValue::Set(data);
                        active.updated_at = ActiveValue::Set(now);
                        active.update(&*tx).await?;
                    }
                    None => {
                        // Create new record.
                        shared_thread::ActiveModel {
                            id: ActiveValue::Set(id),
                            user_id: ActiveValue::Set(user_id),
                            title: ActiveValue::Set(title),
                            data: ActiveValue::Set(data),
                            created_at: ActiveValue::Set(now),
                            updated_at: ActiveValue::Set(now),
                        }
                        .insert(&*tx)
                        .await?;
                    }
                }

                Ok(())
            }
        })
        .await
    }

    /// Get a shared thread by ID.
    pub async fn get_shared_thread(
        &self,
        share_id: SharedThreadId,
    ) -> Result<Option<(shared_thread::Model, String)>> {
        self.transaction(|tx| async move {
            let Some(thread) = shared_thread::Entity::find_by_id(share_id)
                .one(&*tx)
                .await?
            else {
                return Ok(None);
            };

            // Get the sharer's username.
            let user = user::Entity::find_by_id(thread.user_id).one(&*tx).await?;

            let username = user
                .map(|u| u.github_login)
                .unwrap_or_else(|| "Unknown".to_string());

            Ok(Some((thread, username)))
        })
        .await
    }
}
