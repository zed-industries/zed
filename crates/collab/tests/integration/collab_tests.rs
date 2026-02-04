use call::Room;
use client::ChannelId;
use gpui::{Entity, TestAppContext};

mod agent_sharing_tests;
mod channel_buffer_tests;
mod channel_guest_tests;
mod channel_tests;
mod db_tests;
mod editor_tests;
mod following_tests;
mod git_tests;
mod integration_tests;
mod notification_tests;
mod random_channel_buffer_tests;
mod random_project_collaboration_tests;
mod randomized_test_helpers;
mod remote_editing_collaboration_tests;
mod test_server;

pub use randomized_test_helpers::{
    RandomizedTest, TestError, UserTestPlan, run_randomized_test, save_randomized_test_plan,
};
pub use test_server::{TestClient, TestServer};

#[derive(Debug, Eq, PartialEq)]
struct RoomParticipants {
    remote: Vec<String>,
    pending: Vec<String>,
}

fn room_participants(room: &Entity<Room>, cx: &mut TestAppContext) -> RoomParticipants {
    room.read_with(cx, |room, _| {
        let mut remote = room
            .remote_participants()
            .values()
            .map(|participant| participant.user.github_login.clone().to_string())
            .collect::<Vec<_>>();
        let mut pending = room
            .pending_participants()
            .iter()
            .map(|user| user.github_login.clone().to_string())
            .collect::<Vec<_>>();
        remote.sort();
        pending.sort();
        RoomParticipants { remote, pending }
    })
}

fn channel_id(room: &Entity<Room>, cx: &mut TestAppContext) -> Option<ChannelId> {
    cx.read(|cx| room.read(cx).channel_id())
}

mod auth_token_tests {
    use collab::auth::{
        AccessTokenJson, MAX_ACCESS_TOKENS_TO_STORE, VerifyAccessTokenResult, create_access_token,
        verify_access_token,
    };
    use rand::prelude::*;
    use scrypt::Scrypt;
    use scrypt::password_hash::{PasswordHasher, SaltString};
    use sea_orm::EntityTrait;

    use collab::db::{Database, NewUserParams, UserId, access_token};
    use collab::*;

    #[gpui::test]
    async fn test_verify_access_token(cx: &mut gpui::TestAppContext) {
        let test_db = crate::db_tests::TestDb::sqlite(cx.executor());
        let db = test_db.db();

        let user = db
            .create_user(
                "example@example.com",
                None,
                false,
                NewUserParams {
                    github_login: "example".into(),
                    github_user_id: 1,
                },
            )
            .await
            .unwrap();

        let token = create_access_token(db, user.user_id, None).await.unwrap();
        assert!(matches!(
            verify_access_token(&token, user.user_id, db).await.unwrap(),
            VerifyAccessTokenResult {
                is_valid: true,
                impersonator_id: None,
            }
        ));

        let old_token = create_previous_access_token(user.user_id, None, db)
            .await
            .unwrap();

        let old_token_id = serde_json::from_str::<AccessTokenJson>(&old_token)
            .unwrap()
            .id;

        let hash = db
            .transaction(|tx| async move {
                Ok(access_token::Entity::find_by_id(old_token_id)
                    .one(&*tx)
                    .await?)
            })
            .await
            .unwrap()
            .unwrap()
            .hash;
        assert!(hash.starts_with("$scrypt$"));

        assert!(matches!(
            verify_access_token(&old_token, user.user_id, db)
                .await
                .unwrap(),
            VerifyAccessTokenResult {
                is_valid: true,
                impersonator_id: None,
            }
        ));

        let hash = db
            .transaction(|tx| async move {
                Ok(access_token::Entity::find_by_id(old_token_id)
                    .one(&*tx)
                    .await?)
            })
            .await
            .unwrap()
            .unwrap()
            .hash;
        assert!(hash.starts_with("$sha256$"));

        assert!(matches!(
            verify_access_token(&old_token, user.user_id, db)
                .await
                .unwrap(),
            VerifyAccessTokenResult {
                is_valid: true,
                impersonator_id: None,
            }
        ));

        assert!(matches!(
            verify_access_token(&token, user.user_id, db).await.unwrap(),
            VerifyAccessTokenResult {
                is_valid: true,
                impersonator_id: None,
            }
        ));
    }

    async fn create_previous_access_token(
        user_id: UserId,
        impersonated_user_id: Option<UserId>,
        db: &Database,
    ) -> Result<String> {
        let access_token = collab::auth::random_token();
        let access_token_hash = previous_hash_access_token(&access_token)?;
        let id = db
            .create_access_token(
                user_id,
                impersonated_user_id,
                &access_token_hash,
                MAX_ACCESS_TOKENS_TO_STORE,
            )
            .await?;
        Ok(serde_json::to_string(&AccessTokenJson {
            version: 1,
            id,
            token: access_token,
        })?)
    }

    #[expect(clippy::result_large_err)]
    fn previous_hash_access_token(token: &str) -> Result<String> {
        // Avoid slow hashing in debug mode.
        let params = if cfg!(debug_assertions) {
            scrypt::Params::new(1, 1, 1, scrypt::Params::RECOMMENDED_LEN).unwrap()
        } else {
            scrypt::Params::new(14, 8, 1, scrypt::Params::RECOMMENDED_LEN).unwrap()
        };

        Ok(Scrypt
            .hash_password_customized(
                token.as_bytes(),
                None,
                None,
                params,
                &SaltString::generate(PasswordHashRngCompat::new()),
            )
            .map_err(anyhow::Error::new)?
            .to_string())
    }

    // TODO: remove once we password_hash v0.6 is released.
    struct PasswordHashRngCompat(rand::rngs::ThreadRng);

    impl PasswordHashRngCompat {
        fn new() -> Self {
            Self(rand::rng())
        }
    }

    impl scrypt::password_hash::rand_core::RngCore for PasswordHashRngCompat {
        fn next_u32(&mut self) -> u32 {
            self.0.next_u32()
        }

        fn next_u64(&mut self) -> u64 {
            self.0.next_u64()
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            self.0.fill_bytes(dest);
        }

        fn try_fill_bytes(
            &mut self,
            dest: &mut [u8],
        ) -> Result<(), scrypt::password_hash::rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl scrypt::password_hash::rand_core::CryptoRng for PasswordHashRngCompat {}
}
