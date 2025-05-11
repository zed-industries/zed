use sea_orm::{DbErr, entity::prelude::*};
use serde::{Deserialize, Serialize};

use crate::id_type;

id_type!(BillingEventId);
id_type!(ModelId);
id_type!(ProviderId);
id_type!(RevokedAccessTokenId);
id_type!(UsageId);
id_type!(UsageMeasureId);
