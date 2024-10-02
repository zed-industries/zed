use sea_orm::{entity::prelude::*, DbErr};
use serde::{Deserialize, Serialize};

use crate::id_type;

id_type!(ModelId);
id_type!(ProviderId);
id_type!(UsageId);
id_type!(UsageMeasureId);
id_type!(RevokedAccessTokenId);
