use super::*;

impl Database {
    pub async fn create_server(&self, environment: &str) -> Result<ServerId> {
        self.transaction(|tx| async move {
            let server = server::ActiveModel {
                environment: ActiveValue::set(environment.into()),
                ..Default::default()
            }
            .insert(&*tx)
            .await?;
            Ok(server.id)
        })
        .await
    }

    pub async fn stale_room_ids(
        &self,
        environment: &str,
        new_server_id: ServerId,
    ) -> Result<Vec<RoomId>> {
        self.transaction(|tx| async move {
            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            enum QueryAs {
                RoomId,
            }

            let stale_server_epochs = self
                .stale_server_ids(environment, new_server_id, &tx)
                .await?;
            Ok(room_participant::Entity::find()
                .select_only()
                .column(room_participant::Column::RoomId)
                .distinct()
                .filter(
                    room_participant::Column::AnsweringConnectionServerId
                        .is_in(stale_server_epochs),
                )
                .into_values::<_, QueryAs>()
                .all(&*tx)
                .await?)
        })
        .await
    }

    pub async fn delete_stale_servers(
        &self,
        environment: &str,
        new_server_id: ServerId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            server::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(server::Column::Environment.eq(environment))
                        .add(server::Column::Id.ne(new_server_id)),
                )
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    async fn stale_server_ids(
        &self,
        environment: &str,
        new_server_id: ServerId,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<ServerId>> {
        let stale_servers = server::Entity::find()
            .filter(
                Condition::all()
                    .add(server::Column::Environment.eq(environment))
                    .add(server::Column::Id.ne(new_server_id)),
            )
            .all(&*tx)
            .await?;
        Ok(stale_servers.into_iter().map(|server| server.id).collect())
    }
}
