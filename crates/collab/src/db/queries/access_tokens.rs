use super::*;

impl Database {
    pub async fn create_access_token(
        &self,
        user_id: UserId,
        access_token_hash: &str,
        max_access_token_count: usize,
    ) -> Result<AccessTokenId> {
        self.transaction(|tx| async {
            let tx = tx;

            let token = access_token::ActiveModel {
                user_id: ActiveValue::set(user_id),
                hash: ActiveValue::set(access_token_hash.into()),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;

            access_token::Entity::delete_many()
                .filter(
                    access_token::Column::Id.in_subquery(
                        Query::select()
                            .column(access_token::Column::Id)
                            .from(access_token::Entity)
                            .and_where(access_token::Column::UserId.eq(user_id))
                            .order_by(access_token::Column::Id, sea_orm::Order::Desc)
                            .limit(10000)
                            .offset(max_access_token_count as u64)
                            .to_owned(),
                    ),
                )
                .exec(&*tx)
                .await?;
            Ok(token.id)
        })
        .await
    }

    pub async fn get_access_token(
        &self,
        access_token_id: AccessTokenId,
    ) -> Result<access_token::Model> {
        self.transaction(|tx| async move {
            Ok(access_token::Entity::find_by_id(access_token_id)
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such access token"))?)
        })
        .await
    }
}
