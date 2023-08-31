use super::*;
use hyper::StatusCode;

impl Database {
    pub async fn create_invite_from_code(
        &self,
        code: &str,
        email_address: &str,
        device_id: Option<&str>,
        added_to_mailing_list: bool,
    ) -> Result<Invite> {
        self.transaction(|tx| async move {
            let existing_user = user::Entity::find()
                .filter(user::Column::EmailAddress.eq(email_address))
                .one(&*tx)
                .await?;

            if existing_user.is_some() {
                Err(anyhow!("email address is already in use"))?;
            }

            let inviting_user_with_invites = match user::Entity::find()
                .filter(
                    user::Column::InviteCode
                        .eq(code)
                        .and(user::Column::InviteCount.gt(0)),
                )
                .one(&*tx)
                .await?
            {
                Some(inviting_user) => inviting_user,
                None => {
                    return Err(Error::Http(
                        StatusCode::UNAUTHORIZED,
                        "unable to find an invite code with invites remaining".to_string(),
                    ))?
                }
            };
            user::Entity::update_many()
                .filter(
                    user::Column::Id
                        .eq(inviting_user_with_invites.id)
                        .and(user::Column::InviteCount.gt(0)),
                )
                .col_expr(
                    user::Column::InviteCount,
                    Expr::col(user::Column::InviteCount).sub(1),
                )
                .exec(&*tx)
                .await?;

            let signup = signup::Entity::insert(signup::ActiveModel {
                email_address: ActiveValue::set(email_address.into()),
                email_confirmation_code: ActiveValue::set(random_email_confirmation_code()),
                email_confirmation_sent: ActiveValue::set(false),
                inviting_user_id: ActiveValue::set(Some(inviting_user_with_invites.id)),
                platform_linux: ActiveValue::set(false),
                platform_mac: ActiveValue::set(false),
                platform_windows: ActiveValue::set(false),
                platform_unknown: ActiveValue::set(true),
                device_id: ActiveValue::set(device_id.map(|device_id| device_id.into())),
                added_to_mailing_list: ActiveValue::set(added_to_mailing_list),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(signup::Column::EmailAddress)
                    .update_column(signup::Column::InvitingUserId)
                    .to_owned(),
            )
            .exec_with_returning(&*tx)
            .await?;

            Ok(Invite {
                email_address: signup.email_address,
                email_confirmation_code: signup.email_confirmation_code,
            })
        })
        .await
    }

    pub async fn create_user_from_invite(
        &self,
        invite: &Invite,
        user: NewUserParams,
    ) -> Result<Option<NewUserResult>> {
        self.transaction(|tx| async {
            let tx = tx;
            let signup = signup::Entity::find()
                .filter(
                    signup::Column::EmailAddress
                        .eq(invite.email_address.as_str())
                        .and(
                            signup::Column::EmailConfirmationCode
                                .eq(invite.email_confirmation_code.as_str()),
                        ),
                )
                .one(&*tx)
                .await?
                .ok_or_else(|| Error::Http(StatusCode::NOT_FOUND, "no such invite".to_string()))?;

            if signup.user_id.is_some() {
                return Ok(None);
            }

            let user = user::Entity::insert(user::ActiveModel {
                email_address: ActiveValue::set(Some(invite.email_address.clone())),
                github_login: ActiveValue::set(user.github_login.clone()),
                github_user_id: ActiveValue::set(Some(user.github_user_id)),
                admin: ActiveValue::set(false),
                invite_count: ActiveValue::set(user.invite_count),
                invite_code: ActiveValue::set(Some(random_invite_code())),
                metrics_id: ActiveValue::set(Uuid::new_v4()),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(user::Column::GithubLogin)
                    .update_columns([
                        user::Column::EmailAddress,
                        user::Column::GithubUserId,
                        user::Column::Admin,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&*tx)
            .await?;

            let mut signup = signup.into_active_model();
            signup.user_id = ActiveValue::set(Some(user.id));
            let signup = signup.update(&*tx).await?;

            if let Some(inviting_user_id) = signup.inviting_user_id {
                let (user_id_a, user_id_b, a_to_b) = if inviting_user_id < user.id {
                    (inviting_user_id, user.id, true)
                } else {
                    (user.id, inviting_user_id, false)
                };

                contact::Entity::insert(contact::ActiveModel {
                    user_id_a: ActiveValue::set(user_id_a),
                    user_id_b: ActiveValue::set(user_id_b),
                    a_to_b: ActiveValue::set(a_to_b),
                    should_notify: ActiveValue::set(true),
                    accepted: ActiveValue::set(true),
                    ..Default::default()
                })
                .on_conflict(OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&*tx)
                .await?;
            }

            Ok(Some(NewUserResult {
                user_id: user.id,
                metrics_id: user.metrics_id.to_string(),
                inviting_user_id: signup.inviting_user_id,
                signup_device_id: signup.device_id,
            }))
        })
        .await
    }

    pub async fn set_invite_count_for_user(&self, id: UserId, count: i32) -> Result<()> {
        self.transaction(|tx| async move {
            if count > 0 {
                user::Entity::update_many()
                    .filter(
                        user::Column::Id
                            .eq(id)
                            .and(user::Column::InviteCode.is_null()),
                    )
                    .set(user::ActiveModel {
                        invite_code: ActiveValue::set(Some(random_invite_code())),
                        ..Default::default()
                    })
                    .exec(&*tx)
                    .await?;
            }

            user::Entity::update_many()
                .filter(user::Column::Id.eq(id))
                .set(user::ActiveModel {
                    invite_count: ActiveValue::set(count),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    pub async fn get_invite_code_for_user(&self, id: UserId) -> Result<Option<(String, i32)>> {
        self.transaction(|tx| async move {
            match user::Entity::find_by_id(id).one(&*tx).await? {
                Some(user) if user.invite_code.is_some() => {
                    Ok(Some((user.invite_code.unwrap(), user.invite_count)))
                }
                _ => Ok(None),
            }
        })
        .await
    }

    pub async fn get_user_for_invite_code(&self, code: &str) -> Result<User> {
        self.transaction(|tx| async move {
            user::Entity::find()
                .filter(user::Column::InviteCode.eq(code))
                .one(&*tx)
                .await?
                .ok_or_else(|| {
                    Error::Http(
                        StatusCode::NOT_FOUND,
                        "that invite code does not exist".to_string(),
                    )
                })
        })
        .await
    }

    pub async fn create_signup(&self, signup: &NewSignup) -> Result<()> {
        self.transaction(|tx| async move {
            signup::Entity::insert(signup::ActiveModel {
                email_address: ActiveValue::set(signup.email_address.clone()),
                email_confirmation_code: ActiveValue::set(random_email_confirmation_code()),
                email_confirmation_sent: ActiveValue::set(false),
                platform_mac: ActiveValue::set(signup.platform_mac),
                platform_windows: ActiveValue::set(signup.platform_windows),
                platform_linux: ActiveValue::set(signup.platform_linux),
                platform_unknown: ActiveValue::set(false),
                editor_features: ActiveValue::set(Some(signup.editor_features.clone())),
                programming_languages: ActiveValue::set(Some(signup.programming_languages.clone())),
                device_id: ActiveValue::set(signup.device_id.clone()),
                added_to_mailing_list: ActiveValue::set(signup.added_to_mailing_list),
                ..Default::default()
            })
            .on_conflict(
                OnConflict::column(signup::Column::EmailAddress)
                    .update_columns([
                        signup::Column::PlatformMac,
                        signup::Column::PlatformWindows,
                        signup::Column::PlatformLinux,
                        signup::Column::EditorFeatures,
                        signup::Column::ProgrammingLanguages,
                        signup::Column::DeviceId,
                        signup::Column::AddedToMailingList,
                    ])
                    .to_owned(),
            )
            .exec(&*tx)
            .await?;
            Ok(())
        })
        .await
    }

    pub async fn get_signup(&self, email_address: &str) -> Result<signup::Model> {
        self.transaction(|tx| async move {
            let signup = signup::Entity::find()
                .filter(signup::Column::EmailAddress.eq(email_address))
                .one(&*tx)
                .await?
                .ok_or_else(|| {
                    anyhow!("signup with email address {} doesn't exist", email_address)
                })?;

            Ok(signup)
        })
        .await
    }

    pub async fn get_waitlist_summary(&self) -> Result<WaitlistSummary> {
        self.transaction(|tx| async move {
            let query = "
                SELECT
                    COUNT(*) as count,
                    COALESCE(SUM(CASE WHEN platform_linux THEN 1 ELSE 0 END), 0) as linux_count,
                    COALESCE(SUM(CASE WHEN platform_mac THEN 1 ELSE 0 END), 0) as mac_count,
                    COALESCE(SUM(CASE WHEN platform_windows THEN 1 ELSE 0 END), 0) as windows_count,
                    COALESCE(SUM(CASE WHEN platform_unknown THEN 1 ELSE 0 END), 0) as unknown_count
                FROM (
                    SELECT *
                    FROM signups
                    WHERE
                        NOT email_confirmation_sent
                ) AS unsent
            ";
            Ok(
                WaitlistSummary::find_by_statement(Statement::from_sql_and_values(
                    self.pool.get_database_backend(),
                    query.into(),
                    vec![],
                ))
                .one(&*tx)
                .await?
                .ok_or_else(|| anyhow!("invalid result"))?,
            )
        })
        .await
    }

    pub async fn record_sent_invites(&self, invites: &[Invite]) -> Result<()> {
        let emails = invites
            .iter()
            .map(|s| s.email_address.as_str())
            .collect::<Vec<_>>();
        self.transaction(|tx| async {
            let tx = tx;
            signup::Entity::update_many()
                .filter(signup::Column::EmailAddress.is_in(emails.iter().copied()))
                .set(signup::ActiveModel {
                    email_confirmation_sent: ActiveValue::set(true),
                    ..Default::default()
                })
                .exec(&*tx)
                .await?;
            Ok(())
        })
        .await
    }

    pub async fn get_unsent_invites(&self, count: usize) -> Result<Vec<Invite>> {
        self.transaction(|tx| async move {
            Ok(signup::Entity::find()
                .select_only()
                .column(signup::Column::EmailAddress)
                .column(signup::Column::EmailConfirmationCode)
                .filter(
                    signup::Column::EmailConfirmationSent.eq(false).and(
                        signup::Column::PlatformMac
                            .eq(true)
                            .or(signup::Column::PlatformUnknown.eq(true)),
                    ),
                )
                .order_by_asc(signup::Column::CreatedAt)
                .limit(count as u64)
                .into_model()
                .all(&*tx)
                .await?)
        })
        .await
    }
}

fn random_invite_code() -> String {
    nanoid::nanoid!(16)
}

fn random_email_confirmation_code() -> String {
    nanoid::nanoid!(64)
}
