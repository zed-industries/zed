use db::kvp::KEY_VALUE_STORE;
use util::ResultExt;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Session {
    session_id: String,
    old_session_id: Option<String>,
}

impl Session {
    pub async fn new() -> Self {
        let key_name = "session_id".to_string();

        let old_session_id = KEY_VALUE_STORE.read_kvp(&key_name).ok().flatten();

        let session_id = Uuid::new_v4().to_string();

        KEY_VALUE_STORE
            .write_kvp(key_name, session_id.clone())
            .await
            .log_err();

        Self {
            session_id,
            old_session_id,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            old_session_id: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.session_id
    }
    pub fn last_session_id(&self) -> Option<&str> {
        self.old_session_id.as_deref()
    }
}
