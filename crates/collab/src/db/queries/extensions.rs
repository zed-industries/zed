use super::*;

impl Database {
    pub async fn get_extensions(
        &self,
        filter: Option<&str>,
        limit: usize,
    ) -> Result<Vec<(extension::Model, extension_version::Model)>> {
        self.transaction(|tx| async move {
            let mut condition = Condition::all();
            if let Some(filter) = filter {
                let fuzzy_name_filter = Self::fuzzy_like_string(filter);
                condition = condition.add(Expr::cust_with_expr("name ILIKE $1", fuzzy_name_filter));
            }

            let extensions = extension::Entity::find()
                .filter(condition)
                .order_by_desc(extension::Column::TotalDownloadCount)
                .limit(Some(limit as u64))
                .find_also_related(extension_version::Entity)
                .all(&*tx)
                .await?;

            Ok(extensions
                .into_iter()
                .filter_map(|(extension, latest_version)| Some((extension, latest_version?)))
                .collect())
        })
        .await
    }

    pub async fn get_known_extension_versions<'a>(
        &self,
    ) -> Result<HashMap<String, HashSet<String>>> {
        self.transaction(|tx| async move {
            let mut extension_external_ids_by_id = HashMap::default();

            let mut rows = extension::Entity::find().stream(&*tx).await?;
            while let Some(row) = rows.next().await {
                let row = row?;
                extension_external_ids_by_id.insert(row.id, row.external_id);
            }
            drop(rows);

            let mut known_versions_by_extension_id: HashMap<String, HashSet<String>> =
                HashMap::default();
            let mut rows = extension_version::Entity::find().stream(&*tx).await?;
            while let Some(row) = rows.next().await {
                let row = row?;

                let Some(extension_id) = extension_external_ids_by_id.get(&row.extension_id) else {
                    continue;
                };

                known_versions_by_extension_id
                    .entry(extension_id.clone())
                    .or_default()
                    .insert(row.version);
            }
            drop(rows);

            Ok(known_versions_by_extension_id)
        })
        .await
    }

    pub async fn insert_extension_versions(
        &self,
        versions_by_extension_id: &HashMap<&str, Vec<NewExtensionVersion>>,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            let mut extension_ids = Vec::new();
            for (external_id, versions) in versions_by_extension_id {
                if versions.is_empty() {
                    continue;
                }

                let latest_version = versions
                    .iter()
                    .max_by_key(|version| &version.version)
                    .unwrap();

                let mut extension = extension::Entity::insert(extension::ActiveModel {
                    name: ActiveValue::Set(latest_version.name.clone()),
                    external_id: ActiveValue::Set(external_id.to_string()),
                    id: ActiveValue::NotSet,
                    latest_version: ActiveValue::Set(latest_version.version.to_string()),
                    total_download_count: ActiveValue::NotSet,
                })
                .on_conflict(
                    OnConflict::new()
                        .update_column(extension::Column::ExternalId)
                        .to_owned(),
                )
                .exec_with_returning(&*tx)
                .await?;
                extension_ids.push(extension.id);

                extension_version::Entity::insert_many(versions.iter().map(|version| {
                    extension_version::ActiveModel {
                        extension_id: ActiveValue::Set(extension.id),
                        published_at: ActiveValue::Set(version.published_at),
                        version: ActiveValue::Set(version.version.to_string()),
                        authors: ActiveValue::Set(version.authors.join(", ")),
                        repository: ActiveValue::Set(version.repository.clone()),
                        description: ActiveValue::Set(version.description.clone()),
                        download_count: ActiveValue::NotSet,
                    }
                }))
                .on_conflict(OnConflict::new().do_nothing().to_owned())
                .exec(&*tx)
                .await?;

                if let Ok(db_version) = semver::Version::parse(&extension.latest_version) {
                    if db_version >= latest_version.version {
                        continue;
                    }
                }

                extension.latest_version = latest_version.version.to_string();
                extension.name = latest_version.name.clone();
                extension::Entity::update(extension.into_active_model())
                    .exec(&*tx)
                    .await?;
            }

            Ok(())
        })
        .await
    }
}
