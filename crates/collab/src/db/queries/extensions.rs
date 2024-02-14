use super::*;

impl Database {
    pub async fn get_extensions(
        &self,
        filter: Option<&str>,
        count: usize,
    ) -> Result<Vec<(extension::Model, extension_version::Model)>> {
        self.transaction(|tx| async move {
            let extensions = extension::Entity::find()
                .filter(filter)
            //
        })
        .await
    }
}
