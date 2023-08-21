use super::*;

impl Database {
    pub async fn get_contacts(&self, user_id: UserId) -> Result<Vec<Contact>> {
        #[derive(Debug, FromQueryResult)]
        struct ContactWithUserBusyStatuses {
            user_id_a: UserId,
            user_id_b: UserId,
            a_to_b: bool,
            accepted: bool,
            should_notify: bool,
            user_a_busy: bool,
            user_b_busy: bool,
        }

        self.transaction(|tx| async move {
            let user_a_participant = Alias::new("user_a_participant");
            let user_b_participant = Alias::new("user_b_participant");
            let mut db_contacts = contact::Entity::find()
                .column_as(
                    Expr::tbl(user_a_participant.clone(), room_participant::Column::Id)
                        .is_not_null(),
                    "user_a_busy",
                )
                .column_as(
                    Expr::tbl(user_b_participant.clone(), room_participant::Column::Id)
                        .is_not_null(),
                    "user_b_busy",
                )
                .filter(
                    contact::Column::UserIdA
                        .eq(user_id)
                        .or(contact::Column::UserIdB.eq(user_id)),
                )
                .join_as(
                    JoinType::LeftJoin,
                    contact::Relation::UserARoomParticipant.def(),
                    user_a_participant,
                )
                .join_as(
                    JoinType::LeftJoin,
                    contact::Relation::UserBRoomParticipant.def(),
                    user_b_participant,
                )
                .into_model::<ContactWithUserBusyStatuses>()
                .stream(&*tx)
                .await?;

            let mut contacts = Vec::new();
            while let Some(db_contact) = db_contacts.next().await {
                let db_contact = db_contact?;
                if db_contact.user_id_a == user_id {
                    if db_contact.accepted {
                        contacts.push(Contact::Accepted {
                            user_id: db_contact.user_id_b,
                            should_notify: db_contact.should_notify && db_contact.a_to_b,
                            busy: db_contact.user_b_busy,
                        });
                    } else if db_contact.a_to_b {
                        contacts.push(Contact::Outgoing {
                            user_id: db_contact.user_id_b,
                        })
                    } else {
                        contacts.push(Contact::Incoming {
                            user_id: db_contact.user_id_b,
                            should_notify: db_contact.should_notify,
                        });
                    }
                } else if db_contact.accepted {
                    contacts.push(Contact::Accepted {
                        user_id: db_contact.user_id_a,
                        should_notify: db_contact.should_notify && !db_contact.a_to_b,
                        busy: db_contact.user_a_busy,
                    });
                } else if db_contact.a_to_b {
                    contacts.push(Contact::Incoming {
                        user_id: db_contact.user_id_a,
                        should_notify: db_contact.should_notify,
                    });
                } else {
                    contacts.push(Contact::Outgoing {
                        user_id: db_contact.user_id_a,
                    });
                }
            }

            contacts.sort_unstable_by_key(|contact| contact.user_id());

            Ok(contacts)
        })
        .await
    }

    pub async fn is_user_busy(&self, user_id: UserId) -> Result<bool> {
        self.transaction(|tx| async move {
            let participant = room_participant::Entity::find()
                .filter(room_participant::Column::UserId.eq(user_id))
                .one(&*tx)
                .await?;
            Ok(participant.is_some())
        })
        .await
    }

    pub async fn has_contact(&self, user_id_1: UserId, user_id_2: UserId) -> Result<bool> {
        self.transaction(|tx| async move {
            let (id_a, id_b) = if user_id_1 < user_id_2 {
                (user_id_1, user_id_2)
            } else {
                (user_id_2, user_id_1)
            };

            Ok(contact::Entity::find()
                .filter(
                    contact::Column::UserIdA
                        .eq(id_a)
                        .and(contact::Column::UserIdB.eq(id_b))
                        .and(contact::Column::Accepted.eq(true)),
                )
                .one(&*tx)
                .await?
                .is_some())
        })
        .await
    }

    pub async fn send_contact_request(&self, sender_id: UserId, receiver_id: UserId) -> Result<()> {
        self.transaction(|tx| async move {
            let (id_a, id_b, a_to_b) = if sender_id < receiver_id {
                (sender_id, receiver_id, true)
            } else {
                (receiver_id, sender_id, false)
            };

            let rows_affected = contact::Entity::insert(contact::ActiveModel {
                user_id_a: ActiveValue::set(id_a),
                user_id_b: ActiveValue::set(id_b),
                a_to_b: ActiveValue::set(a_to_b),
                accepted: ActiveValue::set(false),
                should_notify: ActiveValue::set(true),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::columns([contact::Column::UserIdA, contact::Column::UserIdB])
                    .values([
                        (contact::Column::Accepted, true.into()),
                        (contact::Column::ShouldNotify, false.into()),
                    ])
                    .action_and_where(
                        contact::Column::Accepted.eq(false).and(
                            contact::Column::AToB
                                .eq(a_to_b)
                                .and(contact::Column::UserIdA.eq(id_b))
                                .or(contact::Column::AToB
                                    .ne(a_to_b)
                                    .and(contact::Column::UserIdA.eq(id_a))),
                        ),
                    )
                    .to_owned(),
            )
            .exec_without_returning(&*tx)
            .await?;

            if rows_affected == 1 {
                Ok(())
            } else {
                Err(anyhow!("contact already requested"))?
            }
        })
        .await
    }

    /// Returns a bool indicating whether the removed contact had originally accepted or not
    ///
    /// Deletes the contact identified by the requester and responder ids, and then returns
    /// whether the deleted contact had originally accepted or was a pending contact request.
    ///
    /// # Arguments
    ///
    /// * `requester_id` - The user that initiates this request
    /// * `responder_id` - The user that will be removed
    pub async fn remove_contact(&self, requester_id: UserId, responder_id: UserId) -> Result<bool> {
        self.transaction(|tx| async move {
            let (id_a, id_b) = if responder_id < requester_id {
                (responder_id, requester_id)
            } else {
                (requester_id, responder_id)
            };

            let contact = contact::Entity::find()
                .filter(
                    contact::Column::UserIdA
                        .eq(id_a)
                        .and(contact::Column::UserIdB.eq(id_b)),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("no such contact"))?;

            contact::Entity::delete_by_id(contact.id).exec(&*tx).await?;
            Ok(contact.accepted)
        })
        .await
    }

    pub async fn dismiss_contact_notification(
        &self,
        user_id: UserId,
        contact_user_id: UserId,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            let (id_a, id_b, a_to_b) = if user_id < contact_user_id {
                (user_id, contact_user_id, true)
            } else {
                (contact_user_id, user_id, false)
            };

            let result = contact::Entity::update_many()
                .set(contact::ActiveModel {
                    should_notify: ActiveValue::set(false),
                    ..Default::default()
                })
                .filter(
                    contact::Column::UserIdA
                        .eq(id_a)
                        .and(contact::Column::UserIdB.eq(id_b))
                        .and(
                            contact::Column::AToB
                                .eq(a_to_b)
                                .and(contact::Column::Accepted.eq(true))
                                .or(contact::Column::AToB
                                    .ne(a_to_b)
                                    .and(contact::Column::Accepted.eq(false))),
                        ),
                )
                .exec(&*tx)
                .await?;
            if result.rows_affected == 0 {
                Err(anyhow!("no such contact request"))?
            } else {
                Ok(())
            }
        })
        .await
    }

    pub async fn respond_to_contact_request(
        &self,
        responder_id: UserId,
        requester_id: UserId,
        accept: bool,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            let (id_a, id_b, a_to_b) = if responder_id < requester_id {
                (responder_id, requester_id, false)
            } else {
                (requester_id, responder_id, true)
            };
            let rows_affected = if accept {
                let result = contact::Entity::update_many()
                    .set(contact::ActiveModel {
                        accepted: ActiveValue::set(true),
                        should_notify: ActiveValue::set(true),
                        ..Default::default()
                    })
                    .filter(
                        contact::Column::UserIdA
                            .eq(id_a)
                            .and(contact::Column::UserIdB.eq(id_b))
                            .and(contact::Column::AToB.eq(a_to_b)),
                    )
                    .exec(&*tx)
                    .await?;
                result.rows_affected
            } else {
                let result = contact::Entity::delete_many()
                    .filter(
                        contact::Column::UserIdA
                            .eq(id_a)
                            .and(contact::Column::UserIdB.eq(id_b))
                            .and(contact::Column::AToB.eq(a_to_b))
                            .and(contact::Column::Accepted.eq(false)),
                    )
                    .exec(&*tx)
                    .await?;

                result.rows_affected
            };

            if rows_affected == 1 {
                Ok(())
            } else {
                Err(anyhow!("no such contact request"))?
            }
        })
        .await
    }
}
