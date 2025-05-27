use std::str::FromStr;
use strum::IntoEnumIterator as _;

use super::*;

impl LlmDatabase {
    pub async fn initialize_usage_measures(&mut self) -> Result<()> {
        let all_measures = self
            .transaction(|tx| async move {
                let existing_measures = usage_measure::Entity::find().all(&*tx).await?;

                let new_measures = UsageMeasure::iter()
                    .filter(|measure| {
                        !existing_measures
                            .iter()
                            .any(|m| m.name == measure.to_string())
                    })
                    .map(|measure| usage_measure::ActiveModel {
                        name: ActiveValue::set(measure.to_string()),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>();

                if !new_measures.is_empty() {
                    usage_measure::Entity::insert_many(new_measures)
                        .exec(&*tx)
                        .await?;
                }

                Ok(usage_measure::Entity::find().all(&*tx).await?)
            })
            .await?;

        self.usage_measure_ids = all_measures
            .into_iter()
            .filter_map(|measure| {
                UsageMeasure::from_str(&measure.name)
                    .ok()
                    .map(|um| (um, measure.id))
            })
            .collect();
        Ok(())
    }
}
