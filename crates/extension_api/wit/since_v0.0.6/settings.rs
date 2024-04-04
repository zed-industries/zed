use std::num::NonZeroU32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageSettings {
    pub tab_size: NonZeroU32,
}
